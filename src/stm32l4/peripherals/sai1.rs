#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Serial audio interface
//!
//! Used by: stm32l412, stm32l4r9, stm32l4x1, stm32l4x2, stm32l4x3, stm32l4x5, stm32l4x6

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// AConfiguration register 1
pub mod CR1A {

    /// Master clock divider
    pub mod MCKDIV {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// No divider
    pub mod NODIV {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MCLK output is enabled. Forces the ratio between FS and MCLK to 256 or 512 according to the OSR value
            pub const MasterClock: u32 = 0b0;

            /// 0b1: MCLK output enable set by the MCKEN bit (where present, else 0). Ratio between FS and MCLK depends on FRL.
            pub const NoDiv: u32 = 0b1;
        }
    }

    /// DMA enable
    pub mod DMAEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Audio block A enable
    pub mod SAIEN {
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

            /// 0b0: SAI audio block disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: SAI audio block enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Output drive
    pub mod OUTDRIV {
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

            /// 0b0: Audio block output driven when SAIEN is set
            pub const OnStart: u32 = 0b0;

            /// 0b1: Audio block output driven immediately after the setting of this bit
            pub const Immediately: u32 = 0b1;
        }
    }

    /// Mono mode
    pub mod MONO {
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

            /// 0b0: Stereo mode
            pub const Stereo: u32 = 0b0;

            /// 0b1: Mono mode
            pub const Mono: u32 = 0b1;
        }
    }

    /// Synchronization enable
    pub mod SYNCEN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: audio sub-block in asynchronous mode
            pub const Asynchronous: u32 = 0b00;

            /// 0b01: audio sub-block is synchronous with the other internal audio sub-block. In this case, the audio sub-block must be configured in slave mode
            pub const Internal: u32 = 0b01;

            /// 0b10: audio sub-block is synchronous with an external SAI embedded peripheral. In this case the audio sub-block should be configured in Slave mode
            pub const External: u32 = 0b10;
        }
    }

    /// Clock strobing edge
    pub mod CKSTR {
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

            /// 0b0: Data strobing edge is falling edge of SCK
            pub const FallingEdge: u32 = 0b0;

            /// 0b1: Data strobing edge is rising edge of SCK
            pub const RisingEdge: u32 = 0b1;
        }
    }

    /// Least significant bit first
    pub mod LSBFIRST {
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

            /// 0b0: Data are transferred with MSB first
            pub const MsbFirst: u32 = 0b0;

            /// 0b1: Data are transferred with LSB first
            pub const LsbFirst: u32 = 0b1;
        }
    }

    /// Data size
    pub mod DS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (3 bits: 0b111 << 5)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b010: 8 bits
            pub const Bit8: u32 = 0b010;

            /// 0b011: 10 bits
            pub const Bit10: u32 = 0b011;

            /// 0b100: 16 bits
            pub const Bit16: u32 = 0b100;

            /// 0b101: 20 bits
            pub const Bit20: u32 = 0b101;

            /// 0b110: 24 bits
            pub const Bit24: u32 = 0b110;

            /// 0b111: 32 bits
            pub const Bit32: u32 = 0b111;
        }
    }

    /// Protocol configuration
    pub mod PRTCFG {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Free protocol. Free protocol allows to use the powerful configuration of the audio block to address a specific audio protocol
            pub const Free: u32 = 0b00;

            /// 0b01: SPDIF protocol
            pub const Spdif: u32 = 0b01;

            /// 0b10: AC’97 protocol
            pub const Ac97: u32 = 0b10;
        }
    }

    /// Audio block mode
    pub mod MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Master transmitter
            pub const MasterTx: u32 = 0b00;

            /// 0b01: Master receiver
            pub const MasterRx: u32 = 0b01;

            /// 0b10: Slave transmitter
            pub const SlaveTx: u32 = 0b10;

            /// 0b11: Slave receiver
            pub const SlaveRx: u32 = 0b11;
        }
    }
}

