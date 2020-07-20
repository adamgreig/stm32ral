#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! JPEG codec
//!
//! Used by: stm32f765, stm32f7x7, stm32f7x9

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// JPEG codec configuration register 0
pub mod JPEG_CONFR0 {

    /// Start
    pub mod START {
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

/// JPEG codec configuration register 1
pub mod JPEG_CONFR1 {

    /// Number of color components
    pub mod NF {
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

    /// Decoding Enable
    pub mod DE {
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

    /// Color Space
    pub mod COLORSPACE {
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

    /// Number of components for Scan
    pub mod NS {
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

    /// Header Processing
    pub mod HDR {
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

    /// Y Size
    pub mod YSIZE {
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

/// JPEG codec configuration register 2
pub mod JPEG_CONFR2 {

    /// Number of MCU
    pub mod NMCU {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (26 bits: 0x3ffffff << 0)
        pub const mask: u32 = 0x3ffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// JPEG codec configuration register 3
pub mod JPEG_CONFR3 {

    /// X size
    pub mod XSIZE {
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

/// JPEG codec configuration register 4
pub mod JPEG_CONFR4 {

    /// Huffman DC
    pub mod HD {
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

    /// Huffman AC
    pub mod HA {
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

    /// Quantization Table
    pub mod QT {
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

    /// Number of Block
    pub mod NB {
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

    /// Vertical Sampling Factor
    pub mod VSF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Horizontal Sampling Factor
    pub mod HSF {
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
}

/// JPEG codec configuration register 5
pub mod JPEG_CONFR5 {
    pub use super::JPEG_CONFR4::HA;
    pub use super::JPEG_CONFR4::HD;
    pub use super::JPEG_CONFR4::HSF;
    pub use super::JPEG_CONFR4::NB;
    pub use super::JPEG_CONFR4::QT;
    pub use super::JPEG_CONFR4::VSF;
}

/// JPEG codec configuration register 6
pub mod JPEG_CONFR6 {
    pub use super::JPEG_CONFR4::HA;
    pub use super::JPEG_CONFR4::HD;
    pub use super::JPEG_CONFR4::HSF;
    pub use super::JPEG_CONFR4::NB;
    pub use super::JPEG_CONFR4::QT;
    pub use super::JPEG_CONFR4::VSF;
}

/// JPEG codec configuration register 7
pub mod JPEG_CONFR7 {
    pub use super::JPEG_CONFR4::HA;
    pub use super::JPEG_CONFR4::HD;
    pub use super::JPEG_CONFR4::HSF;
    pub use super::JPEG_CONFR4::NB;
    pub use super::JPEG_CONFR4::QT;
    pub use super::JPEG_CONFR4::VSF;
}

/// JPEG control register
pub mod JPEG_CR {

    /// JPEG Core Enable
    pub mod JCEN {
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

    /// Input FIFO Threshold Interrupt Enable
    pub mod IFTIE {
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

    /// Input FIFO Not Full Interrupt Enable
    pub mod IFNFIE {
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

    /// Output FIFO Threshold Interrupt Enable
    pub mod OFTIE {
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

    /// Output FIFO Not Empty Interrupt Enable
    pub mod OFNEIE {
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

    /// End of Conversion Interrupt Enable
    pub mod EOCIE {
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

    /// Header Parsing Done Interrupt Enable
    pub mod HPDIE {
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

    /// Input DMA Enable
    pub mod IDMAEN {
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

    /// Output DMA Enable
    pub mod ODMAEN {
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

    /// Input FIFO Flush
    pub mod IFF {
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

    /// Output FIFO Flush
    pub mod OFF {
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

/// JPEG status register
pub mod JPEG_SR {

    /// Input FIFO Threshold Flag
    pub mod IFTF {
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

    /// Input FIFO Not Full Flag
    pub mod IFNFF {
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

    /// Output FIFO Threshold Flag
    pub mod OFTF {
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

    /// Output FIFO Not Empty Flag
    pub mod OFNEF {
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

    /// End of Conversion Flag
    pub mod EOCF {
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

    /// Header Parsing Done Flag
    pub mod HPDF {
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

    /// Codec Operation Flag
    pub mod COF {
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

/// JPEG clear flag register
pub mod JPEG_CFR {

    /// Clear End of Conversion Flag
    pub mod CEOCF {
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

    /// Clear Header Parsing Done Flag
    pub mod CHPDF {
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
}

/// JPEG data input register
pub mod JPEG_DIR {

    /// Data Input FIFO
    pub mod DATAIN {
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

/// JPEG data output register
pub mod JPEG_DOR {

    /// Data Output FIFO
    pub mod DATAOUT {
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

/// JPEG quantization tables
pub mod QMEM0_0 {

