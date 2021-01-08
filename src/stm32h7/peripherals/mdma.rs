#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MDMA
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v, stm32h7b3

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
pub mod ISR0 {

    /// Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register.
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

    /// Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0.
    pub mod CTCIF {
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
    pub mod BRTIF {
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
    pub mod BTIF {
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
    pub mod TCIF {
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
    pub mod CRQA {
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
pub mod IFCR0 {

    /// Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register
    pub mod CTEIF {
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
    pub mod CCTCIF {
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
    pub mod CBRTIF {
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
    pub mod CBTIF {
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
    pub mod CLTCIF {
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
pub mod ESR0 {

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
pub mod CR0 {

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
pub mod TCR0 {

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
pub mod BNDTR0 {

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
pub mod SAR0 {

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
pub mod DAR0 {

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
pub mod BRUR0 {

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
pub mod LAR0 {

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
pub mod TBR0 {

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
pub mod MAR0 {

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
pub mod MDR0 {

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
pub mod ISR1 {
    pub use super::ISR0::BRTIF;
    pub use super::ISR0::BTIF;
    pub use super::ISR0::CRQA;
    pub use super::ISR0::CTCIF;
    pub use super::ISR0::TCIF;
    pub use super::ISR0::TEIF;
}

/// MDMA channel x interrupt flag clear register
pub mod IFCR1 {
    pub use super::IFCR0::CBRTIF;
    pub use super::IFCR0::CBTIF;
    pub use super::IFCR0::CCTCIF;
    pub use super::IFCR0::CLTCIF;
    pub use super::IFCR0::CTEIF;
}

/// MDMA Channel x error status register
pub mod ESR1 {
    pub use super::ESR0::ASE;
    pub use super::ESR0::BSE;
    pub use super::ESR0::TEA;
    pub use super::ESR0::TED;
    pub use super::ESR0::TELD;
    pub use super::ESR0::TEMD;
}

/// This register is used to control the concerned channel.
pub mod CR1 {
    pub use super::CR0::BEX;
    pub use super::CR0::BRTIE;
    pub use super::CR0::BTIE;
    pub use super::CR0::CTCIE;
    pub use super::CR0::EN;
    pub use super::CR0::HEX;
    pub use super::CR0::PL;
    pub use super::CR0::SWRQ;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
    pub use super::CR0::WEX;
}

/// This register is used to configure the concerned channel.
pub mod TCR1 {
    pub use super::TCR0::BWM;
    pub use super::TCR0::DBURST;
    pub use super::TCR0::DINC;
    pub use super::TCR0::DINCOS;
    pub use super::TCR0::DSIZE;
    pub use super::TCR0::PAM;
    pub use super::TCR0::PKE;
    pub use super::TCR0::SBURST;
    pub use super::TCR0::SINC;
    pub use super::TCR0::SINCOS;
    pub use super::TCR0::SSIZE;
    pub use super::TCR0::SWRM;
    pub use super::TCR0::TLEN;
    pub use super::TCR0::TRGM;
}

/// MDMA Channel x block number of data register
pub mod BNDTR1 {
    pub use super::BNDTR0::BNDT;
    pub use super::BNDTR0::BRC;
    pub use super::BNDTR0::BRDUM;
    pub use super::BNDTR0::BRSUM;
}

/// MDMA channel x source address register
pub mod SAR1 {
    pub use super::SAR0::SAR;
}

/// MDMA channel x destination address register
pub mod DAR1 {
    pub use super::DAR0::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod BRUR1 {
    pub use super::BRUR0::DUV;
    pub use super::BRUR0::SUV;
}

/// MDMA channel x Link Address register
pub mod LAR1 {
    pub use super::LAR0::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod TBR1 {
    pub use super::TBR0::DBUS;
    pub use super::TBR0::SBUS;
    pub use super::TBR0::TSEL;
}

/// MDMA channel x Mask address register
pub mod MAR1 {
    pub use super::MAR0::MAR;
}

/// MDMA channel x Mask Data register
pub mod MDR1 {
    pub use super::MDR0::MDR;
}

/// MDMA channel x interrupt/status register
pub mod ISR2 {
    pub use super::ISR0::BRTIF;
    pub use super::ISR0::BTIF;
    pub use super::ISR0::CRQA;
    pub use super::ISR0::CTCIF;
    pub use super::ISR0::TCIF;
    pub use super::ISR0::TEIF;
}

/// MDMA channel x interrupt flag clear register
pub mod IFCR2 {
    pub use super::IFCR0::CBRTIF;
    pub use super::IFCR0::CBTIF;
    pub use super::IFCR0::CCTCIF;
    pub use super::IFCR0::CLTCIF;
    pub use super::IFCR0::CTEIF;
}

/// MDMA Channel x error status register
pub mod ESR2 {
    pub use super::ESR0::ASE;
    pub use super::ESR0::BSE;
    pub use super::ESR0::TEA;
    pub use super::ESR0::TED;
    pub use super::ESR0::TELD;
    pub use super::ESR0::TEMD;
}

/// This register is used to control the concerned channel.
pub mod CR2 {
    pub use super::CR0::BEX;
    pub use super::CR0::BRTIE;
    pub use super::CR0::BTIE;
    pub use super::CR0::CTCIE;
    pub use super::CR0::EN;
    pub use super::CR0::HEX;
    pub use super::CR0::PL;
    pub use super::CR0::SWRQ;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
    pub use super::CR0::WEX;
}

/// This register is used to configure the concerned channel.
pub mod TCR2 {
    pub use super::TCR0::BWM;
    pub use super::TCR0::DBURST;
    pub use super::TCR0::DINC;
    pub use super::TCR0::DINCOS;
    pub use super::TCR0::DSIZE;
    pub use super::TCR0::PAM;
    pub use super::TCR0::PKE;
    pub use super::TCR0::SBURST;
    pub use super::TCR0::SINC;
    pub use super::TCR0::SINCOS;
    pub use super::TCR0::SSIZE;
    pub use super::TCR0::SWRM;
    pub use super::TCR0::TLEN;
    pub use super::TCR0::TRGM;
}

/// MDMA Channel x block number of data register
pub mod BNDTR2 {
    pub use super::BNDTR0::BNDT;
    pub use super::BNDTR0::BRC;
    pub use super::BNDTR0::BRDUM;
    pub use super::BNDTR0::BRSUM;
}

/// MDMA channel x source address register
pub mod SAR2 {
    pub use super::SAR0::SAR;
}

/// MDMA channel x destination address register
pub mod DAR2 {
    pub use super::DAR0::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod BRUR2 {
    pub use super::BRUR0::DUV;
    pub use super::BRUR0::SUV;
}

/// MDMA channel x Link Address register
pub mod LAR2 {
    pub use super::LAR0::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod TBR2 {
    pub use super::TBR0::DBUS;
    pub use super::TBR0::SBUS;
    pub use super::TBR0::TSEL;
}

/// MDMA channel x Mask address register
pub mod MAR2 {
    pub use super::MAR0::MAR;
}

/// MDMA channel x Mask Data register
pub mod MDR2 {
    pub use super::MDR0::MDR;
}

/// MDMA channel x interrupt/status register
pub mod ISR3 {
    pub use super::ISR0::BRTIF;
    pub use super::ISR0::BTIF;
    pub use super::ISR0::CRQA;
    pub use super::ISR0::CTCIF;
    pub use super::ISR0::TCIF;
    pub use super::ISR0::TEIF;
}

/// MDMA channel x interrupt flag clear register
pub mod IFCR3 {
    pub use super::IFCR0::CBRTIF;
    pub use super::IFCR0::CBTIF;
    pub use super::IFCR0::CCTCIF;
    pub use super::IFCR0::CLTCIF;
    pub use super::IFCR0::CTEIF;
}

/// MDMA Channel x error status register
pub mod ESR3 {
    pub use super::ESR0::ASE;
    pub use super::ESR0::BSE;
    pub use super::ESR0::TEA;
    pub use super::ESR0::TED;
    pub use super::ESR0::TELD;
    pub use super::ESR0::TEMD;
}

/// This register is used to control the concerned channel.
pub mod CR3 {
    pub use super::CR0::BEX;
    pub use super::CR0::BRTIE;
    pub use super::CR0::BTIE;
    pub use super::CR0::CTCIE;
    pub use super::CR0::EN;
    pub use super::CR0::HEX;
    pub use super::CR0::PL;
    pub use super::CR0::SWRQ;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
    pub use super::CR0::WEX;
}

/// This register is used to configure the concerned channel.
pub mod TCR3 {
    pub use super::TCR0::BWM;
    pub use super::TCR0::DBURST;
    pub use super::TCR0::DINC;
    pub use super::TCR0::DINCOS;
    pub use super::TCR0::DSIZE;
    pub use super::TCR0::PAM;
    pub use super::TCR0::PKE;
    pub use super::TCR0::SBURST;
    pub use super::TCR0::SINC;
    pub use super::TCR0::SINCOS;
    pub use super::TCR0::SSIZE;
    pub use super::TCR0::SWRM;
    pub use super::TCR0::TLEN;
    pub use super::TCR0::TRGM;
}

/// MDMA Channel x block number of data register
pub mod BNDTR3 {
    pub use super::BNDTR0::BNDT;
    pub use super::BNDTR0::BRC;
    pub use super::BNDTR0::BRDUM;
    pub use super::BNDTR0::BRSUM;
}

/// MDMA channel x source address register
pub mod SAR3 {
    pub use super::SAR0::SAR;
}

/// MDMA channel x destination address register
pub mod DAR3 {
    pub use super::DAR0::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod BRUR3 {
    pub use super::BRUR0::DUV;
    pub use super::BRUR0::SUV;
}

/// MDMA channel x Link Address register
pub mod LAR3 {
    pub use super::LAR0::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod TBR3 {
    pub use super::TBR0::DBUS;
    pub use super::TBR0::SBUS;
    pub use super::TBR0::TSEL;
}

/// MDMA channel x Mask address register
pub mod MAR3 {
    pub use super::MAR0::MAR;
}

/// MDMA channel x Mask Data register
pub mod MDR3 {
    pub use super::MDR0::MDR;
}

/// MDMA channel x interrupt/status register
pub mod ISR4 {
    pub use super::ISR0::BRTIF;
    pub use super::ISR0::BTIF;
    pub use super::ISR0::CRQA;
    pub use super::ISR0::CTCIF;
    pub use super::ISR0::TCIF;
    pub use super::ISR0::TEIF;
}

/// MDMA channel x interrupt flag clear register
pub mod IFCR4 {
    pub use super::IFCR0::CBRTIF;
    pub use super::IFCR0::CBTIF;
    pub use super::IFCR0::CCTCIF;
    pub use super::IFCR0::CLTCIF;
    pub use super::IFCR0::CTEIF;
}

/// MDMA Channel x error status register
pub mod ESR4 {
    pub use super::ESR0::ASE;
    pub use super::ESR0::BSE;
    pub use super::ESR0::TEA;
    pub use super::ESR0::TED;
    pub use super::ESR0::TELD;
    pub use super::ESR0::TEMD;
}

/// This register is used to control the concerned channel.
pub mod CR4 {
    pub use super::CR0::BEX;
    pub use super::CR0::BRTIE;
    pub use super::CR0::BTIE;
    pub use super::CR0::CTCIE;
    pub use super::CR0::EN;
    pub use super::CR0::HEX;
    pub use super::CR0::PL;
    pub use super::CR0::SWRQ;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
    pub use super::CR0::WEX;
}

/// This register is used to configure the concerned channel.
pub mod TCR4 {
    pub use super::TCR0::BWM;
    pub use super::TCR0::DBURST;
    pub use super::TCR0::DINC;
    pub use super::TCR0::DINCOS;
    pub use super::TCR0::DSIZE;
    pub use super::TCR0::PAM;
    pub use super::TCR0::PKE;
    pub use super::TCR0::SBURST;
    pub use super::TCR0::SINC;
    pub use super::TCR0::SINCOS;
    pub use super::TCR0::SSIZE;
    pub use super::TCR0::SWRM;
    pub use super::TCR0::TLEN;
    pub use super::TCR0::TRGM;
}

/// MDMA Channel x block number of data register
pub mod BNDTR4 {
    pub use super::BNDTR0::BNDT;
    pub use super::BNDTR0::BRC;
    pub use super::BNDTR0::BRDUM;
    pub use super::BNDTR0::BRSUM;
}

/// MDMA channel x source address register
pub mod SAR4 {
    pub use super::SAR0::SAR;
}

/// MDMA channel x destination address register
pub mod DAR4 {
    pub use super::DAR0::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod BRUR4 {
    pub use super::BRUR0::DUV;
    pub use super::BRUR0::SUV;
}

/// MDMA channel x Link Address register
pub mod LAR4 {
    pub use super::LAR0::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod TBR4 {
    pub use super::TBR0::DBUS;
    pub use super::TBR0::SBUS;
    pub use super::TBR0::TSEL;
}

/// MDMA channel x Mask address register
pub mod MAR4 {
    pub use super::MAR0::MAR;
}

/// MDMA channel x Mask Data register
pub mod MDR4 {
    pub use super::MDR0::MDR;
}

/// MDMA channel x interrupt/status register
pub mod ISR5 {
    pub use super::ISR0::BRTIF;
    pub use super::ISR0::BTIF;
    pub use super::ISR0::CRQA;
    pub use super::ISR0::CTCIF;
    pub use super::ISR0::TCIF;
    pub use super::ISR0::TEIF;
}

/// MDMA channel x interrupt flag clear register
pub mod IFCR5 {
    pub use super::IFCR0::CBRTIF;
    pub use super::IFCR0::CBTIF;
    pub use super::IFCR0::CCTCIF;
    pub use super::IFCR0::CLTCIF;
    pub use super::IFCR0::CTEIF;
}

/// MDMA Channel x error status register
pub mod ESR5 {
    pub use super::ESR0::ASE;
    pub use super::ESR0::BSE;
    pub use super::ESR0::TEA;
    pub use super::ESR0::TED;
    pub use super::ESR0::TELD;
    pub use super::ESR0::TEMD;
}

/// This register is used to control the concerned channel.
pub mod CR5 {
    pub use super::CR0::BEX;
    pub use super::CR0::BRTIE;
    pub use super::CR0::BTIE;
    pub use super::CR0::CTCIE;
    pub use super::CR0::EN;
    pub use super::CR0::HEX;
    pub use super::CR0::PL;
    pub use super::CR0::SWRQ;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
    pub use super::CR0::WEX;
}

/// This register is used to configure the concerned channel.
pub mod TCR5 {
    pub use super::TCR0::BWM;
    pub use super::TCR0::DBURST;
    pub use super::TCR0::DINC;
    pub use super::TCR0::DINCOS;
    pub use super::TCR0::DSIZE;
    pub use super::TCR0::PAM;
    pub use super::TCR0::PKE;
    pub use super::TCR0::SBURST;
    pub use super::TCR0::SINC;
    pub use super::TCR0::SINCOS;
    pub use super::TCR0::SSIZE;
    pub use super::TCR0::SWRM;
    pub use super::TCR0::TLEN;
    pub use super::TCR0::TRGM;
}

/// MDMA Channel x block number of data register
pub mod BNDTR5 {
    pub use super::BNDTR0::BNDT;
    pub use super::BNDTR0::BRC;
    pub use super::BNDTR0::BRDUM;
    pub use super::BNDTR0::BRSUM;
}

/// MDMA channel x source address register
pub mod SAR5 {
    pub use super::SAR0::SAR;
}

/// MDMA channel x destination address register
pub mod DAR5 {
    pub use super::DAR0::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod BRUR5 {
    pub use super::BRUR0::DUV;
    pub use super::BRUR0::SUV;
}

/// MDMA channel x Link Address register
pub mod LAR5 {
    pub use super::LAR0::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod TBR5 {
    pub use super::TBR0::DBUS;
    pub use super::TBR0::SBUS;
    pub use super::TBR0::TSEL;
}

/// MDMA channel x Mask address register
pub mod MAR5 {
    pub use super::MAR0::MAR;
}

/// MDMA channel x Mask Data register
pub mod MDR5 {
    pub use super::MDR0::MDR;
}

/// MDMA channel x interrupt/status register
pub mod ISR6 {
    pub use super::ISR0::BRTIF;
    pub use super::ISR0::BTIF;
    pub use super::ISR0::CRQA;
    pub use super::ISR0::CTCIF;
    pub use super::ISR0::TCIF;
    pub use super::ISR0::TEIF;
}

/// MDMA channel x interrupt flag clear register
pub mod IFCR6 {
    pub use super::IFCR0::CBRTIF;
    pub use super::IFCR0::CBTIF;
    pub use super::IFCR0::CCTCIF;
    pub use super::IFCR0::CLTCIF;
    pub use super::IFCR0::CTEIF;
}

/// MDMA Channel x error status register
pub mod ESR6 {
    pub use super::ESR0::ASE;
    pub use super::ESR0::BSE;
    pub use super::ESR0::TEA;
    pub use super::ESR0::TED;
    pub use super::ESR0::TELD;
    pub use super::ESR0::TEMD;
}

/// This register is used to control the concerned channel.
pub mod CR6 {
    pub use super::CR0::BEX;
    pub use super::CR0::BRTIE;
    pub use super::CR0::BTIE;
    pub use super::CR0::CTCIE;
    pub use super::CR0::EN;
    pub use super::CR0::HEX;
    pub use super::CR0::PL;
    pub use super::CR0::SWRQ;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
    pub use super::CR0::WEX;
}

/// This register is used to configure the concerned channel.
pub mod TCR6 {
    pub use super::TCR0::BWM;
    pub use super::TCR0::DBURST;
    pub use super::TCR0::DINC;
    pub use super::TCR0::DINCOS;
    pub use super::TCR0::DSIZE;
    pub use super::TCR0::PAM;
    pub use super::TCR0::PKE;
    pub use super::TCR0::SBURST;
    pub use super::TCR0::SINC;
    pub use super::TCR0::SINCOS;
    pub use super::TCR0::SSIZE;
    pub use super::TCR0::SWRM;
    pub use super::TCR0::TLEN;
    pub use super::TCR0::TRGM;
}

/// MDMA Channel x block number of data register
pub mod BNDTR6 {
    pub use super::BNDTR0::BNDT;
    pub use super::BNDTR0::BRC;
    pub use super::BNDTR0::BRDUM;
    pub use super::BNDTR0::BRSUM;
}

/// MDMA channel x source address register
pub mod SAR6 {
    pub use super::SAR0::SAR;
}

/// MDMA channel x destination address register
pub mod DAR6 {
    pub use super::DAR0::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod BRUR6 {
    pub use super::BRUR0::DUV;
    pub use super::BRUR0::SUV;
}

/// MDMA channel x Link Address register
pub mod LAR6 {
    pub use super::LAR0::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod TBR6 {
    pub use super::TBR0::DBUS;
    pub use super::TBR0::SBUS;
    pub use super::TBR0::TSEL;
}

/// MDMA channel x Mask address register
pub mod MAR6 {
    pub use super::MAR0::MAR;
}

/// MDMA channel x Mask Data register
pub mod MDR6 {
    pub use super::MDR0::MDR;
}

/// MDMA channel x interrupt/status register
pub mod ISR7 {
    pub use super::ISR0::BRTIF;
    pub use super::ISR0::BTIF;
    pub use super::ISR0::CRQA;
    pub use super::ISR0::CTCIF;
    pub use super::ISR0::TCIF;
    pub use super::ISR0::TEIF;
}

/// MDMA channel x interrupt flag clear register
pub mod IFCR7 {
    pub use super::IFCR0::CBRTIF;
    pub use super::IFCR0::CBTIF;
    pub use super::IFCR0::CCTCIF;
    pub use super::IFCR0::CLTCIF;
    pub use super::IFCR0::CTEIF;
}

/// MDMA Channel x error status register
pub mod ESR7 {
    pub use super::ESR0::ASE;
    pub use super::ESR0::BSE;
    pub use super::ESR0::TEA;
    pub use super::ESR0::TED;
    pub use super::ESR0::TELD;
    pub use super::ESR0::TEMD;
}

/// This register is used to control the concerned channel.
pub mod CR7 {
    pub use super::CR0::BEX;
    pub use super::CR0::BRTIE;
    pub use super::CR0::BTIE;
    pub use super::CR0::CTCIE;
    pub use super::CR0::EN;
    pub use super::CR0::HEX;
    pub use super::CR0::PL;
    pub use super::CR0::SWRQ;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
    pub use super::CR0::WEX;
}

/// This register is used to configure the concerned channel.
pub mod TCR7 {
    pub use super::TCR0::BWM;
    pub use super::TCR0::DBURST;
    pub use super::TCR0::DINC;
    pub use super::TCR0::DINCOS;
    pub use super::TCR0::DSIZE;
    pub use super::TCR0::PAM;
    pub use super::TCR0::PKE;
    pub use super::TCR0::SBURST;
    pub use super::TCR0::SINC;
    pub use super::TCR0::SINCOS;
    pub use super::TCR0::SSIZE;
    pub use super::TCR0::SWRM;
    pub use super::TCR0::TLEN;
    pub use super::TCR0::TRGM;
}

/// MDMA Channel x block number of data register
pub mod BNDTR7 {
    pub use super::BNDTR0::BNDT;
    pub use super::BNDTR0::BRC;
    pub use super::BNDTR0::BRDUM;
    pub use super::BNDTR0::BRSUM;
}

/// MDMA channel x source address register
pub mod SAR7 {
    pub use super::SAR0::SAR;
}

/// MDMA channel x destination address register
pub mod DAR7 {
    pub use super::DAR0::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod BRUR7 {
    pub use super::BRUR0::DUV;
    pub use super::BRUR0::SUV;
}

/// MDMA channel x Link Address register
pub mod LAR7 {
    pub use super::LAR0::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod TBR7 {
    pub use super::TBR0::DBUS;
    pub use super::TBR0::SBUS;
    pub use super::TBR0::TSEL;
}

/// MDMA channel x Mask address register
pub mod MAR7 {
    pub use super::MAR0::MAR;
}

/// MDMA channel x Mask Data register
pub mod MDR7 {
    pub use super::MDR0::MDR;
}

/// MDMA channel x interrupt/status register
pub mod ISR8 {
    pub use super::ISR0::BRTIF;
    pub use super::ISR0::BTIF;
    pub use super::ISR0::CRQA;
    pub use super::ISR0::CTCIF;
    pub use super::ISR0::TCIF;
    pub use super::ISR0::TEIF;
}

/// MDMA channel x interrupt flag clear register
pub mod IFCR8 {
    pub use super::IFCR0::CBRTIF;
    pub use super::IFCR0::CBTIF;
    pub use super::IFCR0::CCTCIF;
    pub use super::IFCR0::CLTCIF;
    pub use super::IFCR0::CTEIF;
}

/// MDMA Channel x error status register
pub mod ESR8 {
    pub use super::ESR0::ASE;
    pub use super::ESR0::BSE;
    pub use super::ESR0::TEA;
    pub use super::ESR0::TED;
    pub use super::ESR0::TELD;
    pub use super::ESR0::TEMD;
}

/// This register is used to control the concerned channel.
pub mod CR8 {
    pub use super::CR0::BEX;
    pub use super::CR0::BRTIE;
    pub use super::CR0::BTIE;
    pub use super::CR0::CTCIE;
    pub use super::CR0::EN;
    pub use super::CR0::HEX;
    pub use super::CR0::PL;
    pub use super::CR0::SWRQ;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
    pub use super::CR0::WEX;
}

/// This register is used to configure the concerned channel.
pub mod TCR8 {
    pub use super::TCR0::BWM;
    pub use super::TCR0::DBURST;
    pub use super::TCR0::DINC;
    pub use super::TCR0::DINCOS;
    pub use super::TCR0::DSIZE;
    pub use super::TCR0::PAM;
    pub use super::TCR0::PKE;
    pub use super::TCR0::SBURST;
    pub use super::TCR0::SINC;
    pub use super::TCR0::SINCOS;
    pub use super::TCR0::SSIZE;
    pub use super::TCR0::SWRM;
    pub use super::TCR0::TLEN;
    pub use super::TCR0::TRGM;
}

/// MDMA Channel x block number of data register
pub mod BNDTR8 {
    pub use super::BNDTR0::BNDT;
    pub use super::BNDTR0::BRC;
    pub use super::BNDTR0::BRDUM;
    pub use super::BNDTR0::BRSUM;
}

/// MDMA channel x source address register
pub mod SAR8 {
    pub use super::SAR0::SAR;
}

/// MDMA channel x destination address register
pub mod DAR8 {
    pub use super::DAR0::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod BRUR8 {
    pub use super::BRUR0::DUV;
    pub use super::BRUR0::SUV;
}

/// MDMA channel x Link Address register
pub mod LAR8 {
    pub use super::LAR0::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod TBR8 {
    pub use super::TBR0::DBUS;
    pub use super::TBR0::SBUS;
    pub use super::TBR0::TSEL;
}

/// MDMA channel x Mask address register
pub mod MAR8 {
    pub use super::MAR0::MAR;
}

/// MDMA channel x Mask Data register
pub mod MDR8 {
    pub use super::MDR0::MDR;
}

/// MDMA channel x interrupt/status register
pub mod ISR9 {
    pub use super::ISR0::BRTIF;
    pub use super::ISR0::BTIF;
    pub use super::ISR0::CRQA;
    pub use super::ISR0::CTCIF;
    pub use super::ISR0::TCIF;
    pub use super::ISR0::TEIF;
}

/// MDMA channel x interrupt flag clear register
pub mod IFCR9 {
    pub use super::IFCR0::CBRTIF;
    pub use super::IFCR0::CBTIF;
    pub use super::IFCR0::CCTCIF;
    pub use super::IFCR0::CLTCIF;
    pub use super::IFCR0::CTEIF;
}

/// MDMA Channel x error status register
pub mod ESR9 {
    pub use super::ESR0::ASE;
    pub use super::ESR0::BSE;
    pub use super::ESR0::TEA;
    pub use super::ESR0::TED;
    pub use super::ESR0::TELD;
    pub use super::ESR0::TEMD;
}

/// This register is used to control the concerned channel.
pub mod CR9 {
    pub use super::CR0::BEX;
    pub use super::CR0::BRTIE;
    pub use super::CR0::BTIE;
    pub use super::CR0::CTCIE;
    pub use super::CR0::EN;
    pub use super::CR0::HEX;
    pub use super::CR0::PL;
    pub use super::CR0::SWRQ;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
    pub use super::CR0::WEX;
}

/// This register is used to configure the concerned channel.
pub mod TCR9 {
    pub use super::TCR0::BWM;
    pub use super::TCR0::DBURST;
    pub use super::TCR0::DINC;
    pub use super::TCR0::DINCOS;
    pub use super::TCR0::DSIZE;
    pub use super::TCR0::PAM;
    pub use super::TCR0::PKE;
    pub use super::TCR0::SBURST;
    pub use super::TCR0::SINC;
    pub use super::TCR0::SINCOS;
    pub use super::TCR0::SSIZE;
    pub use super::TCR0::SWRM;
    pub use super::TCR0::TLEN;
    pub use super::TCR0::TRGM;
}

/// MDMA Channel x block number of data register
pub mod BNDTR9 {
    pub use super::BNDTR0::BNDT;
    pub use super::BNDTR0::BRC;
    pub use super::BNDTR0::BRDUM;
    pub use super::BNDTR0::BRSUM;
}

/// MDMA channel x source address register
pub mod SAR9 {
    pub use super::SAR0::SAR;
}

/// MDMA channel x destination address register
pub mod DAR9 {
    pub use super::DAR0::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod BRUR9 {
    pub use super::BRUR0::DUV;
    pub use super::BRUR0::SUV;
}

/// MDMA channel x Link Address register
pub mod LAR9 {
    pub use super::LAR0::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod TBR9 {
    pub use super::TBR0::DBUS;
    pub use super::TBR0::SBUS;
    pub use super::TBR0::TSEL;
}

/// MDMA channel x Mask address register
pub mod MAR9 {
    pub use super::MAR0::MAR;
}

/// MDMA channel x Mask Data register
pub mod MDR9 {
    pub use super::MDR0::MDR;
}

/// MDMA channel x interrupt/status register
pub mod ISR10 {
    pub use super::ISR0::BRTIF;
    pub use super::ISR0::BTIF;
    pub use super::ISR0::CRQA;
    pub use super::ISR0::CTCIF;
    pub use super::ISR0::TCIF;
    pub use super::ISR0::TEIF;
}

/// MDMA channel x interrupt flag clear register
pub mod IFCR10 {
    pub use super::IFCR0::CBRTIF;
    pub use super::IFCR0::CBTIF;
    pub use super::IFCR0::CCTCIF;
    pub use super::IFCR0::CLTCIF;
    pub use super::IFCR0::CTEIF;
}

/// MDMA Channel x error status register
pub mod ESR10 {
    pub use super::ESR0::ASE;
    pub use super::ESR0::BSE;
    pub use super::ESR0::TEA;
    pub use super::ESR0::TED;
    pub use super::ESR0::TELD;
    pub use super::ESR0::TEMD;
}

/// This register is used to control the concerned channel.
pub mod CR10 {
    pub use super::CR0::BEX;
    pub use super::CR0::BRTIE;
    pub use super::CR0::BTIE;
    pub use super::CR0::CTCIE;
    pub use super::CR0::EN;
    pub use super::CR0::HEX;
    pub use super::CR0::PL;
    pub use super::CR0::SWRQ;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
    pub use super::CR0::WEX;
}

/// This register is used to configure the concerned channel.
pub mod TCR10 {
    pub use super::TCR0::BWM;
    pub use super::TCR0::DBURST;
    pub use super::TCR0::DINC;
    pub use super::TCR0::DINCOS;
    pub use super::TCR0::DSIZE;
    pub use super::TCR0::PAM;
    pub use super::TCR0::PKE;
    pub use super::TCR0::SBURST;
    pub use super::TCR0::SINC;
    pub use super::TCR0::SINCOS;
    pub use super::TCR0::SSIZE;
    pub use super::TCR0::SWRM;
    pub use super::TCR0::TLEN;
    pub use super::TCR0::TRGM;
}

/// MDMA Channel x block number of data register
pub mod BNDTR10 {
    pub use super::BNDTR0::BNDT;
    pub use super::BNDTR0::BRC;
    pub use super::BNDTR0::BRDUM;
    pub use super::BNDTR0::BRSUM;
}

/// MDMA channel x source address register
pub mod SAR10 {
    pub use super::SAR0::SAR;
}

/// MDMA channel x destination address register
pub mod DAR10 {
    pub use super::DAR0::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod BRUR10 {
    pub use super::BRUR0::DUV;
    pub use super::BRUR0::SUV;
}

/// MDMA channel x Link Address register
pub mod LAR10 {
    pub use super::LAR0::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod TBR10 {
    pub use super::TBR0::DBUS;
    pub use super::TBR0::SBUS;
    pub use super::TBR0::TSEL;
}

/// MDMA channel x Mask address register
pub mod MAR10 {
    pub use super::MAR0::MAR;
}

/// MDMA channel x Mask Data register
pub mod MDR10 {
    pub use super::MDR0::MDR;
}

/// MDMA channel x interrupt/status register
pub mod ISR11 {
    pub use super::ISR0::BRTIF;
    pub use super::ISR0::BTIF;
    pub use super::ISR0::CRQA;
    pub use super::ISR0::CTCIF;
    pub use super::ISR0::TCIF;
    pub use super::ISR0::TEIF;
}

/// MDMA channel x interrupt flag clear register
pub mod IFCR11 {
    pub use super::IFCR0::CBRTIF;
    pub use super::IFCR0::CBTIF;
    pub use super::IFCR0::CCTCIF;
    pub use super::IFCR0::CLTCIF;
    pub use super::IFCR0::CTEIF;
}

/// MDMA Channel x error status register
pub mod ESR11 {
    pub use super::ESR0::ASE;
    pub use super::ESR0::BSE;
    pub use super::ESR0::TEA;
    pub use super::ESR0::TED;
    pub use super::ESR0::TELD;
    pub use super::ESR0::TEMD;
}

/// This register is used to control the concerned channel.
pub mod CR11 {
    pub use super::CR0::BEX;
    pub use super::CR0::BRTIE;
    pub use super::CR0::BTIE;
    pub use super::CR0::CTCIE;
    pub use super::CR0::EN;
    pub use super::CR0::HEX;
    pub use super::CR0::PL;
    pub use super::CR0::SWRQ;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
    pub use super::CR0::WEX;
}

/// This register is used to configure the concerned channel.
pub mod TCR11 {
    pub use super::TCR0::BWM;
    pub use super::TCR0::DBURST;
    pub use super::TCR0::DINC;
    pub use super::TCR0::DINCOS;
    pub use super::TCR0::DSIZE;
    pub use super::TCR0::PAM;
    pub use super::TCR0::PKE;
    pub use super::TCR0::SBURST;
    pub use super::TCR0::SINC;
    pub use super::TCR0::SINCOS;
    pub use super::TCR0::SSIZE;
    pub use super::TCR0::SWRM;
    pub use super::TCR0::TLEN;
    pub use super::TCR0::TRGM;
}

/// MDMA Channel x block number of data register
pub mod BNDTR11 {
    pub use super::BNDTR0::BNDT;
    pub use super::BNDTR0::BRC;
    pub use super::BNDTR0::BRDUM;
    pub use super::BNDTR0::BRSUM;
}

/// MDMA channel x source address register
pub mod SAR11 {
    pub use super::SAR0::SAR;
}

/// MDMA channel x destination address register
pub mod DAR11 {
    pub use super::DAR0::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod BRUR11 {
    pub use super::BRUR0::DUV;
    pub use super::BRUR0::SUV;
}

/// MDMA channel x Link Address register
pub mod LAR11 {
    pub use super::LAR0::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod TBR11 {
    pub use super::TBR0::DBUS;
    pub use super::TBR0::SBUS;
    pub use super::TBR0::TSEL;
}

/// MDMA channel x Mask address register
pub mod MAR11 {
    pub use super::MAR0::MAR;
}

/// MDMA channel x Mask Data register
pub mod MDR11 {
    pub use super::MDR0::MDR;
}

/// MDMA channel x interrupt/status register
pub mod ISR12 {
    pub use super::ISR0::BRTIF;
    pub use super::ISR0::BTIF;
    pub use super::ISR0::CRQA;
    pub use super::ISR0::CTCIF;
    pub use super::ISR0::TCIF;
    pub use super::ISR0::TEIF;
}

/// MDMA channel x interrupt flag clear register
pub mod IFCR12 {
    pub use super::IFCR0::CBRTIF;
    pub use super::IFCR0::CBTIF;
    pub use super::IFCR0::CCTCIF;
    pub use super::IFCR0::CLTCIF;
    pub use super::IFCR0::CTEIF;
}

/// MDMA Channel x error status register
pub mod ESR12 {
    pub use super::ESR0::ASE;
    pub use super::ESR0::BSE;
    pub use super::ESR0::TEA;
    pub use super::ESR0::TED;
    pub use super::ESR0::TELD;
    pub use super::ESR0::TEMD;
}

/// This register is used to control the concerned channel.
pub mod CR12 {
    pub use super::CR0::BEX;
    pub use super::CR0::BRTIE;
    pub use super::CR0::BTIE;
    pub use super::CR0::CTCIE;
    pub use super::CR0::EN;
    pub use super::CR0::HEX;
    pub use super::CR0::PL;
    pub use super::CR0::SWRQ;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
    pub use super::CR0::WEX;
}

/// This register is used to configure the concerned channel.
pub mod TCR12 {
    pub use super::TCR0::BWM;
    pub use super::TCR0::DBURST;
    pub use super::TCR0::DINC;
    pub use super::TCR0::DINCOS;
    pub use super::TCR0::DSIZE;
    pub use super::TCR0::PAM;
    pub use super::TCR0::PKE;
    pub use super::TCR0::SBURST;
    pub use super::TCR0::SINC;
    pub use super::TCR0::SINCOS;
    pub use super::TCR0::SSIZE;
    pub use super::TCR0::SWRM;
    pub use super::TCR0::TLEN;
    pub use super::TCR0::TRGM;
}

/// MDMA Channel x block number of data register
pub mod BNDTR12 {
    pub use super::BNDTR0::BNDT;
    pub use super::BNDTR0::BRC;
    pub use super::BNDTR0::BRDUM;
    pub use super::BNDTR0::BRSUM;
}

/// MDMA channel x source address register
pub mod SAR12 {
    pub use super::SAR0::SAR;
}

/// MDMA channel x destination address register
pub mod DAR12 {
    pub use super::DAR0::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod BRUR12 {
    pub use super::BRUR0::DUV;
    pub use super::BRUR0::SUV;
}

/// MDMA channel x Link Address register
pub mod LAR12 {
    pub use super::LAR0::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod TBR12 {
    pub use super::TBR0::DBUS;
    pub use super::TBR0::SBUS;
    pub use super::TBR0::TSEL;
}

/// MDMA channel x Mask address register
pub mod MAR12 {
    pub use super::MAR0::MAR;
}

/// MDMA channel x Mask Data register
pub mod MDR12 {
    pub use super::MDR0::MDR;
}

/// MDMA channel x interrupt/status register
pub mod ISR13 {
    pub use super::ISR0::BRTIF;
    pub use super::ISR0::BTIF;
    pub use super::ISR0::CRQA;
    pub use super::ISR0::CTCIF;
    pub use super::ISR0::TCIF;
    pub use super::ISR0::TEIF;
}

/// MDMA channel x interrupt flag clear register
pub mod IFCR13 {
    pub use super::IFCR0::CBRTIF;
    pub use super::IFCR0::CBTIF;
    pub use super::IFCR0::CCTCIF;
    pub use super::IFCR0::CLTCIF;
    pub use super::IFCR0::CTEIF;
}

/// MDMA Channel x error status register
pub mod ESR13 {
    pub use super::ESR0::ASE;
    pub use super::ESR0::BSE;
    pub use super::ESR0::TEA;
    pub use super::ESR0::TED;
    pub use super::ESR0::TELD;
    pub use super::ESR0::TEMD;
}

/// This register is used to control the concerned channel.
pub mod CR13 {
    pub use super::CR0::BEX;
    pub use super::CR0::BRTIE;
    pub use super::CR0::BTIE;
    pub use super::CR0::CTCIE;
    pub use super::CR0::EN;
    pub use super::CR0::HEX;
    pub use super::CR0::PL;
    pub use super::CR0::SWRQ;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
    pub use super::CR0::WEX;
}

/// This register is used to configure the concerned channel.
pub mod TCR13 {
    pub use super::TCR0::BWM;
    pub use super::TCR0::DBURST;
    pub use super::TCR0::DINC;
    pub use super::TCR0::DINCOS;
    pub use super::TCR0::DSIZE;
    pub use super::TCR0::PAM;
    pub use super::TCR0::PKE;
    pub use super::TCR0::SBURST;
    pub use super::TCR0::SINC;
    pub use super::TCR0::SINCOS;
    pub use super::TCR0::SSIZE;
    pub use super::TCR0::SWRM;
    pub use super::TCR0::TLEN;
    pub use super::TCR0::TRGM;
}

/// MDMA Channel x block number of data register
pub mod BNDTR13 {
    pub use super::BNDTR0::BNDT;
    pub use super::BNDTR0::BRC;
    pub use super::BNDTR0::BRDUM;
    pub use super::BNDTR0::BRSUM;
}

/// MDMA channel x source address register
pub mod SAR13 {
    pub use super::SAR0::SAR;
}

/// MDMA channel x destination address register
pub mod DAR13 {
    pub use super::DAR0::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod BRUR13 {
    pub use super::BRUR0::DUV;
    pub use super::BRUR0::SUV;
}

/// MDMA channel x Link Address register
pub mod LAR13 {
    pub use super::LAR0::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod TBR13 {
    pub use super::TBR0::DBUS;
    pub use super::TBR0::SBUS;
    pub use super::TBR0::TSEL;
}

/// MDMA channel x Mask address register
pub mod MAR13 {
    pub use super::MAR0::MAR;
}

/// MDMA channel x Mask Data register
pub mod MDR13 {
    pub use super::MDR0::MDR;
}

/// MDMA channel x interrupt/status register
pub mod ISR14 {
    pub use super::ISR0::BRTIF;
    pub use super::ISR0::BTIF;
    pub use super::ISR0::CRQA;
    pub use super::ISR0::CTCIF;
    pub use super::ISR0::TCIF;
    pub use super::ISR0::TEIF;
}

/// MDMA channel x interrupt flag clear register
pub mod IFCR14 {
    pub use super::IFCR0::CBRTIF;
    pub use super::IFCR0::CBTIF;
    pub use super::IFCR0::CCTCIF;
    pub use super::IFCR0::CLTCIF;
    pub use super::IFCR0::CTEIF;
}

/// MDMA Channel x error status register
pub mod ESR14 {
    pub use super::ESR0::ASE;
    pub use super::ESR0::BSE;
    pub use super::ESR0::TEA;
    pub use super::ESR0::TED;
    pub use super::ESR0::TELD;
    pub use super::ESR0::TEMD;
}

/// This register is used to control the concerned channel.
pub mod CR14 {
    pub use super::CR0::BEX;
    pub use super::CR0::BRTIE;
    pub use super::CR0::BTIE;
    pub use super::CR0::CTCIE;
    pub use super::CR0::EN;
    pub use super::CR0::HEX;
    pub use super::CR0::PL;
    pub use super::CR0::SWRQ;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
    pub use super::CR0::WEX;
}

/// This register is used to configure the concerned channel.
pub mod TCR14 {
    pub use super::TCR0::BWM;
    pub use super::TCR0::DBURST;
    pub use super::TCR0::DINC;
    pub use super::TCR0::DINCOS;
    pub use super::TCR0::DSIZE;
    pub use super::TCR0::PAM;
    pub use super::TCR0::PKE;
    pub use super::TCR0::SBURST;
    pub use super::TCR0::SINC;
    pub use super::TCR0::SINCOS;
    pub use super::TCR0::SSIZE;
    pub use super::TCR0::SWRM;
    pub use super::TCR0::TLEN;
    pub use super::TCR0::TRGM;
}

/// MDMA Channel x block number of data register
pub mod BNDTR14 {
    pub use super::BNDTR0::BNDT;
    pub use super::BNDTR0::BRC;
    pub use super::BNDTR0::BRDUM;
    pub use super::BNDTR0::BRSUM;
}

/// MDMA channel x source address register
pub mod SAR14 {
    pub use super::SAR0::SAR;
}

/// MDMA channel x destination address register
pub mod DAR14 {
    pub use super::DAR0::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod BRUR14 {
    pub use super::BRUR0::DUV;
    pub use super::BRUR0::SUV;
}

/// MDMA channel x Link Address register
pub mod LAR14 {
    pub use super::LAR0::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod TBR14 {
    pub use super::TBR0::DBUS;
    pub use super::TBR0::SBUS;
    pub use super::TBR0::TSEL;
}

/// MDMA channel x Mask address register
pub mod MAR14 {
    pub use super::MAR0::MAR;
}

/// MDMA channel x Mask Data register
pub mod MDR14 {
    pub use super::MDR0::MDR;
}

/// MDMA channel x interrupt/status register
pub mod ISR15 {
    pub use super::ISR0::BRTIF;
    pub use super::ISR0::BTIF;
    pub use super::ISR0::CRQA;
    pub use super::ISR0::CTCIF;
    pub use super::ISR0::TCIF;
    pub use super::ISR0::TEIF;
}

/// MDMA channel x interrupt flag clear register
pub mod IFCR15 {
    pub use super::IFCR0::CBRTIF;
    pub use super::IFCR0::CBTIF;
    pub use super::IFCR0::CCTCIF;
    pub use super::IFCR0::CLTCIF;
    pub use super::IFCR0::CTEIF;
}

/// MDMA Channel x error status register
pub mod ESR15 {
    pub use super::ESR0::ASE;
    pub use super::ESR0::BSE;
    pub use super::ESR0::TEA;
    pub use super::ESR0::TED;
    pub use super::ESR0::TELD;
    pub use super::ESR0::TEMD;
}

/// This register is used to control the concerned channel.
pub mod CR15 {
    pub use super::CR0::BEX;
    pub use super::CR0::BRTIE;
    pub use super::CR0::BTIE;
    pub use super::CR0::CTCIE;
    pub use super::CR0::EN;
    pub use super::CR0::HEX;
    pub use super::CR0::PL;
    pub use super::CR0::SWRQ;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
    pub use super::CR0::WEX;
}

/// This register is used to configure the concerned channel.
pub mod TCR15 {
    pub use super::TCR0::BWM;
    pub use super::TCR0::DBURST;
    pub use super::TCR0::DINC;
    pub use super::TCR0::DINCOS;
    pub use super::TCR0::DSIZE;
    pub use super::TCR0::PAM;
    pub use super::TCR0::PKE;
    pub use super::TCR0::SBURST;
    pub use super::TCR0::SINC;
    pub use super::TCR0::SINCOS;
    pub use super::TCR0::SSIZE;
    pub use super::TCR0::SWRM;
    pub use super::TCR0::TLEN;
    pub use super::TCR0::TRGM;
}

/// MDMA Channel x block number of data register
pub mod BNDTR15 {
    pub use super::BNDTR0::BNDT;
    pub use super::BNDTR0::BRC;
    pub use super::BNDTR0::BRDUM;
    pub use super::BNDTR0::BRSUM;
}

/// MDMA channel x source address register
pub mod SAR15 {
    pub use super::SAR0::SAR;
}

/// MDMA channel x destination address register
pub mod DAR15 {
    pub use super::DAR0::DAR;
}

/// MDMA channel x Block Repeat address Update register
pub mod BRUR15 {
    pub use super::BRUR0::DUV;
    pub use super::BRUR0::SUV;
}

/// MDMA channel x Link Address register
pub mod LAR15 {
    pub use super::LAR0::LAR;
}

/// MDMA channel x Trigger and Bus selection Register
pub mod TBR15 {
    pub use super::TBR0::DBUS;
    pub use super::TBR0::SBUS;
    pub use super::TBR0::TSEL;
}

/// MDMA channel x Mask address register
pub mod MAR15 {
    pub use super::MAR0::MAR;
}

/// MDMA channel x Mask Data register
pub mod MDR15 {
    pub use super::MDR0::MDR;
}
#[repr(C)]
pub struct RegisterBlock {
    /// MDMA Global Interrupt/Status Register
    pub GISR0: RORegister<u32>,

