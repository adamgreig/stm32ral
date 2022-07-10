#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Basic Direct Memory Access

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Interrupt status register
pub mod ISR {

    /// Transfer error (TE) flag for channel x
    pub mod TEIF7 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No transfer error on stream x
            pub const NoError: u32 = 0b0;

            /// 0b1: A transfer error occurred on stream x
            pub const Error: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half transfer (HT) flag for channel x
    pub mod HTIF7 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No half transfer event on stream x
            pub const NotHalf: u32 = 0b0;

            /// 0b1: A half transfer event occurred on stream x
            pub const Half: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer complete (TC) flag for channel x
    pub mod TCIF7 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No transfer complete event on stream x
            pub const NotComplete: u32 = 0b0;

            /// 0b1: A transfer complete event occurred on stream x
            pub const Complete: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Global interrupt flag for channel x
    pub mod GIF7 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
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

    /// Transfer error (TE) flag for channel x
    pub mod TEIF6 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        pub use super::TEIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half transfer (HT) flag for channel x
    pub mod HTIF6 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        pub use super::HTIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer complete (TC) flag for channel x
    pub mod TCIF6 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        pub use super::TCIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Global interrupt flag for channel x
    pub mod GIF6 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        pub use super::GIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer error (TE) flag for channel x
    pub mod TEIF5 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        pub use super::TEIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half transfer (HT) flag for channel x
    pub mod HTIF5 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        pub use super::HTIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer complete (TC) flag for channel x
    pub mod TCIF5 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        pub use super::TCIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Global interrupt flag for channel x
    pub mod GIF5 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        pub use super::GIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer error (TE) flag for channel x
    pub mod TEIF4 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        pub use super::TEIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half transfer (HT) flag for channel x
    pub mod HTIF4 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        pub use super::HTIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer complete (TC) flag for channel x
    pub mod TCIF4 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        pub use super::TCIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Global interrupt flag for channel x
    pub mod GIF4 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        pub use super::GIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer error (TE) flag for channel x
    pub mod TEIF3 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        pub use super::TEIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half transfer (HT) flag for channel x
    pub mod HTIF3 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        pub use super::HTIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer complete (TC) flag for channel x
    pub mod TCIF3 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        pub use super::TCIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Global interrupt flag for channel x
    pub mod GIF3 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        pub use super::GIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer error (TE) flag for channel x
    pub mod TEIF2 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        pub use super::TEIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half transfer (HT) flag for channel x
    pub mod HTIF2 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        pub use super::HTIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer complete (TC) flag for channel x
    pub mod TCIF2 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        pub use super::TCIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Global interrupt flag for channel x
    pub mod GIF2 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        pub use super::GIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer error (TE) flag for channel x
    pub mod TEIF1 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        pub use super::TEIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half transfer (HT) flag for channel x
    pub mod HTIF1 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        pub use super::HTIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer complete (TC) flag for channel x
    pub mod TCIF1 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        pub use super::TCIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Global interrupt flag for channel x
    pub mod GIF1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::GIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer error (TE) flag for channel x
    pub mod TEIF0 {
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

    /// Half transfer (HT) flag for channel x
    pub mod HTIF0 {
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

    /// Transfer complete (TC) flag for channel x
    pub mod TCIF0 {
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

    /// Global interrupt flag for channel x
    pub mod GIF0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        pub use super::GIF7::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt flag clear register
pub mod IFCR {

    /// Transfer error (TE) flag clear for channel x
    pub mod CTEIF7 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
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

    /// Half transfer (HT) flag clear for channel x
    pub mod CHTIF7 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
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

    /// Transfer complete (TC) flag clear for channel x
    pub mod CTCIF7 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
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

    /// Global interrupt flag clear for channel x
    pub mod CGIF7 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
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

