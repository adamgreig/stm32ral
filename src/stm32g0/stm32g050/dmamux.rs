#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMAMUX

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DMAMUX request line multiplexer channel x configuration register
pub mod DMAMUX_C0CR {

    /// DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources.
    pub mod DMAREQ_ID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Synchronization overrun interrupt enable
    pub mod SOIE {
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

            /// 0b0: interrupt disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: interrupt enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Event generation enable
    pub mod EGE {
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

            /// 0b0: event generation disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: event generation enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Synchronization enable
    pub mod SE {
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

            /// 0b0: synchronization disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: synchronization enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Synchronization polarity Defines the edge polarity of the selected synchronization input:
    pub mod SPOL {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (2 bits: 0b11 << 17)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: no event, i.e. no synchronization nor detection.
            pub const B_0x0: u32 = 0b00;

            /// 0b01: rising edge
            pub const B_0x1: u32 = 0b01;

            /// 0b10: falling edge
            pub const B_0x2: u32 = 0b10;

            /// 0b11: rising and falling edge
            pub const B_0x3: u32 = 0b11;
        }
    }

    /// Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field shall only be written when both SE and EGE bits are low.
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

    /// Synchronization identification Selects the synchronization input (see inputs to resources STM32G0).
    pub mod SYNC_ID {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMAMUX request line multiplexer channel x configuration register
pub mod DMAMUX_C1CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel x configuration register
pub mod DMAMUX_C2CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel x configuration register
pub mod DMAMUX_C3CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel x configuration register
pub mod DMAMUX_C4CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel x configuration register
pub mod DMAMUX_C5CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel x configuration register
pub mod DMAMUX_C6CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel x configuration register
pub mod DMAMUX_C7CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel x configuration register
pub mod DMAMUX_C8CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel x configuration register
pub mod DMAMUX_C9CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel x configuration register
pub mod DMAMUX_C10CR {
    pub use super::DMAMUX_C0CR::DMAREQ_ID;
    pub use super::DMAMUX_C0CR::EGE;
    pub use super::DMAMUX_C0CR::NBREQ;
    pub use super::DMAMUX_C0CR::SE;
    pub use super::DMAMUX_C0CR::SOIE;
    pub use super::DMAMUX_C0CR::SPOL;
    pub use super::DMAMUX_C0CR::SYNC_ID;
}

/// DMAMUX request line multiplexer channel x configuration register
pub mod DMAMUX_C11CR {
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

    /// Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register.
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

    /// Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register.
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

    /// Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register.
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

    /// Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register.
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

    /// Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register.
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

    /// Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register.
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

    /// Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register.
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

    /// Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register.
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

    /// Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register.
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

    /// Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register.
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

    /// Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register.
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

    /// Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register.
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
}

/// DMAMUX request line multiplexer interrupt clear flag register
pub mod DMAMUX_CFR {

    /// Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
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

    /// Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
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

    /// Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
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

    /// Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
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

    /// Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
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

    /// Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
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

    /// Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
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

    /// Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
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

    /// Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
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

    /// Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
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

    /// Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
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

    /// Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register.
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
}

/// DMAMUX request generator channel x configuration register
pub mod DMAMUX_RG0CR {

    /// Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator
    pub mod SIG_ID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Trigger overrun interrupt enable
    pub mod OIE {
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

            /// 0b0: interrupt on a trigger overrun event occurrence is disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: interrupt on a trigger overrun event occurrence is enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DMA request generator channel x enable
    pub mod GE {
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

            /// 0b0: DMA request generator channel x disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DMA request generator channel x enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DMA request generator trigger polarity Defines the edge polarity of the selected trigger input
    pub mod GPOL {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (2 bits: 0b11 << 17)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: no event. I.e. none trigger detection nor generation.
            pub const B_0x0: u32 = 0b00;

            /// 0b01: rising edge
            pub const B_0x1: u32 = 0b01;

            /// 0b10: falling edge
            pub const B_0x2: u32 = 0b10;

            /// 0b11: rising and falling edge
            pub const B_0x3: u32 = 0b11;
        }
    }

