#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMAMUX1

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DMAMUX request line multiplexer channel 0 configuration register
pub mod DMAMUX_C0CR {

    /// DMAREQ_ID
    pub mod DMAREQ_ID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOIE
    pub mod SOIE {
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

    /// EGE
    pub mod EGE {
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

    /// SE
    pub mod SE {
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

    /// SPOL
    pub mod SPOL {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (2 bits: 0b11 << 17)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NBREQ
    pub mod NBREQ {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (5 bits: 0b11111 << 19)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SYNC_ID
    pub mod SYNC_ID {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMAMUX request line multiplexer channel 1 configuration register
pub mod DMAMUX_C1CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel 2 configuration register
pub mod DMAMUX_C2CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel 3 configuration register
pub mod DMAMUX_C3CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel 4 configuration register
pub mod DMAMUX_C4CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel 5 configuration register
pub mod DMAMUX_C5CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel 6 configuration register
pub mod DMAMUX_C6CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel 7 configuration register
pub mod DMAMUX_C7CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel 8 configuration register
pub mod DMAMUX_C8CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel 9 configuration register
pub mod DMAMUX_C9CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel 10 configuration register
pub mod DMAMUX_C10CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel 11 configuration register
pub mod DMAMUX_C11CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel 12 configuration register
pub mod DMAMUX_C12CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel 13 configuration register
pub mod DMAMUX_C13CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel 14 configuration register
pub mod DMAMUX_C14CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel 15 configuration register
pub mod DMAMUX_C15CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer interrupt channel status register
pub mod DMAMUX_CSR {

    /// SOF0
    pub mod SOF0 {
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

    /// SOF1
    pub mod SOF1 {
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

    /// SOF2
    pub mod SOF2 {
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

    /// SOF3
    pub mod SOF3 {
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

    /// SOF4
    pub mod SOF4 {
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

    /// SOF5
    pub mod SOF5 {
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

    /// SOF6
    pub mod SOF6 {
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

    /// SOF7
    pub mod SOF7 {
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

    /// SOF8
    pub mod SOF8 {
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

    /// SOF9
    pub mod SOF9 {
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

    /// SOF10
    pub mod SOF10 {
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

    /// SOF11
    pub mod SOF11 {
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

    /// SOF12
    pub mod SOF12 {
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

    /// SOF13
    pub mod SOF13 {
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

    /// SOF14
    pub mod SOF14 {
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

    /// SOF15
    pub mod SOF15 {
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
}

/// DMAMUX request line multiplexer interrupt clear flag register
pub mod DMAMUX_CFR {

    /// CSOF0
    pub mod CSOF0 {
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

    /// CSOF1
    pub mod CSOF1 {
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

    /// CSOF2
    pub mod CSOF2 {
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

    /// CSOF3
    pub mod CSOF3 {
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

    /// CSOF4
    pub mod CSOF4 {
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

    /// CSOF5
    pub mod CSOF5 {
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

    /// CSOF6
    pub mod CSOF6 {
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

    /// CSOF7
    pub mod CSOF7 {
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

    /// CSOF8
    pub mod CSOF8 {
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

    /// CSOF9
    pub mod CSOF9 {
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

    /// CSOF10
    pub mod CSOF10 {
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

    /// CSOF11
    pub mod CSOF11 {
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

    /// CSOF12
    pub mod CSOF12 {
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

    /// CSOF13
    pub mod CSOF13 {
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

    /// CSOF14
    pub mod CSOF14 {
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

    /// CSOF15
    pub mod CSOF15 {
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
}

/// DMAMUX request generator channel 0 configuration register
pub mod DMAMUX_RG0CR {

    /// SIG_ID
    pub mod SIG_ID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OIE
    pub mod OIE {
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

    /// GE
    pub mod GE {
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

