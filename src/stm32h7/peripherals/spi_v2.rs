#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Serial peripheral interface
//!
//! Used by: stm32h747cm4, stm32h747cm7, stm32h7b3

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register 1
pub mod CR1 {

    /// Locking the AF configuration of associated IOs
    pub mod IOLOCK {
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

            /// 0b0: IO configuration unlocked
            pub const Unlocked: u32 = 0b0;

            /// 0b1: IO configuration locked
            pub const Locked: u32 = 0b1;
        }
    }

    /// CRC calculation initialization pattern control for transmitter
    pub mod TCRCINI {
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

            /// 0b0: All zeros TX CRC initialization pattern
            pub const AllZeros: u32 = 0b0;

            /// 0b1: All ones TX CRC initialization pattern
            pub const AllOnes: u32 = 0b1;
        }
    }

    /// CRC calculation initialization pattern control for receiver
    pub mod RCRCINI {
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

            /// 0b0: All zeros RX CRC initialization pattern
            pub const AllZeros: u32 = 0b0;

            /// 0b1: All ones RX CRC initialization pattern
            pub const AllOnes: u32 = 0b1;
        }
    }

    /// 32-bit CRC polynomial configuration
    pub mod CRC33_17 {
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

            /// 0b0: Full size (33/17 bit) CRC polynomial is not used
            pub const Disabled: u32 = 0b0;

            /// 0b1: Full size (33/17 bit) CRC polynomial is used
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Internal SS signal input level
    pub mod SSI {
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

            /// 0b0: 0 is forced onto the SS signal and the I/O value of the SS pin is ignored
            pub const SlaveSelected: u32 = 0b0;

            /// 0b1: 1 is forced onto the SS signal and the I/O value of the SS pin is ignored
            pub const SlaveNotSelected: u32 = 0b1;
        }
    }

    /// Rx/Tx direction at Half-duplex mode
    pub mod HDDIR {
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

            /// 0b0: Receiver in half duplex mode
            pub const Receiver: u32 = 0b0;

            /// 0b1: Transmitter in half duplex mode
            pub const Transmitter: u32 = 0b1;
        }
    }

    /// Master SUSPend request
    pub mod CSUSP {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: Do not request master suspend
            pub const NotRequested: u32 = 0b0;

            /// 0b1: Request master suspend
            pub const Requested: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Master transfer start
    pub mod CSTART {
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

            /// 0b0: Do not start master transfer
            pub const NotStarted: u32 = 0b0;

            /// 0b1: Start master transfer
            pub const Started: u32 = 0b1;
        }
    }

    /// Master automatic SUSP in Receive mode
    pub mod MASRX {
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

            /// 0b0: Automatic suspend in master receive-only mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Automatic suspend in master receive-only mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Serial Peripheral Enable
    pub mod SPE {
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

            /// 0b0: Peripheral disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Peripheral enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// control register 2
pub mod CR2 {

    /// Number of data transfer extension to be reload into TSIZE just when a previous
    pub mod TSER {
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

    /// Number of data at current transfer
    pub mod TSIZE {
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

/// configuration register 1
pub mod CFG1 {

    /// Master baud rate
    pub mod MBR {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (3 bits: 0b111 << 28)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: f_spi_ker_ck / 2
            pub const Div2: u32 = 0b000;

            /// 0b001: f_spi_ker_ck / 4
            pub const Div4: u32 = 0b001;

            /// 0b010: f_spi_ker_ck / 8
            pub const Div8: u32 = 0b010;

            /// 0b011: f_spi_ker_ck / 16
            pub const Div16: u32 = 0b011;

            /// 0b100: f_spi_ker_ck / 32
            pub const Div32: u32 = 0b100;

            /// 0b101: f_spi_ker_ck / 64
            pub const Div64: u32 = 0b101;

            /// 0b110: f_spi_ker_ck / 128
            pub const Div128: u32 = 0b110;

            /// 0b111: f_spi_ker_ck / 256
            pub const Div256: u32 = 0b111;
        }
    }

    /// Hardware CRC computation enable
    pub mod CRCEN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
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

    /// Length of CRC frame to be transacted and compared
    pub mod CRCSIZE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Tx DMA stream enable
    pub mod TXDMAEN {
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

            /// 0b0: Tx buffer DMA disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Tx buffer DMA enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Rx DMA stream enable
    pub mod RXDMAEN {
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

            /// 0b0: Rx buffer DMA disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Rx buffer DMA enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Detection of underrun condition at slave transmitter
    pub mod UDRDET {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Underrun is detected at begin of data frame
            pub const StartOfFrame: u32 = 0b00;

            /// 0b01: Underrun is detected at end of last data frame
            pub const EndOfFrame: u32 = 0b01;

            /// 0b10: Underrun is detected at begin of active SS signal
            pub const StartOfSlaveSelect: u32 = 0b10;
        }
    }

    /// Behavior of slave transmitter at underrun condition
    pub mod UDRCFG {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Slave sends a constant underrun pattern
            pub const Constant: u32 = 0b00;

            /// 0b01: Slave repeats last received data frame from master
            pub const RepeatReceived: u32 = 0b01;

            /// 0b10: Slave repeats last transmitted data frame
            pub const RepeatTransmitted: u32 = 0b10;
        }
    }

    /// threshold level
    pub mod FTHLV {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (4 bits: 0b1111 << 5)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: 1 frame
            pub const OneFrame: u32 = 0b0000;

            /// 0b0001: 2 frames
            pub const TwoFrames: u32 = 0b0001;

            /// 0b0010: 3 frames
            pub const ThreeFrames: u32 = 0b0010;

            /// 0b0011: 4 frames
            pub const FourFrames: u32 = 0b0011;

            /// 0b0100: 5 frames
            pub const FiveFrames: u32 = 0b0100;

            /// 0b0101: 6 frames
            pub const SixFrames: u32 = 0b0101;

            /// 0b0110: 7 frames
            pub const SevenFrames: u32 = 0b0110;

            /// 0b0111: 8 frames
            pub const EightFrames: u32 = 0b0111;

            /// 0b1000: 9 frames
            pub const NineFrames: u32 = 0b1000;

            /// 0b1001: 10 frames
            pub const TenFrames: u32 = 0b1001;

            /// 0b1010: 11 frames
            pub const ElevenFrames: u32 = 0b1010;

            /// 0b1011: 12 frames
            pub const TwelveFrames: u32 = 0b1011;

            /// 0b1100: 13 frames
            pub const ThirteenFrames: u32 = 0b1100;

            /// 0b1101: 14 frames
            pub const FourteenFrames: u32 = 0b1101;

            /// 0b1110: 15 frames
            pub const FifteenFrames: u32 = 0b1110;

            /// 0b1111: 16 frames
            pub const SixteenFrames: u32 = 0b1111;
        }
    }

    /// Number of bits in at single SPI data frame
    pub mod DSIZE {
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
}

/// configuration register 2
pub mod CFG2 {

    /// Alternate function GPIOs control
    pub mod AFCNTR {
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

            /// 0b0: Peripheral takes no control of GPIOs while disabled
            pub const NotControlled: u32 = 0b0;

            /// 0b1: Peripheral controls GPIOs while disabled
            pub const Controlled: u32 = 0b1;
        }
    }

    /// SS output management in master mode
    pub mod SSOM {
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

            /// 0b0: SS is asserted until data transfer complete
            pub const Asserted: u32 = 0b0;

            /// 0b1: Data frames interleaved with SS not asserted during MIDI
            pub const NotAsserted: u32 = 0b1;
        }
    }

    /// SS output enable
    pub mod SSOE {
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

            /// 0b0: SS output is disabled in master mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: SS output is enabled in master mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SS input/output polarity
    pub mod SSIOP {
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

            /// 0b0: Low level is active for SS signal
            pub const ActiveLow: u32 = 0b0;

            /// 0b1: High level is active for SS signal
            pub const ActiveHigh: u32 = 0b1;
        }
    }

    /// Software management of SS signal input
    pub mod SSM {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
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

    /// Clock polarity
    pub mod CPOL {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
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
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
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

    /// Data frame format
    pub mod LSBFRST {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
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

    /// SPI Master
    pub mod MASTER {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
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

    /// Serial Protocol
    pub mod SP {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (3 bits: 0b111 << 19)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Motorola SPI protocol
            pub const Motorola: u32 = 0b000;

            /// 0b001: TI SPI protocol
            pub const TI: u32 = 0b001;
        }
    }

    /// SPI Communication Mode
    pub mod COMM {
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

            /// 0b00: Full duplex
            pub const FullDuplex: u32 = 0b00;

            /// 0b01: Simplex transmitter only
            pub const Transmitter: u32 = 0b01;

            /// 0b10: Simplex receiver only
            pub const Receiver: u32 = 0b10;

            /// 0b11: Half duplex
            pub const HalfDuplex: u32 = 0b11;
        }
    }

    /// Swap functionality of MISO and MOSI pins
    pub mod IOSWP {
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

            /// 0b0: MISO and MOSI not swapped
            pub const Disabled: u32 = 0b0;

            /// 0b1: MISO and MOSI swapped
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Master Inter-Data Idleness
    pub mod MIDI {
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

    /// Master SS Idleness
    pub mod MSSI {
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
}

/// Interrupt Enable Register
pub mod IER {

    /// Additional number of transactions reload interrupt enable
    pub mod TSERFIE {
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

            /// 0b0: TSER loaded interrupt masked
            pub const Masked: u32 = 0b0;

            /// 0b1: TSER loaded interrupt not masked
            pub const NotMasked: u32 = 0b1;
        }
    }

    /// Mode Fault interrupt enable
    pub mod MODFIE {
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

            /// 0b0: Mode fault interrupt masked
            pub const Masked: u32 = 0b0;

            /// 0b1: Mode fault interrupt not masked
            pub const NotMasked: u32 = 0b1;
        }
    }

    /// TIFRE interrupt enable
    pub mod TIFREIE {
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

            /// 0b0: TI frame format error interrupt masked
            pub const Masked: u32 = 0b0;

            /// 0b1: TI frame format error interrupt not masked
            pub const NotMasked: u32 = 0b1;
        }
    }

    /// CRC Interrupt enable
    pub mod CRCEIE {
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

            /// 0b0: CRC error interrupt masked
            pub const Masked: u32 = 0b0;

            /// 0b1: CRC error interrupt not masked
            pub const NotMasked: u32 = 0b1;
        }
    }

    /// OVR interrupt enable
    pub mod OVRIE {
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

            /// 0b0: Overrun interrupt masked
            pub const Masked: u32 = 0b0;

            /// 0b1: Overrun interrupt not masked
            pub const NotMasked: u32 = 0b1;
        }
    }

    /// UDR interrupt enable
    pub mod UDRIE {
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

            /// 0b0: Underrun interrupt masked
            pub const Masked: u32 = 0b0;

            /// 0b1: Underrun interrupt not masked
            pub const NotMasked: u32 = 0b1;
        }
    }

    /// TXTFIE interrupt enable
    pub mod TXTFIE {
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

            /// 0b0: Transmission transfer filled interrupt masked
            pub const Masked: u32 = 0b0;

            /// 0b1: Transmission transfer filled interrupt not masked
            pub const NotMasked: u32 = 0b1;
        }
    }

    /// EOT, SUSP and TXC interrupt enable
    pub mod EOTIE {
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

            /// 0b0: End-of-transfer interrupt masked
            pub const Masked: u32 = 0b0;

            /// 0b1: End-of-transfer interrupt not masked
            pub const NotMasked: u32 = 0b1;
        }
    }

    /// DXP interrupt enabled
    pub mod DXPIE {
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

            /// 0b0: Duplex transfer complete interrupt masked
            pub const Masked: u32 = 0b0;

            /// 0b1: Duplex transfer complete interrupt not masked
            pub const NotMasked: u32 = 0b1;
        }
    }

    /// TXP interrupt enable
    pub mod TXPIE {
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

            /// 0b0: TX space available interrupt masked
            pub const Masked: u32 = 0b0;

            /// 0b1: TX space available interrupt not masked
            pub const NotMasked: u32 = 0b1;
        }
    }

    /// RXP Interrupt Enable
    pub mod RXPIE {
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

            /// 0b0: RX data available interrupt masked
            pub const Masked: u32 = 0b0;

            /// 0b1: RX data available interrupt not masked
            pub const NotMasked: u32 = 0b1;
        }
    }
}