    /// Transfer error (TE) flag clear for channel x
    pub mod CTEIF6 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTEIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half transfer (HT) flag clear for channel x
    pub mod CHTIF6 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CHTIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer complete (TC) flag clear for channel x
    pub mod CTCIF6 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTCIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Global interrupt flag clear for channel x
    pub mod CGIF6 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CGIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer error (TE) flag clear for channel x
    pub mod CTEIF5 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTEIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half transfer (HT) flag clear for channel x
    pub mod CHTIF5 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CHTIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer complete (TC) flag clear for channel x
    pub mod CTCIF5 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTCIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Global interrupt flag clear for channel x
    pub mod CGIF5 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CGIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer error (TE) flag clear for channel x
    pub mod CTEIF4 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTEIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half transfer (HT) flag clear for channel x
    pub mod CHTIF4 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CHTIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer complete (TC) flag clear for channel x
    pub mod CTCIF4 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTCIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Global interrupt flag clear for channel x
    pub mod CGIF4 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CGIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer error (TE) flag clear for channel x
    pub mod CTEIF3 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTEIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half transfer (HT) flag clear for channel x
    pub mod CHTIF3 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CHTIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer complete (TC) flag clear for channel x
    pub mod CTCIF3 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTCIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Global interrupt flag clear for channel x
    pub mod CGIF3 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CGIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer error (TE) flag clear for channel x
    pub mod CTEIF2 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTEIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half transfer (HT) flag clear for channel x
    pub mod CHTIF2 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CHTIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer complete (TC) flag clear for channel x
    pub mod CTCIF2 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTCIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Global interrupt flag clear for channel x
    pub mod CGIF2 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CGIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer error (TE) flag clear for channel x
    pub mod CTEIF1 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTEIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half transfer (HT) flag clear for channel x
    pub mod CHTIF1 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CHTIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer complete (TC) flag clear for channel x
    pub mod CTCIF1 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTCIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Global interrupt flag clear for channel x
    pub mod CGIF1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CGIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer error (TE) flag clear for channel x
    pub mod CTEIF0 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTEIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half transfer (HT) flag clear for channel x
    pub mod CHTIF0 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CHTIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer complete (TC) flag clear for channel x
    pub mod CTCIF0 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTCIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Global interrupt flag clear for channel x
    pub mod CGIF0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CGIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel x configuration register
pub mod CR0 {

    /// Current target memory of DMA transfer in double-buffer mode
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

    /// Memory-to-memory mode
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

    /// Priority level
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
        pub use super::MSIZE::RW;
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Address pointer is fixed
            pub const Fixed: u32 = 0b0;

            /// 0b1: Address pointer is incremented after each data transfer
            pub const Incremented: u32 = 0b1;
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
        pub use super::MINC::RW;
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

            /// 0b0: Circular mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Circular mode enabled
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

            /// 0b0: Peripheral-to-memory
            pub const PeripheralToMemory: u32 = 0b0;

            /// 0b1: Memory-to-peripheral
            pub const MemoryToPeripheral: u32 = 0b1;
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

            /// 0b0: TE interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: TE interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Half transfer error interrupt enable
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

            /// 0b0: TC interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: TC interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Channel enable
    pub mod EN {
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