    /// GPOL
    pub mod GPOL {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (2 bits: 0b11 << 17)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GNBREQ
    pub mod GNBREQ {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (5 bits: 0b11111 << 19)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMAMUX request generator channel 1 configuration register
pub mod DMAMUX_RG1CR {
    pub use super::DMAMUX_RG0CR::GE;
    pub use super::DMAMUX_RG0CR::GNBREQ;
    pub use super::DMAMUX_RG0CR::GPOL;
    pub use super::DMAMUX_RG0CR::OIE;
    pub use super::DMAMUX_RG0CR::SIG_ID;
}

/// DMAMUX request generator channel 2 configuration register
pub mod DMAMUX_RG2CR {
    pub use super::DMAMUX_RG0CR::GE;
    pub use super::DMAMUX_RG0CR::GNBREQ;
    pub use super::DMAMUX_RG0CR::GPOL;
    pub use super::DMAMUX_RG0CR::OIE;
    pub use super::DMAMUX_RG0CR::SIG_ID;
}

/// DMAMUX request generator channel 3 configuration register
pub mod DMAMUX_RG3CR {
    pub use super::DMAMUX_RG0CR::GE;
    pub use super::DMAMUX_RG0CR::GNBREQ;
    pub use super::DMAMUX_RG0CR::GPOL;
    pub use super::DMAMUX_RG0CR::OIE;
    pub use super::DMAMUX_RG0CR::SIG_ID;
}

/// DMAMUX request generator channel 4 configuration register
pub mod DMAMUX_RG4CR {
    pub use super::DMAMUX_RG0CR::GE;
    pub use super::DMAMUX_RG0CR::GNBREQ;
    pub use super::DMAMUX_RG0CR::GPOL;
    pub use super::DMAMUX_RG0CR::OIE;
    pub use super::DMAMUX_RG0CR::SIG_ID;
}

/// DMAMUX request generator channel 5 configuration register
pub mod DMAMUX_RG5CR {
    pub use super::DMAMUX_RG0CR::GE;
    pub use super::DMAMUX_RG0CR::GNBREQ;
    pub use super::DMAMUX_RG0CR::GPOL;
    pub use super::DMAMUX_RG0CR::OIE;
    pub use super::DMAMUX_RG0CR::SIG_ID;
}

/// DMAMUX request generator channel 6 configuration register
pub mod DMAMUX_RG6CR {
    pub use super::DMAMUX_RG0CR::GE;
    pub use super::DMAMUX_RG0CR::GNBREQ;
    pub use super::DMAMUX_RG0CR::GPOL;
    pub use super::DMAMUX_RG0CR::OIE;
    pub use super::DMAMUX_RG0CR::SIG_ID;
}

/// DMAMUX request generator channel 7 configuration register
pub mod DMAMUX_RG7CR {
    pub use super::DMAMUX_RG0CR::GE;
    pub use super::DMAMUX_RG0CR::GNBREQ;
    pub use super::DMAMUX_RG0CR::GPOL;
    pub use super::DMAMUX_RG0CR::OIE;
    pub use super::DMAMUX_RG0CR::SIG_ID;
}

/// DMAMUX request generator interrupt status register
pub mod DMAMUX_RGSR {

    /// OF0
    pub mod OF0 {
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

    /// OF1
    pub mod OF1 {
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

    /// OF2
    pub mod OF2 {
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

    /// OF3
    pub mod OF3 {
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

    /// OF4
    pub mod OF4 {
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

    /// OF5
    pub mod OF5 {
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

    /// OF6
    pub mod OF6 {
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

    /// OF7
    pub mod OF7 {
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

/// DMAMUX request generator interrupt clear flag register
pub mod DMAMUX_RGCFR {

    /// COF0
    pub mod COF0 {
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

    /// COF1
    pub mod COF1 {
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

    /// COF2
    pub mod COF2 {
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

    /// COF3
    pub mod COF3 {
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

    /// COF4
    pub mod COF4 {
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

    /// COF5
    pub mod COF5 {
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

    /// COF6
    pub mod COF6 {
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

    /// COF7
    pub mod COF7 {
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

/// DMAMUX hardware configuration 2 register
pub mod DMAMUX_HWCFGR2 {

    /// NUM_DMA_EXT_REQ
    pub mod NUM_DMA_EXT_REQ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMAMUX hardware configuration 1 register
pub mod DMAMUX_HWCFGR1 {

