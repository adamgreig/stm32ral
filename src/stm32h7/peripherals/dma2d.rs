#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA2D
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v

use crate::{RORegister, RWRegister, UnsafeRWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DMA2D control register
pub mod CR {

    /// Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers
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

    /// Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset.
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

    /// Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset.
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

    /// Transfer error interrupt enable This bit is set and cleared by software.
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

    /// Transfer complete interrupt enable This bit is set and cleared by software.
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

    /// Transfer watermark interrupt enable This bit is set and cleared by software.
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

    /// CLUT access error interrupt enable This bit is set and cleared by software.
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

    /// CLUT transfer complete interrupt enable This bit is set and cleared by software.
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

    /// Configuration Error Interrupt Enable This bit is set and cleared by software.
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

    /// DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing.
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
}

/// DMA2D Interrupt Status Register
pub mod ISR {

    /// Transfer error interrupt flag This bit is set when an error occurs during a DMA transfer (data transfer or automatic CLUT loading).
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

    /// Transfer complete interrupt flag This bit is set when a DMA2D transfer operation is complete (data transfer only).
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

    /// Transfer watermark interrupt flag This bit is set when the last pixel of the watermarked line has been transferred.
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

    /// CLUT access error interrupt flag This bit is set when the CPU accesses the CLUT while the CLUT is being automatically copied from a system memory to the internal DMA2D.
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

    /// CLUT transfer complete interrupt flag This bit is set when the CLUT copy from a system memory area to the internal DMA2D memory is complete.
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

    /// Configuration error interrupt flag This bit is set when the START bit of DMA2D_CR, DMA2DFGPFCCR or DMA2D_BGPFCCR is set and a wrong configuration has been programmed.
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
}

/// DMA2D interrupt flag clear register
pub mod IFCR {

    /// Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register
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

    /// Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register
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

    /// Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register
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

    /// Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register
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

    /// Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register
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

    /// Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register
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
}

/// DMA2D foreground memory address register
pub mod FGMAR {

    /// Memory address Address of the data used for the foreground image. This register can only be written when data transfers are disabled. Once the data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned.
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

/// DMA2D foreground offset register
pub mod FGOR {

    /// Line offset Line offset used for the foreground expressed in pixel. This value is used to generate the address. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once a data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even.
    pub mod LO {
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

/// DMA2D background memory address register
pub mod BGMAR {
    pub use super::FGMAR::MA;
}

/// DMA2D background offset register
pub mod BGOR {
    pub use super::FGOR::LO;
}

/// DMA2D foreground PFC control register
pub mod FGPFCCR {

    /// Color mode These bits defines the color format of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless
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

            /// 0b1011: Color mode YCbCr
            pub const YCbCr: u32 = 0b1011;
        }
    }

    /// CLUT color mode This bit defines the color format of the CLUT. It can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only.
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

    /// Start This bit can be set to start the automatic loading of the CLUT. It is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already ongoing (data transfer or automatic background CLUT transfer).
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

    /// CLUT size These bits define the size of the CLUT used for the foreground image. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\[7:0\] + 1.
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

    /// Alpha mode These bits select the alpha channel value to be used for the foreground image. They can only be written data the transfer are disabled. Once the transfer has started, they become read-only. other configurations are meaningless
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

    /// Chroma Sub-Sampling These bits define the chroma sub-sampling mode for YCbCr color mode. Once the transfer has started, these bits are read-only. others: meaningless
    pub mod CSS {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only.
    pub mod AI {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Regular alpha
            pub const RegularAlpha: u32 = 0b0;

            /// 0b1: Inverted alpha
            pub const InvertedAlpha: u32 = 0b1;
        }
    }

    /// Red Blue Swap This bit allows to swap the R &amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.
    pub mod RBS {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Red Blue Swap (RGB or ARGB)
            pub const Regular: u32 = 0b0;

            /// 0b1: Red Blue Swap (BGR or ABGR)
            pub const Swap: u32 = 0b1;
        }
    }

    /// Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied by the original alpha value according to the alpha mode selected through the AM\[1:0\] bits. These bits can only be written when data transfers are disabled. Once a transfer has started, they become read-only.
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
}

/// DMA2D foreground color register
pub mod FGCOLR {

    /// Blue Value These bits defines the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only.
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

    /// Green Value These bits defines the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only.
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

    /// Red Value These bits defines the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
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
}

/// DMA2D background PFC control register
pub mod BGPFCCR {

    /// Color mode These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless
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

    /// CLUT Color mode These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only.
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

    /// Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer).
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

    /// CLUT size These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\[7:0\] + 1.
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

    /// Alpha mode These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless
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

    /// Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only.
    pub mod AI {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Regular alpha
            pub const RegularAlpha: u32 = 0b0;

            /// 0b1: Inverted alpha
            pub const InvertedAlpha: u32 = 0b1;
        }
    }

    /// Red Blue Swap This bit allows to swap the R &amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.
    pub mod RBS {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Red Blue Swap (RGB or ARGB)
            pub const Regular: u32 = 0b0;

            /// 0b1: Red Blue Swap (BGR or ABGR)
            pub const Swap: u32 = 0b1;
        }
    }

    /// Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied with the original alpha value according to the alpha mode selected with bits AM\[1: 0\]. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
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
}