            /// 0b0: Channel disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Channel enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Channel x number of data to transfer register
pub mod NDTR0 {

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

/// Channel x peripheral address register
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

/// Channel x memory 0 address register
pub mod M0AR0 {

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

/// Channel x memory 1 address register
pub mod M1AR0 {
    pub use super::M0AR0::MA;
}

/// Channel x configuration register
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

/// Channel x number of data to transfer register
pub mod NDTR1 {
    pub use super::NDTR0::NDT;
}

/// Channel x peripheral address register
pub mod PAR1 {
    pub use super::PAR0::PA;
}

/// Channel x memory 0 address register
pub mod M0AR1 {
    pub use super::M0AR0::MA;
}

/// Channel x memory 1 address register
pub mod M1AR1 {
    pub use super::M0AR0::MA;
}

/// Channel x configuration register
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

/// Channel x number of data to transfer register
pub mod NDTR2 {
    pub use super::NDTR0::NDT;
}

/// Channel x peripheral address register
pub mod PAR2 {
    pub use super::PAR0::PA;
}

/// Channel x memory 0 address register
pub mod M0AR2 {
    pub use super::M0AR0::MA;
}

/// Channel x memory 1 address register
pub mod M1AR2 {
    pub use super::M0AR0::MA;
}

/// Channel x configuration register
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

/// Channel x number of data to transfer register
pub mod NDTR3 {
    pub use super::NDTR0::NDT;
}

/// Channel x peripheral address register
pub mod PAR3 {
    pub use super::PAR0::PA;
}

/// Channel x memory 0 address register
pub mod M0AR3 {
    pub use super::M0AR0::MA;
}

/// Channel x memory 1 address register
pub mod M1AR3 {
    pub use super::M0AR0::MA;
}

/// Channel x configuration register
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

/// Channel x number of data to transfer register
pub mod NDTR4 {
    pub use super::NDTR0::NDT;
}

/// Channel x peripheral address register
pub mod PAR4 {
    pub use super::PAR0::PA;
}

/// Channel x memory 0 address register
pub mod M0AR4 {
    pub use super::M0AR0::MA;
}

/// Channel x memory 1 address register
pub mod M1AR4 {
    pub use super::M0AR0::MA;
}

/// Channel x configuration register
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

/// Channel x number of data to transfer register
pub mod NDTR5 {
    pub use super::NDTR0::NDT;
}

/// Channel x peripheral address register
pub mod PAR5 {
    pub use super::PAR0::PA;
}

/// Channel x memory 0 address register
pub mod M0AR5 {
    pub use super::M0AR0::MA;
}

/// Channel x memory 1 address register
pub mod M1AR5 {
    pub use super::M0AR0::MA;
}

/// Channel x configuration register
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

/// Channel x number of data to transfer register
pub mod NDTR6 {
    pub use super::NDTR0::NDT;
}

/// Channel x peripheral address register
pub mod PAR6 {
    pub use super::PAR0::PA;
}

/// Channel x memory 0 address register
pub mod M0AR6 {
    pub use super::M0AR0::MA;
}

/// Channel x memory 1 address register
pub mod M1AR6 {
    pub use super::M0AR0::MA;
}

/// Channel x configuration register
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

/// Channel x number of data to transfer register
pub mod NDTR7 {
    pub use super::NDTR0::NDT;
}

/// Channel x peripheral address register
pub mod PAR7 {
    pub use super::PAR0::PA;
}

/// Channel x memory 0 address register
pub mod M0AR7 {
    pub use super::M0AR0::MA;
}

/// Channel x memory 1 address register
pub mod M1AR7 {
    pub use super::M0AR0::MA;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt status register
    pub ISR: RORegister<u32>,

    /// Interrupt flag clear register
    pub IFCR: WORegister<u32>,

    /// Channel x configuration register
    pub CR0: RWRegister<u32>,

    /// Channel x number of data to transfer register
    pub NDTR0: RWRegister<u32>,

    /// Channel x peripheral address register
    pub PAR0: RWRegister<u32>,

    /// Channel x memory 0 address register
    pub M0AR0: RWRegister<u32>,

    /// Channel x memory 1 address register
    pub M1AR0: RWRegister<u32>,

    /// Channel x configuration register
    pub CR1: RWRegister<u32>,

    /// Channel x number of data to transfer register
    pub NDTR1: RWRegister<u32>,

    /// Channel x peripheral address register
    pub PAR1: RWRegister<u32>,

    /// Channel x memory 0 address register
    pub M0AR1: RWRegister<u32>,

    /// Channel x memory 1 address register
    pub M1AR1: RWRegister<u32>,

    /// Channel x configuration register
    pub CR2: RWRegister<u32>,

    /// Channel x number of data to transfer register
    pub NDTR2: RWRegister<u32>,