    /// NUM_DMA_STREAMS
    pub mod NUM_DMA_STREAMS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NUM_DMA_PERIPH_REQ
    pub mod NUM_DMA_PERIPH_REQ {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NUM_DMA_TRIG
    pub mod NUM_DMA_TRIG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NUM_DMA_REQGEN
    pub mod NUM_DMA_REQGEN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register identifies the IP version.
pub mod DMAMUX_VERR {

    /// MINREV
    pub mod MINREV {
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

    /// MAJREV
    pub mod MAJREV {
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
}

/// This register identifies the IP.
pub mod DMAMUX_IPIDR {

    /// ID
    pub mod ID {
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

/// DMAMUX size identification register
pub mod DMAMUX_SIDR {

    /// SID
    pub mod SID {
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
#[repr(C)]
pub struct RegisterBlock {
    /// DMAMUX request line multiplexer channel 0 configuration register
    pub DMAMUX_C0CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel 1 configuration register
    pub DMAMUX_C1CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel 2 configuration register
    pub DMAMUX_C2CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel 3 configuration register
    pub DMAMUX_C3CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel 4 configuration register
    pub DMAMUX_C4CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel 5 configuration register
    pub DMAMUX_C5CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel 6 configuration register
    pub DMAMUX_C6CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel 7 configuration register
    pub DMAMUX_C7CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel 8 configuration register
    pub DMAMUX_C8CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel 9 configuration register
    pub DMAMUX_C9CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel 10 configuration register
    pub DMAMUX_C10CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel 11 configuration register
    pub DMAMUX_C11CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel 12 configuration register
    pub DMAMUX_C12CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel 13 configuration register
    pub DMAMUX_C13CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel 14 configuration register
    pub DMAMUX_C14CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel 15 configuration register
    pub DMAMUX_C15CR: RWRegister<u32>,

    _reserved1: [u32; 16],

    /// DMAMUX request line multiplexer interrupt channel status register
    pub DMAMUX_CSR: RORegister<u32>,

    /// DMAMUX request line multiplexer interrupt clear flag register
    pub DMAMUX_CFR: WORegister<u32>,

    _reserved2: [u32; 30],

    /// DMAMUX request generator channel 0 configuration register
    pub DMAMUX_RG0CR: RWRegister<u32>,

    /// DMAMUX request generator channel 1 configuration register
    pub DMAMUX_RG1CR: RWRegister<u32>,

    /// DMAMUX request generator channel 2 configuration register
    pub DMAMUX_RG2CR: RWRegister<u32>,

    /// DMAMUX request generator channel 3 configuration register
    pub DMAMUX_RG3CR: RWRegister<u32>,

    /// DMAMUX request generator channel 4 configuration register
    pub DMAMUX_RG4CR: RWRegister<u32>,

    /// DMAMUX request generator channel 5 configuration register
    pub DMAMUX_RG5CR: RWRegister<u32>,

    /// DMAMUX request generator channel 6 configuration register
    pub DMAMUX_RG6CR: RWRegister<u32>,

    /// DMAMUX request generator channel 7 configuration register
    pub DMAMUX_RG7CR: RWRegister<u32>,

    _reserved3: [u32; 8],

    /// DMAMUX request generator interrupt status register
    pub DMAMUX_RGSR: RORegister<u32>,

    /// DMAMUX request generator interrupt clear flag register
    pub DMAMUX_RGCFR: WORegister<u32>,

    _reserved4: [u32; 169],

    /// DMAMUX hardware configuration 2 register
    pub DMAMUX_HWCFGR2: RORegister<u32>,

    /// DMAMUX hardware configuration 1 register
    pub DMAMUX_HWCFGR1: RORegister<u32>,

    /// This register identifies the IP version.
    pub DMAMUX_VERR: RORegister<u32>,

    /// This register identifies the IP.
    pub DMAMUX_IPIDR: RORegister<u32>,