/// Status Register
pub mod SR {

    /// Number of data frames remaining in current TSIZE session
    pub mod CTSIZE {
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

    /// RxFIFO Word Not Empty
    pub mod RXWNE {
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

            /// 0b0: Less than 32-bit data frame received
            pub const LessThan32: u32 = 0b0;

            /// 0b1: At least 32-bit data frame received
            pub const AtLeast32: u32 = 0b1;
        }
    }

    /// RxFIFO Packing LeVeL
    pub mod RXPLVL {
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

            /// 0b00: Zero frames beyond packing ratio available
            pub const ZeroFrames: u32 = 0b00;

            /// 0b01: One frame beyond packing ratio available
            pub const OneFrame: u32 = 0b01;

            /// 0b10: Two frame beyond packing ratio available
            pub const TwoFrames: u32 = 0b10;

            /// 0b11: Three frame beyond packing ratio available
            pub const ThreeFrames: u32 = 0b11;
        }
    }

    /// TxFIFO transmission complete
    pub mod TXC {
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

            /// 0b0: Transmission ongoing
            pub const Ongoing: u32 = 0b0;

            /// 0b1: Transmission completed
            pub const Completed: u32 = 0b1;
        }
    }

    /// SUSPend
    pub mod SUSP {
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

            /// 0b0: Master not suspended
            pub const NotSuspended: u32 = 0b0;

            /// 0b1: Master suspended
            pub const Suspended: u32 = 0b1;
        }
    }

    /// Additional number of SPI data to be transacted was reload
    pub mod TSERF {
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

            /// 0b0: Additional number of SPI data to be transacted not yet loaded
            pub const NotLoaded: u32 = 0b0;

            /// 0b1: Additional number of SPI data to be transacted was reloaded
            pub const Loaded: u32 = 0b1;
        }
    }

    /// Mode Fault
    pub mod MODF {
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

            /// 0b0: No mode fault detected
            pub const NoFault: u32 = 0b0;

            /// 0b1: Mode fault detected
            pub const Fault: u32 = 0b1;
        }
    }

    /// TI frame format error
    pub mod TIFRE {
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

            /// 0b0: TI frame format error detected
            pub const NoError: u32 = 0b0;

            /// 0b1: TI frame format error detected
            pub const Error: u32 = 0b1;
        }
    }

    /// CRC Error
    pub mod CRCE {
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

            /// 0b0: No CRC error detected
            pub const NoError: u32 = 0b0;

            /// 0b1: CRC error detected
            pub const Error: u32 = 0b1;
        }
    }

    /// Overrun
    pub mod OVR {
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

            /// 0b0: No overrun occurred
            pub const NoOverrun: u32 = 0b0;

            /// 0b1: Overrun occurred
            pub const Overrun: u32 = 0b1;
        }
    }

    /// Underrun at slave transmission mode
    pub mod UDR {
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

            /// 0b0: No underrun occurred
            pub const NoUnderrun: u32 = 0b0;

            /// 0b1: Underrun occurred
            pub const Underrun: u32 = 0b1;
        }
    }

    /// Transmission Transfer Filled
    pub mod TXTF {
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

            /// 0b0: Transmission buffer incomplete
            pub const NotCompleted: u32 = 0b0;

            /// 0b1: Transmission buffer filled with at least one transfer
            pub const Completed: u32 = 0b1;
        }
    }

    /// End Of Transfer
    pub mod EOT {
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

            /// 0b0: Transfer ongoing or not started
            pub const NotCompleted: u32 = 0b0;

            /// 0b1: Transfer complete
            pub const Completed: u32 = 0b1;
        }
    }

    /// Duplex Packet
    pub mod DXP {
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

            /// 0b0: Duplex packet unavailable: no space for transmission and/or no data received
            pub const Unavailable: u32 = 0b0;

            /// 0b1: Duplex packet available: space for transmission and data received
            pub const Available: u32 = 0b1;
        }
    }

    /// Tx-Packet space available
    pub mod TXP {
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

            /// 0b0: Tx buffer full
            pub const Full: u32 = 0b0;

            /// 0b1: Tx buffer not full
            pub const NotFull: u32 = 0b1;
        }
    }

    /// Rx-Packet available
    pub mod RXP {
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

            /// 0b0: Rx buffer empty
            pub const Empty: u32 = 0b0;

            /// 0b1: Rx buffer not empty
            pub const NotEmpty: u32 = 0b1;
        }
    }
}