    /// Channel x peripheral address register
    pub PAR2: RWRegister<u32>,

    /// Channel x memory 0 address register
    pub M0AR2: RWRegister<u32>,

    /// Channel x memory 1 address register
    pub M1AR2: RWRegister<u32>,

    /// Channel x configuration register
    pub CR3: RWRegister<u32>,

    /// Channel x number of data to transfer register
    pub NDTR3: RWRegister<u32>,

    /// Channel x peripheral address register
    pub PAR3: RWRegister<u32>,

    /// Channel x memory 0 address register
    pub M0AR3: RWRegister<u32>,

    /// Channel x memory 1 address register
    pub M1AR3: RWRegister<u32>,

    /// Channel x configuration register
    pub CR4: RWRegister<u32>,

    /// Channel x number of data to transfer register
    pub NDTR4: RWRegister<u32>,

    /// Channel x peripheral address register
    pub PAR4: RWRegister<u32>,

    /// Channel x memory 0 address register
    pub M0AR4: RWRegister<u32>,

    /// Channel x memory 1 address register
    pub M1AR4: RWRegister<u32>,

    /// Channel x configuration register
    pub CR5: RWRegister<u32>,

    /// Channel x number of data to transfer register
    pub NDTR5: RWRegister<u32>,

    /// Channel x peripheral address register
    pub PAR5: RWRegister<u32>,

    /// Channel x memory 0 address register
    pub M0AR5: RWRegister<u32>,

    /// Channel x memory 1 address register
    pub M1AR5: RWRegister<u32>,

    /// Channel x configuration register
    pub CR6: RWRegister<u32>,

    /// Channel x number of data to transfer register
    pub NDTR6: RWRegister<u32>,

    /// Channel x peripheral address register
    pub PAR6: RWRegister<u32>,

    /// Channel x memory 0 address register
    pub M0AR6: RWRegister<u32>,

    /// Channel x memory 1 address register
    pub M1AR6: RWRegister<u32>,

    /// Channel x configuration register
    pub CR7: RWRegister<u32>,

    /// Channel x number of data to transfer register
    pub NDTR7: RWRegister<u32>,

    /// Channel x peripheral address register
    pub PAR7: RWRegister<u32>,

    /// Channel x memory 0 address register
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

/// Access functions for the BDMA peripheral instance
pub mod BDMA {
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
        ISR: 0x00000000,
        IFCR: 0x00000000,
        CR0: 0x00000000,
        NDTR0: 0x00000000,
        PAR0: 0x00000000,
        M0AR0: 0x00000000,
        M1AR0: 0x00000000,
        CR1: 0x00000000,
        NDTR1: 0x00000000,
        PAR1: 0x00000000,
        M0AR1: 0x00000000,
        M1AR1: 0x00000000,
        CR2: 0x00000000,
        NDTR2: 0x00000000,
        PAR2: 0x00000000,
        M0AR2: 0x00000000,
        M1AR2: 0x00000000,
        CR3: 0x00000000,
        NDTR3: 0x00000000,
        PAR3: 0x00000000,
        M0AR3: 0x00000000,
        M1AR3: 0x00000000,
        CR4: 0x00000000,
        NDTR4: 0x00000000,
        PAR4: 0x00000000,
        M0AR4: 0x00000000,
        M1AR4: 0x00000000,
        CR5: 0x00000000,
        NDTR5: 0x00000000,
        PAR5: 0x00000000,
        M0AR5: 0x00000000,
        M1AR5: 0x00000000,
        CR6: 0x00000000,
        NDTR6: 0x00000000,
        PAR6: 0x00000000,
        M0AR6: 0x00000000,
        M1AR6: 0x00000000,
        CR7: 0x00000000,
        NDTR7: 0x00000000,
        PAR7: 0x00000000,
        M0AR7: 0x00000000,
        M1AR7: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
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

    /// Unsafely steal BDMA
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        BDMA_TAKEN = true;
        INSTANCE
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