    _reserved1: [u32; 15],

    /// MDMA channel x interrupt/status register
    pub ISR0: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR0: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR0: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR0: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR0: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR0: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR0: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR0: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR0: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR0: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR0: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR0: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR0: RWRegister<u32>,

    _reserved3: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub ISR1: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR1: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR1: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR1: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR1: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR1: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR1: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR1: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR1: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR1: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR1: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR1: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR1: RWRegister<u32>,

    _reserved5: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub ISR2: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR2: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR2: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR2: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR2: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR2: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR2: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR2: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR2: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR2: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR2: RWRegister<u32>,

    _reserved6: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR2: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR2: RWRegister<u32>,

    _reserved7: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub ISR3: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR3: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR3: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR3: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR3: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR3: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR3: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR3: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR3: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR3: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR3: RWRegister<u32>,

    _reserved8: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR3: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR3: RWRegister<u32>,

    _reserved9: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub ISR4: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR4: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR4: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR4: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR4: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR4: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR4: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR4: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR4: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR4: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR4: RWRegister<u32>,

    _reserved10: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR4: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR4: RWRegister<u32>,

    _reserved11: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub ISR5: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR5: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR5: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR5: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR5: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR5: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR5: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR5: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR5: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR5: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR5: RWRegister<u32>,

