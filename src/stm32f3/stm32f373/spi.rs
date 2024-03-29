#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Serial peripheral interface/Inter-IC sound

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register 1
pub mod CR1 {

    /// Bidirectional data mode enable
    pub mod BIDIMODE {
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

            /// 0b0: 2-line unidirectional data mode selected
            pub const Unidirectional: u32 = 0b0;

            /// 0b1: 1-line bidirectional data mode selected
            pub const Bidirectional: u32 = 0b1;
        }
    }

    /// Output enable in bidirectional mode
    pub mod BIDIOE {
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

            /// 0b0: Output disabled (receive-only mode)
            pub const OutputDisabled: u32 = 0b0;

            /// 0b1: Output enabled (transmit-only mode)
            pub const OutputEnabled: u32 = 0b1;
        }
    }

    /// Hardware CRC calculation enable
    pub mod CRCEN {
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

            /// 0b0: CRC calculation disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CRC calculation enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CRC transfer next
    pub mod CRCNEXT {
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

            /// 0b0: Next transmit value is from Tx buffer
            pub const TxBuffer: u32 = 0b0;

            /// 0b1: Next transmit value is from Tx CRC register
            pub const CRC: u32 = 0b1;
        }
    }

    /// CRC length
    pub mod CRCL {
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

            /// 0b0: 8-bit CRC length
            pub const EightBit: u32 = 0b0;

            /// 0b1: 16-bit CRC length
            pub const SixteenBit: u32 = 0b1;
        }
    }

    /// Receive only
    pub mod RXONLY {
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

            /// 0b0: Full duplex (Transmit and receive)
            pub const FullDuplex: u32 = 0b0;

            /// 0b1: Output disabled (Receive-only mode)
            pub const OutputDisabled: u32 = 0b1;
        }
    }

    /// Software slave management
    pub mod SSM {
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

            /// 0b0: Software slave management disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Software slave management enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Internal slave select
    pub mod SSI {
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

            /// 0b0: 0 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
            pub const SlaveSelected: u32 = 0b0;

            /// 0b1: 1 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
            pub const SlaveNotSelected: u32 = 0b1;
        }
    }

    /// Frame format
    pub mod LSBFIRST {
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

            /// 0b0: Data is transmitted/received with the MSB first
            pub const MSBFirst: u32 = 0b0;

            /// 0b1: Data is transmitted/received with the LSB first
            pub const LSBFirst: u32 = 0b1;
        }
    }

    /// SPI enable
    pub mod SPE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Peripheral disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Peripheral enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Baud rate control
    pub mod BR {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: f_PCLK / 2
            pub const Div2: u32 = 0b000;

            /// 0b001: f_PCLK / 4
            pub const Div4: u32 = 0b001;

            /// 0b010: f_PCLK / 8
            pub const Div8: u32 = 0b010;

            /// 0b011: f_PCLK / 16
            pub const Div16: u32 = 0b011;

            /// 0b100: f_PCLK / 32
            pub const Div32: u32 = 0b100;

            /// 0b101: f_PCLK / 64
            pub const Div64: u32 = 0b101;

            /// 0b110: f_PCLK / 128
            pub const Div128: u32 = 0b110;

            /// 0b111: f_PCLK / 256
            pub const Div256: u32 = 0b111;
        }
    }

    /// Master selection
    pub mod MSTR {
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

            /// 0b0: Slave configuration
            pub const Slave: u32 = 0b0;

            /// 0b1: Master configuration
            pub const Master: u32 = 0b1;
        }
    }

    /// Clock polarity
    pub mod CPOL {
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

            /// 0b0: CK to 0 when idle
            pub const IdleLow: u32 = 0b0;

            /// 0b1: CK to 1 when idle
            pub const IdleHigh: u32 = 0b1;
        }
    }

    /// Clock phase
    pub mod CPHA {
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

            /// 0b0: The first clock transition is the first data capture edge
            pub const FirstEdge: u32 = 0b0;

            /// 0b1: The second clock transition is the first data capture edge
            pub const SecondEdge: u32 = 0b1;
        }
    }
}

/// control register 2
pub mod CR2 {

    /// Rx buffer DMA enable
    pub mod RXDMAEN {
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

            /// 0b0: Rx buffer DMA disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Rx buffer DMA enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Tx buffer DMA enable
    pub mod TXDMAEN {
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

            /// 0b0: Tx buffer DMA disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Tx buffer DMA enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SS output enable
    pub mod SSOE {
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

            /// 0b0: SS output is disabled in master mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: SS output is enabled in master mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// NSS pulse management
    pub mod NSSP {
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

            /// 0b0: No NSS pulse
            pub const NoPulse: u32 = 0b0;

