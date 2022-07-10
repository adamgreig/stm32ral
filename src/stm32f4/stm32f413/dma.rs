#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA controller

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

    /// Stream x half transfer interrupt flag (x=3..0)
    pub mod HTIF3 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
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

    /// Stream x transfer error interrupt flag (x=3..0)
    pub mod TEIF3 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
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

    /// Stream x direct mode error interrupt flag (x=3..0)
    pub mod DMEIF3 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No Direct Mode error on stream x
            pub const NoError: u32 = 0b0;

            /// 0b1: A Direct Mode error occurred on stream x
            pub const Error: u32 = 0b1;
        }
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
        /// Read-only values
        pub mod R {

            /// 0b0: No FIFO error event on stream x
            pub const NoError: u32 = 0b0;

            /// 0b1: A FIFO error event occurred on stream x
            pub const Error: u32 = 0b1;
        }
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
        pub use super::TCIF3::R;
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
        pub use super::HTIF3::R;
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
        pub use super::TEIF3::R;
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
        pub use super::DMEIF3::R;
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
        pub use super::FEIF3::R;
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
        pub use super::TCIF3::R;
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
        pub use super::HTIF3::R;
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
        pub use super::TEIF3::R;
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
        pub use super::DMEIF3::R;
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
        pub use super::FEIF3::R;
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
        pub use super::TCIF3::R;
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
        pub use super::HTIF3::R;
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
        pub use super::TEIF3::R;
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
        pub use super::DMEIF3::R;
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
        pub use super::FEIF3::R;
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

    /// Stream x half transfer interrupt flag (x=7..4)
    pub mod HTIF7 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
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

    /// Stream x transfer error interrupt flag (x=7..4)
    pub mod TEIF7 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
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

    /// Stream x direct mode error interrupt flag (x=7..4)
    pub mod DMEIF7 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No Direct Mode error on stream x
            pub const NoError: u32 = 0b0;

            /// 0b1: A Direct Mode error occurred on stream x
            pub const Error: u32 = 0b1;
        }
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
        /// Read-only values
        pub mod R {

            /// 0b0: No FIFO error event on stream x
            pub const NoError: u32 = 0b0;

            /// 0b1: A FIFO error event occurred on stream x
            pub const Error: u32 = 0b1;
        }
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
        pub use super::TCIF7::R;
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
        pub use super::HTIF7::R;
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
        pub use super::TEIF7::R;
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
        pub use super::DMEIF7::R;
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
        pub use super::FEIF7::R;
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
        pub use super::TCIF7::R;
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
        pub use super::HTIF7::R;
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
        pub use super::TEIF7::R;
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
        pub use super::DMEIF7::R;
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
        pub use super::FEIF7::R;
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
        pub use super::TCIF7::R;
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
        pub use super::HTIF7::R;
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
        pub use super::TEIF7::R;
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
        pub use super::DMEIF7::R;
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
        pub use super::FEIF7::R;
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the corresponding TCIFx flag
            pub const Clear: u32 = 0b1;
        }
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the corresponding HTIFx flag
            pub const Clear: u32 = 0b1;
        }
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the corresponding TEIFx flag
            pub const Clear: u32 = 0b1;
        }
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the corresponding DMEIFx flag
            pub const Clear: u32 = 0b1;
        }
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the corresponding CFEIFx flag
            pub const Clear: u32 = 0b1;
        }
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
        pub use super::CTCIF3::W;
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
        pub use super::CHTIF3::W;
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
        pub use super::CTEIF3::W;
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
        pub use super::CDMEIF3::W;
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
        pub use super::CFEIF3::W;
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
        pub use super::CTCIF3::W;
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
        pub use super::CHTIF3::W;
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
        pub use super::CTEIF3::W;
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
        pub use super::CDMEIF3::W;
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
        pub use super::CFEIF3::W;
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
        pub use super::CTCIF3::W;
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
        pub use super::CHTIF3::W;
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
        pub use super::CTEIF3::W;
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
        pub use super::CDMEIF3::W;
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
        pub use super::CFEIF3::W;
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the corresponding TCIFx flag
            pub const Clear: u32 = 0b1;
        }
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the corresponding HTIFx flag
            pub const Clear: u32 = 0b1;
        }
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the corresponding TEIFx flag
            pub const Clear: u32 = 0b1;
        }
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the corresponding DMEIFx flag
            pub const Clear: u32 = 0b1;
        }
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the corresponding CFEIFx flag
            pub const Clear: u32 = 0b1;
        }
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
        pub use super::CTCIF7::W;
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
        pub use super::CHTIF7::W;
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
        pub use super::CTEIF7::W;
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
        pub use super::CDMEIF7::W;
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
        pub use super::CFEIF7::W;
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
        pub use super::CTCIF7::W;
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
        pub use super::CHTIF7::W;
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
        pub use super::CTEIF7::W;
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
        pub use super::CDMEIF7::W;
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
        pub use super::CFEIF7::W;
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
        pub use super::CTCIF7::W;
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
        pub use super::CHTIF7::W;
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
        pub use super::CTEIF7::W;
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
        pub use super::CDMEIF7::W;
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
        pub use super::CFEIF7::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// stream x configuration register
