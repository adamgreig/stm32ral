#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! JPEG codec
//!
//! Used by: stm32f765, stm32f7x7, stm32f7x9

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// JPEG codec configuration register 0
pub mod CONFR0 {

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
pub mod CONFR1 {

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
pub mod CONFR2 {

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
pub mod CONFR3 {

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
pub mod CONFR4 {

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
pub mod CONFR5 {
    pub use super::CONFR4::HA;
    pub use super::CONFR4::HD;
    pub use super::CONFR4::HSF;
    pub use super::CONFR4::NB;
    pub use super::CONFR4::QT;
    pub use super::CONFR4::VSF;
}

/// JPEG codec configuration register 6
pub mod CONFR6 {
    pub use super::CONFR4::HA;
    pub use super::CONFR4::HD;
    pub use super::CONFR4::HSF;
    pub use super::CONFR4::NB;
    pub use super::CONFR4::QT;
    pub use super::CONFR4::VSF;
}

/// JPEG codec configuration register 7
pub mod CONFR7 {
    pub use super::CONFR4::HA;
    pub use super::CONFR4::HD;
    pub use super::CONFR4::HSF;
    pub use super::CONFR4::NB;
    pub use super::CONFR4::QT;
    pub use super::CONFR4::VSF;
}

/// JPEG control register
pub mod CR {

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
pub mod SR {

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
pub mod CFR {

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
pub mod DIR {

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
pub mod DOR {

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
pub mod QMEM00 {

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
pub mod QMEM01 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM02 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM03 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM04 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM05 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM06 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM07 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM08 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM09 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM010 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM011 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM012 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM013 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM014 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM015 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM10 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM11 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM12 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM13 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM14 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM15 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM16 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM17 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM18 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM19 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM110 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM111 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM112 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM113 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM114 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM115 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM20 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM21 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM22 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM23 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM24 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM25 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM26 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM27 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM28 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM29 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM210 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM211 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM212 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM213 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM214 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM215 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM30 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM31 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM32 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM33 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM34 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM35 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM36 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM37 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM38 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM39 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM310 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM311 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM312 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM313 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM314 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG quantization tables
pub mod QMEM315 {
    pub use super::QMEM00::QMem_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN0 {

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
pub mod HUFFMIN1 {
    pub use super::HUFFMIN0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN2 {
    pub use super::HUFFMIN0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN3 {
    pub use super::HUFFMIN0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN4 {
    pub use super::HUFFMIN0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN5 {
    pub use super::HUFFMIN0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN6 {
    pub use super::HUFFMIN0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN7 {
    pub use super::HUFFMIN0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN8 {
    pub use super::HUFFMIN0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN9 {
    pub use super::HUFFMIN0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN10 {
    pub use super::HUFFMIN0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN11 {
    pub use super::HUFFMIN0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN12 {
    pub use super::HUFFMIN0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN13 {
    pub use super::HUFFMIN0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN14 {
    pub use super::HUFFMIN0::HuffMin_RAM;
}

/// JPEG HuffMin tables
pub mod HUFFMIN15 {
    pub use super::HUFFMIN0::HuffMin_RAM;
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

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC00 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC01 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC02 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC03 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC04 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC05 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC06 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC07 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC08 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC09 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC010 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC011 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC012 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC013 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC014 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC015 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC016 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC017 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC018 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC019 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC020 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC021 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC022 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC023 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC024 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC025 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC026 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC027 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC028 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC029 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC030 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC031 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC032 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC033 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC034 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC035 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC036 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC037 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC038 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC039 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC040 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC041 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC042 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC043 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC044 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC045 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC046 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC047 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC048 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC049 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC050 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC051 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC052 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC053 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC054 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC055 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC056 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC057 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC058 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC059 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC060 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC061 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC062 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC063 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC064 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC065 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC066 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC067 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC068 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC069 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC070 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC071 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC072 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC073 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC074 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC075 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC076 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC077 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC078 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC079 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC080 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC081 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC082 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC083 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC084 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC085 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC086 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table %s
pub mod HUFFENC_AC087 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC10 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC11 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC12 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC13 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC14 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC15 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC16 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC17 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC18 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC19 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC110 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC111 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC112 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC113 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC114 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC115 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC116 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC117 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC118 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC119 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC120 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC121 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC122 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC123 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC124 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC125 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC126 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC127 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC128 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC129 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC130 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC131 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC132 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC133 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC134 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC135 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC136 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC137 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC138 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC139 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC140 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC141 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC142 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC143 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC144 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC145 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC146 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC147 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC148 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC149 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC150 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC151 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC152 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC153 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC154 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC155 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC156 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC157 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC158 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC159 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC160 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC161 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC162 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC163 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC164 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC165 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC166 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC167 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC168 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC169 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC170 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC171 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC172 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC173 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC174 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC175 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC176 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC177 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC178 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC179 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC180 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC181 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC182 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC183 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC184 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC185 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC186 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, AC Huffman table 1
pub mod HUFFENC_AC187 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table %s
pub mod HUFFENC_DC00 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table %s
pub mod HUFFENC_DC01 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table %s
pub mod HUFFENC_DC02 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table %s
pub mod HUFFENC_DC03 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table %s
pub mod HUFFENC_DC04 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table %s
pub mod HUFFENC_DC05 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table %s
pub mod HUFFENC_DC06 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table %s
pub mod HUFFENC_DC07 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC10 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC11 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC12 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC13 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC14 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC15 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC16 {
    pub use super::DHTMEM0::DHTMem_RAM;
}

/// JPEG encoder, DC Huffman table 1
pub mod HUFFENC_DC17 {
    pub use super::DHTMEM0::DHTMem_RAM;
}
#[repr(C)]
pub struct RegisterBlock {
    /// JPEG codec configuration register 0
    pub CONFR0: WORegister<u32>,

    /// JPEG codec configuration register 1
    pub CONFR1: RWRegister<u32>,

    /// JPEG codec configuration register 2
    pub CONFR2: RWRegister<u32>,

    /// JPEG codec configuration register 3
    pub CONFR3: RWRegister<u32>,

    /// JPEG codec configuration register 4
    pub CONFR4: RWRegister<u32>,

    /// JPEG codec configuration register 5
    pub CONFR5: RWRegister<u32>,

    /// JPEG codec configuration register 6
    pub CONFR6: RWRegister<u32>,

    /// JPEG codec configuration register 7
    pub CONFR7: RWRegister<u32>,

    _reserved1: [u8; 16],

    /// JPEG control register
    pub CR: RWRegister<u32>,

    /// JPEG status register
    pub SR: RORegister<u32>,

    /// JPEG clear flag register
    pub CFR: WORegister<u32>,

    _reserved2: [u8; 4],

    /// JPEG data input register
    pub DIR: WORegister<u32>,

    /// JPEG data output register
    pub DOR: RORegister<u32>,

    _reserved3: [u8; 8],

    /// JPEG quantization tables
    pub QMEM00: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM01: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM02: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM03: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM04: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM05: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM06: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM07: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM08: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM09: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM010: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM011: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM012: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM013: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM014: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM015: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM10: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM11: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM12: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM13: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM14: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM15: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM16: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM17: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM18: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM19: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM110: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM111: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM112: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM113: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM114: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM115: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM20: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM21: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM22: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM23: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM24: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM25: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM26: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM27: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM28: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM29: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM210: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM211: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM212: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM213: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM214: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM215: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM30: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM31: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM32: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM33: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM34: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM35: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM36: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM37: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM38: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM39: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM310: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM311: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM312: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM313: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM314: RWRegister<u32>,

    /// JPEG quantization tables
    pub QMEM315: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN0: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN1: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN2: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN3: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN4: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN5: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN6: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN7: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN8: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN9: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN10: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN11: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN12: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN13: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN14: RWRegister<u32>,

    /// JPEG HuffMin tables
    pub HUFFMIN15: RWRegister<u32>,

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

    _reserved4: [u8; 4],

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC00: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC01: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC02: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC03: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC04: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC05: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC06: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC07: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC08: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC09: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC010: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC011: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC012: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC013: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC014: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC015: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC016: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC017: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC018: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC019: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC020: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC021: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC022: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC023: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC024: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC025: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC026: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC027: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC028: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC029: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC030: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC031: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC032: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC033: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC034: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC035: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC036: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC037: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC038: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC039: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC040: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC041: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC042: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC043: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC044: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC045: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC046: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC047: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC048: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC049: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC050: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC051: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC052: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC053: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC054: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC055: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC056: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC057: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC058: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC059: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC060: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC061: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC062: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC063: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC064: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC065: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC066: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC067: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC068: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC069: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC070: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC071: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC072: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC073: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC074: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC075: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC076: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC077: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC078: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC079: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC080: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC081: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC082: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC083: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC084: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC085: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC086: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table %s
    pub HUFFENC_AC087: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC10: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC11: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC12: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC13: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC14: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC15: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC16: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC17: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC18: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC19: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC110: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC111: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC112: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC113: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC114: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC115: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC116: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC117: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC118: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC119: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC120: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC121: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC122: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC123: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC124: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC125: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC126: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC127: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC128: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC129: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC130: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC131: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC132: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC133: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC134: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC135: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC136: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC137: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC138: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC139: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC140: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC141: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC142: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC143: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC144: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC145: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC146: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC147: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC148: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC149: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC150: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC151: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC152: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC153: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC154: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC155: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC156: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC157: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC158: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC159: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC160: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC161: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC162: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC163: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC164: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC165: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC166: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC167: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC168: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC169: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC170: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC171: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC172: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC173: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC174: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC175: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC176: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC177: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC178: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC179: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC180: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC181: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC182: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC183: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC184: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC185: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC186: RWRegister<u32>,

    /// JPEG encoder, AC Huffman table 1
    pub HUFFENC_AC187: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table %s
    pub HUFFENC_DC00: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table %s
    pub HUFFENC_DC01: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table %s
    pub HUFFENC_DC02: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table %s
    pub HUFFENC_DC03: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table %s
    pub HUFFENC_DC04: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table %s
    pub HUFFENC_DC05: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table %s
    pub HUFFENC_DC06: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table %s
    pub HUFFENC_DC07: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC10: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC11: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC12: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC13: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC14: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC15: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC16: RWRegister<u32>,

    /// JPEG encoder, DC Huffman table 1
    pub HUFFENC_DC17: RWRegister<u32>,
}
pub struct ResetValues {
    pub CONFR0: u32,
    pub CONFR1: u32,
    pub CONFR2: u32,
    pub CONFR3: u32,
    pub CONFR4: u32,
    pub CONFR5: u32,
    pub CONFR6: u32,
    pub CONFR7: u32,
    pub CR: u32,
    pub SR: u32,
    pub CFR: u32,
    pub DIR: u32,
    pub DOR: u32,
    pub QMEM00: u32,
    pub QMEM01: u32,
    pub QMEM02: u32,
    pub QMEM03: u32,
    pub QMEM04: u32,
    pub QMEM05: u32,
    pub QMEM06: u32,
    pub QMEM07: u32,
    pub QMEM08: u32,
    pub QMEM09: u32,
    pub QMEM010: u32,
    pub QMEM011: u32,
    pub QMEM012: u32,
    pub QMEM013: u32,
    pub QMEM014: u32,
    pub QMEM015: u32,
    pub QMEM10: u32,
    pub QMEM11: u32,
    pub QMEM12: u32,
    pub QMEM13: u32,
    pub QMEM14: u32,
    pub QMEM15: u32,
    pub QMEM16: u32,
    pub QMEM17: u32,
    pub QMEM18: u32,
    pub QMEM19: u32,
    pub QMEM110: u32,
    pub QMEM111: u32,
    pub QMEM112: u32,
    pub QMEM113: u32,
    pub QMEM114: u32,
    pub QMEM115: u32,
    pub QMEM20: u32,
    pub QMEM21: u32,
    pub QMEM22: u32,
    pub QMEM23: u32,
    pub QMEM24: u32,
    pub QMEM25: u32,
    pub QMEM26: u32,
    pub QMEM27: u32,
    pub QMEM28: u32,
    pub QMEM29: u32,
    pub QMEM210: u32,
    pub QMEM211: u32,
    pub QMEM212: u32,
    pub QMEM213: u32,
    pub QMEM214: u32,
    pub QMEM215: u32,
    pub QMEM30: u32,
    pub QMEM31: u32,
    pub QMEM32: u32,
    pub QMEM33: u32,
    pub QMEM34: u32,
    pub QMEM35: u32,
    pub QMEM36: u32,
    pub QMEM37: u32,
    pub QMEM38: u32,
    pub QMEM39: u32,
    pub QMEM310: u32,
    pub QMEM311: u32,
    pub QMEM312: u32,
    pub QMEM313: u32,
    pub QMEM314: u32,
    pub QMEM315: u32,
    pub HUFFMIN0: u32,
    pub HUFFMIN1: u32,
    pub HUFFMIN2: u32,
    pub HUFFMIN3: u32,
    pub HUFFMIN4: u32,
    pub HUFFMIN5: u32,
    pub HUFFMIN6: u32,
    pub HUFFMIN7: u32,
    pub HUFFMIN8: u32,
    pub HUFFMIN9: u32,
    pub HUFFMIN10: u32,
    pub HUFFMIN11: u32,
    pub HUFFMIN12: u32,
    pub HUFFMIN13: u32,
    pub HUFFMIN14: u32,
    pub HUFFMIN15: u32,
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
    pub HUFFENC_AC00: u32,
    pub HUFFENC_AC01: u32,
    pub HUFFENC_AC02: u32,
    pub HUFFENC_AC03: u32,
    pub HUFFENC_AC04: u32,
    pub HUFFENC_AC05: u32,
    pub HUFFENC_AC06: u32,
    pub HUFFENC_AC07: u32,
    pub HUFFENC_AC08: u32,
    pub HUFFENC_AC09: u32,
    pub HUFFENC_AC010: u32,
    pub HUFFENC_AC011: u32,
    pub HUFFENC_AC012: u32,
    pub HUFFENC_AC013: u32,
    pub HUFFENC_AC014: u32,
    pub HUFFENC_AC015: u32,
    pub HUFFENC_AC016: u32,
    pub HUFFENC_AC017: u32,
    pub HUFFENC_AC018: u32,
    pub HUFFENC_AC019: u32,
    pub HUFFENC_AC020: u32,
    pub HUFFENC_AC021: u32,
    pub HUFFENC_AC022: u32,
    pub HUFFENC_AC023: u32,
    pub HUFFENC_AC024: u32,
    pub HUFFENC_AC025: u32,
    pub HUFFENC_AC026: u32,
    pub HUFFENC_AC027: u32,
    pub HUFFENC_AC028: u32,
    pub HUFFENC_AC029: u32,
    pub HUFFENC_AC030: u32,
    pub HUFFENC_AC031: u32,
    pub HUFFENC_AC032: u32,
    pub HUFFENC_AC033: u32,
    pub HUFFENC_AC034: u32,
    pub HUFFENC_AC035: u32,
    pub HUFFENC_AC036: u32,
    pub HUFFENC_AC037: u32,
    pub HUFFENC_AC038: u32,
    pub HUFFENC_AC039: u32,
    pub HUFFENC_AC040: u32,
    pub HUFFENC_AC041: u32,
    pub HUFFENC_AC042: u32,
    pub HUFFENC_AC043: u32,
    pub HUFFENC_AC044: u32,
    pub HUFFENC_AC045: u32,
    pub HUFFENC_AC046: u32,
    pub HUFFENC_AC047: u32,
    pub HUFFENC_AC048: u32,
    pub HUFFENC_AC049: u32,
    pub HUFFENC_AC050: u32,
    pub HUFFENC_AC051: u32,
    pub HUFFENC_AC052: u32,
    pub HUFFENC_AC053: u32,
    pub HUFFENC_AC054: u32,
    pub HUFFENC_AC055: u32,
    pub HUFFENC_AC056: u32,
    pub HUFFENC_AC057: u32,
    pub HUFFENC_AC058: u32,
    pub HUFFENC_AC059: u32,
    pub HUFFENC_AC060: u32,
    pub HUFFENC_AC061: u32,
    pub HUFFENC_AC062: u32,
    pub HUFFENC_AC063: u32,
    pub HUFFENC_AC064: u32,
    pub HUFFENC_AC065: u32,
    pub HUFFENC_AC066: u32,
    pub HUFFENC_AC067: u32,
    pub HUFFENC_AC068: u32,
    pub HUFFENC_AC069: u32,
    pub HUFFENC_AC070: u32,
    pub HUFFENC_AC071: u32,
    pub HUFFENC_AC072: u32,
    pub HUFFENC_AC073: u32,
    pub HUFFENC_AC074: u32,
    pub HUFFENC_AC075: u32,
    pub HUFFENC_AC076: u32,
    pub HUFFENC_AC077: u32,
    pub HUFFENC_AC078: u32,
    pub HUFFENC_AC079: u32,
    pub HUFFENC_AC080: u32,
    pub HUFFENC_AC081: u32,
    pub HUFFENC_AC082: u32,
    pub HUFFENC_AC083: u32,
    pub HUFFENC_AC084: u32,
    pub HUFFENC_AC085: u32,
    pub HUFFENC_AC086: u32,
    pub HUFFENC_AC087: u32,
    pub HUFFENC_AC10: u32,
    pub HUFFENC_AC11: u32,
    pub HUFFENC_AC12: u32,
    pub HUFFENC_AC13: u32,
    pub HUFFENC_AC14: u32,
    pub HUFFENC_AC15: u32,
    pub HUFFENC_AC16: u32,
    pub HUFFENC_AC17: u32,
    pub HUFFENC_AC18: u32,
    pub HUFFENC_AC19: u32,
    pub HUFFENC_AC110: u32,
    pub HUFFENC_AC111: u32,
    pub HUFFENC_AC112: u32,
    pub HUFFENC_AC113: u32,
    pub HUFFENC_AC114: u32,
    pub HUFFENC_AC115: u32,
    pub HUFFENC_AC116: u32,
    pub HUFFENC_AC117: u32,
    pub HUFFENC_AC118: u32,
    pub HUFFENC_AC119: u32,
    pub HUFFENC_AC120: u32,
    pub HUFFENC_AC121: u32,
    pub HUFFENC_AC122: u32,
    pub HUFFENC_AC123: u32,
    pub HUFFENC_AC124: u32,
    pub HUFFENC_AC125: u32,
    pub HUFFENC_AC126: u32,
    pub HUFFENC_AC127: u32,
    pub HUFFENC_AC128: u32,
    pub HUFFENC_AC129: u32,
    pub HUFFENC_AC130: u32,
    pub HUFFENC_AC131: u32,
    pub HUFFENC_AC132: u32,
    pub HUFFENC_AC133: u32,
    pub HUFFENC_AC134: u32,
    pub HUFFENC_AC135: u32,
    pub HUFFENC_AC136: u32,
    pub HUFFENC_AC137: u32,
    pub HUFFENC_AC138: u32,
    pub HUFFENC_AC139: u32,
    pub HUFFENC_AC140: u32,
    pub HUFFENC_AC141: u32,
    pub HUFFENC_AC142: u32,
    pub HUFFENC_AC143: u32,
    pub HUFFENC_AC144: u32,
    pub HUFFENC_AC145: u32,
    pub HUFFENC_AC146: u32,
    pub HUFFENC_AC147: u32,
    pub HUFFENC_AC148: u32,
    pub HUFFENC_AC149: u32,
    pub HUFFENC_AC150: u32,
    pub HUFFENC_AC151: u32,
    pub HUFFENC_AC152: u32,
    pub HUFFENC_AC153: u32,
    pub HUFFENC_AC154: u32,
    pub HUFFENC_AC155: u32,
    pub HUFFENC_AC156: u32,
    pub HUFFENC_AC157: u32,
    pub HUFFENC_AC158: u32,
    pub HUFFENC_AC159: u32,
    pub HUFFENC_AC160: u32,
    pub HUFFENC_AC161: u32,
    pub HUFFENC_AC162: u32,
    pub HUFFENC_AC163: u32,
    pub HUFFENC_AC164: u32,
    pub HUFFENC_AC165: u32,
    pub HUFFENC_AC166: u32,
    pub HUFFENC_AC167: u32,
    pub HUFFENC_AC168: u32,
    pub HUFFENC_AC169: u32,
    pub HUFFENC_AC170: u32,
    pub HUFFENC_AC171: u32,
    pub HUFFENC_AC172: u32,
    pub HUFFENC_AC173: u32,
    pub HUFFENC_AC174: u32,
    pub HUFFENC_AC175: u32,
    pub HUFFENC_AC176: u32,
    pub HUFFENC_AC177: u32,
    pub HUFFENC_AC178: u32,
    pub HUFFENC_AC179: u32,
    pub HUFFENC_AC180: u32,
    pub HUFFENC_AC181: u32,
    pub HUFFENC_AC182: u32,
    pub HUFFENC_AC183: u32,
    pub HUFFENC_AC184: u32,
    pub HUFFENC_AC185: u32,
    pub HUFFENC_AC186: u32,
    pub HUFFENC_AC187: u32,
    pub HUFFENC_DC00: u32,
    pub HUFFENC_DC01: u32,
    pub HUFFENC_DC02: u32,
    pub HUFFENC_DC03: u32,
    pub HUFFENC_DC04: u32,
    pub HUFFENC_DC05: u32,
    pub HUFFENC_DC06: u32,
    pub HUFFENC_DC07: u32,
    pub HUFFENC_DC10: u32,
    pub HUFFENC_DC11: u32,
    pub HUFFENC_DC12: u32,
    pub HUFFENC_DC13: u32,
    pub HUFFENC_DC14: u32,
    pub HUFFENC_DC15: u32,
    pub HUFFENC_DC16: u32,
    pub HUFFENC_DC17: u32,
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