/// AConfiguration register 2
pub mod CR2A {

    /// Companding mode
    pub mod COMP {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No companding algorithm
            pub const NoCompanding: u32 = 0b00;

            /// 0b10: μ-Law algorithm
            pub const MuLaw: u32 = 0b10;

            /// 0b11: A-Law algorithm
            pub const ALaw: u32 = 0b11;
        }
    }

    /// Complement bit
    pub mod CPL {
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

            /// 0b0: 1’s complement representation
            pub const OnesComplement: u32 = 0b0;

            /// 0b1: 2’s complement representation
            pub const TwosComplement: u32 = 0b1;
        }
    }

    /// Mute counter
    pub mod MUTECN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (6 bits: 0x3f << 7)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Mute value
    pub mod MUTEVAL {
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

            /// 0b0: Bit value 0 is sent during the mute mode
            pub const SendZero: u32 = 0b0;

            /// 0b1: Last values are sent during the mute mode
            pub const SendLast: u32 = 0b1;
        }
    }

    /// Mute
    pub mod MUTE {
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

            /// 0b0: No mute mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: Mute mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Tristate management on data line
    pub mod TRIS {
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

    /// FIFO flush
    pub mod FFLUSH {
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

            /// 0b0: No FIFO flush
            pub const NoFlush: u32 = 0b0;

            /// 0b1: FIFO flush. Programming this bit to 1 triggers the FIFO Flush. All the internal FIFO pointers (read and write) are cleared
            pub const Flush: u32 = 0b1;
        }
    }

    /// FIFO threshold
    pub mod FTH {
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

            /// 0b000: FIFO empty
            pub const Empty: u32 = 0b000;

            /// 0b001: 1⁄4 FIFO
            pub const Quarter1: u32 = 0b001;

            /// 0b010: 1⁄2 FIFO
            pub const Quarter2: u32 = 0b010;

            /// 0b011: 3⁄4 FIFO
            pub const Quarter3: u32 = 0b011;

            /// 0b100: FIFO full
            pub const Full: u32 = 0b100;
        }
    }
}

/// AFRCR
pub mod FRCRA {

    /// Frame synchronization offset
    pub mod FSOFF {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: FS is asserted on the first bit of the slot 0
            pub const OnFirst: u32 = 0b0;

            /// 0b1: FS is asserted one bit before the first bit of the slot 0
            pub const BeforeFirst: u32 = 0b1;
        }
    }

    /// Frame synchronization polarity
    pub mod FSPOL {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: FS is active low (falling edge)
            pub const FallingEdge: u32 = 0b0;

            /// 0b1: FS is active high (rising edge)
            pub const RisingEdge: u32 = 0b1;
        }
    }

    /// Frame synchronization definition
    pub mod FSDEF {
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

    /// Frame synchronization active level length
    pub mod FSALL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Frame length
    pub mod FRL {
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

/// ASlot register
pub mod SLOTRA {

    /// Slot enable
    pub mod SLOTEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000000000000: Inactive slot
            pub const Inactive: u32 = 0b0000000000000000;

            /// 0b0000000000000001: Active slot
            pub const Active: u32 = 0b0000000000000001;
        }
    }

    /// Number of slots in an audio frame
    pub mod NBSLOT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Slot size
    pub mod SLOTSZ {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: The slot size is equivalent to the data size (specified in DS\[3:0\] in the SAI_xCR1 register)
            pub const DataSize: u32 = 0b00;

            /// 0b01: 16-bit
            pub const Bit16: u32 = 0b01;

            /// 0b10: 32-bit
            pub const Bit32: u32 = 0b10;
        }
    }

    /// First bit offset
    pub mod FBOFF {
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

/// AInterrupt mask register2
pub mod IMA {

    /// Late frame synchronization detection interrupt enable
    pub mod LFSDETIE {
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

            /// 0b0: Interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Anticipated frame synchronization detection interrupt enable
    pub mod AFSDETIE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LFSDETIE::RW;
    }

    /// Codec not ready interrupt enable
    pub mod CNRDYIE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LFSDETIE::RW;
    }

    /// FIFO request interrupt enable
    pub mod FREQIE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LFSDETIE::RW;
    }

    /// Wrong clock configuration interrupt enable
    pub mod WCKCFGIE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LFSDETIE::RW;
    }

    /// Mute detection interrupt enable
    pub mod MUTEDETIE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LFSDETIE::RW;
    }

    /// Overrun/underrun interrupt enable
    pub mod OVRUDRIE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LFSDETIE::RW;
    }
}

