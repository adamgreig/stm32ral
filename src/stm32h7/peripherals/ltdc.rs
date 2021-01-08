#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LCD-TFT Controller
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h753, stm32h753v, stm32h7b3

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Synchronization Size Configuration Register
pub mod SSCR {

    /// Horizontal Synchronization Width (in units of pixel clock period)
    pub mod HSW {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Vertical Synchronization Height (in units of horizontal scan line)
    pub mod VSH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Back Porch Configuration Register
pub mod BPCR {

    /// Accumulated Horizontal back porch (in units of pixel clock period)
    pub mod AHBP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Accumulated Vertical back porch (in units of horizontal scan line)
    pub mod AVBP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Active Width Configuration Register
pub mod AWCR {

    /// Accumulated Active Width (in units of pixel clock period)
    pub mod AAW {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Accumulated Active Height (in units of horizontal scan line)
    pub mod AAH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Total Width Configuration Register
pub mod TWCR {

    /// Total Width (in units of pixel clock period)
    pub mod TOTALW {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Total Height (in units of horizontal scan line)
    pub mod TOTALH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Global Control Register
pub mod GCR {

    /// Horizontal Synchronization Polarity
    pub mod HSPOL {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Horizontal synchronization polarity is active low
            pub const ActiveLow: u32 = 0b0;

            /// 0b1: Horizontal synchronization polarity is active high
            pub const ActiveHigh: u32 = 0b1;
        }
    }

    /// Vertical Synchronization Polarity
    pub mod VSPOL {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Vertical synchronization polarity is active low
            pub const ActiveLow: u32 = 0b0;

            /// 0b1: Vertical synchronization polarity is active high
            pub const ActiveHigh: u32 = 0b1;
        }
    }

    /// Data Enable Polarity
    pub mod DEPOL {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Data enable polarity is active low
            pub const ActiveLow: u32 = 0b0;

            /// 0b1: Data enable polarity is active high
            pub const ActiveHigh: u32 = 0b1;
        }
    }

    /// Pixel Clock Polarity
    pub mod PCPOL {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Pixel clock on rising edge
            pub const RisingEdge: u32 = 0b0;

            /// 0b1: Pixel clock on falling edge
            pub const FallingEdge: u32 = 0b1;
        }
    }

    /// Dither Enable
    pub mod DEN {
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

            /// 0b0: Dither disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Dither enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Dither Red Width
    pub mod DRW {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Dither Green Width
    pub mod DGW {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Dither Blue Width
    pub mod DBW {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LCD-TFT controller enable bit
    pub mod LTDCEN {
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

            /// 0b0: LCD-TFT controller disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: LCD-TFT controller enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Shadow Reload Configuration Register
pub mod SRCR {

    /// Vertical Blanking Reload
    pub mod VBR {
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

            /// 0b1: The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area).
            pub const Reload: u32 = 0b1;

            /// 0b0: This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)
            pub const NoEffect: u32 = 0b0;
        }
    }

    /// Immediate Reload
    pub mod IMR {
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

            /// 0b1: The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload
            pub const Reload: u32 = 0b1;

            /// 0b0: This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)
            pub const NoEffect: u32 = 0b0;
        }
    }
}

/// Background Color Configuration Register
pub mod BCCR {

    /// Background Color Blue value
    pub mod BCBLUE {
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

    /// Background Color Green value
    pub mod BCGREEN {
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

    /// Background Color Red value
    pub mod BCRED {
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
}

/// Interrupt Enable Register
pub mod IER {

    /// Register Reload interrupt enable
    pub mod RRIE {
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

            /// 0b0: Register reload interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Register reload interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Transfer Error Interrupt Enable
    pub mod TERRIE {
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

            /// 0b0: Transfer error interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Transfer error interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// FIFO Underrun Interrupt Enable
    pub mod FUIE {
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

            /// 0b0: FIFO underrun interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: FIFO underrun interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Line Interrupt Enable
    pub mod LIE {
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

            /// 0b0: Line interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Line interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Interrupt Status Register
pub mod ISR {

    /// Register Reload Interrupt Flag
    pub mod RRIF {
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

            /// 0b0: No register reload
            pub const NoReload: u32 = 0b0;

            /// 0b1: Register reload interrupt generated when a vertical blanking reload occurs (and the first line after the active area is reached)
            pub const Reload: u32 = 0b1;
        }
    }

    /// Transfer Error interrupt flag
    pub mod TERRIF {
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

            /// 0b0: No transfer error
            pub const NoError: u32 = 0b0;