    _reserved12: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR5: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR5: RWRegister<u32>,

    _reserved13: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub ISR6: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR6: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR6: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR6: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR6: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR6: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR6: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR6: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR6: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR6: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR6: RWRegister<u32>,

    _reserved14: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR6: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR6: RWRegister<u32>,

    _reserved15: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub ISR7: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR7: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR7: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR7: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR7: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR7: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR7: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR7: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR7: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR7: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR7: RWRegister<u32>,

    _reserved16: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR7: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR7: RWRegister<u32>,

    _reserved17: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub ISR8: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR8: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR8: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR8: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR8: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR8: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR8: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR8: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR8: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR8: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR8: RWRegister<u32>,

    _reserved18: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR8: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR8: RWRegister<u32>,

    _reserved19: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub ISR9: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR9: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR9: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR9: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR9: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR9: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR9: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR9: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR9: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR9: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR9: RWRegister<u32>,

    _reserved20: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR9: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR9: RWRegister<u32>,

    _reserved21: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub ISR10: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR10: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR10: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR10: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR10: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR10: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR10: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR10: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR10: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR10: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR10: RWRegister<u32>,

    _reserved22: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR10: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR10: RWRegister<u32>,

    _reserved23: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub ISR11: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR11: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR11: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR11: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR11: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR11: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR11: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR11: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR11: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR11: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR11: RWRegister<u32>,

