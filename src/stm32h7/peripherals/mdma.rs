#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MDMA
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// MDMA Global Interrupt/Status Register
pub mod GISR0 {

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF0 {
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

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF1 {
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

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF2 {
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

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF3 {
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

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF4 {
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

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF5 {
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

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF6 {
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

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF7 {
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

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF8 {
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

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF9 {
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

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF10 {
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

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF11 {
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

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF12 {
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

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF13 {
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

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF14 {
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

    /// Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    pub mod GIF15 {
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

/// MDMA channel x interrupt/status register
pub mod C0ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF0 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF0 {
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF0 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF0 {
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

    /// channel x buffer transfer complete
    pub mod TCIF0 {
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

    /// channel x request active flag
    pub mod CRQA0 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C0IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF0 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF0 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF0 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF0 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF0 {
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
}

/// MDMA Channel x error status register
pub mod C0ESR {

    /// Transfer Error Address These bits are set and cleared by HW, in case of an MDMA data transfer error. It is used in conjunction with TED. This field indicates the 7 LSBits of the address which generated a transfer/access error. It may be used by SW to retrieve the failing address, by adding this value (truncated to the buffer transfer length size) to the current SAR/DAR value. Note: The SAR/DAR current value doesnt reflect this last address due to the FIFO management system. The SAR/DAR are only updated at the end of a (buffer) transfer (of TLEN+1 bytes). Note: It is not set in case of a link data error.
    pub mod TEA {
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

    /// Transfer Error Direction These bit is set and cleared by HW, in case of an MDMA data transfer error.
    pub mod TED {
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

    /// Transfer Error Link Data These bit is set by HW, in case of a transfer error while reading the block link data structure. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register.
    pub mod TELD {
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

    /// Transfer Error Mask Data These bit is set by HW, in case of a transfer error while writing the Mask Data. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register.
    pub mod TEMD {
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

    /// Address/Size Error These bit is set by HW, when the programmed address is not aligned with the data size. TED will indicate whether the problem is on the source or destination. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register.
    pub mod ASE {
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

    /// Block Size Error These bit is set by HW, when the block size is not an integer multiple of the data size either for source or destination. TED will indicate whether the problem is on the source or destination. It is cleared by software writing 1 to the CTEIFx bit in the DMA_IFCRy register.
    pub mod BSE {
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

/// This register is used to control the concerned channel.
pub mod C0CR {

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

    /// Transfer error interrupt enable This bit is set and cleared by software.
    pub mod TEIE {
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

    /// Channel Transfer Complete interrupt enable This bit is set and cleared by software.
    pub mod CTCIE {
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

    /// Block Repeat transfer interrupt enable This bit is set and cleared by software.
    pub mod BRTIE {
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

    /// Block Transfer interrupt enable This bit is set and cleared by software.
    pub mod BTIE {
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

    /// buffer Transfer Complete interrupt enable This bit is set and cleared by software.
    pub mod TCIE {
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

    /// Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0.
    pub mod PL {
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

    /// byte Endianness exchange
    pub mod BEX {
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

    /// Half word Endianes exchange
    pub mod HEX {
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

    /// Word Endianness exchange
    pub mod WEX {
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

    /// SW ReQuest Writing a 1 into this bit sets the CRQAx in MDMA_ISRy register, activating the request on Channel x Note: Either the whole CxCR register or the 8-bit/16-bit register @ Address offset: 0x4E + 0x40 chn may be used for SWRQ activation. In case of a SW request, acknowledge is not generated (neither HW signal, nor CxMAR write access).
    pub mod SWRQ {
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
}

/// This register is used to configure the concerned channel.
pub mod C0TCR {

    /// Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub mod SINC {
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

    /// Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden.
    pub mod DINC {
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

    /// Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1).
    pub mod SSIZE {
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

    /// Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1).
    pub mod DSIZE {
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

    /// source increment offset size
    pub mod SINCOS {
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

    /// Destination increment offset
    pub mod DINCOS {
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

    /// source burst transfer configuration
    pub mod SBURST {
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

    /// Destination burst transfer configuration
    pub mod DBURST {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (3 bits: 0b111 << 15)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// buffer transfer lengh
    pub mod TLEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (7 bits: 0x7f << 18)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM\[0\] value. This bit is protected and can be written only if EN is 0
    pub mod PKE {
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

    /// Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0
    pub mod PAM {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0.
    pub mod TRGM {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0.
    pub mod SWRM {
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

    /// Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable.
    pub mod BWM {
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

/// MDMA Channel x block number of data register
pub mod C0BNDTR {

    /// block number of data to transfer
    pub mod BNDT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (17 bits: 0x1ffff << 0)
        pub const mask: u32 = 0x1ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0.
    pub mod BRSUM {
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

    /// Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0.
    pub mod BRDUM {
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

    /// Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0.
    pub mod BRC {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (12 bits: 0xfff << 20)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MDMA channel x source address register
pub mod C0SAR {

    /// source adr base
    pub mod SAR {
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

/// MDMA channel x destination address register
pub mod C0DAR {

    /// Destination adr base
    pub mod DAR {
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

/// MDMA channel x Block Repeat address Update register
pub mod C0BRUR {

    /// source adresse update value
    pub mod SUV {
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

    /// destination address update
    pub mod DUV {
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

/// MDMA channel x Link Address register
pub mod C0LAR {

    /// Link address register
    pub mod LAR {
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

/// MDMA channel x Trigger and Bus selection Register
pub mod C0TBR {

    /// Trigger selection
    pub mod TSEL {
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

    /// Source BUS select This bit is protected and can be written only if EN is 0.
    pub mod SBUS {
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

    /// Destination BUS slect This bit is protected and can be written only if EN is 0.
    pub mod DBUS {
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
}

/// MDMA channel x Mask address register
pub mod C0MAR {

    /// Mask address
    pub mod MAR {
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

/// MDMA channel x Mask Data register
pub mod C0MDR {

    /// Mask data
    pub mod MDR {
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

/// MDMA channel x interrupt/status register
pub mod C1ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF1 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF1 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF1 {
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

    /// channel x buffer transfer complete
    pub mod TCIF1 {
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

    /// channel x request active flag
    pub mod CRQA1 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C1IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF1 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF1 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF1 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF1 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF1 {
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
}

/// MDMA Channel x error status register
pub mod C1ESR {
    pub use super::C0ESR::ASE;
    pub use super::C0ESR::BSE;
    pub use super::C0ESR::TEA;
    pub use super::C0ESR::TED;
    pub use super::C0ESR::TELD;
    pub use super::C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod C1CR {
    pub use super::C0CR::BEX;
    pub use super::C0CR::BRTIE;
    pub use super::C0CR::BTIE;
    pub use super::C0CR::CTCIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HEX;
    pub use super::C0CR::PL;
    pub use super::C0CR::SWRQ;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TEIE;
    pub use super::C0CR::WEX;
}

/// This register is used to configure the concerned channel.
pub mod C1TCR {
    pub use super::C0TCR::BWM;
    pub use super::C0TCR::DBURST;
    pub use super::C0TCR::DINC;
    pub use super::C0TCR::DINCOS;
    pub use super::C0TCR::DSIZE;
    pub use super::C0TCR::PAM;
    pub use super::C0TCR::PKE;
    pub use super::C0TCR::SBURST;
    pub use super::C0TCR::SINC;
    pub use super::C0TCR::SINCOS;
    pub use super::C0TCR::SSIZE;
    pub use super::C0TCR::SWRM;
    pub use super::C0TCR::TLEN;
    pub use super::C0TCR::TRGM;
}

/// MDMA Channel x block number of data register
pub mod C1BNDTR {
    pub use super::C0BNDTR::BNDT;
    pub use super::C0BNDTR::BRC;
    pub use super::C0BNDTR::BRDUM;
    pub use super::C0BNDTR::BRSUM;
}

/// MDMA channel x source address register
pub mod C1SAR {
    pub use super::C0SAR::SAR;
}

/// MDMA channel x destination address register
pub mod C1DAR {
    pub use super::C0DAR::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod C1BRUR {
    pub use super::C0BRUR::DUV;
    pub use super::C0BRUR::SUV;
}

/// MDMA channel x Link Address register
pub mod C1LAR {
    pub use super::C0LAR::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod C1TBR {
    pub use super::C0TBR::DBUS;
    pub use super::C0TBR::SBUS;
    pub use super::C0TBR::TSEL;
}

/// MDMA channel x Mask address register
pub mod C1MAR {
    pub use super::C0MAR::MAR;
}

/// MDMA channel x Mask Data register
pub mod C1MDR {
    pub use super::C0MDR::MDR;
}

/// MDMA channel x interrupt/status register
pub mod C2ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF2 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF2 {
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF2 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF2 {
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

    /// channel x buffer transfer complete
    pub mod TCIF2 {
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

    /// channel x request active flag
    pub mod CRQA2 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C2IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF2 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF2 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF2 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF2 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF2 {
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
}

/// MDMA Channel x error status register
pub mod C2ESR {
    pub use super::C0ESR::ASE;
    pub use super::C0ESR::BSE;
    pub use super::C0ESR::TEA;
    pub use super::C0ESR::TED;
    pub use super::C0ESR::TELD;
    pub use super::C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod C2CR {
    pub use super::C0CR::BEX;
    pub use super::C0CR::BRTIE;
    pub use super::C0CR::BTIE;
    pub use super::C0CR::CTCIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HEX;
    pub use super::C0CR::PL;
    pub use super::C0CR::SWRQ;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TEIE;
    pub use super::C0CR::WEX;
}

/// This register is used to configure the concerned channel.
pub mod C2TCR {
    pub use super::C0TCR::BWM;
    pub use super::C0TCR::DBURST;
    pub use super::C0TCR::DINC;
    pub use super::C0TCR::DINCOS;
    pub use super::C0TCR::DSIZE;
    pub use super::C0TCR::PAM;
    pub use super::C0TCR::PKE;
    pub use super::C0TCR::SBURST;
    pub use super::C0TCR::SINC;
    pub use super::C0TCR::SINCOS;
    pub use super::C0TCR::SSIZE;
    pub use super::C0TCR::SWRM;
    pub use super::C0TCR::TLEN;
    pub use super::C0TCR::TRGM;
}

/// MDMA Channel x block number of data register
pub mod C2BNDTR {
    pub use super::C0BNDTR::BNDT;
    pub use super::C0BNDTR::BRC;
    pub use super::C0BNDTR::BRDUM;
    pub use super::C0BNDTR::BRSUM;
}

/// MDMA channel x source address register
pub mod C2SAR {
    pub use super::C0SAR::SAR;
}

/// MDMA channel x destination address register
pub mod C2DAR {
    pub use super::C0DAR::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod C2BRUR {
    pub use super::C0BRUR::DUV;
    pub use super::C0BRUR::SUV;
}

/// MDMA channel x Link Address register
pub mod C2LAR {
    pub use super::C0LAR::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod C2TBR {
    pub use super::C0TBR::DBUS;
    pub use super::C0TBR::SBUS;
    pub use super::C0TBR::TSEL;
}

/// MDMA channel x Mask address register
pub mod C2MAR {
    pub use super::C0MAR::MAR;
}

/// MDMA channel x Mask Data register
pub mod C2MDR {
    pub use super::C0MDR::MDR;
}

/// MDMA channel x interrupt/status register
pub mod C3ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF3 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF3 {
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF3 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF3 {
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

    /// channel x buffer transfer complete
    pub mod TCIF3 {
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

    /// channel x request active flag
    pub mod CRQA3 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C3IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF3 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF3 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF3 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF3 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF3 {
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
}

/// MDMA Channel x error status register
pub mod C3ESR {
    pub use super::C0ESR::ASE;
    pub use super::C0ESR::BSE;
    pub use super::C0ESR::TEA;
    pub use super::C0ESR::TED;
    pub use super::C0ESR::TELD;
    pub use super::C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod C3CR {
    pub use super::C0CR::BEX;
    pub use super::C0CR::BRTIE;
    pub use super::C0CR::BTIE;
    pub use super::C0CR::CTCIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HEX;
    pub use super::C0CR::PL;
    pub use super::C0CR::SWRQ;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TEIE;
    pub use super::C0CR::WEX;
}

/// This register is used to configure the concerned channel.
pub mod C3TCR {
    pub use super::C0TCR::BWM;
    pub use super::C0TCR::DBURST;
    pub use super::C0TCR::DINC;
    pub use super::C0TCR::DINCOS;
    pub use super::C0TCR::DSIZE;
    pub use super::C0TCR::PAM;
    pub use super::C0TCR::PKE;
    pub use super::C0TCR::SBURST;
    pub use super::C0TCR::SINC;
    pub use super::C0TCR::SINCOS;
    pub use super::C0TCR::SSIZE;
    pub use super::C0TCR::SWRM;
    pub use super::C0TCR::TLEN;
    pub use super::C0TCR::TRGM;
}

/// MDMA Channel x block number of data register
pub mod C3BNDTR {
    pub use super::C0BNDTR::BNDT;
    pub use super::C0BNDTR::BRC;
    pub use super::C0BNDTR::BRDUM;
    pub use super::C0BNDTR::BRSUM;
}

/// MDMA channel x source address register
pub mod C3SAR {
    pub use super::C0SAR::SAR;
}

/// MDMA channel x destination address register
pub mod C3DAR {
    pub use super::C0DAR::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod C3BRUR {
    pub use super::C0BRUR::DUV;
    pub use super::C0BRUR::SUV;
}

/// MDMA channel x Link Address register
pub mod C3LAR {
    pub use super::C0LAR::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod C3TBR {
    pub use super::C0TBR::DBUS;
    pub use super::C0TBR::SBUS;
    pub use super::C0TBR::TSEL;
}

/// MDMA channel x Mask address register
pub mod C3MAR {
    pub use super::C0MAR::MAR;
}

/// MDMA channel x Mask Data register
pub mod C3MDR {
    pub use super::C0MDR::MDR;
}

/// MDMA channel x interrupt/status register
pub mod C4ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF4 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF4 {
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF4 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF4 {
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

    /// channel x buffer transfer complete
    pub mod TCIF4 {
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

    /// channel x request active flag
    pub mod CRQA4 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C4IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF4 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF4 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF4 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF4 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF4 {
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
}

/// MDMA Channel x error status register
pub mod C4ESR {
    pub use super::C0ESR::ASE;
    pub use super::C0ESR::BSE;
    pub use super::C0ESR::TEA;
    pub use super::C0ESR::TED;
    pub use super::C0ESR::TELD;
    pub use super::C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod C4CR {
    pub use super::C0CR::BEX;
    pub use super::C0CR::BRTIE;
    pub use super::C0CR::BTIE;
    pub use super::C0CR::CTCIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HEX;
    pub use super::C0CR::PL;
    pub use super::C0CR::SWRQ;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TEIE;
    pub use super::C0CR::WEX;
}

/// This register is used to configure the concerned channel.
pub mod C4TCR {
    pub use super::C0TCR::BWM;
    pub use super::C0TCR::DBURST;
    pub use super::C0TCR::DINC;
    pub use super::C0TCR::DINCOS;
    pub use super::C0TCR::DSIZE;
    pub use super::C0TCR::PAM;
    pub use super::C0TCR::PKE;
    pub use super::C0TCR::SBURST;
    pub use super::C0TCR::SINC;
    pub use super::C0TCR::SINCOS;
    pub use super::C0TCR::SSIZE;
    pub use super::C0TCR::SWRM;
    pub use super::C0TCR::TLEN;
    pub use super::C0TCR::TRGM;
}

/// MDMA Channel x block number of data register
pub mod C4BNDTR {
    pub use super::C0BNDTR::BNDT;
    pub use super::C0BNDTR::BRC;
    pub use super::C0BNDTR::BRDUM;
    pub use super::C0BNDTR::BRSUM;
}

/// MDMA channel x source address register
pub mod C4SAR {
    pub use super::C0SAR::SAR;
}

/// MDMA channel x destination address register
pub mod C4DAR {
    pub use super::C0DAR::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod C4BRUR {
    pub use super::C0BRUR::DUV;
    pub use super::C0BRUR::SUV;
}

/// MDMA channel x Link Address register
pub mod C4LAR {
    pub use super::C0LAR::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod C4TBR {
    pub use super::C0TBR::DBUS;
    pub use super::C0TBR::SBUS;
    pub use super::C0TBR::TSEL;
}

/// MDMA channel x Mask address register
pub mod C4MAR {
    pub use super::C0MAR::MAR;
}

/// MDMA channel x Mask Data register
pub mod C4MDR {
    pub use super::C0MDR::MDR;
}

/// MDMA channel x interrupt/status register
pub mod C5ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF5 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF5 {
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF5 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF5 {
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

    /// channel x buffer transfer complete
    pub mod TCIF5 {
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

    /// channel x request active flag
    pub mod CRQA5 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C5IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF5 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF5 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF5 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF5 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF5 {
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
}

/// MDMA Channel x error status register
pub mod C5ESR {
    pub use super::C0ESR::ASE;
    pub use super::C0ESR::BSE;
    pub use super::C0ESR::TEA;
    pub use super::C0ESR::TED;
    pub use super::C0ESR::TELD;
    pub use super::C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod C5CR {
    pub use super::C0CR::BEX;
    pub use super::C0CR::BRTIE;
    pub use super::C0CR::BTIE;
    pub use super::C0CR::CTCIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HEX;
    pub use super::C0CR::PL;
    pub use super::C0CR::SWRQ;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TEIE;
    pub use super::C0CR::WEX;
}

/// This register is used to configure the concerned channel.
pub mod C5TCR {
    pub use super::C0TCR::BWM;
    pub use super::C0TCR::DBURST;
    pub use super::C0TCR::DINC;
    pub use super::C0TCR::DINCOS;
    pub use super::C0TCR::DSIZE;
    pub use super::C0TCR::PAM;
    pub use super::C0TCR::PKE;
    pub use super::C0TCR::SBURST;
    pub use super::C0TCR::SINC;
    pub use super::C0TCR::SINCOS;
    pub use super::C0TCR::SSIZE;
    pub use super::C0TCR::SWRM;
    pub use super::C0TCR::TLEN;
    pub use super::C0TCR::TRGM;
}

/// MDMA Channel x block number of data register
pub mod C5BNDTR {
    pub use super::C0BNDTR::BNDT;
    pub use super::C0BNDTR::BRC;
    pub use super::C0BNDTR::BRDUM;
    pub use super::C0BNDTR::BRSUM;
}

/// MDMA channel x source address register
pub mod C5SAR {
    pub use super::C0SAR::SAR;
}

/// MDMA channel x destination address register
pub mod C5DAR {
    pub use super::C0DAR::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod C5BRUR {
    pub use super::C0BRUR::DUV;
    pub use super::C0BRUR::SUV;
}

/// MDMA channel x Link Address register
pub mod C5LAR {
    pub use super::C0LAR::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod C5TBR {
    pub use super::C0TBR::DBUS;
    pub use super::C0TBR::SBUS;
    pub use super::C0TBR::TSEL;
}

/// MDMA channel x Mask address register
pub mod C5MAR {
    pub use super::C0MAR::MAR;
}

/// MDMA channel x Mask Data register
pub mod C5MDR {
    pub use super::C0MDR::MDR;
}

/// MDMA channel x interrupt/status register
pub mod C6ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF6 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF6 {
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF6 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF6 {
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

    /// channel x buffer transfer complete
    pub mod TCIF6 {
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

    /// channel x request active flag
    pub mod CRQA6 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C6IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF6 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF6 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF6 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF6 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF6 {
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
}

/// MDMA Channel x error status register
pub mod C6ESR {
    pub use super::C0ESR::ASE;
    pub use super::C0ESR::BSE;
    pub use super::C0ESR::TEA;
    pub use super::C0ESR::TED;
    pub use super::C0ESR::TELD;
    pub use super::C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod C6CR {
    pub use super::C0CR::BEX;
    pub use super::C0CR::BRTIE;
    pub use super::C0CR::BTIE;
    pub use super::C0CR::CTCIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HEX;
    pub use super::C0CR::PL;
    pub use super::C0CR::SWRQ;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TEIE;
    pub use super::C0CR::WEX;
}

/// This register is used to configure the concerned channel.
pub mod C6TCR {
    pub use super::C0TCR::BWM;
    pub use super::C0TCR::DBURST;
    pub use super::C0TCR::DINC;
    pub use super::C0TCR::DINCOS;
    pub use super::C0TCR::DSIZE;
    pub use super::C0TCR::PAM;
    pub use super::C0TCR::PKE;
    pub use super::C0TCR::SBURST;
    pub use super::C0TCR::SINC;
    pub use super::C0TCR::SINCOS;
    pub use super::C0TCR::SSIZE;
    pub use super::C0TCR::SWRM;
    pub use super::C0TCR::TLEN;
    pub use super::C0TCR::TRGM;
}

/// MDMA Channel x block number of data register
pub mod C6BNDTR {
    pub use super::C0BNDTR::BNDT;
    pub use super::C0BNDTR::BRC;
    pub use super::C0BNDTR::BRDUM;
    pub use super::C0BNDTR::BRSUM;
}

/// MDMA channel x source address register
pub mod C6SAR {
    pub use super::C0SAR::SAR;
}

/// MDMA channel x destination address register
pub mod C6DAR {
    pub use super::C0DAR::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod C6BRUR {
    pub use super::C0BRUR::DUV;
    pub use super::C0BRUR::SUV;
}

/// MDMA channel x Link Address register
pub mod C6LAR {
    pub use super::C0LAR::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod C6TBR {
    pub use super::C0TBR::DBUS;
    pub use super::C0TBR::SBUS;
    pub use super::C0TBR::TSEL;
}

/// MDMA channel x Mask address register
pub mod C6MAR {
    pub use super::C0MAR::MAR;
}

/// MDMA channel x Mask Data register
pub mod C6MDR {
    pub use super::C0MDR::MDR;
}

/// MDMA channel x interrupt/status register
pub mod C7ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF7 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF7 {
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF7 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF7 {
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

    /// channel x buffer transfer complete
    pub mod TCIF7 {
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

    /// channel x request active flag
    pub mod CRQA7 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C7IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF7 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF7 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF7 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF7 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF7 {
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
}

/// MDMA Channel x error status register
pub mod C7ESR {
    pub use super::C0ESR::ASE;
    pub use super::C0ESR::BSE;
    pub use super::C0ESR::TEA;
    pub use super::C0ESR::TED;
    pub use super::C0ESR::TELD;
    pub use super::C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod C7CR {
    pub use super::C0CR::BEX;
    pub use super::C0CR::BRTIE;
    pub use super::C0CR::BTIE;
    pub use super::C0CR::CTCIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HEX;
    pub use super::C0CR::PL;
    pub use super::C0CR::SWRQ;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TEIE;
    pub use super::C0CR::WEX;
}

/// This register is used to configure the concerned channel.
pub mod C7TCR {
    pub use super::C0TCR::BWM;
    pub use super::C0TCR::DBURST;
    pub use super::C0TCR::DINC;
    pub use super::C0TCR::DINCOS;
    pub use super::C0TCR::DSIZE;
    pub use super::C0TCR::PAM;
    pub use super::C0TCR::PKE;
    pub use super::C0TCR::SBURST;
    pub use super::C0TCR::SINC;
    pub use super::C0TCR::SINCOS;
    pub use super::C0TCR::SSIZE;
    pub use super::C0TCR::SWRM;
    pub use super::C0TCR::TLEN;
    pub use super::C0TCR::TRGM;
}

/// MDMA Channel x block number of data register
pub mod C7BNDTR {
    pub use super::C0BNDTR::BNDT;
    pub use super::C0BNDTR::BRC;
    pub use super::C0BNDTR::BRDUM;
    pub use super::C0BNDTR::BRSUM;
}

/// MDMA channel x source address register
pub mod C7SAR {
    pub use super::C0SAR::SAR;
}

/// MDMA channel x destination address register
pub mod C7DAR {
    pub use super::C0DAR::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod C7BRUR {
    pub use super::C0BRUR::DUV;
    pub use super::C0BRUR::SUV;
}

/// MDMA channel x Link Address register
pub mod C7LAR {
    pub use super::C0LAR::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod C7TBR {
    pub use super::C0TBR::DBUS;
    pub use super::C0TBR::SBUS;
    pub use super::C0TBR::TSEL;
}

/// MDMA channel x Mask address register
pub mod C7MAR {
    pub use super::C0MAR::MAR;
}

/// MDMA channel x Mask Data register
pub mod C7MDR {
    pub use super::C0MDR::MDR;
}

/// MDMA channel x interrupt/status register
pub mod C8ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF8 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF8 {
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF8 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF8 {
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

    /// channel x buffer transfer complete
    pub mod TCIF8 {
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

    /// channel x request active flag
    pub mod CRQA8 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C8IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF8 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF8 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF8 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF8 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF8 {
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
}

/// MDMA Channel x error status register
pub mod C8ESR {
    pub use super::C0ESR::ASE;
    pub use super::C0ESR::BSE;
    pub use super::C0ESR::TEA;
    pub use super::C0ESR::TED;
    pub use super::C0ESR::TELD;
    pub use super::C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod C8CR {
    pub use super::C0CR::BEX;
    pub use super::C0CR::BRTIE;
    pub use super::C0CR::BTIE;
    pub use super::C0CR::CTCIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HEX;
    pub use super::C0CR::PL;
    pub use super::C0CR::SWRQ;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TEIE;
    pub use super::C0CR::WEX;
}

/// This register is used to configure the concerned channel.
pub mod C8TCR {
    pub use super::C0TCR::BWM;
    pub use super::C0TCR::DBURST;
    pub use super::C0TCR::DINC;
    pub use super::C0TCR::DINCOS;
    pub use super::C0TCR::DSIZE;
    pub use super::C0TCR::PAM;
    pub use super::C0TCR::PKE;
    pub use super::C0TCR::SBURST;
    pub use super::C0TCR::SINC;
    pub use super::C0TCR::SINCOS;
    pub use super::C0TCR::SSIZE;
    pub use super::C0TCR::SWRM;
    pub use super::C0TCR::TLEN;
    pub use super::C0TCR::TRGM;
}

/// MDMA Channel x block number of data register
pub mod C8BNDTR {
    pub use super::C0BNDTR::BNDT;
    pub use super::C0BNDTR::BRC;
    pub use super::C0BNDTR::BRDUM;
    pub use super::C0BNDTR::BRSUM;
}

/// MDMA channel x source address register
pub mod C8SAR {
    pub use super::C0SAR::SAR;
}

/// MDMA channel x destination address register
pub mod C8DAR {
    pub use super::C0DAR::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod C8BRUR {
    pub use super::C0BRUR::DUV;
    pub use super::C0BRUR::SUV;
}

/// MDMA channel x Link Address register
pub mod C8LAR {
    pub use super::C0LAR::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod C8TBR {
    pub use super::C0TBR::DBUS;
    pub use super::C0TBR::SBUS;
    pub use super::C0TBR::TSEL;
}

/// MDMA channel x Mask address register
pub mod C8MAR {
    pub use super::C0MAR::MAR;
}

/// MDMA channel x Mask Data register
pub mod C8MDR {
    pub use super::C0MDR::MDR;
}

/// MDMA channel x interrupt/status register
pub mod C9ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF9 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF9 {
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF9 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF9 {
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

    /// channel x buffer transfer complete
    pub mod TCIF9 {
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

    /// channel x request active flag
    pub mod CRQA9 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C9IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF9 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF9 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF9 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF9 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF9 {
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
}

/// MDMA Channel x error status register
pub mod C9ESR {
    pub use super::C0ESR::ASE;
    pub use super::C0ESR::BSE;
    pub use super::C0ESR::TEA;
    pub use super::C0ESR::TED;
    pub use super::C0ESR::TELD;
    pub use super::C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod C9CR {
    pub use super::C0CR::BEX;
    pub use super::C0CR::BRTIE;
    pub use super::C0CR::BTIE;
    pub use super::C0CR::CTCIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HEX;
    pub use super::C0CR::PL;
    pub use super::C0CR::SWRQ;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TEIE;
    pub use super::C0CR::WEX;
}

/// This register is used to configure the concerned channel.
pub mod C9TCR {
    pub use super::C0TCR::BWM;
    pub use super::C0TCR::DBURST;
    pub use super::C0TCR::DINC;
    pub use super::C0TCR::DINCOS;
    pub use super::C0TCR::DSIZE;
    pub use super::C0TCR::PAM;
    pub use super::C0TCR::PKE;
    pub use super::C0TCR::SBURST;
    pub use super::C0TCR::SINC;
    pub use super::C0TCR::SINCOS;
    pub use super::C0TCR::SSIZE;
    pub use super::C0TCR::SWRM;
    pub use super::C0TCR::TLEN;
    pub use super::C0TCR::TRGM;
}

/// MDMA Channel x block number of data register
pub mod C9BNDTR {
    pub use super::C0BNDTR::BNDT;
    pub use super::C0BNDTR::BRC;
    pub use super::C0BNDTR::BRDUM;
    pub use super::C0BNDTR::BRSUM;
}

/// MDMA channel x source address register
pub mod C9SAR {
    pub use super::C0SAR::SAR;
}

/// MDMA channel x destination address register
pub mod C9DAR {
    pub use super::C0DAR::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod C9BRUR {
    pub use super::C0BRUR::DUV;
    pub use super::C0BRUR::SUV;
}

/// MDMA channel x Link Address register
pub mod C9LAR {
    pub use super::C0LAR::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod C9TBR {
    pub use super::C0TBR::DBUS;
    pub use super::C0TBR::SBUS;
    pub use super::C0TBR::TSEL;
}

/// MDMA channel x Mask address register
pub mod C9MAR {
    pub use super::C0MAR::MAR;
}

/// MDMA channel x Mask Data register
pub mod C9MDR {
    pub use super::C0MDR::MDR;
}

/// MDMA channel x interrupt/status register
pub mod C10ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF10 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF10 {
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF10 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF10 {
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

    /// channel x buffer transfer complete
    pub mod TCIF10 {
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

    /// channel x request active flag
    pub mod CRQA10 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C10IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF10 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF10 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF10 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF10 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF10 {
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
}

/// MDMA Channel x error status register
pub mod C10ESR {
    pub use super::C0ESR::ASE;
    pub use super::C0ESR::BSE;
    pub use super::C0ESR::TEA;
    pub use super::C0ESR::TED;
    pub use super::C0ESR::TELD;
    pub use super::C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod C10CR {
    pub use super::C0CR::BEX;
    pub use super::C0CR::BRTIE;
    pub use super::C0CR::BTIE;
    pub use super::C0CR::CTCIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HEX;
    pub use super::C0CR::PL;
    pub use super::C0CR::SWRQ;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TEIE;
    pub use super::C0CR::WEX;
}

/// This register is used to configure the concerned channel.
pub mod C10TCR {
    pub use super::C0TCR::BWM;
    pub use super::C0TCR::DBURST;
    pub use super::C0TCR::DINC;
    pub use super::C0TCR::DINCOS;
    pub use super::C0TCR::DSIZE;
    pub use super::C0TCR::PAM;
    pub use super::C0TCR::PKE;
    pub use super::C0TCR::SBURST;
    pub use super::C0TCR::SINC;
    pub use super::C0TCR::SINCOS;
    pub use super::C0TCR::SSIZE;
    pub use super::C0TCR::SWRM;
    pub use super::C0TCR::TLEN;
    pub use super::C0TCR::TRGM;
}

/// MDMA Channel x block number of data register
pub mod C10BNDTR {
    pub use super::C0BNDTR::BNDT;
    pub use super::C0BNDTR::BRC;
    pub use super::C0BNDTR::BRDUM;
    pub use super::C0BNDTR::BRSUM;
}

/// MDMA channel x source address register
pub mod C10SAR {
    pub use super::C0SAR::SAR;
}

/// MDMA channel x destination address register
pub mod C10DAR {
    pub use super::C0DAR::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod C10BRUR {
    pub use super::C0BRUR::DUV;
    pub use super::C0BRUR::SUV;
}

/// MDMA channel x Link Address register
pub mod C10LAR {
    pub use super::C0LAR::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod C10TBR {
    pub use super::C0TBR::DBUS;
    pub use super::C0TBR::SBUS;
    pub use super::C0TBR::TSEL;
}

/// MDMA channel x Mask address register
pub mod C10MAR {
    pub use super::C0MAR::MAR;
}

/// MDMA channel x Mask Data register
pub mod C10MDR {
    pub use super::C0MDR::MDR;
}

/// MDMA channel x interrupt/status register
pub mod C11ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF11 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF11 {
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF11 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF11 {
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

    /// channel x buffer transfer complete
    pub mod TCIF11 {
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

    /// channel x request active flag
    pub mod CRQA11 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C11IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF11 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF11 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF11 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF11 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF11 {
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
}

/// MDMA Channel x error status register
pub mod C11ESR {
    pub use super::C0ESR::ASE;
    pub use super::C0ESR::BSE;
    pub use super::C0ESR::TEA;
    pub use super::C0ESR::TED;
    pub use super::C0ESR::TELD;
    pub use super::C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod C11CR {
    pub use super::C0CR::BEX;
    pub use super::C0CR::BRTIE;
    pub use super::C0CR::BTIE;
    pub use super::C0CR::CTCIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HEX;
    pub use super::C0CR::PL;
    pub use super::C0CR::SWRQ;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TEIE;
    pub use super::C0CR::WEX;
}

/// This register is used to configure the concerned channel.
pub mod C11TCR {
    pub use super::C0TCR::BWM;
    pub use super::C0TCR::DBURST;
    pub use super::C0TCR::DINC;
    pub use super::C0TCR::DINCOS;
    pub use super::C0TCR::DSIZE;
    pub use super::C0TCR::PAM;
    pub use super::C0TCR::PKE;
    pub use super::C0TCR::SBURST;
    pub use super::C0TCR::SINC;
    pub use super::C0TCR::SINCOS;
    pub use super::C0TCR::SSIZE;
    pub use super::C0TCR::SWRM;
    pub use super::C0TCR::TLEN;
    pub use super::C0TCR::TRGM;
}

/// MDMA Channel x block number of data register
pub mod C11BNDTR {
    pub use super::C0BNDTR::BNDT;
    pub use super::C0BNDTR::BRC;
    pub use super::C0BNDTR::BRDUM;
    pub use super::C0BNDTR::BRSUM;
}

/// MDMA channel x source address register
pub mod C11SAR {
    pub use super::C0SAR::SAR;
}

/// MDMA channel x destination address register
pub mod C11DAR {
    pub use super::C0DAR::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod C11BRUR {
    pub use super::C0BRUR::DUV;
    pub use super::C0BRUR::SUV;
}

/// MDMA channel x Link Address register
pub mod C11LAR {
    pub use super::C0LAR::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod C11TBR {
    pub use super::C0TBR::DBUS;
    pub use super::C0TBR::SBUS;
    pub use super::C0TBR::TSEL;
}

/// MDMA channel x Mask address register
pub mod C11MAR {
    pub use super::C0MAR::MAR;
}

/// MDMA channel x Mask Data register
pub mod C11MDR {
    pub use super::C0MDR::MDR;
}

/// MDMA channel x interrupt/status register
pub mod C12ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF12 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF12 {
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF12 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF12 {
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

    /// channel x buffer transfer complete
    pub mod TCIF12 {
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

    /// channel x request active flag
    pub mod CRQA12 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C12IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF12 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF12 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF12 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF12 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF12 {
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
}

/// MDMA Channel x error status register
pub mod C12ESR {
    pub use super::C0ESR::ASE;
    pub use super::C0ESR::BSE;
    pub use super::C0ESR::TEA;
    pub use super::C0ESR::TED;
    pub use super::C0ESR::TELD;
    pub use super::C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod C12CR {
    pub use super::C0CR::BEX;
    pub use super::C0CR::BRTIE;
    pub use super::C0CR::BTIE;
    pub use super::C0CR::CTCIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HEX;
    pub use super::C0CR::PL;
    pub use super::C0CR::SWRQ;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TEIE;
    pub use super::C0CR::WEX;
}

/// This register is used to configure the concerned channel.
pub mod C12TCR {
    pub use super::C0TCR::BWM;
    pub use super::C0TCR::DBURST;
    pub use super::C0TCR::DINC;
    pub use super::C0TCR::DINCOS;
    pub use super::C0TCR::DSIZE;
    pub use super::C0TCR::PAM;
    pub use super::C0TCR::PKE;
    pub use super::C0TCR::SBURST;
    pub use super::C0TCR::SINC;
    pub use super::C0TCR::SINCOS;
    pub use super::C0TCR::SSIZE;
    pub use super::C0TCR::SWRM;
    pub use super::C0TCR::TLEN;
    pub use super::C0TCR::TRGM;
}

/// MDMA Channel x block number of data register
pub mod C12BNDTR {
    pub use super::C0BNDTR::BNDT;
    pub use super::C0BNDTR::BRC;
    pub use super::C0BNDTR::BRDUM;
    pub use super::C0BNDTR::BRSUM;
}

/// MDMA channel x source address register
pub mod C12SAR {
    pub use super::C0SAR::SAR;
}

/// MDMA channel x destination address register
pub mod C12DAR {
    pub use super::C0DAR::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod C12BRUR {
    pub use super::C0BRUR::DUV;
    pub use super::C0BRUR::SUV;
}

/// MDMA channel x Link Address register
pub mod C12LAR {
    pub use super::C0LAR::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod C12TBR {
    pub use super::C0TBR::DBUS;
    pub use super::C0TBR::SBUS;
    pub use super::C0TBR::TSEL;
}

/// MDMA channel x Mask address register
pub mod C12MAR {
    pub use super::C0MAR::MAR;
}

/// MDMA channel x Mask Data register
pub mod C12MDR {
    pub use super::C0MDR::MDR;
}

/// MDMA channel x interrupt/status register
pub mod C13ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF13 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF13 {
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF13 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF13 {
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

    /// channel x buffer transfer complete
    pub mod TCIF13 {
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

    /// channel x request active flag
    pub mod CRQA13 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C13IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF13 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF13 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF13 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF13 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF13 {
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
}

/// MDMA Channel x error status register
pub mod C13ESR {
    pub use super::C0ESR::ASE;
    pub use super::C0ESR::BSE;
    pub use super::C0ESR::TEA;
    pub use super::C0ESR::TED;
    pub use super::C0ESR::TELD;
    pub use super::C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod C13CR {
    pub use super::C0CR::BEX;
    pub use super::C0CR::BRTIE;
    pub use super::C0CR::BTIE;
    pub use super::C0CR::CTCIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HEX;
    pub use super::C0CR::PL;
    pub use super::C0CR::SWRQ;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TEIE;
    pub use super::C0CR::WEX;
}

/// This register is used to configure the concerned channel.
pub mod C13TCR {
    pub use super::C0TCR::BWM;
    pub use super::C0TCR::DBURST;
    pub use super::C0TCR::DINC;
    pub use super::C0TCR::DINCOS;
    pub use super::C0TCR::DSIZE;
    pub use super::C0TCR::PAM;
    pub use super::C0TCR::PKE;
    pub use super::C0TCR::SBURST;
    pub use super::C0TCR::SINC;
    pub use super::C0TCR::SINCOS;
    pub use super::C0TCR::SSIZE;
    pub use super::C0TCR::SWRM;
    pub use super::C0TCR::TLEN;
    pub use super::C0TCR::TRGM;
}

/// MDMA Channel x block number of data register
pub mod C13BNDTR {
    pub use super::C0BNDTR::BNDT;
    pub use super::C0BNDTR::BRC;
    pub use super::C0BNDTR::BRDUM;
    pub use super::C0BNDTR::BRSUM;
}

/// MDMA channel x source address register
pub mod C13SAR {
    pub use super::C0SAR::SAR;
}

/// MDMA channel x destination address register
pub mod C13DAR {
    pub use super::C0DAR::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod C13BRUR {
    pub use super::C0BRUR::DUV;
    pub use super::C0BRUR::SUV;
}

/// MDMA channel x Link Address register
pub mod C13LAR {
    pub use super::C0LAR::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod C13TBR {
    pub use super::C0TBR::DBUS;
    pub use super::C0TBR::SBUS;
    pub use super::C0TBR::TSEL;
}

/// MDMA channel x Mask address register
pub mod C13MAR {
    pub use super::C0MAR::MAR;
}

/// MDMA channel x Mask Data register
pub mod C13MDR {
    pub use super::C0MDR::MDR;
}

/// MDMA channel x interrupt/status register
pub mod C14ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF14 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF14 {
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF14 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF14 {
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

    /// channel x buffer transfer complete
    pub mod TCIF14 {
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

    /// channel x request active flag
    pub mod CRQA14 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C14IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF14 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF14 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF14 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF14 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF14 {
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
}

/// MDMA Channel x error status register
pub mod C14ESR {
    pub use super::C0ESR::ASE;
    pub use super::C0ESR::BSE;
    pub use super::C0ESR::TEA;
    pub use super::C0ESR::TED;
    pub use super::C0ESR::TELD;
    pub use super::C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod C14CR {
    pub use super::C0CR::BEX;
    pub use super::C0CR::BRTIE;
    pub use super::C0CR::BTIE;
    pub use super::C0CR::CTCIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HEX;
    pub use super::C0CR::PL;
    pub use super::C0CR::SWRQ;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TEIE;
    pub use super::C0CR::WEX;
}

/// This register is used to configure the concerned channel.
pub mod C14TCR {
    pub use super::C0TCR::BWM;
    pub use super::C0TCR::DBURST;
    pub use super::C0TCR::DINC;
    pub use super::C0TCR::DINCOS;
    pub use super::C0TCR::DSIZE;
    pub use super::C0TCR::PAM;
    pub use super::C0TCR::PKE;
    pub use super::C0TCR::SBURST;
    pub use super::C0TCR::SINC;
    pub use super::C0TCR::SINCOS;
    pub use super::C0TCR::SSIZE;
    pub use super::C0TCR::SWRM;
    pub use super::C0TCR::TLEN;
    pub use super::C0TCR::TRGM;
}

/// MDMA Channel x block number of data register
pub mod C14BNDTR {
    pub use super::C0BNDTR::BNDT;
    pub use super::C0BNDTR::BRC;
    pub use super::C0BNDTR::BRDUM;
    pub use super::C0BNDTR::BRSUM;
}

/// MDMA channel x source address register
pub mod C14SAR {
    pub use super::C0SAR::SAR;
}

/// MDMA channel x destination address register
pub mod C14DAR {
    pub use super::C0DAR::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod C14BRUR {
    pub use super::C0BRUR::DUV;
    pub use super::C0BRUR::SUV;
}

/// MDMA channel x Link Address register
pub mod C14LAR {
    pub use super::C0LAR::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod C14TBR {
    pub use super::C0TBR::DBUS;
    pub use super::C0TBR::SBUS;
    pub use super::C0TBR::TSEL;
}

/// MDMA channel x Mask address register
pub mod C14MAR {
    pub use super::C0MAR::MAR;
}

/// MDMA channel x Mask Data register
pub mod C14MDR {
    pub use super::C0MDR::MDR;
}

/// MDMA channel x interrupt/status register
pub mod C15ISR {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod TEIF15 {
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF15 {
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

    /// Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BRTIF15 {
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

    /// Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
    pub mod BTIF15 {
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

    /// channel x buffer transfer complete
    pub mod TCIF15 {
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

    /// channel x request active flag
    pub mod CRQA15 {
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
}

/// MDMA channel x interrupt flag clear register
pub mod C15IFCR {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF15 {
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

    /// Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register
    pub mod CCTCIF15 {
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

    /// Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register
    pub mod CBRTIF15 {
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

    /// Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register
    pub mod CBTIF15 {
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

    /// CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register
    pub mod CLTCIF15 {
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
}

/// MDMA Channel x error status register
pub mod C15ESR {
    pub use super::C0ESR::ASE;
    pub use super::C0ESR::BSE;
    pub use super::C0ESR::TEA;
    pub use super::C0ESR::TED;
    pub use super::C0ESR::TELD;
    pub use super::C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod C15CR {
    pub use super::C0CR::BEX;
    pub use super::C0CR::BRTIE;
    pub use super::C0CR::BTIE;
    pub use super::C0CR::CTCIE;
    pub use super::C0CR::EN;
    pub use super::C0CR::HEX;
    pub use super::C0CR::PL;
    pub use super::C0CR::SWRQ;
    pub use super::C0CR::TCIE;
    pub use super::C0CR::TEIE;
    pub use super::C0CR::WEX;
}

/// This register is used to configure the concerned channel.
pub mod C15TCR {
    pub use super::C0TCR::BWM;
    pub use super::C0TCR::DBURST;
    pub use super::C0TCR::DINC;
    pub use super::C0TCR::DINCOS;
    pub use super::C0TCR::DSIZE;
    pub use super::C0TCR::PAM;
    pub use super::C0TCR::PKE;
    pub use super::C0TCR::SBURST;
    pub use super::C0TCR::SINC;
    pub use super::C0TCR::SINCOS;
    pub use super::C0TCR::SSIZE;
    pub use super::C0TCR::SWRM;
    pub use super::C0TCR::TLEN;
    pub use super::C0TCR::TRGM;
}

/// MDMA Channel x block number of data register
pub mod C15BNDTR {
    pub use super::C0BNDTR::BNDT;
    pub use super::C0BNDTR::BRC;
    pub use super::C0BNDTR::BRDUM;
    pub use super::C0BNDTR::BRSUM;
}

/// MDMA channel x source address register
pub mod C15SAR {
    pub use super::C0SAR::SAR;
}

/// MDMA channel x destination address register
pub mod C15DAR {
    pub use super::C0DAR::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod C15BRUR {
    pub use super::C0BRUR::DUV;
    pub use super::C0BRUR::SUV;
}

/// MDMA channel x Link Address register
pub mod C15LAR {
    pub use super::C0LAR::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod C15TBR {
    pub use super::C0TBR::DBUS;
    pub use super::C0TBR::SBUS;
    pub use super::C0TBR::TSEL;
}

/// MDMA channel x Mask address register
pub mod C15MAR {
    pub use super::C0MAR::MAR;
}

/// MDMA channel x Mask Data register
pub mod C15MDR {
    pub use super::C0MDR::MDR;
}
#[repr(C)]
pub struct RegisterBlock {
    /// MDMA Global Interrupt/Status Register
    pub GISR0: RORegister<u32>,

    _reserved1: [u32; 15],

    /// MDMA channel x interrupt/status register
    pub C0ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C0IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C0ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C0CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C0TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C0BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C0SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C0DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C0BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C0LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C0TBR: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// MDMA channel x Mask address register
    pub C0MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C0MDR: RWRegister<u32>,

    _reserved3: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub C1ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C1IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C1ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C1CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C1TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C1BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C1SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C1DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C1BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C1LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C1TBR: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// MDMA channel x Mask address register
    pub C1MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C1MDR: RWRegister<u32>,

    _reserved5: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub C2ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C2IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C2ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C2CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C2TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C2BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C2SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C2DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C2BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C2LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C2TBR: RWRegister<u32>,

    _reserved6: [u32; 1],

    /// MDMA channel x Mask address register
    pub C2MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C2MDR: RWRegister<u32>,

    _reserved7: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub C3ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C3IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C3ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C3CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C3TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C3BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C3SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C3DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C3BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C3LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C3TBR: RWRegister<u32>,

    _reserved8: [u32; 1],

    /// MDMA channel x Mask address register
    pub C3MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C3MDR: RWRegister<u32>,

    _reserved9: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub C4ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C4IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C4ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C4CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C4TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C4BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C4SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C4DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C4BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C4LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C4TBR: RWRegister<u32>,

    _reserved10: [u32; 1],

    /// MDMA channel x Mask address register
    pub C4MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C4MDR: RWRegister<u32>,

    _reserved11: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub C5ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C5IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C5ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C5CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C5TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C5BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C5SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C5DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C5BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C5LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C5TBR: RWRegister<u32>,

    _reserved12: [u32; 1],

    /// MDMA channel x Mask address register
    pub C5MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C5MDR: RWRegister<u32>,

    _reserved13: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub C6ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C6IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C6ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C6CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C6TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C6BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C6SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C6DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C6BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C6LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C6TBR: RWRegister<u32>,

    _reserved14: [u32; 1],

    /// MDMA channel x Mask address register
    pub C6MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C6MDR: RWRegister<u32>,

    _reserved15: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub C7ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C7IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C7ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C7CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C7TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C7BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C7SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C7DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C7BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C7LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C7TBR: RWRegister<u32>,

    _reserved16: [u32; 1],

    /// MDMA channel x Mask address register
    pub C7MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C7MDR: RWRegister<u32>,

    _reserved17: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub C8ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C8IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C8ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C8CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C8TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C8BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C8SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C8DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C8BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C8LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C8TBR: RWRegister<u32>,

    _reserved18: [u32; 1],

    /// MDMA channel x Mask address register
    pub C8MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C8MDR: RWRegister<u32>,

    _reserved19: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub C9ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C9IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C9ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C9CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C9TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C9BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C9SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C9DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C9BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C9LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C9TBR: RWRegister<u32>,

    _reserved20: [u32; 1],

    /// MDMA channel x Mask address register
    pub C9MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C9MDR: RWRegister<u32>,

    _reserved21: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub C10ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C10IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C10ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C10CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C10TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C10BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C10SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C10DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C10BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C10LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C10TBR: RWRegister<u32>,

    _reserved22: [u32; 1],

    /// MDMA channel x Mask address register
    pub C10MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C10MDR: RWRegister<u32>,

    _reserved23: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub C11ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C11IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C11ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C11CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C11TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C11BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C11SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C11DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C11BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C11LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C11TBR: RWRegister<u32>,

    _reserved24: [u32; 1],

    /// MDMA channel x Mask address register
    pub C11MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C11MDR: RWRegister<u32>,

    _reserved25: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub C12ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C12IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C12ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C12CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C12TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C12BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C12SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C12DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C12BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C12LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C12TBR: RWRegister<u32>,

    _reserved26: [u32; 1],

    /// MDMA channel x Mask address register
    pub C12MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C12MDR: RWRegister<u32>,

    _reserved27: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub C13ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C13IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C13ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C13CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C13TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C13BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C13SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C13DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C13BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C13LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C13TBR: RWRegister<u32>,

    _reserved28: [u32; 1],

    /// MDMA channel x Mask address register
    pub C13MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C13MDR: RWRegister<u32>,

    _reserved29: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub C14ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C14IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C14ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C14CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C14TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C14BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C14SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C14DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C14BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C14LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C14TBR: RWRegister<u32>,

    _reserved30: [u32; 1],

    /// MDMA channel x Mask address register
    pub C14MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C14MDR: RWRegister<u32>,

    _reserved31: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub C15ISR: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub C15IFCR: WORegister<u32>,

    /// MDMA Channel x error status register
    pub C15ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub C15CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub C15TCR: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub C15BNDTR: RWRegister<u32>,

    /// MDMA channel x source address register
    pub C15SAR: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub C15DAR: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub C15BRUR: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub C15LAR: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub C15TBR: RWRegister<u32>,

    _reserved32: [u32; 1],

    /// MDMA channel x Mask address register
    pub C15MAR: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub C15MDR: RWRegister<u32>,
}
pub struct ResetValues {
    pub GISR0: u32,
    pub C0ISR: u32,
    pub C0IFCR: u32,
    pub C0ESR: u32,
    pub C0CR: u32,
    pub C0TCR: u32,
    pub C0BNDTR: u32,
    pub C0SAR: u32,
    pub C0DAR: u32,
    pub C0BRUR: u32,
    pub C0LAR: u32,
    pub C0TBR: u32,
    pub C0MAR: u32,
    pub C0MDR: u32,
    pub C1ISR: u32,
    pub C1IFCR: u32,
    pub C1ESR: u32,
    pub C1CR: u32,
    pub C1TCR: u32,
    pub C1BNDTR: u32,
    pub C1SAR: u32,
    pub C1DAR: u32,
    pub C1BRUR: u32,
    pub C1LAR: u32,
    pub C1TBR: u32,
    pub C1MAR: u32,
    pub C1MDR: u32,
    pub C2ISR: u32,
    pub C2IFCR: u32,
    pub C2ESR: u32,
    pub C2CR: u32,
    pub C2TCR: u32,
    pub C2BNDTR: u32,
    pub C2SAR: u32,
    pub C2DAR: u32,
    pub C2BRUR: u32,
    pub C2LAR: u32,
    pub C2TBR: u32,
    pub C2MAR: u32,
    pub C2MDR: u32,
    pub C3ISR: u32,
    pub C3IFCR: u32,
    pub C3ESR: u32,
    pub C3CR: u32,
    pub C3TCR: u32,
    pub C3BNDTR: u32,
    pub C3SAR: u32,
    pub C3DAR: u32,
    pub C3BRUR: u32,
    pub C3LAR: u32,
    pub C3TBR: u32,
    pub C3MAR: u32,
    pub C3MDR: u32,
    pub C4ISR: u32,
    pub C4IFCR: u32,
    pub C4ESR: u32,
    pub C4CR: u32,
    pub C4TCR: u32,
    pub C4BNDTR: u32,
    pub C4SAR: u32,
    pub C4DAR: u32,
    pub C4BRUR: u32,
    pub C4LAR: u32,
    pub C4TBR: u32,
    pub C4MAR: u32,
    pub C4MDR: u32,
    pub C5ISR: u32,
    pub C5IFCR: u32,
    pub C5ESR: u32,
    pub C5CR: u32,
    pub C5TCR: u32,
    pub C5BNDTR: u32,
    pub C5SAR: u32,
    pub C5DAR: u32,
    pub C5BRUR: u32,
    pub C5LAR: u32,
    pub C5TBR: u32,
    pub C5MAR: u32,
    pub C5MDR: u32,
    pub C6ISR: u32,
    pub C6IFCR: u32,
    pub C6ESR: u32,
    pub C6CR: u32,
    pub C6TCR: u32,
    pub C6BNDTR: u32,
    pub C6SAR: u32,
    pub C6DAR: u32,
    pub C6BRUR: u32,
    pub C6LAR: u32,
    pub C6TBR: u32,
    pub C6MAR: u32,
    pub C6MDR: u32,
    pub C7ISR: u32,
    pub C7IFCR: u32,
    pub C7ESR: u32,
    pub C7CR: u32,
    pub C7TCR: u32,
    pub C7BNDTR: u32,
    pub C7SAR: u32,
    pub C7DAR: u32,
    pub C7BRUR: u32,
    pub C7LAR: u32,
    pub C7TBR: u32,
    pub C7MAR: u32,
    pub C7MDR: u32,
    pub C8ISR: u32,
    pub C8IFCR: u32,
    pub C8ESR: u32,
    pub C8CR: u32,
    pub C8TCR: u32,
    pub C8BNDTR: u32,
    pub C8SAR: u32,
    pub C8DAR: u32,
    pub C8BRUR: u32,
    pub C8LAR: u32,
    pub C8TBR: u32,
    pub C8MAR: u32,
    pub C8MDR: u32,
    pub C9ISR: u32,
    pub C9IFCR: u32,
    pub C9ESR: u32,
    pub C9CR: u32,
    pub C9TCR: u32,
    pub C9BNDTR: u32,
    pub C9SAR: u32,
    pub C9DAR: u32,
    pub C9BRUR: u32,
    pub C9LAR: u32,
    pub C9TBR: u32,
    pub C9MAR: u32,
    pub C9MDR: u32,
    pub C10ISR: u32,
    pub C10IFCR: u32,
    pub C10ESR: u32,
    pub C10CR: u32,
    pub C10TCR: u32,
    pub C10BNDTR: u32,
    pub C10SAR: u32,
    pub C10DAR: u32,
    pub C10BRUR: u32,
    pub C10LAR: u32,
    pub C10TBR: u32,
    pub C10MAR: u32,
    pub C10MDR: u32,
    pub C11ISR: u32,
    pub C11IFCR: u32,
    pub C11ESR: u32,
    pub C11CR: u32,
    pub C11TCR: u32,
    pub C11BNDTR: u32,
    pub C11SAR: u32,
    pub C11DAR: u32,
    pub C11BRUR: u32,
    pub C11LAR: u32,
    pub C11TBR: u32,
    pub C11MAR: u32,
    pub C11MDR: u32,
    pub C12ISR: u32,
    pub C12IFCR: u32,
    pub C12ESR: u32,
    pub C12CR: u32,
    pub C12TCR: u32,
    pub C12BNDTR: u32,
    pub C12SAR: u32,
    pub C12DAR: u32,
    pub C12BRUR: u32,
    pub C12LAR: u32,
    pub C12TBR: u32,
    pub C12MAR: u32,
    pub C12MDR: u32,
    pub C13ISR: u32,
    pub C13IFCR: u32,
    pub C13ESR: u32,
    pub C13CR: u32,
    pub C13TCR: u32,
    pub C13BNDTR: u32,
    pub C13SAR: u32,
    pub C13DAR: u32,
    pub C13BRUR: u32,
    pub C13LAR: u32,
    pub C13TBR: u32,
    pub C13MAR: u32,
    pub C13MDR: u32,
    pub C14ISR: u32,
    pub C14IFCR: u32,
    pub C14ESR: u32,
    pub C14CR: u32,
    pub C14TCR: u32,
    pub C14BNDTR: u32,
    pub C14SAR: u32,
    pub C14DAR: u32,
    pub C14BRUR: u32,
    pub C14LAR: u32,
    pub C14TBR: u32,
    pub C14MAR: u32,
    pub C14MDR: u32,
    pub C15ISR: u32,
    pub C15IFCR: u32,
    pub C15ESR: u32,
    pub C15CR: u32,
    pub C15TCR: u32,
    pub C15BNDTR: u32,
    pub C15SAR: u32,
    pub C15DAR: u32,
    pub C15BRUR: u32,
    pub C15LAR: u32,
    pub C15TBR: u32,
    pub C15MAR: u32,
    pub C15MDR: u32,
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