    /// Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field shall only be written when GE bit is disabled.
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

/// DMAMUX request generator channel x configuration register
pub mod DMAMUX_RG1CR {
    pub use super::DMAMUX_RG0CR::GE;
    pub use super::DMAMUX_RG0CR::GNBREQ;
    pub use super::DMAMUX_RG0CR::GPOL;
    pub use super::DMAMUX_RG0CR::OIE;
    pub use super::DMAMUX_RG0CR::SIG_ID;
}

/// DMAMUX request generator channel x configuration register
pub mod DMAMUX_RG2CR {
    pub use super::DMAMUX_RG0CR::GE;
    pub use super::DMAMUX_RG0CR::GNBREQ;
    pub use super::DMAMUX_RG0CR::GPOL;
    pub use super::DMAMUX_RG0CR::OIE;
    pub use super::DMAMUX_RG0CR::SIG_ID;
}

/// DMAMUX request generator channel x configuration register
pub mod DMAMUX_RG3CR {
    pub use super::DMAMUX_RG0CR::GE;
    pub use super::DMAMUX_RG0CR::GNBREQ;
    pub use super::DMAMUX_RG0CR::GPOL;
    pub use super::DMAMUX_RG0CR::OIE;
    pub use super::DMAMUX_RG0CR::SIG_ID;
}

/// DMAMUX request generator interrupt status register
pub mod DMAMUX_RGSR {

    /// Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.
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

    /// Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.
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

    /// Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.
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

    /// Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.
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
}

/// DMAMUX request generator interrupt clear flag register
pub mod DMAMUX_RGCFR {

    /// Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register.
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

    /// Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register.
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

    /// Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register.
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

    /// Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register.
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// DMAMUX request line multiplexer channel x configuration register
    pub DMAMUX_C0CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel x configuration register
    pub DMAMUX_C1CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel x configuration register
    pub DMAMUX_C2CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel x configuration register
    pub DMAMUX_C3CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel x configuration register
    pub DMAMUX_C4CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel x configuration register
    pub DMAMUX_C5CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel x configuration register
    pub DMAMUX_C6CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel x configuration register
    pub DMAMUX_C7CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel x configuration register
    pub DMAMUX_C8CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel x configuration register
    pub DMAMUX_C9CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel x configuration register
    pub DMAMUX_C10CR: RWRegister<u32>,

    /// DMAMUX request line multiplexer channel x configuration register
    pub DMAMUX_C11CR: RWRegister<u32>,

    _reserved1: [u32; 20],

    /// DMAMUX request line multiplexer interrupt channel status register
    pub DMAMUX_CSR: RORegister<u32>,

    /// DMAMUX request line multiplexer interrupt clear flag register
    pub DMAMUX_CFR: WORegister<u32>,

    _reserved2: [u32; 30],

    /// DMAMUX request generator channel x configuration register
    pub DMAMUX_RG0CR: RWRegister<u32>,

    /// DMAMUX request generator channel x configuration register
    pub DMAMUX_RG1CR: RWRegister<u32>,

    /// DMAMUX request generator channel x configuration register
    pub DMAMUX_RG2CR: RWRegister<u32>,

    /// DMAMUX request generator channel x configuration register
    pub DMAMUX_RG3CR: RWRegister<u32>,

    _reserved3: [u32; 12],

    /// DMAMUX request generator interrupt status register
    pub DMAMUX_RGSR: RORegister<u32>,

    /// DMAMUX request generator interrupt clear flag register
    pub DMAMUX_RGCFR: WORegister<u32>,
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
    pub DMAMUX_CSR: u32,
    pub DMAMUX_CFR: u32,
    pub DMAMUX_RG0CR: u32,
    pub DMAMUX_RG1CR: u32,
    pub DMAMUX_RG2CR: u32,
    pub DMAMUX_RG3CR: u32,
    pub DMAMUX_RGSR: u32,
    pub DMAMUX_RGCFR: u32,
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

/// Access functions for the DMAMUX peripheral instance
pub mod DMAMUX {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40020800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMAMUX
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
        DMAMUX_CSR: 0x00000000,
        DMAMUX_CFR: 0x00000000,
        DMAMUX_RG0CR: 0x00000000,
        DMAMUX_RG1CR: 0x00000000,
        DMAMUX_RG2CR: 0x00000000,
        DMAMUX_RG3CR: 0x00000000,
        DMAMUX_RGSR: 0x00000000,
        DMAMUX_RGCFR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DMAMUX_TAKEN: bool = false;

    /// Safe access to DMAMUX
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
            if DMAMUX_TAKEN {
                None
            } else {
                DMAMUX_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMAMUX
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMAMUX_TAKEN && inst.addr == INSTANCE.addr {
                DMAMUX_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DMAMUX
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMAMUX_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DMAMUX
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMAMUX: *const RegisterBlock = 0x40020800 as *const _;