    _reserved24: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR11: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR11: RWRegister<u32>,

    _reserved25: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub ISR12: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR12: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR12: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR12: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR12: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR12: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR12: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR12: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR12: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR12: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR12: RWRegister<u32>,

    _reserved26: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR12: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR12: RWRegister<u32>,

    _reserved27: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub ISR13: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR13: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR13: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR13: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR13: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR13: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR13: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR13: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR13: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR13: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR13: RWRegister<u32>,

    _reserved28: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR13: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR13: RWRegister<u32>,

    _reserved29: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub ISR14: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR14: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR14: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR14: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR14: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR14: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR14: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR14: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR14: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR14: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR14: RWRegister<u32>,

    _reserved30: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR14: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR14: RWRegister<u32>,

    _reserved31: [u32; 2],

    /// MDMA channel x interrupt/status register
    pub ISR15: RORegister<u32>,

    /// MDMA channel x interrupt flag clear register
    pub IFCR15: WORegister<u32>,

    /// MDMA Channel x error status register
    pub ESR15: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub CR15: RWRegister<u32>,

    /// This register is used to configure the concerned channel.
    pub TCR15: RWRegister<u32>,

    /// MDMA Channel x block number of data register
    pub BNDTR15: RWRegister<u32>,

    /// MDMA channel x source address register
    pub SAR15: RWRegister<u32>,