    /// QMem RAM
    pub mod QMem_RAM {
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

/// JPEG quantization tables
pub mod QMEM0_1 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM0_2 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM0_3 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM0_4 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM0_5 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM0_6 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM0_7 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM0_8 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM0_9 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM0_10 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM0_11 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM0_12 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM0_13 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM0_14 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM0_15 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_0 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_1 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_2 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_3 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_4 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_5 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_6 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_7 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_8 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_9 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_10 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_11 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_12 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_13 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_14 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM1_15 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_0 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_1 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_2 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_3 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_4 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_5 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_6 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_7 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_8 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_9 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_10 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_11 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_12 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_13 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_14 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM2_15 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_0 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_1 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_2 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_3 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_4 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_5 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_6 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_7 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_8 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_9 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_10 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_11 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_12 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_13 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_14 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM3_15 {
    pub use super::QMEM0_0::QMem_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN_0 {

    /// HuffMin RAM
    pub mod HuffMin_RAM {
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

/// JPEG HuffMin tables
pub mod HUFFMIN_1 {
    pub use super::HUFFMIN_0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN_2 {
    pub use super::HUFFMIN_0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN_3 {
    pub use super::HUFFMIN_0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN_4 {
    pub use super::HUFFMIN_0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN_5 {
    pub use super::HUFFMIN_0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN_6 {
    pub use super::HUFFMIN_0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN_7 {
    pub use super::HUFFMIN_0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN_8 {
    pub use super::HUFFMIN_0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN_9 {
    pub use super::HUFFMIN_0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN_10 {
    pub use super::HUFFMIN_0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN_11 {
    pub use super::HUFFMIN_0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN_12 {
    pub use super::HUFFMIN_0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN_13 {
    pub use super::HUFFMIN_0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN_14 {
    pub use super::HUFFMIN_0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN_15 {
    pub use super::HUFFMIN_0::HuffMin_RAM;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE0 {

    /// HuffBase RAM
    pub mod HuffBase_RAM_0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HuffBase RAM
    pub mod HuffBase_RAM_1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (9 bits: 0x1ff << 16)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// JPEG HuffSymb tables
pub mod HUFFBASE1 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE2 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE3 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE4 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE5 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE6 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE7 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE8 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE9 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE10 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE11 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE12 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE13 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE14 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE15 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE16 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE17 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE18 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE19 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE20 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE21 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE22 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE23 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE24 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE25 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE26 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE27 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE28 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE29 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE30 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HuffSymb tables
pub mod HUFFBASE31 {
    pub use super::HUFFBASE0::HuffBase_RAM_0;
    pub use super::HUFFBASE0::HuffBase_RAM_1;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB0 {

    /// DHTSymb RAM
    pub mod HuffSymb_RAM {
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

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB1 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB2 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB3 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB4 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB5 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB6 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB7 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB8 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB9 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB10 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB11 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB12 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB13 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB14 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB15 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB16 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB17 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB18 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB19 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB20 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB21 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB22 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB23 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB24 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB25 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB26 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB27 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB28 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB29 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB30 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB31 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB32 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB33 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB34 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB35 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB36 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB37 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB38 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB39 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB40 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB41 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB42 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB43 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB44 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB45 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB46 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB47 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB48 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB49 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB50 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB51 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB52 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB53 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB54 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB55 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB56 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB57 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB58 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB59 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB60 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB61 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB62 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB63 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB64 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB65 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB66 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB67 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB68 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB69 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB70 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB71 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB72 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB73 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB74 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB75 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB76 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB77 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB78 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB79 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB80 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB81 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB82 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG HUFFSYMB tables
pub mod HUFFSYMB83 {
    pub use super::HUFFSYMB0::HuffSymb_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM0 {

    /// DHTMem RAM
    pub mod DHTMem_RAM {
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

/// JPEG DHTMem tables
pub mod DHTMEM2 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM3 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM4 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM5 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM6 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM7 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM8 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM9 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM10 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM11 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM12 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM13 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM14 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM15 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM16 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM17 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM18 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM19 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM20 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM21 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM22 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM23 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM24 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM25 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM26 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM27 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM28 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM29 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM30 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM31 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM32 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM33 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM34 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM35 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM36 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM37 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM38 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM39 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM40 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM41 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM42 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM43 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM44 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM45 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM46 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM47 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM48 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM49 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM50 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM51 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM52 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM53 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM54 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM55 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM56 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM57 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM58 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM59 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM60 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM61 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM62 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM63 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM64 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM65 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM66 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM67 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM68 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM69 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM70 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM71 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM72 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM73 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM74 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM75 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM76 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM77 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM78 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM79 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM80 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM81 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM82 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM83 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM84 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM85 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM86 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM87 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM88 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM89 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM90 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM91 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM92 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM93 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM94 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM95 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM96 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM97 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM98 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM99 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM100 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM101 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM102 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG DHTMem tables
pub mod DHTMEM103 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_0 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_1 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_2 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_3 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_4 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_5 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_6 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_7 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_8 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_9 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_10 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_11 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_12 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_13 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_14 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_15 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_16 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_17 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_18 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_19 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_20 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_21 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_22 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_23 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_24 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_25 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_26 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_27 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_28 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_29 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_30 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_31 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_32 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_33 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_34 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_35 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_36 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_37 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_38 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_39 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_40 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_41 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_42 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_43 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_44 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_45 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_46 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_47 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_48 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_49 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_50 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_51 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_52 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_53 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_54 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_55 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_56 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_57 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_58 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_59 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_60 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_61 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_62 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_63 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_64 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_65 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_66 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_67 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_68 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_69 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_70 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_71 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_72 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_73 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_74 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_75 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_76 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_77 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_78 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_79 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_80 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_81 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_82 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_83 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_84 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_85 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_86 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 0
pub mod HUFFENC_AC0_87 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_0 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_1 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_2 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_3 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_4 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_5 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_6 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_7 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_8 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_9 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_10 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_11 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_12 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_13 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_14 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_15 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_16 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_17 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_18 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_19 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_20 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_21 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_22 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_23 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_24 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_25 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_26 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_27 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_28 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_29 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_30 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_31 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_32 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_33 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_34 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_35 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_36 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_37 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_38 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_39 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_40 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_41 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_42 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_43 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_44 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_45 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_46 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_47 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_48 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_49 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_50 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_51 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_52 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_53 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_54 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_55 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_56 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_57 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_58 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_59 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_60 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_61 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_62 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_63 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_64 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_65 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_66 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_67 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_68 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_69 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_70 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_71 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_72 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_73 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_74 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_75 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_76 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_77 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_78 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_79 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_80 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_81 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_82 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_83 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_84 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_85 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_86 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC1_87 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 0
pub mod HUFFENC_DC0_0 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 0
pub mod HUFFENC_DC0_1 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 0
pub mod HUFFENC_DC0_2 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 0
pub mod HUFFENC_DC0_3 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 0
pub mod HUFFENC_DC0_4 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 0
pub mod HUFFENC_DC0_5 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 0
pub mod HUFFENC_DC0_6 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 0
pub mod HUFFENC_DC0_7 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC1_0 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC1_1 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC1_2 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC1_3 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC1_4 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC1_5 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC1_6 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC1_7 {
    pub use super::DHTMEM0::DHTMem_RAM;
}
#[repr(C)]
pub struct RegisterBlock {
    /// JPEG codec configuration register 0
    pub JPEG_CONFR0: WORegister<u32>,

    /// JPEG codec configuration register 1
    pub JPEG_CONFR1: RWRegister<u32>,

    /// JPEG codec configuration register 2
    pub JPEG_CONFR2: RWRegister<u32>,

    /// JPEG codec configuration register 3
    pub JPEG_CONFR3: RWRegister<u32>,

    /// JPEG codec configuration register 4
    pub JPEG_CONFR4: RWRegister<u32>,

    /// JPEG codec configuration register 5
    pub JPEG_CONFR5: RWRegister<u32>,

    /// JPEG codec configuration register 6
    pub JPEG_CONFR6: RWRegister<u32>,

    /// JPEG codec configuration register 7
    pub JPEG_CONFR7: RWRegister<u32>,

    _reserved1: [u32; 4],

    /// JPEG control register
    pub JPEG_CR: RWRegister<u32>,

    /// JPEG status register
    pub JPEG_SR: RORegister<u32>,

    /// JPEG clear flag register
    pub JPEG_CFR: WORegister<u32>,

    _reserved2: [u32; 1],

    /// JPEG data input register
    pub JPEG_DIR: WORegister<u32>,

    /// JPEG data output register
    pub JPEG_DOR: RORegister<u32>,

    _reserved3: [u32; 2],

    /// JPEG quantization tables
    pub QMEM0_0: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM0_1: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM0_2: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM0_3: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM0_4: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM0_5: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM0_6: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM0_7: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM0_8: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM0_9: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM0_10: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM0_11: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM0_12: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM0_13: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM0_14: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM0_15: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_0: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_1: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_2: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_3: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_4: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_5: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_6: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_7: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_8: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_9: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_10: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_11: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_12: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_13: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_14: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM1_15: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_0: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_1: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_2: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_3: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_4: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_5: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_6: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_7: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_8: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_9: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_10: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_11: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_12: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_13: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_14: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM2_15: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_0: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_1: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_2: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_3: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_4: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_5: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_6: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_7: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_8: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_9: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_10: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_11: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_12: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_13: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_14: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM3_15: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_0: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_1: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_2: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_3: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_4: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_5: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_6: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_7: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_8: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_9: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_10: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_11: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_12: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_13: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_14: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN_15: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE0: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE1: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE2: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE3: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE4: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE5: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE6: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE7: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE8: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE9: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE10: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE11: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE12: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE13: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE14: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE15: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE16: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE17: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE18: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE19: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE20: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE21: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE22: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE23: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE24: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE25: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE26: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE27: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE28: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE29: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE30: RWRegister<u32>,

    /// JPEG HuffSymb tables
    pub HUFFBASE31: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB0: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB1: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB2: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB3: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB4: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB5: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB6: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB7: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB8: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB9: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB10: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB11: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB12: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB13: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB14: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB15: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB16: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB17: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB18: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB19: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB20: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB21: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB22: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB23: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB24: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB25: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB26: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB27: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB28: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB29: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB30: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB31: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB32: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB33: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB34: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB35: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB36: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB37: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB38: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB39: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB40: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB41: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB42: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB43: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB44: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB45: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB46: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB47: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB48: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB49: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB50: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB51: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB52: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB53: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB54: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB55: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB56: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB57: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB58: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB59: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB60: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB61: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB62: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB63: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB64: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB65: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB66: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB67: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB68: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB69: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB70: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB71: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB72: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB73: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB74: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB75: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB76: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB77: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB78: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB79: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB80: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB81: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB82: RWRegister<u32>,

    /// JPEG HUFFSYMB tables
    pub HUFFSYMB83: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM0: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM2: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM3: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM4: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM5: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM6: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM7: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM8: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM9: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM10: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM11: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM12: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM13: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM14: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM15: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM16: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM17: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM18: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM19: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM20: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM21: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM22: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM23: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM24: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM25: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM26: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM27: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM28: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM29: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM30: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM31: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM32: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM33: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM34: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM35: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM36: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM37: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM38: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM39: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM40: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM41: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM42: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM43: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM44: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM45: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM46: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM47: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM48: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM49: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM50: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM51: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM52: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM53: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM54: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM55: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM56: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM57: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM58: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM59: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM60: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM61: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM62: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM63: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM64: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM65: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM66: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM67: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM68: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM69: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM70: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM71: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM72: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM73: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM74: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM75: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM76: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM77: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM78: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM79: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM80: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM81: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM82: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM83: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM84: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM85: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM86: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM87: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM88: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM89: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM90: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM91: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM92: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM93: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM94: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM95: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM96: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM97: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM98: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM99: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM100: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM101: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM102: RWRegister<u32>,

    /// JPEG DHTMem tables
    pub DHTMEM103: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_0: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_1: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_2: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_3: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_4: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_5: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_6: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_7: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_8: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_9: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_10: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_11: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_12: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_13: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_14: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_15: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_16: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_17: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_18: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_19: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_20: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_21: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_22: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_23: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_24: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_25: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_26: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_27: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_28: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_29: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_30: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_31: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_32: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_33: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_34: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_35: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_36: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_37: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_38: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_39: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_40: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_41: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_42: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_43: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_44: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_45: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_46: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_47: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_48: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_49: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_50: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_51: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_52: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_53: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_54: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_55: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_56: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_57: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_58: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_59: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_60: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_61: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_62: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_63: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_64: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_65: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_66: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_67: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_68: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_69: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_70: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_71: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_72: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_73: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_74: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_75: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_76: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_77: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_78: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_79: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_80: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_81: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_82: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_83: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_84: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_85: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_86: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 0
    pub HUFFENC_AC0_87: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_0: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_1: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_2: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_3: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_4: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_5: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_6: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_7: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_8: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_9: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_10: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_11: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_12: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_13: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_14: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_15: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_16: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_17: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_18: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_19: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_20: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_21: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_22: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_23: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_24: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_25: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_26: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_27: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_28: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_29: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_30: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_31: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_32: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_33: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_34: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_35: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_36: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_37: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_38: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_39: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_40: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_41: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_42: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_43: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_44: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_45: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_46: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_47: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_48: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_49: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_50: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_51: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_52: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_53: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_54: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_55: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_56: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_57: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_58: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_59: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_60: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_61: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_62: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_63: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_64: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_65: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_66: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_67: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_68: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_69: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_70: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_71: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_72: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_73: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_74: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_75: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_76: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_77: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_78: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_79: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_80: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_81: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_82: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_83: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_84: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_85: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_86: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC1_87: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 0
    pub HUFFENC_DC0_0: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 0
    pub HUFFENC_DC0_1: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 0
    pub HUFFENC_DC0_2: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 0
    pub HUFFENC_DC0_3: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 0
    pub HUFFENC_DC0_4: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 0
    pub HUFFENC_DC0_5: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 0
    pub HUFFENC_DC0_6: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 0
    pub HUFFENC_DC0_7: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC1_0: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC1_1: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC1_2: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC1_3: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC1_4: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC1_5: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC1_6: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC1_7: RWRegister<u32>,
}
pub struct ResetValues {
    pub JPEG_CONFR0: u32,
    pub JPEG_CONFR1: u32,
    pub JPEG_CONFR2: u32,
    pub JPEG_CONFR3: u32,
    pub JPEG_CONFR4: u32,
    pub JPEG_CONFR5: u32,
    pub JPEG_CONFR6: u32,
    pub JPEG_CONFR7: u32,
    pub JPEG_CR: u32,
    pub JPEG_SR: u32,
    pub JPEG_CFR: u32,
    pub JPEG_DIR: u32,
    pub JPEG_DOR: u32,
    pub QMEM0_0: u32,
    pub QMEM0_1: u32,
    pub QMEM0_2: u32,
    pub QMEM0_3: u32,
    pub QMEM0_4: u32,
    pub QMEM0_5: u32,
    pub QMEM0_6: u32,
    pub QMEM0_7: u32,
    pub QMEM0_8: u32,
    pub QMEM0_9: u32,
    pub QMEM0_10: u32,
    pub QMEM0_11: u32,
    pub QMEM0_12: u32,
    pub QMEM0_13: u32,
    pub QMEM0_14: u32,
    pub QMEM0_15: u32,
    pub QMEM1_0: u32,
    pub QMEM1_1: u32,
    pub QMEM1_2: u32,
    pub QMEM1_3: u32,
    pub QMEM1_4: u32,
    pub QMEM1_5: u32,
    pub QMEM1_6: u32,
    pub QMEM1_7: u32,
    pub QMEM1_8: u32,
    pub QMEM1_9: u32,
    pub QMEM1_10: u32,
    pub QMEM1_11: u32,
    pub QMEM1_12: u32,
    pub QMEM1_13: u32,
    pub QMEM1_14: u32,
    pub QMEM1_15: u32,
    pub QMEM2_0: u32,
    pub QMEM2_1: u32,
    pub QMEM2_2: u32,
    pub QMEM2_3: u32,
    pub QMEM2_4: u32,
    pub QMEM2_5: u32,
    pub QMEM2_6: u32,
    pub QMEM2_7: u32,
    pub QMEM2_8: u32,
    pub QMEM2_9: u32,
    pub QMEM2_10: u32,
    pub QMEM2_11: u32,
    pub QMEM2_12: u32,
    pub QMEM2_13: u32,
    pub QMEM2_14: u32,
    pub QMEM2_15: u32,
    pub QMEM3_0: u32,
    pub QMEM3_1: u32,
    pub QMEM3_2: u32,
    pub QMEM3_3: u32,
    pub QMEM3_4: u32,
    pub QMEM3_5: u32,
    pub QMEM3_6: u32,
    pub QMEM3_7: u32,
    pub QMEM3_8: u32,
    pub QMEM3_9: u32,
    pub QMEM3_10: u32,
    pub QMEM3_11: u32,
    pub QMEM3_12: u32,
    pub QMEM3_13: u32,
    pub QMEM3_14: u32,
    pub QMEM3_15: u32,
    pub HUFFMIN_0: u32,
    pub HUFFMIN_1: u32,
    pub HUFFMIN_2: u32,
    pub HUFFMIN_3: u32,
    pub HUFFMIN_4: u32,
    pub HUFFMIN_5: u32,
    pub HUFFMIN_6: u32,
    pub HUFFMIN_7: u32,
    pub HUFFMIN_8: u32,
    pub HUFFMIN_9: u32,
    pub HUFFMIN_10: u32,
    pub HUFFMIN_11: u32,
    pub HUFFMIN_12: u32,
    pub HUFFMIN_13: u32,
    pub HUFFMIN_14: u32,
    pub HUFFMIN_15: u32,
    pub HUFFBASE0: u32,
    pub HUFFBASE1: u32,
    pub HUFFBASE2: u32,
    pub HUFFBASE3: u32,
    pub HUFFBASE4: u32,
    pub HUFFBASE5: u32,
    pub HUFFBASE6: u32,
    pub HUFFBASE7: u32,
    pub HUFFBASE8: u32,
    pub HUFFBASE9: u32,
    pub HUFFBASE10: u32,
    pub HUFFBASE11: u32,
    pub HUFFBASE12: u32,
    pub HUFFBASE13: u32,
    pub HUFFBASE14: u32,
    pub HUFFBASE15: u32,
    pub HUFFBASE16: u32,
    pub HUFFBASE17: u32,
    pub HUFFBASE18: u32,
    pub HUFFBASE19: u32,
    pub HUFFBASE20: u32,
    pub HUFFBASE21: u32,
    pub HUFFBASE22: u32,
    pub HUFFBASE23: u32,
    pub HUFFBASE24: u32,
    pub HUFFBASE25: u32,
    pub HUFFBASE26: u32,
    pub HUFFBASE27: u32,
    pub HUFFBASE28: u32,
    pub HUFFBASE29: u32,
    pub HUFFBASE30: u32,
    pub HUFFBASE31: u32,
    pub HUFFSYMB0: u32,
    pub HUFFSYMB1: u32,
    pub HUFFSYMB2: u32,
    pub HUFFSYMB3: u32,
    pub HUFFSYMB4: u32,
    pub HUFFSYMB5: u32,
    pub HUFFSYMB6: u32,
    pub HUFFSYMB7: u32,
    pub HUFFSYMB8: u32,
    pub HUFFSYMB9: u32,
    pub HUFFSYMB10: u32,
    pub HUFFSYMB11: u32,
    pub HUFFSYMB12: u32,
    pub HUFFSYMB13: u32,
    pub HUFFSYMB14: u32,
    pub HUFFSYMB15: u32,
    pub HUFFSYMB16: u32,
    pub HUFFSYMB17: u32,
    pub HUFFSYMB18: u32,
    pub HUFFSYMB19: u32,
    pub HUFFSYMB20: u32,
    pub HUFFSYMB21: u32,
    pub HUFFSYMB22: u32,
    pub HUFFSYMB23: u32,
    pub HUFFSYMB24: u32,
    pub HUFFSYMB25: u32,
    pub HUFFSYMB26: u32,
    pub HUFFSYMB27: u32,
    pub HUFFSYMB28: u32,
    pub HUFFSYMB29: u32,
    pub HUFFSYMB30: u32,
    pub HUFFSYMB31: u32,
    pub HUFFSYMB32: u32,
    pub HUFFSYMB33: u32,
    pub HUFFSYMB34: u32,
    pub HUFFSYMB35: u32,
    pub HUFFSYMB36: u32,
    pub HUFFSYMB37: u32,
    pub HUFFSYMB38: u32,
    pub HUFFSYMB39: u32,
    pub HUFFSYMB40: u32,
    pub HUFFSYMB41: u32,
    pub HUFFSYMB42: u32,
    pub HUFFSYMB43: u32,
    pub HUFFSYMB44: u32,
    pub HUFFSYMB45: u32,
    pub HUFFSYMB46: u32,
    pub HUFFSYMB47: u32,
    pub HUFFSYMB48: u32,
    pub HUFFSYMB49: u32,
    pub HUFFSYMB50: u32,
    pub HUFFSYMB51: u32,
    pub HUFFSYMB52: u32,
    pub HUFFSYMB53: u32,
    pub HUFFSYMB54: u32,
    pub HUFFSYMB55: u32,
    pub HUFFSYMB56: u32,
    pub HUFFSYMB57: u32,
    pub HUFFSYMB58: u32,
    pub HUFFSYMB59: u32,
    pub HUFFSYMB60: u32,
    pub HUFFSYMB61: u32,
    pub HUFFSYMB62: u32,
    pub HUFFSYMB63: u32,
    pub HUFFSYMB64: u32,
    pub HUFFSYMB65: u32,
    pub HUFFSYMB66: u32,
    pub HUFFSYMB67: u32,
    pub HUFFSYMB68: u32,
    pub HUFFSYMB69: u32,
    pub HUFFSYMB70: u32,
    pub HUFFSYMB71: u32,
    pub HUFFSYMB72: u32,
    pub HUFFSYMB73: u32,
    pub HUFFSYMB74: u32,
    pub HUFFSYMB75: u32,
    pub HUFFSYMB76: u32,
    pub HUFFSYMB77: u32,
    pub HUFFSYMB78: u32,
    pub HUFFSYMB79: u32,
    pub HUFFSYMB80: u32,
    pub HUFFSYMB81: u32,
    pub HUFFSYMB82: u32,
    pub HUFFSYMB83: u32,
    pub DHTMEM0: u32,
    pub DHTMEM2: u32,
    pub DHTMEM3: u32,
    pub DHTMEM4: u32,
    pub DHTMEM5: u32,
    pub DHTMEM6: u32,
    pub DHTMEM7: u32,
    pub DHTMEM8: u32,
    pub DHTMEM9: u32,
    pub DHTMEM10: u32,
    pub DHTMEM11: u32,
    pub DHTMEM12: u32,
    pub DHTMEM13: u32,
    pub DHTMEM14: u32,
    pub DHTMEM15: u32,
    pub DHTMEM16: u32,
    pub DHTMEM17: u32,
    pub DHTMEM18: u32,
    pub DHTMEM19: u32,
    pub DHTMEM20: u32,
    pub DHTMEM21: u32,
    pub DHTMEM22: u32,
    pub DHTMEM23: u32,
    pub DHTMEM24: u32,
    pub DHTMEM25: u32,
    pub DHTMEM26: u32,
    pub DHTMEM27: u32,
    pub DHTMEM28: u32,
    pub DHTMEM29: u32,
    pub DHTMEM30: u32,
    pub DHTMEM31: u32,
    pub DHTMEM32: u32,
    pub DHTMEM33: u32,
    pub DHTMEM34: u32,
    pub DHTMEM35: u32,
    pub DHTMEM36: u32,
    pub DHTMEM37: u32,
    pub DHTMEM38: u32,
    pub DHTMEM39: u32,
    pub DHTMEM40: u32,
    pub DHTMEM41: u32,
    pub DHTMEM42: u32,
    pub DHTMEM43: u32,
    pub DHTMEM44: u32,
    pub DHTMEM45: u32,
    pub DHTMEM46: u32,
    pub DHTMEM47: u32,
    pub DHTMEM48: u32,
    pub DHTMEM49: u32,
    pub DHTMEM50: u32,
    pub DHTMEM51: u32,
    pub DHTMEM52: u32,
    pub DHTMEM53: u32,
    pub DHTMEM54: u32,
    pub DHTMEM55: u32,
    pub DHTMEM56: u32,
    pub DHTMEM57: u32,
    pub DHTMEM58: u32,
    pub DHTMEM59: u32,
    pub DHTMEM60: u32,
    pub DHTMEM61: u32,
    pub DHTMEM62: u32,
    pub DHTMEM63: u32,
    pub DHTMEM64: u32,
    pub DHTMEM65: u32,
    pub DHTMEM66: u32,
    pub DHTMEM67: u32,
    pub DHTMEM68: u32,
    pub DHTMEM69: u32,
    pub DHTMEM70: u32,
    pub DHTMEM71: u32,
    pub DHTMEM72: u32,
    pub DHTMEM73: u32,
    pub DHTMEM74: u32,
    pub DHTMEM75: u32,
    pub DHTMEM76: u32,
    pub DHTMEM77: u32,
    pub DHTMEM78: u32,
    pub DHTMEM79: u32,
    pub DHTMEM80: u32,
    pub DHTMEM81: u32,
    pub DHTMEM82: u32,
    pub DHTMEM83: u32,
    pub DHTMEM84: u32,
    pub DHTMEM85: u32,
    pub DHTMEM86: u32,
    pub DHTMEM87: u32,
    pub DHTMEM88: u32,
    pub DHTMEM89: u32,
    pub DHTMEM90: u32,
    pub DHTMEM91: u32,
    pub DHTMEM92: u32,
    pub DHTMEM93: u32,
    pub DHTMEM94: u32,
    pub DHTMEM95: u32,
    pub DHTMEM96: u32,
    pub DHTMEM97: u32,
    pub DHTMEM98: u32,
    pub DHTMEM99: u32,
    pub DHTMEM100: u32,
    pub DHTMEM101: u32,
    pub DHTMEM102: u32,
    pub DHTMEM103: u32,
    pub HUFFENC_AC0_0: u32,
    pub HUFFENC_AC0_1: u32,
    pub HUFFENC_AC0_2: u32,
    pub HUFFENC_AC0_3: u32,
    pub HUFFENC_AC0_4: u32,
    pub HUFFENC_AC0_5: u32,
    pub HUFFENC_AC0_6: u32,
    pub HUFFENC_AC0_7: u32,
    pub HUFFENC_AC0_8: u32,
    pub HUFFENC_AC0_9: u32,
    pub HUFFENC_AC0_10: u32,
    pub HUFFENC_AC0_11: u32,
    pub HUFFENC_AC0_12: u32,
    pub HUFFENC_AC0_13: u32,
    pub HUFFENC_AC0_14: u32,
    pub HUFFENC_AC0_15: u32,
    pub HUFFENC_AC0_16: u32,
    pub HUFFENC_AC0_17: u32,
    pub HUFFENC_AC0_18: u32,
    pub HUFFENC_AC0_19: u32,
    pub HUFFENC_AC0_20: u32,
    pub HUFFENC_AC0_21: u32,
    pub HUFFENC_AC0_22: u32,
    pub HUFFENC_AC0_23: u32,
    pub HUFFENC_AC0_24: u32,
    pub HUFFENC_AC0_25: u32,
    pub HUFFENC_AC0_26: u32,
    pub HUFFENC_AC0_27: u32,
    pub HUFFENC_AC0_28: u32,
    pub HUFFENC_AC0_29: u32,
    pub HUFFENC_AC0_30: u32,
    pub HUFFENC_AC0_31: u32,
    pub HUFFENC_AC0_32: u32,
    pub HUFFENC_AC0_33: u32,
    pub HUFFENC_AC0_34: u32,
    pub HUFFENC_AC0_35: u32,
    pub HUFFENC_AC0_36: u32,
    pub HUFFENC_AC0_37: u32,
    pub HUFFENC_AC0_38: u32,
    pub HUFFENC_AC0_39: u32,
    pub HUFFENC_AC0_40: u32,
    pub HUFFENC_AC0_41: u32,
    pub HUFFENC_AC0_42: u32,
    pub HUFFENC_AC0_43: u32,
    pub HUFFENC_AC0_44: u32,
    pub HUFFENC_AC0_45: u32,
    pub HUFFENC_AC0_46: u32,
    pub HUFFENC_AC0_47: u32,
    pub HUFFENC_AC0_48: u32,
    pub HUFFENC_AC0_49: u32,
    pub HUFFENC_AC0_50: u32,
    pub HUFFENC_AC0_51: u32,
    pub HUFFENC_AC0_52: u32,
    pub HUFFENC_AC0_53: u32,
    pub HUFFENC_AC0_54: u32,
    pub HUFFENC_AC0_55: u32,
    pub HUFFENC_AC0_56: u32,
    pub HUFFENC_AC0_57: u32,
    pub HUFFENC_AC0_58: u32,
    pub HUFFENC_AC0_59: u32,
    pub HUFFENC_AC0_60: u32,
    pub HUFFENC_AC0_61: u32,
    pub HUFFENC_AC0_62: u32,
    pub HUFFENC_AC0_63: u32,
    pub HUFFENC_AC0_64: u32,
    pub HUFFENC_AC0_65: u32,
    pub HUFFENC_AC0_66: u32,
    pub HUFFENC_AC0_67: u32,
    pub HUFFENC_AC0_68: u32,
    pub HUFFENC_AC0_69: u32,
    pub HUFFENC_AC0_70: u32,
    pub HUFFENC_AC0_71: u32,
    pub HUFFENC_AC0_72: u32,
    pub HUFFENC_AC0_73: u32,
    pub HUFFENC_AC0_74: u32,
    pub HUFFENC_AC0_75: u32,
    pub HUFFENC_AC0_76: u32,
    pub HUFFENC_AC0_77: u32,
    pub HUFFENC_AC0_78: u32,
    pub HUFFENC_AC0_79: u32,
    pub HUFFENC_AC0_80: u32,
    pub HUFFENC_AC0_81: u32,
    pub HUFFENC_AC0_82: u32,
    pub HUFFENC_AC0_83: u32,
    pub HUFFENC_AC0_84: u32,
    pub HUFFENC_AC0_85: u32,
    pub HUFFENC_AC0_86: u32,
    pub HUFFENC_AC0_87: u32,
    pub HUFFENC_AC1_0: u32,
    pub HUFFENC_AC1_1: u32,
    pub HUFFENC_AC1_2: u32,
    pub HUFFENC_AC1_3: u32,
    pub HUFFENC_AC1_4: u32,
    pub HUFFENC_AC1_5: u32,
    pub HUFFENC_AC1_6: u32,
    pub HUFFENC_AC1_7: u32,
    pub HUFFENC_AC1_8: u32,
    pub HUFFENC_AC1_9: u32,
    pub HUFFENC_AC1_10: u32,
    pub HUFFENC_AC1_11: u32,
    pub HUFFENC_AC1_12: u32,
    pub HUFFENC_AC1_13: u32,
    pub HUFFENC_AC1_14: u32,
    pub HUFFENC_AC1_15: u32,
    pub HUFFENC_AC1_16: u32,
    pub HUFFENC_AC1_17: u32,
    pub HUFFENC_AC1_18: u32,
    pub HUFFENC_AC1_19: u32,
    pub HUFFENC_AC1_20: u32,
    pub HUFFENC_AC1_21: u32,
    pub HUFFENC_AC1_22: u32,
    pub HUFFENC_AC1_23: u32,
    pub HUFFENC_AC1_24: u32,
    pub HUFFENC_AC1_25: u32,
    pub HUFFENC_AC1_26: u32,
    pub HUFFENC_AC1_27: u32,
    pub HUFFENC_AC1_28: u32,
    pub HUFFENC_AC1_29: u32,
    pub HUFFENC_AC1_30: u32,
    pub HUFFENC_AC1_31: u32,
    pub HUFFENC_AC1_32: u32,
    pub HUFFENC_AC1_33: u32,
    pub HUFFENC_AC1_34: u32,
    pub HUFFENC_AC1_35: u32,
    pub HUFFENC_AC1_36: u32,
    pub HUFFENC_AC1_37: u32,
    pub HUFFENC_AC1_38: u32,
    pub HUFFENC_AC1_39: u32,
    pub HUFFENC_AC1_40: u32,
    pub HUFFENC_AC1_41: u32,
    pub HUFFENC_AC1_42: u32,
    pub HUFFENC_AC1_43: u32,
    pub HUFFENC_AC1_44: u32,
    pub HUFFENC_AC1_45: u32,
    pub HUFFENC_AC1_46: u32,
    pub HUFFENC_AC1_47: u32,
    pub HUFFENC_AC1_48: u32,
    pub HUFFENC_AC1_49: u32,
    pub HUFFENC_AC1_50: u32,
    pub HUFFENC_AC1_51: u32,
    pub HUFFENC_AC1_52: u32,
    pub HUFFENC_AC1_53: u32,
    pub HUFFENC_AC1_54: u32,
    pub HUFFENC_AC1_55: u32,
    pub HUFFENC_AC1_56: u32,
    pub HUFFENC_AC1_57: u32,
    pub HUFFENC_AC1_58: u32,
    pub HUFFENC_AC1_59: u32,
    pub HUFFENC_AC1_60: u32,
    pub HUFFENC_AC1_61: u32,
    pub HUFFENC_AC1_62: u32,
    pub HUFFENC_AC1_63: u32,
    pub HUFFENC_AC1_64: u32,
    pub HUFFENC_AC1_65: u32,
    pub HUFFENC_AC1_66: u32,
    pub HUFFENC_AC1_67: u32,
    pub HUFFENC_AC1_68: u32,
    pub HUFFENC_AC1_69: u32,
    pub HUFFENC_AC1_70: u32,
    pub HUFFENC_AC1_71: u32,
    pub HUFFENC_AC1_72: u32,
    pub HUFFENC_AC1_73: u32,
    pub HUFFENC_AC1_74: u32,
    pub HUFFENC_AC1_75: u32,
    pub HUFFENC_AC1_76: u32,
    pub HUFFENC_AC1_77: u32,
    pub HUFFENC_AC1_78: u32,
    pub HUFFENC_AC1_79: u32,
    pub HUFFENC_AC1_80: u32,
    pub HUFFENC_AC1_81: u32,
    pub HUFFENC_AC1_82: u32,
    pub HUFFENC_AC1_83: u32,
    pub HUFFENC_AC1_84: u32,
    pub HUFFENC_AC1_85: u32,
    pub HUFFENC_AC1_86: u32,
    pub HUFFENC_AC1_87: u32,
    pub HUFFENC_DC0_0: u32,
    pub HUFFENC_DC0_1: u32,
    pub HUFFENC_DC0_2: u32,
    pub HUFFENC_DC0_3: u32,
    pub HUFFENC_DC0_4: u32,
    pub HUFFENC_DC0_5: u32,
    pub HUFFENC_DC0_6: u32,
    pub HUFFENC_DC0_7: u32,
    pub HUFFENC_DC1_0: u32,
    pub HUFFENC_DC1_1: u32,
    pub HUFFENC_DC1_2: u32,
    pub HUFFENC_DC1_3: u32,
    pub HUFFENC_DC1_4: u32,
    pub HUFFENC_DC1_5: u32,
    pub HUFFENC_DC1_6: u32,
    pub HUFFENC_DC1_7: u32,
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