            /// 0b1: NSS pulse generated
            pub const PulseGenerated: u32 = 0b1;
        }
    }

    /// Frame format
    pub mod FRF {
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

            /// 0b0: SPI Motorola mode
            pub const Motorola: u32 = 0b0;

            /// 0b1: SPI TI mode
            pub const TI: u32 = 0b1;
        }
    }

    /// Error interrupt enable
    pub mod ERRIE {
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

            /// 0b0: Error interrupt masked
            pub const Masked: u32 = 0b0;

            /// 0b1: Error interrupt not masked
            pub const NotMasked: u32 = 0b1;
        }
    }

    /// RX buffer not empty interrupt enable
    pub mod RXNEIE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: RXE interrupt masked
            pub const Masked: u32 = 0b0;

            /// 0b1: RXE interrupt not masked
            pub const NotMasked: u32 = 0b1;
        }
    }

    /// Tx buffer empty interrupt enable
    pub mod TXEIE {
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

            /// 0b0: TXE interrupt masked
            pub const Masked: u32 = 0b0;

            /// 0b1: TXE interrupt not masked
            pub const NotMasked: u32 = 0b1;
        }
    }

    /// Data size
    pub mod DS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0011: 4-bit
            pub const FourBit: u32 = 0b0011;

            /// 0b0100: 5-bit
            pub const FiveBit: u32 = 0b0100;

            /// 0b0101: 6-bit
            pub const SixBit: u32 = 0b0101;

            /// 0b0110: 7-bit
            pub const SevenBit: u32 = 0b0110;

            /// 0b0111: 8-bit
            pub const EightBit: u32 = 0b0111;

            /// 0b1000: 9-bit
            pub const NineBit: u32 = 0b1000;

            /// 0b1001: 10-bit
            pub const TenBit: u32 = 0b1001;

            /// 0b1010: 11-bit
            pub const ElevenBit: u32 = 0b1010;

            /// 0b1011: 12-bit
            pub const TwelveBit: u32 = 0b1011;

            /// 0b1100: 13-bit
            pub const ThirteenBit: u32 = 0b1100;

            /// 0b1101: 14-bit
            pub const FourteenBit: u32 = 0b1101;

            /// 0b1110: 15-bit
            pub const FifteenBit: u32 = 0b1110;

            /// 0b1111: 16-bit
            pub const SixteenBit: u32 = 0b1111;
        }
    }

    /// FIFO reception threshold
    pub mod FRXTH {
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

            /// 0b0: RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)
            pub const Half: u32 = 0b0;

            /// 0b1: RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)
            pub const Quarter: u32 = 0b1;
        }
    }

    /// Last DMA transfer for reception
    pub mod LDMA_RX {
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

            /// 0b0: Number of data to transfer for receive is even
            pub const Even: u32 = 0b0;

            /// 0b1: Number of data to transfer for receive is odd
            pub const Odd: u32 = 0b1;
        }
    }

    /// Last DMA transfer for transmission
    pub mod LDMA_TX {
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

            /// 0b0: Number of data to transfer for transmit is even
            pub const Even: u32 = 0b0;

            /// 0b1: Number of data to transfer for transmit is odd
            pub const Odd: u32 = 0b1;
        }
    }
}

/// status register
pub mod SR {

    /// Receive buffer not empty
    pub mod RXNE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Rx buffer empty
            pub const Empty: u32 = 0b0;

            /// 0b1: Rx buffer not empty
            pub const NotEmpty: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transmit buffer empty
    pub mod TXE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Tx buffer not empty
            pub const NotEmpty: u32 = 0b0;

            /// 0b1: Tx buffer empty
            pub const Empty: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel side
    pub mod CHSIDE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Channel left has to be transmitted or has been received
            pub const Left: u32 = 0b0;

            /// 0b1: Channel right has to be transmitted or has been received
            pub const Right: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Underrun flag
    pub mod UDR {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No underrun occurred
            pub const NoUnderrun: u32 = 0b0;

            /// 0b1: Underrun occurred
            pub const Underrun: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CRC error flag
    pub mod CRCERR {
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

            /// 0b0: CRC value received matches the SPIx_RXCRCR value
            pub const Match: u32 = 0b0;

            /// 0b1: CRC value received does not match the SPIx_RXCRCR value
            pub const NoMatch: u32 = 0b1;
        }
    }

    /// Mode fault
    pub mod MODF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No mode fault occurred
            pub const NoFault: u32 = 0b0;

            /// 0b1: Mode fault occurred
            pub const Fault: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overrun flag
    pub mod OVR {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No overrun occurred
            pub const NoOverrun: u32 = 0b0;

