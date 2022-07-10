#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Direct memory access controller

use crate::{RORegister, RWRegister, UnsafeRWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// interrupt status register
pub mod ISR {

    /// Channel x transfer error flag (x = 1 ..7)
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

    /// Channel x half transfer flag (x = 1 ..7)
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

    /// Channel x transfer complete flag (x = 1 ..7)
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

    /// Channel x global interrupt flag (x = 1 ..7)
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

    /// Channel x transfer error flag (x = 1 ..7)
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

    /// Channel x half transfer flag (x = 1 ..7)
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

    /// Channel x transfer complete flag (x = 1 ..7)
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

    /// Channel x global interrupt flag (x = 1 ..7)
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

    /// Channel x transfer error flag (x = 1 ..7)
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

    /// Channel x half transfer flag (x = 1 ..7)
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

    /// Channel x transfer complete flag (x = 1 ..7)
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

    /// Channel x global interrupt flag (x = 1 ..7)
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

    /// Channel x transfer error flag (x = 1 ..7)
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

    /// Channel x half transfer flag (x = 1 ..7)
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

    /// Channel x transfer complete flag (x = 1 ..7)
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

    /// Channel x global interrupt flag (x = 1 ..7)
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

    /// Channel x transfer error flag (x = 1 ..7)
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

    /// Channel x half transfer flag (x = 1 ..7)
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

    /// Channel x transfer complete flag (x = 1 ..7)
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

    /// Channel x global interrupt flag (x = 1 ..7)
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

    /// Channel x transfer error flag (x = 1 ..7)
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

    /// Channel x half transfer flag (x = 1 ..7)
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

    /// Channel x transfer complete flag (x = 1 ..7)
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

    /// Channel x global interrupt flag (x = 1 ..7)
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

    /// Channel x transfer error flag (x = 1 ..7)
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

    /// Channel x half transfer flag (x = 1 ..7)
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

    /// Channel x transfer complete flag (x = 1 ..7)
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

    /// Channel x global interrupt flag (x = 1 ..7)
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

    /// global interrupt flag for channel 8
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

    /// transfer complete (TC) flag for channel 8
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

    /// half transfer (HT) flag for channel 8
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

    /// transfer error (TE) flag for channel 8
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

/// interrupt flag clear register
pub mod IFCR {

    /// Channel x transfer error clear (x = 1 ..7)
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

    /// Channel x half transfer clear (x = 1 ..7)
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

    /// Channel x transfer complete clear (x = 1 ..7)
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

    /// Channel x global interrupt clear (x = 1 ..7)
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

    /// Channel x transfer error clear (x = 1 ..7)
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

    /// Channel x half transfer clear (x = 1 ..7)
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

    /// Channel x transfer complete clear (x = 1 ..7)
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

    /// Channel x global interrupt clear (x = 1 ..7)
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

    /// Channel x transfer error clear (x = 1 ..7)
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

    /// Channel x half transfer clear (x = 1 ..7)
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

    /// Channel x transfer complete clear (x = 1 ..7)
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

    /// Channel x global interrupt clear (x = 1 ..7)
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

    /// Channel x transfer error clear (x = 1 ..7)
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

    /// Channel x half transfer clear (x = 1 ..7)
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

    /// Channel x transfer complete clear (x = 1 ..7)
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

    /// Channel x global interrupt clear (x = 1 ..7)
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

    /// Channel x transfer error clear (x = 1 ..7)
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

    /// Channel x half transfer clear (x = 1 ..7)
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

    /// Channel x transfer complete clear (x = 1 ..7)
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

    /// Channel x global interrupt clear (x = 1 ..7)
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

    /// Channel x transfer error clear (x = 1 ..7)
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

    /// Channel x half transfer clear (x = 1 ..7)
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

    /// Channel x transfer complete clear (x = 1 ..7)
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

    /// Channel x global interrupt clear (x = 1 ..7)
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

    /// Channel x transfer error clear (x = 1 ..7)
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

    /// Channel x half transfer clear (x = 1 ..7)
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

    /// Channel x transfer complete clear (x = 1 ..7)
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

    /// Channel x global interrupt clear (x = 1 ..7)
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

    /// global interrupt flag clear for channel 8
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

    /// transfer complete flag clear for channel 8
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

    /// half transfer flag clear for channel 8
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

    /// transfer error flag clear for channel 8
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

/// channel x configuration register
pub mod CCR1 {

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
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel priority level
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
        /// Read-write values (empty)
        pub mod RW {}
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
        /// Read-write values (empty)
        pub mod RW {}
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
        /// Read-write values (empty)
        pub mod RW {}
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
        /// Read-write values (empty)
        pub mod RW {}
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
        /// Read-write values (empty)
        pub mod RW {}
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
        /// Read-write values (empty)
        pub mod RW {}
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
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half transfer interrupt enable
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
        /// Read-write values (empty)
        pub mod RW {}
    }

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
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// double-buffer mode
    pub mod DBM {
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

    /// current target memory of DMA transfer in double-buffer mode
    pub mod CT {
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

    /// secure mode
    pub mod SECM {
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

    /// security of the DMA transfer from the source
    pub mod SSEC {
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

    /// security of the DMA transfer to the destination
    pub mod DSEC {
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

    /// privileged mode
    pub mod PRIV {
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
}

/// channel x number of data register
pub mod CNDTR1 {

    /// Number of data to transfer
    pub mod NDT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (18 bits: 0x3ffff << 0)
        pub const mask: u32 = 0x3ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel x peripheral address register
pub mod CPAR1 {

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

/// channel x memory address register
pub mod CM0AR1 {

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

/// channel x memory address register
pub mod CM1AR1 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::CT;
    pub use super::CCR1::DBM;
    pub use super::CCR1::DIR;
    pub use super::CCR1::DSEC;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PRIV;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::SECM;
    pub use super::CCR1::SSEC;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// channel x configuration register
pub mod CCR2 {
    pub use super::CNDTR1::NDT;
}

/// channel x number of data register
pub mod CNDTR2 {
    pub use super::CPAR1::PA;
}

/// channel x peripheral address register
pub mod CPAR2 {
    pub use super::CM0AR1::MA;
}

/// channel x memory address register
pub mod CM0AR2 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::CT;
    pub use super::CCR1::DBM;
    pub use super::CCR1::DIR;
    pub use super::CCR1::DSEC;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PRIV;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::SECM;
    pub use super::CCR1::SSEC;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// channel x memory address register
pub mod CM1AR2 {
    pub use super::CNDTR1::NDT;
}

/// channel x configuration register
pub mod CCR3 {
    pub use super::CPAR1::PA;
}

/// channel x number of data register
pub mod CNDTR3 {
    pub use super::CM0AR1::MA;
}

/// channel x peripheral address register
pub mod CPAR3 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::CT;
    pub use super::CCR1::DBM;
    pub use super::CCR1::DIR;
    pub use super::CCR1::DSEC;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PRIV;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::SECM;
    pub use super::CCR1::SSEC;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// channel x memory address register
pub mod CM0AR3 {
    pub use super::CNDTR1::NDT;
}

/// channel x memory address register
pub mod CM1AR3 {
    pub use super::CPAR1::PA;
}

/// channel x configuration register
pub mod CCR4 {
    pub use super::CM0AR1::MA;
}

/// channel x number of data register
pub mod CNDTR4 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::CT;
    pub use super::CCR1::DBM;
    pub use super::CCR1::DIR;
    pub use super::CCR1::DSEC;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PRIV;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::SECM;
    pub use super::CCR1::SSEC;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// channel x peripheral address register
pub mod CPAR4 {
    pub use super::CNDTR1::NDT;
}

/// channel x memory address register
pub mod CM0AR4 {
    pub use super::CPAR1::PA;
}

/// channel x memory address register
pub mod CM1AR4 {
    pub use super::CM0AR1::MA;
}

/// channel x configuration register
pub mod CCR5 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::CT;
    pub use super::CCR1::DBM;
    pub use super::CCR1::DIR;
    pub use super::CCR1::DSEC;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PRIV;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::SECM;
    pub use super::CCR1::SSEC;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// channel x number of data register
pub mod CNDTR5 {
    pub use super::CNDTR1::NDT;
}

/// channel x peripheral address register
pub mod CPAR5 {
    pub use super::CPAR1::PA;
}

/// channel x memory address register
pub mod CM0AR5 {
    pub use super::CM0AR1::MA;
}

/// channel x memory address register
pub mod CM1AR5 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::CT;
    pub use super::CCR1::DBM;
    pub use super::CCR1::DIR;
    pub use super::CCR1::DSEC;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PRIV;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::SECM;
    pub use super::CCR1::SSEC;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// channel x configuration register
pub mod CCR6 {
    pub use super::CNDTR1::NDT;
}

/// channel x number of data register
pub mod CNDTR6 {
    pub use super::CPAR1::PA;
}

/// channel x peripheral address register
pub mod CPAR6 {
    pub use super::CM0AR1::MA;
}

/// channel x memory address register
pub mod CM0AR6 {

    /// DMA channel 7 selection
    pub mod C7S {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA channel 6 selection
    pub mod C6S {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA channel 5 selection
    pub mod C5S {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA channel 4 selection
    pub mod C4S {
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

    /// DMA channel 3 selection
    pub mod C3S {
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

    /// DMA channel 2 selection
    pub mod C2S {
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

    /// DMA channel 1 selection
    pub mod C1S {
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

/// channel x memory address register
pub mod CM1AR6 {
    pub use super::CM0AR1::MA;
}

/// channel x configuration register
pub mod CCR7 {
    pub use super::CM0AR1::MA;
}

/// channel x number of data register
pub mod CNDTR7 {
    pub use super::CM0AR1::MA;
}

/// channel x peripheral address register
pub mod CPAR7 {
    pub use super::CM0AR1::MA;
}

/// channel x memory address register
pub mod CM0AR7 {
    pub use super::CM0AR1::MA;
}

/// channel x memory address register
pub mod CM1AR7 {
    pub use super::CM0AR1::MA;
}

/// channel x configuration register
pub mod CCR8 {
    pub use super::CM0AR1::MA;
}

/// channel x number of data register
pub mod CNDTR8 {

    /// channel enable
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

    /// transfer complete interrupt enable
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

    /// half transfer interrupt enable
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

    /// transfer error interrupt enable
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

    /// data transfer direction
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

    /// circular mode
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

    /// peripheral increment mode
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

    /// memory increment mode
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

    /// peripheral size
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

    /// memory size
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

    /// priority level
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

    /// memory-to-memory mode
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

    /// double-buffer mode
    pub mod DBM {
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

    /// current target memory of DMA transfer in double-buffer mode
    pub mod CT {
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

    /// secure mode
    pub mod SECM {
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

    /// security of the DMA transfer from the source
    pub mod SSEC {
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

    /// security of the DMA transfer to the destination
    pub mod DSEC {
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

    /// privileged mode
    pub mod PRIV {
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
}

/// channel x peripheral address register
pub mod CPAR8 {
    pub use super::CNDTR1::NDT;
}

/// channel x peripheral address register
pub mod CM0AR8 {
    pub use super::CPAR1::PA;
}

/// channel x peripheral address register
pub mod CM1AR8 {
    pub use super::CM0AR1::MA;
}

/// channel selection register
pub mod CSELR {
    pub use super::CM0AR1::MA;
}
#[repr(C)]
pub struct RegisterBlock {
    /// interrupt status register
    pub ISR: RORegister<u32>,

    /// interrupt flag clear register
    pub IFCR: WORegister<u32>,

    /// channel x configuration register
    pub CCR1: RWRegister<u32>,

    /// channel x number of data register
    pub CNDTR1: RWRegister<u32>,

    /// channel x peripheral address register
    pub CPAR1: UnsafeRWRegister<u32>,

    /// channel x memory address register
    pub CM0AR1: RWRegister<u32>,

    /// channel x memory address register
    pub CM1AR1: RWRegister<u32>,

    /// channel x configuration register
    pub CCR2: RWRegister<u32>,

    /// channel x number of data register
    pub CNDTR2: RWRegister<u32>,

    /// channel x peripheral address register
    pub CPAR2: UnsafeRWRegister<u32>,

    /// channel x memory address register
    pub CM0AR2: RWRegister<u32>,

    /// channel x memory address register
    pub CM1AR2: RWRegister<u32>,

    /// channel x configuration register
    pub CCR3: RWRegister<u32>,

    /// channel x number of data register
    pub CNDTR3: RWRegister<u32>,

    /// channel x peripheral address register
    pub CPAR3: UnsafeRWRegister<u32>,

    /// channel x memory address register
    pub CM0AR3: RWRegister<u32>,

    /// channel x memory address register
    pub CM1AR3: RWRegister<u32>,

    /// channel x configuration register
    pub CCR4: RWRegister<u32>,

    /// channel x number of data register
    pub CNDTR4: RWRegister<u32>,

    /// channel x peripheral address register
    pub CPAR4: UnsafeRWRegister<u32>,

    /// channel x memory address register
    pub CM0AR4: RWRegister<u32>,

    /// channel x memory address register
    pub CM1AR4: RWRegister<u32>,

    /// channel x configuration register
    pub CCR5: RWRegister<u32>,

    /// channel x number of data register
    pub CNDTR5: RWRegister<u32>,

    /// channel x peripheral address register
    pub CPAR5: UnsafeRWRegister<u32>,

    /// channel x memory address register
    pub CM0AR5: RWRegister<u32>,

    /// channel x memory address register
    pub CM1AR5: RWRegister<u32>,

    /// channel x configuration register
    pub CCR6: RWRegister<u32>,

    /// channel x number of data register
    pub CNDTR6: RWRegister<u32>,

    /// channel x peripheral address register
    pub CPAR6: UnsafeRWRegister<u32>,

    /// channel x memory address register
    pub CM0AR6: RWRegister<u32>,

    /// channel x memory address register
    pub CM1AR6: RWRegister<u32>,

    /// channel x configuration register
    pub CCR7: RWRegister<u32>,

    /// channel x number of data register
    pub CNDTR7: RWRegister<u32>,

    /// channel x peripheral address register
    pub CPAR7: UnsafeRWRegister<u32>,

    /// channel x memory address register
    pub CM0AR7: RWRegister<u32>,

    /// channel x memory address register
    pub CM1AR7: RWRegister<u32>,

    /// channel x configuration register
    pub CCR8: RWRegister<u32>,

    /// channel x number of data register
    pub CNDTR8: RWRegister<u32>,

    /// channel x peripheral address register
    pub CPAR8: UnsafeRWRegister<u32>,

    /// channel x peripheral address register
    pub CM0AR8: RWRegister<u32>,

    /// channel x peripheral address register
    pub CM1AR8: RWRegister<u32>,

    /// channel selection register
    pub CSELR: RWRegister<u32>,
}
pub struct ResetValues {
    pub ISR: u32,
    pub IFCR: u32,
    pub CCR1: u32,
    pub CNDTR1: u32,
    pub CPAR1: u32,
    pub CM0AR1: u32,
    pub CM1AR1: u32,
    pub CCR2: u32,
    pub CNDTR2: u32,
    pub CPAR2: u32,
    pub CM0AR2: u32,
    pub CM1AR2: u32,
    pub CCR3: u32,
    pub CNDTR3: u32,
    pub CPAR3: u32,
    pub CM0AR3: u32,
    pub CM1AR3: u32,
    pub CCR4: u32,
    pub CNDTR4: u32,
    pub CPAR4: u32,
    pub CM0AR4: u32,
    pub CM1AR4: u32,
    pub CCR5: u32,
    pub CNDTR5: u32,
    pub CPAR5: u32,
    pub CM0AR5: u32,
    pub CM1AR5: u32,
    pub CCR6: u32,
    pub CNDTR6: u32,
    pub CPAR6: u32,
    pub CM0AR6: u32,
    pub CM1AR6: u32,
    pub CCR7: u32,
    pub CNDTR7: u32,
    pub CPAR7: u32,
    pub CM0AR7: u32,
    pub CM1AR7: u32,
    pub CCR8: u32,
    pub CNDTR8: u32,
    pub CPAR8: u32,
    pub CM0AR8: u32,
    pub CM1AR8: u32,
    pub CSELR: u32,
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

/// Access functions for the DMA1 peripheral instance
pub mod DMA1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40020000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMA1
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IFCR: 0x00000000,
        CCR1: 0x00000000,
        CNDTR1: 0x00000000,
        CPAR1: 0x00000000,
        CM0AR1: 0x00000000,
        CM1AR1: 0x00000000,
        CCR2: 0x00000000,
        CNDTR2: 0x00000000,
        CPAR2: 0x00000000,
        CM0AR2: 0x00000000,
        CM1AR2: 0x00000000,
        CCR3: 0x00000000,
        CNDTR3: 0x00000000,
        CPAR3: 0x00000000,
        CM0AR3: 0x00000000,
        CM1AR3: 0x00000000,
        CCR4: 0x00000000,
        CNDTR4: 0x00000000,
        CPAR4: 0x00000000,
        CM0AR4: 0x00000000,
        CM1AR4: 0x00000000,
        CCR5: 0x00000000,
        CNDTR5: 0x00000000,
        CPAR5: 0x00000000,
        CM0AR5: 0x00000000,
        CM1AR5: 0x00000000,
        CCR6: 0x00000000,
        CNDTR6: 0x00000000,
        CPAR6: 0x00000000,
        CM0AR6: 0x00000000,
        CM1AR6: 0x00000000,
        CCR7: 0x00000000,
        CNDTR7: 0x00000000,
        CPAR7: 0x00000000,
        CM0AR7: 0x00000000,
        CM1AR7: 0x00000000,
        CCR8: 0x00000000,
        CNDTR8: 0x00000000,
        CPAR8: 0x00000000,
        CM0AR8: 0x00000000,
        CM1AR8: 0x00000000,
        CSELR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DMA1_TAKEN: bool = false;

    /// Safe access to DMA1
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
            if DMA1_TAKEN {
                None
            } else {
                DMA1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMA1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMA1_TAKEN && inst.addr == INSTANCE.addr {
                DMA1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DMA1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMA1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DMA1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMA1: *const RegisterBlock = 0x40020000 as *const _;

/// Access functions for the DMA2 peripheral instance
pub mod DMA2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40020400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMA2
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IFCR: 0x00000000,
        CCR1: 0x00000000,
        CNDTR1: 0x00000000,
        CPAR1: 0x00000000,
        CM0AR1: 0x00000000,
        CM1AR1: 0x00000000,
        CCR2: 0x00000000,
        CNDTR2: 0x00000000,
        CPAR2: 0x00000000,
        CM0AR2: 0x00000000,
        CM1AR2: 0x00000000,
        CCR3: 0x00000000,
        CNDTR3: 0x00000000,
        CPAR3: 0x00000000,
        CM0AR3: 0x00000000,
        CM1AR3: 0x00000000,
        CCR4: 0x00000000,
        CNDTR4: 0x00000000,
        CPAR4: 0x00000000,
        CM0AR4: 0x00000000,
        CM1AR4: 0x00000000,
        CCR5: 0x00000000,
        CNDTR5: 0x00000000,
        CPAR5: 0x00000000,
        CM0AR5: 0x00000000,
        CM1AR5: 0x00000000,
        CCR6: 0x00000000,
        CNDTR6: 0x00000000,
        CPAR6: 0x00000000,
        CM0AR6: 0x00000000,
        CM1AR6: 0x00000000,
        CCR7: 0x00000000,
        CNDTR7: 0x00000000,
        CPAR7: 0x00000000,
        CM0AR7: 0x00000000,
        CM1AR7: 0x00000000,
        CCR8: 0x00000000,
        CNDTR8: 0x00000000,
        CPAR8: 0x00000000,
        CM0AR8: 0x00000000,
        CM1AR8: 0x00000000,
        CSELR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DMA2_TAKEN: bool = false;

    /// Safe access to DMA2
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
            if DMA2_TAKEN {
                None
            } else {
                DMA2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMA2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMA2_TAKEN && inst.addr == INSTANCE.addr {
                DMA2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DMA2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMA2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DMA2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMA2: *const RegisterBlock = 0x40020400 as *const _;

/// Access functions for the SEC_DMA1 peripheral instance
pub mod SEC_DMA1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50020000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_DMA1
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IFCR: 0x00000000,
        CCR1: 0x00000000,
        CNDTR1: 0x00000000,
        CPAR1: 0x00000000,
        CM0AR1: 0x00000000,
        CM1AR1: 0x00000000,
        CCR2: 0x00000000,
        CNDTR2: 0x00000000,
        CPAR2: 0x00000000,
        CM0AR2: 0x00000000,
        CM1AR2: 0x00000000,
        CCR3: 0x00000000,
        CNDTR3: 0x00000000,
        CPAR3: 0x00000000,
        CM0AR3: 0x00000000,
        CM1AR3: 0x00000000,
        CCR4: 0x00000000,
        CNDTR4: 0x00000000,
        CPAR4: 0x00000000,
        CM0AR4: 0x00000000,
        CM1AR4: 0x00000000,
        CCR5: 0x00000000,
        CNDTR5: 0x00000000,
        CPAR5: 0x00000000,
        CM0AR5: 0x00000000,
        CM1AR5: 0x00000000,
        CCR6: 0x00000000,
        CNDTR6: 0x00000000,
        CPAR6: 0x00000000,
        CM0AR6: 0x00000000,
        CM1AR6: 0x00000000,
        CCR7: 0x00000000,
        CNDTR7: 0x00000000,
        CPAR7: 0x00000000,
        CM0AR7: 0x00000000,
        CM1AR7: 0x00000000,
        CCR8: 0x00000000,
        CNDTR8: 0x00000000,
        CPAR8: 0x00000000,
        CM0AR8: 0x00000000,
        CM1AR8: 0x00000000,
        CSELR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_DMA1_TAKEN: bool = false;

    /// Safe access to SEC_DMA1
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
            if SEC_DMA1_TAKEN {
                None
            } else {
                SEC_DMA1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_DMA1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_DMA1_TAKEN && inst.addr == INSTANCE.addr {
                SEC_DMA1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_DMA1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_DMA1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_DMA1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_DMA1: *const RegisterBlock = 0x50020000 as *const _;

/// Access functions for the SEC_DMA2 peripheral instance
pub mod SEC_DMA2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50020400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_DMA2
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IFCR: 0x00000000,
        CCR1: 0x00000000,
        CNDTR1: 0x00000000,
        CPAR1: 0x00000000,
        CM0AR1: 0x00000000,
        CM1AR1: 0x00000000,
        CCR2: 0x00000000,
        CNDTR2: 0x00000000,
        CPAR2: 0x00000000,
        CM0AR2: 0x00000000,
        CM1AR2: 0x00000000,
        CCR3: 0x00000000,
        CNDTR3: 0x00000000,
        CPAR3: 0x00000000,
        CM0AR3: 0x00000000,
        CM1AR3: 0x00000000,
        CCR4: 0x00000000,
        CNDTR4: 0x00000000,
        CPAR4: 0x00000000,
        CM0AR4: 0x00000000,
        CM1AR4: 0x00000000,
        CCR5: 0x00000000,
        CNDTR5: 0x00000000,
        CPAR5: 0x00000000,
        CM0AR5: 0x00000000,
        CM1AR5: 0x00000000,
        CCR6: 0x00000000,
        CNDTR6: 0x00000000,
        CPAR6: 0x00000000,
        CM0AR6: 0x00000000,
        CM1AR6: 0x00000000,
        CCR7: 0x00000000,
        CNDTR7: 0x00000000,
        CPAR7: 0x00000000,
        CM0AR7: 0x00000000,
        CM1AR7: 0x00000000,
        CCR8: 0x00000000,
        CNDTR8: 0x00000000,
        CPAR8: 0x00000000,
        CM0AR8: 0x00000000,
        CM1AR8: 0x00000000,
        CSELR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_DMA2_TAKEN: bool = false;

    /// Safe access to SEC_DMA2
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
            if SEC_DMA2_TAKEN {
                None
            } else {
                SEC_DMA2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_DMA2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_DMA2_TAKEN && inst.addr == INSTANCE.addr {
                SEC_DMA2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_DMA2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_DMA2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_DMA2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_DMA2: *const RegisterBlock = 0x50020400 as *const _;