pub mod CR0 {

    /// Channel selection
    pub mod CHSEL {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (4 bits: 0b1111 << 25)
        pub const mask: u32 = 0b1111 << offset;
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
        /// Read-write values
        pub mod RW {

            /// 0b00: Single transfer
            pub const Single: u32 = 0b00;

            /// 0b01: Incremental burst of 4 beats
            pub const INCR4: u32 = 0b01;

            /// 0b10: Incremental burst of 8 beats
            pub const INCR8: u32 = 0b10;

            /// 0b11: Incremental burst of 16 beats
            pub const INCR16: u32 = 0b11;
        }
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
        pub use super::MBURST::RW;
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
        /// Read-write values
        pub mod RW {

            /// 0b0: The current target memory is Memory 0
            pub const Memory0: u32 = 0b0;

            /// 0b1: The current target memory is Memory 1
            pub const Memory1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: No buffer switching at the end of transfer
            pub const Disabled: u32 = 0b0;

            /// 0b1: Memory target switched at the end of the DMA transfer
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: The offset size for the peripheral address calculation is linked to the PSIZE
            pub const PSIZE: u32 = 0b0;

            /// 0b1: The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)
            pub const Fixed4: u32 = 0b1;
        }
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
        pub use super::MSIZE::RW;
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
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MINC::RW;
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
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Peripheral-to-memory
            pub const PeripheralToMemory: u32 = 0b00;

            /// 0b01: Memory-to-peripheral
            pub const MemoryToPeripheral: u32 = 0b01;

            /// 0b10: Memory-to-memory
            pub const MemoryToMemory: u32 = 0b10;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: The DMA is the flow controller
            pub const DMA: u32 = 0b0;

            /// 0b1: The peripheral is the flow controller
            pub const Peripheral: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: TC interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: TC interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: HT interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: HT interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: TE interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: TE interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: DME interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DME interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Stream disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Stream enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: FE interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: FE interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// FIFO status
    pub mod FS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values
        pub mod R {

            /// 0b000: 0 < fifo_level < 1/4
            pub const Quarter1: u32 = 0b000;

            /// 0b001: 1/4 <= fifo_level < 1/2
            pub const Quarter2: u32 = 0b001;

            /// 0b010: 1/2 <= fifo_level < 3/4
            pub const Quarter3: u32 = 0b010;

            /// 0b011: 3/4 <= fifo_level < full
            pub const Quarter4: u32 = 0b011;

            /// 0b100: FIFO is empty
            pub const Empty: u32 = 0b100;

            /// 0b101: FIFO is full
            pub const Full: u32 = 0b101;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Direct mode is enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: Direct mode is disabled
            pub const Disabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b00: 1/4 full FIFO
            pub const Quarter: u32 = 0b00;

            /// 0b01: 1/2 full FIFO
            pub const Half: u32 = 0b01;

            /// 0b10: 3/4 full FIFO
            pub const ThreeQuarters: u32 = 0b10;

            /// 0b11: Full FIFO
            pub const Full: u32 = 0b11;
        }
    }
}

