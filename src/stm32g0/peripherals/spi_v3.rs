#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Serial peripheral interface
//!
//! Used by: stm32g0b1, stm32g0c1

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// SPI control register 1
pub mod CR1 {

    /// Clock phase Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode.
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

    /// Clock polarity Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode.
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

    /// Master selection Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode.
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

    /// Baud rate control Note: These bits should not be changed when communication is ongoing. These bits are not used in I2S mode.
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

    /// SPI enable Note: When disabling the SPI, follow the procedure described in SPI on pageÂ 1021. This bit is not used in I2S mode.
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

    /// Frame format Note: 1. This bit should not be changed when communication is ongoing. 2. This bit is not used in I2S mode and SPI TI mode.
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

    /// Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I2S mode and SPI TI mode.
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

    /// Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I2S mode and SPI TI mode.
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

    /// Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I2S mode.
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

    /// CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = '0â) for correct operation. This bit is not used in I2S mode.
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

    /// Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPI_DR register. This bit is not used in I2S mode.
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

    /// Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = '0â) for correct operation. This bit is not used in I2S mode.
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

    /// Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. This bit is not used in I2S mode.
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

    /// Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I2S mode.
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
}

/// SPI control register 2
pub mod CR2 {

    /// Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set.
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

    /// Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set.
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

    /// SS output enable Note: This bit is not used in I2S mode and SPI TI mode.
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

    /// NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = â1â, or FRF = â1â. Note: 1. This bit must be written only when the SPI is disabled (SPE=0). 2. This bit is not used in I2S mode and SPI TI mode.
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

    /// Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0). This bit is not used in I2S mode.
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

    /// Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I2S mode).
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

    /// Data size These bits configure the data length for SPI transfers. If software attempts to write one of the âNot usedâ values, they are forced to the value â0111â (8-bit) Note: These bits are not used in I2S mode.
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

    /// FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event Note: This bit is not used in I2S mode.
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

    /// Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =< 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to if the CRCEN bit is set. This bit is not used in IÂ²S mode.
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

    /// Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =< 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to if the CRCEN bit is set. This bit is not used in IÂ²S mode.
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

/// SPI status register
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

    /// Channel side Note: This bit is not used in SPI mode. It has no significance in PCM mode.
    pub mod CHSIDE {
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

    /// Underrun flag This flag is set by hardware and reset by a software sequence. Refer to pageÂ 1057 for the software sequence. Note: This bit is not used in SPI mode.
    pub mod UDR {
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

    /// CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode.
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

    /// Mode fault This flag is set by hardware and reset by a software sequence. Refer to (MODF) on pageÂ 1031 for the software sequence. Note: This bit is not used in I2S mode.
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

    /// Overrun flag This flag is set by hardware and reset by a software sequence. Refer to pageÂ 1057 for the software sequence.
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

    /// Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to and .
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

    /// Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to error flags and . This flag is set by hardware and reset when SPI_SR is read by software.
    pub mod FRE {
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

    /// FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in IÂ²S mode and in SPI receive-only mode while CRC calculation is enabled.
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

    /// FIFO transmission level These bits are set and cleared by hardware. Note: This bit is not used in I2S mode.
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

/// SPI data register
pub mod DR {

    /// Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See ). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used.
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

/// SPI CRC polynomial register
pub mod CRCPR {

    /// CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0x0007) is the reset value of this register. Another polynomial can be configured as required.
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

/// SPI Rx CRC register
pub mod RXCRCR {

    /// Rx CRC register When CRC calculation is enabled, the RXCRC\[15:0\] bits contain the computed CRC value of the subsequently received bytes. This register is reset when the CRCEN bit in SPI_CR1 register is written to 1. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPR register. Only the 8 LSB bits are considered when the CRC frame format is set to be 8-bit length (CRCL bit in the SPI_CR1 is cleared). CRC calculation is done based on any CRC8 standard. The entire 16-bits of this register are considered when a 16-bit CRC frame format is selected (CRCL bit in the SPI_CR1 register is set). CRC calculation is done based on any CRC16 standard. Note: A read to this register when the BSY Flag is set could return an incorrect value. These bits are not used in I2S mode.
    pub mod RXCRC {
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

/// SPI Tx CRC register
pub mod TXCRCR {

    /// Tx CRC register When CRC calculation is enabled, the TXCRC\[7:0\] bits contain the computed CRC value of the subsequently transmitted bytes. This register is reset when the CRCEN bit of SPI_CR1 is written to 1. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPR register. Only the 8 LSB bits are considered when the CRC frame format is set to be 8-bit length (CRCL bit in the SPI_CR1 is cleared). CRC calculation is done based on any CRC8 standard. The entire 16-bits of this register are considered when a 16-bit CRC frame format is selected (CRCL bit in the SPI_CR1 register is set). CRC calculation is done based on any CRC16 standard. Note: A read to this register when the BSY flag is set could return an incorrect value. These bits are not used in I2S mode.
    pub mod TXCRC {
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

/// SPI_I2S configuration register
pub mod I2SCFGR {

    /// Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode.
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

    /// Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.
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

    /// Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals.
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

    /// I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.
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

    /// PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode.
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

    /// I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode.
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

    /// I2S enable Note: This bit is not used in SPI mode.
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

    /// I2S mode selection Note: This bit should be configured when the SPI is disabled.
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

    /// Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information.
    pub mod ASTRTEN {
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

            /// 0b0: Asynchronous start disabled
            pub const AsyncStartDisabled: u32 = 0b0;

            /// 0b1: Asynchronous start enabled
            pub const AsyncStartEnabled: u32 = 0b1;
        }
    }
}

/// SPI_I2S prescaler register
pub mod I2SPR {

    /// I2S linear prescaler I2SDIV \[7:0\] = 0 or I2SDIV \[7:0\] = 1 are forbidden values. Refer to . Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. They are not used in SPI mode.
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

    /// Odd factor for the prescaler Refer to . Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode.
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

    /// Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode.
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// SPI control register 1
    pub CR1: RWRegister<u32>,

    /// SPI control register 2
    pub CR2: RWRegister<u32>,

    /// SPI status register
    pub SR: RWRegister<u32>,

    /// SPI data register
    pub DR: RWRegister<u32>,

    /// SPI CRC polynomial register
    pub CRCPR: RWRegister<u32>,

    /// SPI Rx CRC register
    pub RXCRCR: RORegister<u32>,

    /// SPI Tx CRC register
    pub TXCRCR: RORegister<u32>,

    /// SPI_I2S configuration register
    pub I2SCFGR: RWRegister<u32>,

    /// SPI_I2S prescaler register
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