/// AStatus register
pub mod SRA {

    /// FIFO level threshold
    pub mod FLVL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values
        pub mod R {

            /// 0b000: FIFO empty
            pub const Empty: u32 = 0b000;

            /// 0b001: FIFO <= 1⁄4 but not empty
            pub const Quarter1: u32 = 0b001;

            /// 0b010: 1⁄4 < FIFO <= 1⁄2
            pub const Quarter2: u32 = 0b010;

            /// 0b011: 1⁄2 < FIFO <= 3⁄4
            pub const Quarter3: u32 = 0b011;

            /// 0b100: 3⁄4 < FIFO but not full
            pub const Quarter4: u32 = 0b100;

            /// 0b101: FIFO full
            pub const Full: u32 = 0b101;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Late frame synchronization detection
    pub mod LFSDET {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: Frame synchronization signal is not present at the right time
            pub const NoSync: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Anticipated frame synchronization detection
    pub mod AFSDET {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: Frame synchronization signal is detected earlier than expected
            pub const EarlySync: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Codec not ready
    pub mod CNRDY {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: External AC’97 Codec is ready
            pub const Ready: u32 = 0b0;

            /// 0b1: External AC’97 Codec is not ready
            pub const NotReady: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FIFO request
    pub mod FREQ {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No FIFO request
            pub const NoRequest: u32 = 0b0;

            /// 0b1: FIFO request to read or to write the SAI_xDR
            pub const Request: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wrong clock configuration flag. This bit is read only
    pub mod WCKCFG {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Clock configuration is correct
            pub const Correct: u32 = 0b0;

            /// 0b1: Clock configuration does not respect the rule concerning the frame length specification
            pub const Wrong: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Mute detection
    pub mod MUTEDET {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No MUTE detection on the SD input line
            pub const NoMute: u32 = 0b0;

            /// 0b1: MUTE value detected on the SD input line (0 value) for a specified number of consecutive audio frame
            pub const Mute: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overrun / underrun
    pub mod OVRUDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No overrun/underrun error
            pub const NoError: u32 = 0b0;

            /// 0b1: Overrun/underrun error detection
            pub const Overrun: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// AClear flag register
pub mod CLRFRA {

    /// Clear late frame synchronization detection flag
    pub mod CLFSDET {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the LFSDET flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear anticipated frame synchronization detection flag
    pub mod CAFSDET {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the AFSDET flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear codec not ready flag
    pub mod CCNRDY {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the CNRDY flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear wrong clock configuration flag
    pub mod CWCKCFG {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the WCKCFG flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Mute detection flag
    pub mod CMUTEDET {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the MUTEDET flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear overrun / underrun
    pub mod COVRUDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the OVRUDR flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// AData register
pub mod DRA {