/// stream x configuration register
pub mod CR1 {
    pub use super::CR0::CHSEL;
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
    pub use super::CR0::CHSEL;
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
    pub use super::CR0::CHSEL;
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
    pub use super::CR0::CHSEL;
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
    pub use super::CR0::CHSEL;
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
    pub use super::CR0::CHSEL;
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
    pub use super::CR0::CHSEL;
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
#[repr(C)]
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
#[cfg(feature = "rtic")]
unsafe impl Send for Instance {}

/// Access functions for the DMA1 peripheral instance
pub mod DMA1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40026000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMA1
    pub const reset: ResetValues = ResetValues {
        LISR: 0x00000000,
        HISR: 0x00000000,
        LIFCR: 0x00000000,
        HIFCR: 0x00000000,
        CR0: 0x00000000,
        NDTR0: 0x00000000,
        PAR0: 0x00000000,
        M0AR0: 0x00000000,
        M1AR0: 0x00000000,
        FCR0: 0x00000021,
        CR1: 0x00000000,
        NDTR1: 0x00000000,
        PAR1: 0x00000000,
        M0AR1: 0x00000000,
        M1AR1: 0x00000000,
        FCR1: 0x00000021,
        CR2: 0x00000000,
        NDTR2: 0x00000000,
        PAR2: 0x00000000,
        M0AR2: 0x00000000,
        M1AR2: 0x00000000,
        FCR2: 0x00000021,
        CR3: 0x00000000,
        NDTR3: 0x00000000,
        PAR3: 0x00000000,
        M0AR3: 0x00000000,
        M1AR3: 0x00000000,
        FCR3: 0x00000021,
        CR4: 0x00000000,
        NDTR4: 0x00000000,
        PAR4: 0x00000000,
        M0AR4: 0x00000000,
        M1AR4: 0x00000000,
        FCR4: 0x00000021,
        CR5: 0x00000000,
        NDTR5: 0x00000000,
        PAR5: 0x00000000,
        M0AR5: 0x00000000,
        M1AR5: 0x00000000,
        FCR5: 0x00000021,
        CR6: 0x00000000,
        NDTR6: 0x00000000,
        PAR6: 0x00000000,
        M0AR6: 0x00000000,
        M1AR6: 0x00000000,
        FCR6: 0x00000021,
        CR7: 0x00000000,
        NDTR7: 0x00000000,
        PAR7: 0x00000000,
        M0AR7: 0x00000000,
        M1AR7: 0x00000000,
        FCR7: 0x00000021,
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
pub const DMA1: *const RegisterBlock = 0x40026000 as *const _;

/// Access functions for the DMA2 peripheral instance
pub mod DMA2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40026400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMA2
    pub const reset: ResetValues = ResetValues {
        LISR: 0x00000000,
        HISR: 0x00000000,
        LIFCR: 0x00000000,
        HIFCR: 0x00000000,
        CR0: 0x00000000,
        NDTR0: 0x00000000,
        PAR0: 0x00000000,
        M0AR0: 0x00000000,
        M1AR0: 0x00000000,
        FCR0: 0x00000021,
        CR1: 0x00000000,
        NDTR1: 0x00000000,
        PAR1: 0x00000000,
        M0AR1: 0x00000000,
        M1AR1: 0x00000000,
        FCR1: 0x00000021,
        CR2: 0x00000000,
        NDTR2: 0x00000000,
        PAR2: 0x00000000,
        M0AR2: 0x00000000,
        M1AR2: 0x00000000,
        FCR2: 0x00000021,
        CR3: 0x00000000,
        NDTR3: 0x00000000,
        PAR3: 0x00000000,
        M0AR3: 0x00000000,
        M1AR3: 0x00000000,
        FCR3: 0x00000021,
        CR4: 0x00000000,
        NDTR4: 0x00000000,
        PAR4: 0x00000000,
        M0AR4: 0x00000000,
        M1AR4: 0x00000000,
        FCR4: 0x00000021,
        CR5: 0x00000000,
        NDTR5: 0x00000000,
        PAR5: 0x00000000,
        M0AR5: 0x00000000,
        M1AR5: 0x00000000,
        FCR5: 0x00000021,
        CR6: 0x00000000,
        NDTR6: 0x00000000,
        PAR6: 0x00000000,
        M0AR6: 0x00000000,
        M1AR6: 0x00000000,
        FCR6: 0x00000021,
        CR7: 0x00000000,
        NDTR7: 0x00000000,
        PAR7: 0x00000000,
        M0AR7: 0x00000000,
        M1AR7: 0x00000000,
        FCR7: 0x00000021,
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
pub const DMA2: *const RegisterBlock = 0x40026400 as *const _;