    /// DMAMUX size identification register
    pub DMAMUX_SIDR: RORegister<u32>,
}
pub struct ResetValues {
    pub DMAMUX_C0CR: u32,
    pub DMAMUX_C1CR: u32,
    pub DMAMUX_C2CR: u32,
    pub DMAMUX_C3CR: u32,
    pub DMAMUX_C4CR: u32,
    pub DMAMUX_C5CR: u32,
    pub DMAMUX_C6CR: u32,
    pub DMAMUX_C7CR: u32,
    pub DMAMUX_C8CR: u32,
    pub DMAMUX_C9CR: u32,
    pub DMAMUX_C10CR: u32,
    pub DMAMUX_C11CR: u32,
    pub DMAMUX_C12CR: u32,
    pub DMAMUX_C13CR: u32,
    pub DMAMUX_C14CR: u32,
    pub DMAMUX_C15CR: u32,
    pub DMAMUX_CSR: u32,
    pub DMAMUX_CFR: u32,
    pub DMAMUX_RG0CR: u32,
    pub DMAMUX_RG1CR: u32,
    pub DMAMUX_RG2CR: u32,
    pub DMAMUX_RG3CR: u32,
    pub DMAMUX_RG4CR: u32,
    pub DMAMUX_RG5CR: u32,
    pub DMAMUX_RG6CR: u32,
    pub DMAMUX_RG7CR: u32,
    pub DMAMUX_RGSR: u32,
    pub DMAMUX_RGCFR: u32,
    pub DMAMUX_HWCFGR2: u32,
    pub DMAMUX_HWCFGR1: u32,
    pub DMAMUX_VERR: u32,
    pub DMAMUX_IPIDR: u32,
    pub DMAMUX_SIDR: u32,
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

/// Access functions for the DMAMUX1 peripheral instance
pub mod DMAMUX1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x48002000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMAMUX1
    pub const reset: ResetValues = ResetValues {
        DMAMUX_C0CR: 0x00000000,
        DMAMUX_C1CR: 0x00000000,
        DMAMUX_C2CR: 0x00000000,
        DMAMUX_C3CR: 0x00000000,
        DMAMUX_C4CR: 0x00000000,
        DMAMUX_C5CR: 0x00000000,
        DMAMUX_C6CR: 0x00000000,
        DMAMUX_C7CR: 0x00000000,
        DMAMUX_C8CR: 0x00000000,
        DMAMUX_C9CR: 0x00000000,
        DMAMUX_C10CR: 0x00000000,
        DMAMUX_C11CR: 0x00000000,
        DMAMUX_C12CR: 0x00000000,
        DMAMUX_C13CR: 0x00000000,
        DMAMUX_C14CR: 0x00000000,
        DMAMUX_C15CR: 0x00000000,
        DMAMUX_CSR: 0x00000000,
        DMAMUX_CFR: 0x00000000,
        DMAMUX_RG0CR: 0x00000000,
        DMAMUX_RG1CR: 0x00000000,
        DMAMUX_RG2CR: 0x00000000,
        DMAMUX_RG3CR: 0x00000000,
        DMAMUX_RG4CR: 0x00000000,
        DMAMUX_RG5CR: 0x00000000,
        DMAMUX_RG6CR: 0x00000000,
        DMAMUX_RG7CR: 0x00000000,
        DMAMUX_RGSR: 0x00000000,
        DMAMUX_RGCFR: 0x00000000,
        DMAMUX_HWCFGR2: 0x00000008,
        DMAMUX_HWCFGR1: 0x08086C10,
        DMAMUX_VERR: 0x00000011,
        DMAMUX_IPIDR: 0x00100011,
        DMAMUX_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DMAMUX1_TAKEN: bool = false;

    /// Safe access to DMAMUX1
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
            if DMAMUX1_TAKEN {
                None
            } else {
                DMAMUX1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMAMUX1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMAMUX1_TAKEN && inst.addr == INSTANCE.addr {
                DMAMUX1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DMAMUX1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMAMUX1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DMAMUX1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMAMUX1: *const RegisterBlock = 0x48002000 as *const _;