/// DMA2D background color register
pub mod BGCOLR {
    pub use super::FGCOLR::BLUE;
    pub use super::FGCOLR::GREEN;
    pub use super::FGCOLR::RED;
}

/// DMA2D foreground CLUT memory address register
pub mod FGCMAR {
    pub use super::FGMAR::MA;
}

/// DMA2D background CLUT memory address register
pub mod BGCMAR {
    pub use super::FGMAR::MA;
}

/// DMA2D output PFC control register
pub mod OPFCCR {

    /// Color mode These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless
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

    /// Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only.
    pub mod AI {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Regular alpha
            pub const RegularAlpha: u32 = 0b0;

            /// 0b1: Inverted alpha
            pub const InvertedAlpha: u32 = 0b1;
        }
    }

    /// Red Blue Swap This bit allows to swap the R &amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.
    pub mod RBS {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Red Blue Swap (RGB or ARGB)
            pub const Regular: u32 = 0b0;

            /// 0b1: Red Blue Swap (BGR or ABGR)
            pub const Swap: u32 = 0b1;
        }
    }

    /// Swap Bytes
    pub mod SB {
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

            /// 0b0: Regular byte order
            pub const Regular: u32 = 0b0;

            /// 0b1: Bytes are swapped two by two
            pub const SwapBytes: u32 = 0b1;
        }
    }
}

/// DMA2D output color register
pub mod OCOLR {

    /// Blue Value These bits define the blue value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
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

    /// Green Value These bits define the green value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
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

    /// Red Value These bits define the red value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
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

    /// Alpha Channel Value These bits define the alpha channel of the output color. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
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
}

/// DMA2D output memory address register
pub mod OMAR {
    pub use super::FGMAR::MA;
}

/// DMA2D output offset register
pub mod OOR {
    pub use super::FGOR::LO;
}

/// DMA2D number of line register
pub mod NLR {

    /// Number of lines Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
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

    /// Pixel per lines Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even.
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
}

/// DMA2D line watermark register
pub mod LWR {

    /// Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
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

/// DMA2D AXI master timer configuration register
pub mod AMTCR {

    /// Enable Enables the dead time functionality.
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

    /// Dead Time Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses.
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// DMA2D control register
    pub CR: RWRegister<u32>,

    /// DMA2D Interrupt Status Register
    pub ISR: RORegister<u32>,

    /// DMA2D interrupt flag clear register
    pub IFCR: RWRegister<u32>,

    /// DMA2D foreground memory address register
    pub FGMAR: UnsafeRWRegister<u32>,

    /// DMA2D foreground offset register
    pub FGOR: RWRegister<u32>,

    /// DMA2D background memory address register
    pub BGMAR: UnsafeRWRegister<u32>,

    /// DMA2D background offset register
    pub BGOR: RWRegister<u32>,

    /// DMA2D foreground PFC control register
    pub FGPFCCR: RWRegister<u32>,

    /// DMA2D foreground color register
    pub FGCOLR: RWRegister<u32>,

    /// DMA2D background PFC control register
    pub BGPFCCR: RWRegister<u32>,

    /// DMA2D background color register
    pub BGCOLR: RWRegister<u32>,

    /// DMA2D foreground CLUT memory address register
    pub FGCMAR: UnsafeRWRegister<u32>,

    /// DMA2D background CLUT memory address register
    pub BGCMAR: UnsafeRWRegister<u32>,

    /// DMA2D output PFC control register
    pub OPFCCR: RWRegister<u32>,

    /// DMA2D output color register
    pub OCOLR: RWRegister<u32>,

    /// DMA2D output memory address register
    pub OMAR: UnsafeRWRegister<u32>,

    /// DMA2D output offset register
    pub OOR: RWRegister<u32>,

    /// DMA2D number of line register
    pub NLR: RWRegister<u32>,

    /// DMA2D line watermark register
    pub LWR: RWRegister<u32>,

    /// DMA2D AXI master timer configuration register
    pub AMTCR: RWRegister<u32>,
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
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}