            /// 0b1: Transfer error interrupt generated when a bus error occurs
            pub const Error: u32 = 0b1;
        }
    }

    /// FIFO Underrun Interrupt flag
    pub mod FUIF {
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

            /// 0b0: No FIFO underrun
            pub const NoUnderrun: u32 = 0b0;

            /// 0b1: FIFO underrun interrupt generated, if one of the layer FIFOs is empty and pixel data is read from the FIFO
            pub const Underrun: u32 = 0b1;
        }
    }

    /// Line Interrupt flag
    pub mod LIF {
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

            /// 0b0: Programmed line not reached
            pub const NotReached: u32 = 0b0;

            /// 0b1: Line interrupt generated when a programmed line is reached
            pub const Reached: u32 = 0b1;
        }
    }
}

/// Interrupt Clear Register
pub mod ICR {

    /// Clears Register Reload Interrupt Flag
    pub mod CRRIF {
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

            /// 0b1: Clears the RRIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Clears the Transfer Error Interrupt Flag
    pub mod CTERRIF {
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

            /// 0b1: Clears the TERRIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Clears the FIFO Underrun Interrupt flag
    pub mod CFUIF {
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

            /// 0b1: Clears the FUIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Clears the Line Interrupt Flag
    pub mod CLIF {
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

            /// 0b1: Clears the LIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }
}

/// Line Interrupt Position Configuration Register
pub mod LIPCR {

    /// Line Interrupt Position
    pub mod LIPOS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Current Position Status Register
pub mod CPSR {

    /// Current X Position
    pub mod CXPOS {
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

    /// Current Y Position
    pub mod CYPOS {
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

/// Current Display Status Register
pub mod CDSR {

    /// Horizontal Synchronization display Status
    pub mod HSYNCS {
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

            /// 0b0: Currently not in HSYNC phase
            pub const NotActive: u32 = 0b0;

            /// 0b1: Currently in HSYNC phase
            pub const Active: u32 = 0b1;
        }
    }

    /// Vertical Synchronization display Status
    pub mod VSYNCS {
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

            /// 0b0: Currently not in VSYNC phase
            pub const NotActive: u32 = 0b0;

            /// 0b1: Currently in VSYNC phase
            pub const Active: u32 = 0b1;
        }
    }

    /// Horizontal Data Enable display Status
    pub mod HDES {
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

            /// 0b0: Currently not in horizontal Data Enable phase
            pub const NotActive: u32 = 0b0;

            /// 0b1: Currently in horizontal Data Enable phase
            pub const Active: u32 = 0b1;
        }
    }

    /// Vertical Data Enable display Status
    pub mod VDES {
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

            /// 0b0: Currently not in vertical Data Enable phase
            pub const NotActive: u32 = 0b0;

            /// 0b1: Currently in vertical Data Enable phase
            pub const Active: u32 = 0b1;
        }
    }
}

/// Layerx Control Register
pub mod CR1 {

    /// Color Look-Up Table Enable
    pub mod CLUTEN {
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

            /// 0b0: Color look-up table disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Color look-up table enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Color Keying Enable
    pub mod COLKEN {
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

            /// 0b0: Color keying disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Color keying enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Layer Enable
    pub mod LEN {
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

            /// 0b0: Layer disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Layer enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Layerx Window Horizontal Position Configuration Register
pub mod WHPCR1 {

    /// Window Horizontal Stop Position
    pub mod WHSPPOS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Window Horizontal Start Position
    pub mod WHSTPOS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layerx Window Vertical Position Configuration Register
pub mod WVPCR1 {

    /// Window Vertical Stop Position
    pub mod WVSPPOS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Window Vertical Start Position
    pub mod WVSTPOS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layerx Color Keying Configuration Register
pub mod CKCR1 {

    /// Color Key Red value
    pub mod CKRED {
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

    /// Color Key Green value
    pub mod CKGREEN {
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

    /// Color Key Blue value
    pub mod CKBLUE {
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

/// Layerx Pixel Format Configuration Register
pub mod PFCR1 {

    /// Pixel Format
    pub mod PF {
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

            /// 0b101: L8 (8-bit luminance)
            pub const L8: u32 = 0b101;

            /// 0b110: AL44 (4-bit alpha, 4-bit luminance)
            pub const AL44: u32 = 0b110;

            /// 0b111: AL88 (8-bit alpha, 8-bit luminance)
            pub const AL88: u32 = 0b111;
        }
    }
}

/// Layerx Constant Alpha Configuration Register
pub mod CACR1 {