            /// 0b1: Overrun occurred
            pub const Overrun: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Busy flag
    pub mod BSY {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: SPI not busy
            pub const NotBusy: u32 = 0b0;

            /// 0b1: SPI busy
            pub const Busy: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TI frame format error
    pub mod TIFRFE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No frame format error
            pub const NoError: u32 = 0b0;

            /// 0b1: A frame format error occurred
            pub const Error: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FIFO reception level
    pub mod FRLVL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values
        pub mod R {

            /// 0b00: Rx FIFO Empty
            pub const Empty: u32 = 0b00;

            /// 0b01: Rx 1/4 FIFO
            pub const Quarter: u32 = 0b01;

            /// 0b10: Rx 1/2 FIFO
            pub const Half: u32 = 0b10;

            /// 0b11: Rx FIFO full
            pub const Full: u32 = 0b11;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FIFO transmission level
    pub mod FTLVL {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values
        pub mod R {

            /// 0b00: Tx FIFO Empty
            pub const Empty: u32 = 0b00;

            /// 0b01: Tx 1/4 FIFO
            pub const Quarter: u32 = 0b01;

            /// 0b10: Tx 1/2 FIFO
            pub const Half: u32 = 0b10;

            /// 0b11: Tx FIFO full
            pub const Full: u32 = 0b11;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// data register
pub mod DR {

    /// Data register
    pub mod DR {
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

/// CRC polynomial register
pub mod CRCPR {

    /// CRC polynomial register
    pub mod CRCPOLY {
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

/// RX CRC register
pub mod RXCRCR {

    /// Rx CRC register
    pub mod RxCRC {
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

/// TX CRC register
pub mod TXCRCR {

    /// Tx CRC register
    pub mod TxCRC {
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

/// I2S configuration register
pub mod I2SCFGR {

    /// I2S mode selection
    pub mod I2SMOD {
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

            /// 0b0: SPI mode is selected
            pub const SPIMode: u32 = 0b0;

            /// 0b1: I2S mode is selected
            pub const I2SMode: u32 = 0b1;
        }
    }

    /// I2S Enable
    pub mod I2SE {
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

            /// 0b0: I2S peripheral is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: I2S peripheral is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// I2S configuration mode
    pub mod I2SCFG {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Slave - transmit
            pub const SlaveTx: u32 = 0b00;

            /// 0b01: Slave - receive
            pub const SlaveRx: u32 = 0b01;

            /// 0b10: Master - transmit
            pub const MasterTx: u32 = 0b10;

            /// 0b11: Master - receive
            pub const MasterRx: u32 = 0b11;
        }
    }

    /// PCM frame synchronization
    pub mod PCMSYNC {
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

            /// 0b0: Short frame synchronisation
            pub const Short: u32 = 0b0;

            /// 0b1: Long frame synchronisation
            pub const Long: u32 = 0b1;
        }
    }

    /// I2S standard selection
    pub mod I2SSTD {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: I2S Philips standard
            pub const Philips: u32 = 0b00;

            /// 0b01: MSB justified standard
            pub const MSB: u32 = 0b01;

            /// 0b10: LSB justified standard
            pub const LSB: u32 = 0b10;

            /// 0b11: PCM standard
            pub const PCM: u32 = 0b11;
        }
    }

    /// Steady state clock polarity
    pub mod CKPOL {
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

            /// 0b0: I2S clock inactive state is low level
            pub const IdleLow: u32 = 0b0;

            /// 0b1: I2S clock inactive state is high level
            pub const IdleHigh: u32 = 0b1;
        }
    }

    /// Data length to be transferred
    pub mod DATLEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 16-bit data length
            pub const SixteenBit: u32 = 0b00;

            /// 0b01: 24-bit data length
            pub const TwentyFourBit: u32 = 0b01;

            /// 0b10: 32-bit data length
            pub const ThirtyTwoBit: u32 = 0b10;
        }
    }

    /// Channel length (number of bits per audio channel)
    pub mod CHLEN {
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

            /// 0b0: 16-bit wide
            pub const SixteenBit: u32 = 0b0;

            /// 0b1: 32-bit wide
            pub const ThirtyTwoBit: u32 = 0b1;
        }
    }
}

/// I2S prescaler register
pub mod I2SPR {

    /// Master clock output enable
    pub mod MCKOE {
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

            /// 0b0: Master clock output is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Master clock output is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Odd factor for the prescaler
    pub mod ODD {
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

            /// 0b0: Real divider value is I2SDIV * 2
            pub const Even: u32 = 0b0;

            /// 0b1: Real divider value is (I2SDIV * 2) + 1
            pub const Odd: u32 = 0b1;
        }
    }

    /// I2S Linear prescaler
    pub mod I2SDIV {
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
#[repr(C)]
pub struct RegisterBlock {
    /// control register 1
    pub CR1: RWRegister<u32>,