    /// MDMA channel x destination address register
    pub DAR15: RWRegister<u32>,

    /// MDMA channel x Block Repeat address Update register
    pub BRUR15: RWRegister<u32>,

    /// MDMA channel x Link Address register
    pub LAR15: RWRegister<u32>,

    /// MDMA channel x Trigger and Bus selection Register
    pub TBR15: RWRegister<u32>,

    _reserved32: [u32; 1],

    /// MDMA channel x Mask address register
    pub MAR15: RWRegister<u32>,

    /// MDMA channel x Mask Data register
    pub MDR15: RWRegister<u32>,
}
pub struct ResetValues {
    pub GISR0: u32,
    pub ISR0: u32,
    pub IFCR0: u32,
    pub ESR0: u32,
    pub CR0: u32,
    pub TCR0: u32,
    pub BNDTR0: u32,
    pub SAR0: u32,
    pub DAR0: u32,
    pub BRUR0: u32,
    pub LAR0: u32,
    pub TBR0: u32,
    pub MAR0: u32,
    pub MDR0: u32,
    pub ISR1: u32,
    pub IFCR1: u32,
    pub ESR1: u32,
    pub CR1: u32,
    pub TCR1: u32,
    pub BNDTR1: u32,
    pub SAR1: u32,
    pub DAR1: u32,
    pub BRUR1: u32,
    pub LAR1: u32,
    pub TBR1: u32,
    pub MAR1: u32,
    pub MDR1: u32,
    pub ISR2: u32,
    pub IFCR2: u32,
    pub ESR2: u32,
    pub CR2: u32,
    pub TCR2: u32,
    pub BNDTR2: u32,
    pub SAR2: u32,
    pub DAR2: u32,
    pub BRUR2: u32,
    pub LAR2: u32,
    pub TBR2: u32,
    pub MAR2: u32,
    pub MDR2: u32,
    pub ISR3: u32,
    pub IFCR3: u32,
    pub ESR3: u32,
    pub CR3: u32,
    pub TCR3: u32,
    pub BNDTR3: u32,
    pub SAR3: u32,
    pub DAR3: u32,
    pub BRUR3: u32,
    pub LAR3: u32,
    pub TBR3: u32,
    pub MAR3: u32,
    pub MDR3: u32,
    pub ISR4: u32,
    pub IFCR4: u32,
    pub ESR4: u32,
    pub CR4: u32,
    pub TCR4: u32,
    pub BNDTR4: u32,
    pub SAR4: u32,
    pub DAR4: u32,
    pub BRUR4: u32,
    pub LAR4: u32,
    pub TBR4: u32,
    pub MAR4: u32,
    pub MDR4: u32,
    pub ISR5: u32,
    pub IFCR5: u32,
    pub ESR5: u32,
    pub CR5: u32,
    pub TCR5: u32,
    pub BNDTR5: u32,
    pub SAR5: u32,
    pub DAR5: u32,
    pub BRUR5: u32,
    pub LAR5: u32,
    pub TBR5: u32,
    pub MAR5: u32,
    pub MDR5: u32,
    pub ISR6: u32,
    pub IFCR6: u32,
    pub ESR6: u32,
    pub CR6: u32,
    pub TCR6: u32,
    pub BNDTR6: u32,
    pub SAR6: u32,
    pub DAR6: u32,
    pub BRUR6: u32,
    pub LAR6: u32,
    pub TBR6: u32,
    pub MAR6: u32,
    pub MDR6: u32,
    pub ISR7: u32,
    pub IFCR7: u32,
    pub ESR7: u32,
    pub CR7: u32,
    pub TCR7: u32,
    pub BNDTR7: u32,
    pub SAR7: u32,
    pub DAR7: u32,
    pub BRUR7: u32,
    pub LAR7: u32,
    pub TBR7: u32,
    pub MAR7: u32,
    pub MDR7: u32,
    pub ISR8: u32,
    pub IFCR8: u32,
    pub ESR8: u32,
    pub CR8: u32,
    pub TCR8: u32,
    pub BNDTR8: u32,
    pub SAR8: u32,
    pub DAR8: u32,
    pub BRUR8: u32,
    pub LAR8: u32,
    pub TBR8: u32,
    pub MAR8: u32,
    pub MDR8: u32,
    pub ISR9: u32,
    pub IFCR9: u32,
    pub ESR9: u32,
    pub CR9: u32,
    pub TCR9: u32,
    pub BNDTR9: u32,
    pub SAR9: u32,
    pub DAR9: u32,
    pub BRUR9: u32,
    pub LAR9: u32,
    pub TBR9: u32,
    pub MAR9: u32,
    pub MDR9: u32,
    pub ISR10: u32,
    pub IFCR10: u32,
    pub ESR10: u32,
    pub CR10: u32,
    pub TCR10: u32,
    pub BNDTR10: u32,
    pub SAR10: u32,
    pub DAR10: u32,
    pub BRUR10: u32,
    pub LAR10: u32,
    pub TBR10: u32,
    pub MAR10: u32,
    pub MDR10: u32,
    pub ISR11: u32,
    pub IFCR11: u32,
    pub ESR11: u32,
    pub CR11: u32,
    pub TCR11: u32,
    pub BNDTR11: u32,
    pub SAR11: u32,
    pub DAR11: u32,
    pub BRUR11: u32,
    pub LAR11: u32,
    pub TBR11: u32,
    pub MAR11: u32,
    pub MDR11: u32,
    pub ISR12: u32,
    pub IFCR12: u32,
    pub ESR12: u32,
    pub CR12: u32,
    pub TCR12: u32,
    pub BNDTR12: u32,
    pub SAR12: u32,
    pub DAR12: u32,
    pub BRUR12: u32,
    pub LAR12: u32,
    pub TBR12: u32,
    pub MAR12: u32,
    pub MDR12: u32,
    pub ISR13: u32,
    pub IFCR13: u32,
    pub ESR13: u32,
    pub CR13: u32,
    pub TCR13: u32,
    pub BNDTR13: u32,
    pub SAR13: u32,
    pub DAR13: u32,
    pub BRUR13: u32,
    pub LAR13: u32,
    pub TBR13: u32,
    pub MAR13: u32,
    pub MDR13: u32,
    pub ISR14: u32,
    pub IFCR14: u32,
    pub ESR14: u32,
    pub CR14: u32,
    pub TCR14: u32,
    pub BNDTR14: u32,
    pub SAR14: u32,
    pub DAR14: u32,
    pub BRUR14: u32,
    pub LAR14: u32,
    pub TBR14: u32,
    pub MAR14: u32,
    pub MDR14: u32,
    pub ISR15: u32,
    pub IFCR15: u32,
    pub ESR15: u32,
    pub CR15: u32,
    pub TCR15: u32,
    pub BNDTR15: u32,
    pub SAR15: u32,
    pub DAR15: u32,
    pub BRUR15: u32,
    pub LAR15: u32,
    pub TBR15: u32,
    pub MAR15: u32,
    pub MDR15: u32,
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