    /// Data
    pub mod DATA {
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

/// AConfiguration register 1
pub mod CR1B {
    pub use super::CR1A::CKSTR;
    pub use super::CR1A::DMAEN;
    pub use super::CR1A::DS;
    pub use super::CR1A::LSBFIRST;
    pub use super::CR1A::MCKDIV;
    pub use super::CR1A::MODE;
    pub use super::CR1A::MONO;
    pub use super::CR1A::NODIV;
    pub use super::CR1A::OUTDRIV;
    pub use super::CR1A::PRTCFG;
    pub use super::CR1A::SAIEN;
    pub use super::CR1A::SYNCEN;
}

/// AConfiguration register 2
pub mod CR2B {
    pub use super::CR2A::COMP;
    pub use super::CR2A::CPL;
    pub use super::CR2A::FFLUSH;
    pub use super::CR2A::FTH;
    pub use super::CR2A::MUTE;
    pub use super::CR2A::MUTECN;
    pub use super::CR2A::MUTEVAL;
    pub use super::CR2A::TRIS;
}

/// AFRCR
pub mod FRCRB {
    pub use super::FRCRA::FRL;
    pub use super::FRCRA::FSALL;
    pub use super::FRCRA::FSDEF;
    pub use super::FRCRA::FSOFF;
    pub use super::FRCRA::FSPOL;
}

/// ASlot register
pub mod SLOTRB {
    pub use super::SLOTRA::FBOFF;
    pub use super::SLOTRA::NBSLOT;
    pub use super::SLOTRA::SLOTEN;
    pub use super::SLOTRA::SLOTSZ;
}

/// AInterrupt mask register2
pub mod IMB {
    pub use super::IMA::AFSDETIE;
    pub use super::IMA::CNRDYIE;
    pub use super::IMA::FREQIE;
    pub use super::IMA::LFSDETIE;
    pub use super::IMA::MUTEDETIE;
    pub use super::IMA::OVRUDRIE;
    pub use super::IMA::WCKCFGIE;
}

/// AStatus register
pub mod SRB {
    pub use super::SRA::AFSDET;
    pub use super::SRA::CNRDY;
    pub use super::SRA::FLVL;
    pub use super::SRA::FREQ;
    pub use super::SRA::LFSDET;
    pub use super::SRA::MUTEDET;
    pub use super::SRA::OVRUDR;
    pub use super::SRA::WCKCFG;
}

/// AClear flag register
pub mod CLRFRB {
    pub use super::CLRFRA::CAFSDET;
    pub use super::CLRFRA::CCNRDY;
    pub use super::CLRFRA::CLFSDET;
    pub use super::CLRFRA::CMUTEDET;
    pub use super::CLRFRA::COVRUDR;
    pub use super::CLRFRA::CWCKCFG;
}

/// AData register
pub mod DRB {
    pub use super::DRA::DATA;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 1],

    /// AConfiguration register 1
    pub CR1A: RWRegister<u32>,

    /// AConfiguration register 2
    pub CR2A: RWRegister<u32>,

    /// AFRCR
    pub FRCRA: RWRegister<u32>,

    /// ASlot register
    pub SLOTRA: RWRegister<u32>,

    /// AInterrupt mask register2
    pub IMA: RWRegister<u32>,

    /// AStatus register
    pub SRA: RWRegister<u32>,

    /// AClear flag register
    pub CLRFRA: RWRegister<u32>,

    /// AData register
    pub DRA: RWRegister<u32>,

    /// AConfiguration register 1
    pub CR1B: RWRegister<u32>,

    /// AConfiguration register 2
    pub CR2B: RWRegister<u32>,

    /// AFRCR
    pub FRCRB: RWRegister<u32>,

    /// ASlot register
    pub SLOTRB: RWRegister<u32>,

    /// AInterrupt mask register2
    pub IMB: RWRegister<u32>,

    /// AStatus register
    pub SRB: RWRegister<u32>,

    /// AClear flag register
    pub CLRFRB: RWRegister<u32>,

    /// AData register
    pub DRB: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1A: u32,
    pub CR2A: u32,
    pub FRCRA: u32,
    pub SLOTRA: u32,
    pub IMA: u32,
    pub SRA: u32,
    pub CLRFRA: u32,
    pub DRA: u32,
    pub CR1B: u32,
    pub CR2B: u32,
    pub FRCRB: u32,
    pub SLOTRB: u32,
    pub IMB: u32,
    pub SRB: u32,
    pub CLRFRB: u32,
    pub DRB: u32,
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