    /// Constant Alpha
    pub mod CONSTA {
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

/// Layerx Default Color Configuration Register
pub mod DCCR1 {

    /// Default Color Alpha
    pub mod DCALPHA {
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

    /// Default Color Red
    pub mod DCRED {
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

    /// Default Color Green
    pub mod DCGREEN {
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

    /// Default Color Blue
    pub mod DCBLUE {
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

/// Layerx Blending Factors Configuration Register
pub mod BFCR1 {

    /// Blending Factor 1
    pub mod BF1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b100: BF1 = constant alpha
            pub const Constant: u32 = 0b100;

            /// 0b110: BF1 = pixel alpha * constant alpha
            pub const Pixel: u32 = 0b110;
        }
    }

    /// Blending Factor 2
    pub mod BF2 {
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

            /// 0b101: BF2 = 1 - constant alpha
            pub const Constant: u32 = 0b101;

            /// 0b111: BF2 = 1 - pixel alpha * constant alpha
            pub const Pixel: u32 = 0b111;
        }
    }
}

/// Layerx Color Frame Buffer Address Register
pub mod CFBAR1 {

    /// Color Frame Buffer Start Address
    pub mod CFBADD {
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

/// Layerx Color Frame Buffer Length Register
pub mod CFBLR1 {

    /// Color Frame Buffer Pitch in bytes
    pub mod CFBP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (13 bits: 0x1fff << 16)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Color Frame Buffer Line Length
    pub mod CFBLL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (13 bits: 0x1fff << 0)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layerx ColorFrame Buffer Line Number Register
pub mod CFBLNR1 {

    /// Frame Buffer Line Number
    pub mod CFBLNBR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layerx CLUT Write Register
pub mod CLUTWR1 {

    /// CLUT Address
    pub mod CLUTADD {
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

    /// Red value
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

    /// Green value
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

    /// Blue value
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

/// Layerx Control Register
pub mod CR2 {
    pub use super::CR1::CLUTEN;
    pub use super::CR1::COLKEN;
    pub use super::CR1::LEN;
}

/// Layerx Window Horizontal Position Configuration Register
pub mod WHPCR2 {
    pub use super::WHPCR1::WHSPPOS;
    pub use super::WHPCR1::WHSTPOS;
}

/// Layerx Window Vertical Position Configuration Register
pub mod WVPCR2 {
    pub use super::WVPCR1::WVSPPOS;
    pub use super::WVPCR1::WVSTPOS;
}

/// Layerx Color Keying Configuration Register
pub mod CKCR2 {
    pub use super::CKCR1::CKBLUE;
    pub use super::CKCR1::CKGREEN;
    pub use super::CKCR1::CKRED;
}

/// Layerx Pixel Format Configuration Register
pub mod PFCR2 {
    pub use super::PFCR1::PF;
}

/// Layerx Constant Alpha Configuration Register
pub mod CACR2 {
    pub use super::CACR1::CONSTA;
}

/// Layerx Default Color Configuration Register
pub mod DCCR2 {
    pub use super::DCCR1::DCALPHA;
    pub use super::DCCR1::DCBLUE;
    pub use super::DCCR1::DCGREEN;
    pub use super::DCCR1::DCRED;
}

/// Layerx Blending Factors Configuration Register
pub mod BFCR2 {
    pub use super::BFCR1::BF1;
    pub use super::BFCR1::BF2;
}

/// Layerx Color Frame Buffer Address Register
pub mod CFBAR2 {
    pub use super::CFBAR1::CFBADD;
}

/// Layerx Color Frame Buffer Length Register
pub mod CFBLR2 {
    pub use super::CFBLR1::CFBLL;
    pub use super::CFBLR1::CFBP;
}

/// Layerx ColorFrame Buffer Line Number Register
pub mod CFBLNR2 {
    pub use super::CFBLNR1::CFBLNBR;
}

/// Layerx CLUT Write Register
pub mod CLUTWR2 {
    pub use super::CLUTWR1::BLUE;
    pub use super::CLUTWR1::CLUTADD;
    pub use super::CLUTWR1::GREEN;
    pub use super::CLUTWR1::RED;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 2],

    /// Synchronization Size Configuration Register
    pub SSCR: RWRegister<u32>,

    /// Back Porch Configuration Register
    pub BPCR: RWRegister<u32>,

    /// Active Width Configuration Register
    pub AWCR: RWRegister<u32>,

    /// Total Width Configuration Register
    pub TWCR: RWRegister<u32>,