/// Interrupt/Status Flags Clear Register
pub mod IFCR {

    /// SUSPend flag clear
    pub mod SUSPC {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear interrupt flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TSERFC flag clear
    pub mod TSERFC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SUSPC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Mode Fault flag clear
    pub mod MODFC {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SUSPC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TI frame format error flag clear
    pub mod TIFREC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SUSPC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CRC Error flag clear
    pub mod CRCEC {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SUSPC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overrun flag clear
    pub mod OVRC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SUSPC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Underrun flag clear
    pub mod UDRC {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SUSPC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transmission Transfer Filled flag clear
    pub mod TXTFC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SUSPC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// End Of Transfer flag clear
    pub mod EOTC {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SUSPC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmit Data Register
pub mod TXDR {

    /// Transmit data register
    pub mod TXDR {
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

/// Receive Data Register
pub mod RXDR {

    /// Receive data register
    pub mod RXDR {
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

/// Polynomial Register
pub mod CRCPOLY {

    /// CRC polynomial register
    pub mod CRCPOLY {
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

/// Transmitter CRC Register
pub mod TXCRC {

    /// CRC register for transmitter
    pub mod TXCRC {
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

/// Receiver CRC Register
pub mod RXCRC {

    /// CRC register for receiver
    pub mod RXCRC {
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

/// Underrun Data Register
pub mod UDRDR {

