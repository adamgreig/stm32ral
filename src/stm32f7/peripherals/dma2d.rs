#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA2D controller
//!
//! Used by: stm32f745, stm32f765, stm32f7x6, stm32f7x7, stm32f7x9

use crate::{RORegister, RWRegister, UnsafeRWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register
pub mod CR {

    /// DMA2D mode
    pub mod MODE {
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

            /// 0b00: Memory-to-memory (FG fetch only)
            pub const MemoryToMemory: u32 = 0b00;

            /// 0b01: Memory-to-memory with PFC (FG fetch only with FG PFC active)
            pub const MemoryToMemoryPFC: u32 = 0b01;

            /// 0b10: Memory-to-memory with blending (FG and BG fetch with PFC and blending)
            pub const MemoryToMemoryPFCBlending: u32 = 0b10;

            /// 0b11: Register-to-memory
            pub const RegisterToMemory: u32 = 0b11;
        }
    }

    /// Configuration Error Interrupt Enable
    pub mod CEIE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: CE interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CE interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CLUT transfer complete interrupt enable
    pub mod CTCIE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: CTC interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CTC interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CLUT access error interrupt enable
    pub mod CAEIE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: CAE interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CAE interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Transfer watermark interrupt enable
    pub mod TWIE {
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

            /// 0b0: TW interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: TW interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Transfer complete interrupt enable
    pub mod TCIE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
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

    /// Transfer error interrupt enable
    pub mod TEIE {
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

            /// 0b0: TE interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: TE interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Abort
    pub mod ABORT {
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

            /// 0b1: Transfer abort requested
            pub const AbortRequest: u32 = 0b1;
        }
    }

    /// Suspend
    pub mod SUSP {
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

            /// 0b0: Transfer not suspended
            pub const NotSuspended: u32 = 0b0;

            /// 0b1: Transfer suspended
            pub const Suspended: u32 = 0b1;
        }
    }

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
        /// Read-write values
        pub mod RW {

            /// 0b1: Launch the DMA2D
            pub const Start: u32 = 0b1;
        }
    }
}

/// Interrupt Status Register
pub mod ISR {

    /// Configuration error interrupt flag
    pub mod CEIF {
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

    /// CLUT transfer complete interrupt flag
    pub mod CTCIF {
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

    /// CLUT access error interrupt flag
    pub mod CAEIF {
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

    /// Transfer watermark interrupt flag
    pub mod TWIF {
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

    /// Transfer complete interrupt flag
    pub mod TCIF {
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

    /// Transfer error interrupt flag
    pub mod TEIF {
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

/// interrupt flag clear register
pub mod IFCR {

    /// Clear configuration error interrupt flag
    pub mod CCEIF {
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

            /// 0b1: Clear the CEIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Clear CLUT transfer complete interrupt flag
    pub mod CCTCIF {
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

            /// 0b1: Clear the CTCIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Clear CLUT access error interrupt flag
    pub mod CAECIF {
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

            /// 0b1: Clear the CAEIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Clear transfer watermark interrupt flag
    pub mod CTWIF {
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

            /// 0b1: Clear the TWIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Clear transfer complete interrupt flag
    pub mod CTCIF {
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

            /// 0b1: Clear the TCIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Clear Transfer error interrupt flag
    pub mod CTEIF {
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

            /// 0b1: Clear the TEIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }
}

/// foreground memory address register
pub mod FGMAR {

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

/// foreground offset register
pub mod FGOR {

    /// Line offset
    pub mod LO {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (14 bits: 0x3fff << 0)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// background memory address register
pub mod BGMAR {
    pub use super::FGMAR::MA;
}

/// background offset register
pub mod BGOR {
    pub use super::FGOR::LO;
}

/// foreground PFC control register
pub mod FGPFCCR {

    /// Alpha value
    pub mod ALPHA {
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

    /// Alpha mode
    pub mod AM {
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

            /// 0b00: No modification of alpha channel
            pub const NoModify: u32 = 0b00;

            /// 0b01: Replace with value in ALPHA\[7:0\]
            pub const Replace: u32 = 0b01;

            /// 0b10: Multiply with value in ALPHA\[7:0\]
            pub const Multiply: u32 = 0b10;
        }
    }

    /// CLUT size
    pub mod CS {
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

    /// Start
    pub mod START {
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

            /// 0b1: Start the automatic loading of the CLUT
            pub const Start: u32 = 0b1;
        }
    }

    /// CLUT color mode
    pub mod CCM {
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

            /// 0b0: CLUT color format ARGB8888
            pub const ARGB8888: u32 = 0b0;

            /// 0b1: CLUT color format RGB888
            pub const RGB888: u32 = 0b1;
        }
    }

    /// Color mode
    pub mod CM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Color mode ARGB8888
            pub const ARGB8888: u32 = 0b0000;

            /// 0b0001: Color mode RGB888
            pub const RGB888: u32 = 0b0001;

            /// 0b0010: Color mode RGB565
            pub const RGB565: u32 = 0b0010;

            /// 0b0011: Color mode ARGB1555
            pub const ARGB1555: u32 = 0b0011;

            /// 0b0100: Color mode ARGB4444
            pub const ARGB4444: u32 = 0b0100;

            /// 0b0101: Color mode L8
            pub const L8: u32 = 0b0101;

            /// 0b0110: Color mode AL44
            pub const AL44: u32 = 0b0110;

            /// 0b0111: Color mode AL88
            pub const AL88: u32 = 0b0111;

            /// 0b1000: Color mode L4
            pub const L4: u32 = 0b1000;