    /// control register 2
    pub CR2: RWRegister<u32>,

    /// status register
    pub SR: RWRegister<u32>,

    /// data register
    pub DR: RWRegister<u32>,

    /// CRC polynomial register
    pub CRCPR: RWRegister<u32>,

    /// RX CRC register
    pub RXCRCR: RORegister<u32>,

    /// TX CRC register
    pub TXCRCR: RORegister<u32>,

    /// I2S configuration register
    pub I2SCFGR: RWRegister<u32>,

    /// I2S prescaler register
    pub I2SPR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1: u32,
    pub CR2: u32,
    pub SR: u32,
    pub DR: u32,
    pub CRCPR: u32,
    pub RXCRCR: u32,
    pub TXCRCR: u32,
    pub I2SCFGR: u32,
    pub I2SPR: u32,
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

/// Access functions for the I2S2ext peripheral instance
pub mod I2S2ext {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40003400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in I2S2ext
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        SR: 0x00000002,
        DR: 0x00000000,
        CRCPR: 0x00000007,
        RXCRCR: 0x00000000,
        TXCRCR: 0x00000000,
        I2SCFGR: 0x00000000,
        I2SPR: 0x00000010,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut I2S2ext_TAKEN: bool = false;

    /// Safe access to I2S2ext
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
            if I2S2ext_TAKEN {
                None
            } else {
                I2S2ext_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to I2S2ext
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if I2S2ext_TAKEN && inst.addr == INSTANCE.addr {
                I2S2ext_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal I2S2ext
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        I2S2ext_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to I2S2ext
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const I2S2ext: *const RegisterBlock = 0x40003400 as *const _;

/// Access functions for the I2S3ext peripheral instance
pub mod I2S3ext {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40004000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in I2S3ext
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        SR: 0x00000002,
        DR: 0x00000000,
        CRCPR: 0x00000007,
        RXCRCR: 0x00000000,
        TXCRCR: 0x00000000,
        I2SCFGR: 0x00000000,
        I2SPR: 0x00000010,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut I2S3ext_TAKEN: bool = false;

    /// Safe access to I2S3ext
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
            if I2S3ext_TAKEN {
                None
            } else {
                I2S3ext_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to I2S3ext
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if I2S3ext_TAKEN && inst.addr == INSTANCE.addr {
                I2S3ext_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal I2S3ext
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        I2S3ext_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to I2S3ext
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const I2S3ext: *const RegisterBlock = 0x40004000 as *const _;

/// Access functions for the SPI1 peripheral instance
pub mod SPI1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40013000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SPI1
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        SR: 0x00000002,
        DR: 0x00000000,
        CRCPR: 0x00000007,
        RXCRCR: 0x00000000,
        TXCRCR: 0x00000000,
        I2SCFGR: 0x00000000,
        I2SPR: 0x00000010,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SPI1_TAKEN: bool = false;

    /// Safe access to SPI1
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
            if SPI1_TAKEN {
                None
            } else {
                SPI1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SPI1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SPI1_TAKEN && inst.addr == INSTANCE.addr {
                SPI1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SPI1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SPI1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SPI1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SPI1: *const RegisterBlock = 0x40013000 as *const _;

/// Access functions for the SPI2 peripheral instance
pub mod SPI2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40003800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SPI2
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        SR: 0x00000002,
        DR: 0x00000000,
        CRCPR: 0x00000007,
        RXCRCR: 0x00000000,
        TXCRCR: 0x00000000,
        I2SCFGR: 0x00000000,
        I2SPR: 0x00000010,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SPI2_TAKEN: bool = false;

    /// Safe access to SPI2
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
            if SPI2_TAKEN {
                None
            } else {
                SPI2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SPI2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SPI2_TAKEN && inst.addr == INSTANCE.addr {
                SPI2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SPI2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SPI2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SPI2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SPI2: *const RegisterBlock = 0x40003800 as *const _;

/// Access functions for the SPI3 peripheral instance
pub mod SPI3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40003c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SPI3
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        SR: 0x00000002,
        DR: 0x00000000,
        CRCPR: 0x00000007,
        RXCRCR: 0x00000000,
        TXCRCR: 0x00000000,
        I2SCFGR: 0x00000000,
        I2SPR: 0x00000010,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SPI3_TAKEN: bool = false;

    /// Safe access to SPI3
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
            if SPI3_TAKEN {
                None
            } else {
                SPI3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SPI3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SPI3_TAKEN && inst.addr == INSTANCE.addr {
                SPI3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SPI3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SPI3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SPI3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SPI3: *const RegisterBlock = 0x40003c00 as *const _;