    /// Data at slave underrun condition
    pub mod UDRDR {
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

/// configuration register
pub mod I2SCFGR {

    /// Master clock output enable
    pub mod MCKOE {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Master clock output disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Master clock output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Odd factor for the prescaler
    pub mod ODD {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Real divider value is I2SDIV*2
            pub const Even: u32 = 0b0;

            /// 0b1: Real divider value is I2SDIV*2 + 1
            pub const Odd: u32 = 0b1;
        }
    }

    /// I2S linear prescaler
    pub mod I2SDIV {
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

    /// Data format
    pub mod DATFMT {
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

            /// 0b0: The data inside RXDR and TXDR are right aligned
            pub const RightAligned: u32 = 0b0;

            /// 0b1: The data inside RXDR and TXDR are left aligned
            pub const LeftAligned: u32 = 0b1;
        }
    }

    /// Fixed channel length in SLAVE
    pub mod WSINV {
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

            /// 0b0: Word select inversion disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Word select inversion enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Word select inversion
    pub mod FIXCH {
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

            /// 0b0: The channel length in slave mode is different from 16 or 32 bits (CHLEN not taken into account)
            pub const NotFixed: u32 = 0b0;

            /// 0b1: The channel length in slave mode is supposed to be 16 or 32 bits (according to CHLEN)
            pub const Fixed: u32 = 0b1;
        }
    }