    /// Global Control Register
    pub GCR: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// Shadow Reload Configuration Register
    pub SRCR: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// Background Color Configuration Register
    pub BCCR: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// Interrupt Enable Register
    pub IER: RWRegister<u32>,

    /// Interrupt Status Register
    pub ISR: RORegister<u32>,

    /// Interrupt Clear Register
    pub ICR: WORegister<u32>,

    /// Line Interrupt Position Configuration Register
    pub LIPCR: RWRegister<u32>,

    /// Current Position Status Register
    pub CPSR: RORegister<u32>,

    /// Current Display Status Register
    pub CDSR: RORegister<u32>,

    _reserved5: [u32; 14],

    /// Layerx Control Register
    pub CR1: RWRegister<u32>,

    /// Layerx Window Horizontal Position Configuration Register
    pub WHPCR1: RWRegister<u32>,

    /// Layerx Window Vertical Position Configuration Register
    pub WVPCR1: RWRegister<u32>,

    /// Layerx Color Keying Configuration Register
    pub CKCR1: RWRegister<u32>,

    /// Layerx Pixel Format Configuration Register
    pub PFCR1: RWRegister<u32>,

    /// Layerx Constant Alpha Configuration Register
    pub CACR1: RWRegister<u32>,

    /// Layerx Default Color Configuration Register
    pub DCCR1: RWRegister<u32>,

    /// Layerx Blending Factors Configuration Register
    pub BFCR1: RWRegister<u32>,

    _reserved6: [u32; 2],

    /// Layerx Color Frame Buffer Address Register
    pub CFBAR1: RWRegister<u32>,

    /// Layerx Color Frame Buffer Length Register
    pub CFBLR1: RWRegister<u32>,

    /// Layerx ColorFrame Buffer Line Number Register
    pub CFBLNR1: RWRegister<u32>,

    _reserved7: [u32; 3],

    /// Layerx CLUT Write Register
    pub CLUTWR1: WORegister<u32>,

    _reserved8: [u32; 15],

    /// Layerx Control Register
    pub CR2: RWRegister<u32>,

    /// Layerx Window Horizontal Position Configuration Register
    pub WHPCR2: RWRegister<u32>,

    /// Layerx Window Vertical Position Configuration Register
    pub WVPCR2: RWRegister<u32>,

    /// Layerx Color Keying Configuration Register
    pub CKCR2: RWRegister<u32>,

    /// Layerx Pixel Format Configuration Register
    pub PFCR2: RWRegister<u32>,

    /// Layerx Constant Alpha Configuration Register
    pub CACR2: RWRegister<u32>,

    /// Layerx Default Color Configuration Register
    pub DCCR2: RWRegister<u32>,

    /// Layerx Blending Factors Configuration Register
    pub BFCR2: RWRegister<u32>,

    _reserved9: [u32; 2],

    /// Layerx Color Frame Buffer Address Register
    pub CFBAR2: RWRegister<u32>,

    /// Layerx Color Frame Buffer Length Register
    pub CFBLR2: RWRegister<u32>,

    /// Layerx ColorFrame Buffer Line Number Register
    pub CFBLNR2: RWRegister<u32>,

    _reserved10: [u32; 3],

    /// Layerx CLUT Write Register
    pub CLUTWR2: WORegister<u32>,
}
pub struct ResetValues {
    pub SSCR: u32,
    pub BPCR: u32,
    pub AWCR: u32,
    pub TWCR: u32,
    pub GCR: u32,
    pub SRCR: u32,
    pub BCCR: u32,
    pub IER: u32,
    pub ISR: u32,
    pub ICR: u32,
    pub LIPCR: u32,
    pub CPSR: u32,
    pub CDSR: u32,
    pub CR1: u32,
    pub WHPCR1: u32,
    pub WVPCR1: u32,
    pub CKCR1: u32,
    pub PFCR1: u32,
    pub CACR1: u32,
    pub DCCR1: u32,
    pub BFCR1: u32,
    pub CFBAR1: u32,
    pub CFBLR1: u32,
    pub CFBLNR1: u32,
    pub CLUTWR1: u32,
    pub CR2: u32,
    pub WHPCR2: u32,
    pub WVPCR2: u32,
    pub CKCR2: u32,
    pub PFCR2: u32,
    pub CACR2: u32,
    pub DCCR2: u32,
    pub BFCR2: u32,
    pub CFBAR2: u32,
    pub CFBLR2: u32,
    pub CFBLNR2: u32,
    pub CLUTWR2: u32,
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