            /// 0b1001: Color mode A8
            pub const A8: u32 = 0b1001;

            /// 0b1010: Color mode A4
            pub const A4: u32 = 0b1010;
        }
    }
}

/// foreground color register
pub mod FGCOLR {

    /// Red Value
    pub mod RED {
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

    /// Green Value
    pub mod GREEN {
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

    /// Blue Value
    pub mod BLUE {
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

/// background PFC control register
pub mod BGPFCCR {
    pub use super::FGPFCCR::ALPHA;
    pub use super::FGPFCCR::AM;
    pub use super::FGPFCCR::CCM;
    pub use super::FGPFCCR::CM;
    pub use super::FGPFCCR::CS;
    pub use super::FGPFCCR::START;
}

/// background color register
pub mod BGCOLR {
    pub use super::FGCOLR::BLUE;
    pub use super::FGCOLR::GREEN;
    pub use super::FGCOLR::RED;
}

/// foreground CLUT memory address register
pub mod FGCMAR {
    pub use super::FGMAR::MA;
}

/// background CLUT memory address register
pub mod BGCMAR {
    pub use super::FGMAR::MA;
}

/// output PFC control register
pub mod OPFCCR {

    /// Color mode
    pub mod CM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: ARGB8888
            pub const ARGB8888: u32 = 0b000;

            /// 0b001: RGB888
            pub const RGB888: u32 = 0b001;

            /// 0b010: RGB565
            pub const RGB565: u32 = 0b010;

            /// 0b011: ARGB1555
            pub const ARGB1555: u32 = 0b011;

            /// 0b100: ARGB4444
            pub const ARGB4444: u32 = 0b100;
        }
    }
}

/// output color register
pub mod OCOLR {

    /// Alpha Channel Value
    pub mod APLHA {
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

    /// Red Value
    pub mod RED {
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

    /// Green Value
    pub mod GREEN {
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

    /// Blue Value
    pub mod BLUE {
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

/// output memory address register
pub mod OMAR {
    pub use super::FGMAR::MA;
}

/// output offset register
pub mod OOR {
    pub use super::FGOR::LO;
}

/// number of line register
pub mod NLR {

    /// Pixel per lines
    pub mod PL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (14 bits: 0x3fff << 16)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of lines
    pub mod NL {
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

/// line watermark register
pub mod LWR {

    /// Line watermark
    pub mod LW {
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

/// AHB master timer configuration register
pub mod AMTCR {

    /// Dead Time
    pub mod DT {
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

    /// Enable
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

            /// 0b0: Disabled AHB/AXI dead-time functionality
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enabled AHB/AXI dead-time functionality
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// FGCLUT
pub mod FGCLUT {
    pub use super::OCOLR::APLHA;
    pub use super::OCOLR::BLUE;
    pub use super::OCOLR::GREEN;
    pub use super::OCOLR::RED;
}

/// BGCLUT
pub mod BGCLUT {
    pub use super::OCOLR::APLHA;
    pub use super::OCOLR::BLUE;
    pub use super::OCOLR::GREEN;
    pub use super::OCOLR::RED;
}
#[repr(C)]
pub struct RegisterBlock {
    /// control register
    pub CR: RWRegister<u32>,

    /// Interrupt Status Register
    pub ISR: RORegister<u32>,

    /// interrupt flag clear register
    pub IFCR: RWRegister<u32>,

    /// foreground memory address register
    pub FGMAR: UnsafeRWRegister<u32>,

    /// foreground offset register
    pub FGOR: RWRegister<u32>,

    /// background memory address register
    pub BGMAR: UnsafeRWRegister<u32>,

    /// background offset register
    pub BGOR: RWRegister<u32>,

    /// foreground PFC control register
    pub FGPFCCR: RWRegister<u32>,

    /// foreground color register
    pub FGCOLR: RWRegister<u32>,

    /// background PFC control register
    pub BGPFCCR: RWRegister<u32>,

    /// background color register
    pub BGCOLR: RWRegister<u32>,

    /// foreground CLUT memory address register
    pub FGCMAR: UnsafeRWRegister<u32>,

    /// background CLUT memory address register
    pub BGCMAR: UnsafeRWRegister<u32>,

    /// output PFC control register
    pub OPFCCR: RWRegister<u32>,

    /// output color register
    pub OCOLR: RWRegister<u32>,

    /// output memory address register
    pub OMAR: UnsafeRWRegister<u32>,

    /// output offset register
    pub OOR: RWRegister<u32>,

    /// number of line register
    pub NLR: RWRegister<u32>,

    /// line watermark register
    pub LWR: RWRegister<u32>,

    /// AHB master timer configuration register
    pub AMTCR: RWRegister<u32>,

    _reserved1: [u32; 236],

    /// FGCLUT
    pub FGCLUT: RWRegister<u32>,

    _reserved2: [u32; 255],

    /// BGCLUT
    pub BGCLUT: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub ISR: u32,
    pub IFCR: u32,
    pub FGMAR: u32,
    pub FGOR: u32,
    pub BGMAR: u32,
    pub BGOR: u32,
    pub FGPFCCR: u32,
    pub FGCOLR: u32,
    pub BGPFCCR: u32,
    pub BGCOLR: u32,
    pub FGCMAR: u32,
    pub BGCMAR: u32,
    pub OPFCCR: u32,
    pub OCOLR: u32,
    pub OMAR: u32,
    pub OOR: u32,
    pub NLR: u32,
    pub LWR: u32,
    pub AMTCR: u32,
    pub FGCLUT: u32,
    pub BGCLUT: u32,
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