    /// Serial audio clock polarity
    pub mod CKPOL {
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

            /// 0b0: Signals are sampled on rising and changed on falling clock edges
            pub const SampleOnRising: u32 = 0b0;

            /// 0b1: Signals are sampled on falling and changed on rising clock edges
            pub const SampleOnFalling: u32 = 0b1;
        }
    }

    /// Channel length (number of bits per audio channel)
    pub mod CHLEN {
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

            /// 0b0: 16 bit per channel
            pub const Bits16: u32 = 0b0;

            /// 0b1: 32 bit per channel
            pub const Bits32: u32 = 0b1;
        }
    }

    /// Data length to be transferred
    pub mod DATLEN {
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

            /// 0b00: 16 bit data length
            pub const Bits16: u32 = 0b00;

            /// 0b01: 24 bit data length
            pub const Bits24: u32 = 0b01;

            /// 0b10: 32 bit data length
            pub const Bits32: u32 = 0b10;
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

            /// 0b0: Short PCM frame synchronization
            pub const Short: u32 = 0b0;

            /// 0b1: Long PCM frame synchronization
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

            /// 0b01: MSB/left justified standard
            pub const LeftAligned: u32 = 0b01;

            /// 0b10: LSB/right justified standard
            pub const RightAligned: u32 = 0b10;

            /// 0b11: PCM standard
            pub const PCM: u32 = 0b11;
        }
    }

    /// I2S configuration mode
    pub mod I2SCFG {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Slave, transmit
            pub const SlaveTransmit: u32 = 0b000;

            /// 0b001: Slave, recteive
            pub const SlaveReceive: u32 = 0b001;

            /// 0b010: Master, transmit
            pub const MasterTransmit: u32 = 0b010;

            /// 0b011: Master, receive
            pub const MasterReceive: u32 = 0b011;

            /// 0b100: Slave, full duplex
            pub const SlaveFullDuplex: u32 = 0b100;

            /// 0b101: Master, full duplex
            pub const MasterFullDuplex: u32 = 0b101;
        }
    }

    /// I2S mode selection
    pub mod I2SMOD {
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

            /// 0b0: SPI mode selected
            pub const SPI: u32 = 0b0;

            /// 0b1: I2S/PCM mode selected
            pub const I2S: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// control register 1
    pub CR1: RWRegister<u32>,

    /// control register 2
    pub CR2: RWRegister<u32>,

    /// configuration register 1
    pub CFG1: RWRegister<u32>,

    /// configuration register 2
    pub CFG2: RWRegister<u32>,

    /// Interrupt Enable Register
    pub IER: RWRegister<u32>,

    /// Status Register
    pub SR: RORegister<u32>,

    /// Interrupt/Status Flags Clear Register
    pub IFCR: WORegister<u32>,

    _reserved1: [u32; 1],

    /// Transmit Data Register
    pub TXDR: WORegister<u32>,

    _reserved2: [u32; 3],

    /// Receive Data Register
    pub RXDR: RORegister<u32>,

    _reserved3: [u32; 3],

    /// Polynomial Register
    pub CRCPOLY: RWRegister<u32>,

    /// Transmitter CRC Register
    pub TXCRC: RWRegister<u32>,

    /// Receiver CRC Register
    pub RXCRC: RWRegister<u32>,

    /// Underrun Data Register
    pub UDRDR: RWRegister<u32>,

    /// configuration register
    pub I2SCFGR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1: u32,
    pub CR2: u32,
    pub CFG1: u32,
    pub CFG2: u32,
    pub IER: u32,
    pub SR: u32,
    pub IFCR: u32,
    pub TXDR: u32,
    pub RXDR: u32,
    pub CRCPOLY: u32,
    pub TXCRC: u32,
    pub RXCRC: u32,
    pub UDRDR: u32,
    pub I2SCFGR: u32,
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
