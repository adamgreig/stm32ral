#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Ethernet: DMA controller operation

use crate::{RORegister, RWRegister, UnsafeRWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Ethernet DMA bus mode register
pub mod DMABMR {

    /// Software reset
    pub mod SR {
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

            /// 0b1: Reset all MAC subsystem internal registers and logic. Cleared automatically
            pub const Reset: u32 = 0b1;
        }
    }

    /// DMA arbitration
    pub mod DA {
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

            /// 0b0: Round-robin with Rx:Tx priority given by PM
            pub const RoundRobin: u32 = 0b0;

            /// 0b1: Rx has priority over Tx
            pub const RxPriority: u32 = 0b1;
        }
    }

    /// Descriptor skip length
    pub mod DSL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (5 bits: 0b11111 << 2)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Programmable burst length
    pub mod PBL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000001: Maximum of 1 beat per DMA transaction
            pub const PBL1: u32 = 0b000001;

            /// 0b000010: Maximum of 2 beats per DMA transaction
            pub const PBL2: u32 = 0b000010;

            /// 0b000100: Maximum of 4 beats per DMA transaction
            pub const PBL4: u32 = 0b000100;

            /// 0b001000: Maximum of 8 beats per DMA transaction
            pub const PBL8: u32 = 0b001000;

            /// 0b010000: Maximum of 16 beats per DMA transaction
            pub const PBL16: u32 = 0b010000;

            /// 0b100000: Maximum of 32 beats per DMA transaction
            pub const PBL32: u32 = 0b100000;
        }
    }

    /// Rx-Tx priority ratio
    pub mod PM {
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

            /// 0b00: RxDMA priority over TxDMA is 1:1
            pub const OneToOne: u32 = 0b00;

            /// 0b01: RxDMA priority over TxDMA is 2:1
            pub const TwoToOne: u32 = 0b01;

            /// 0b10: RxDMA priority over TxDMA is 3:1
            pub const ThreeToOne: u32 = 0b10;

            /// 0b11: RxDMA priority over TxDMA is 4:1
            pub const FourToOne: u32 = 0b11;
        }
    }

    /// Fixed burst
    pub mod FB {
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

            /// 0b0: AHB uses SINGLE and INCR burst transfers
            pub const Variable: u32 = 0b0;

            /// 0b1: AHB uses only fixed burst transfers
            pub const Fixed: u32 = 0b1;
        }
    }

    /// Rx DMA PBL
    pub mod RDP {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (6 bits: 0x3f << 17)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000001: 1 beat per RxDMA transaction
            pub const RDP1: u32 = 0b000001;

            /// 0b000010: 2 beats per RxDMA transaction
            pub const RDP2: u32 = 0b000010;

            /// 0b000100: 4 beats per RxDMA transaction
            pub const RDP4: u32 = 0b000100;

            /// 0b001000: 8 beats per RxDMA transaction
            pub const RDP8: u32 = 0b001000;

            /// 0b010000: 16 beats per RxDMA transaction
            pub const RDP16: u32 = 0b010000;

            /// 0b100000: 32 beats per RxDMA transaction
            pub const RDP32: u32 = 0b100000;
        }
    }

    /// Use separate PBL
    pub mod USP {
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

            /// 0b0: PBL value used for both Rx and Tx DMA
            pub const Combined: u32 = 0b0;

            /// 0b1: RxDMA uses RDP value, TxDMA uses PBL value
            pub const Separate: u32 = 0b1;
        }
    }

    /// 4xPBL mode
    pub mod FPM {
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

            /// 0b0: PBL values used as-is
            pub const x1: u32 = 0b0;

            /// 0b1: PBL values multiplied by 4
            pub const x4: u32 = 0b1;
        }
    }

    /// Address-aligned beats
    pub mod AAB {
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

            /// 0b0: Bursts are not aligned
            pub const Unaligned: u32 = 0b0;

            /// 0b1: Align bursts to start address LS bits. First burst alignment depends on FB bit
            pub const Aligned: u32 = 0b1;
        }
    }
}

/// Ethernet DMA transmit poll demand register
pub mod DMATPDR {

    /// Transmit poll demand
    pub mod TPD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: Poll the transmit descriptor list
            pub const Poll: u32 = 0b00000000000000000000000000000000;
        }
    }
}

/// EHERNET DMA receive poll demand register
pub mod DMARPDR {

    /// Receive poll demand
    pub mod RPD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: Poll the receive descriptor list
            pub const Poll: u32 = 0b00000000000000000000000000000000;
        }
    }
}

/// Ethernet DMA receive descriptor list address register
pub mod DMARDLAR {

    /// Start of receive list
    pub mod SRL {
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

/// Ethernet DMA transmit descriptor list address register
pub mod DMATDLAR {

    /// Start of transmit list
    pub mod STL {
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

/// Ethernet DMA status register
pub mod DMASR {

    /// Transmit status
    pub mod TS {
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

    /// Transmit process stopped status
    pub mod TPSS {
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

    /// Transmit buffer unavailable status
    pub mod TBUS {
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

    /// Transmit jabber timeout status
    pub mod TJTS {
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

    /// Receive overflow status
    pub mod ROS {
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

    /// Transmit underflow status
    pub mod TUS {
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

    /// Receive status
    pub mod RS {
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

    /// Receive buffer unavailable status
    pub mod RBUS {
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

    /// Receive process stopped status
    pub mod RPSS {
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

    /// Receive watchdog timeout status
    pub mod PWTS {
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

    /// Early transmit status
    pub mod ETS {
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

    /// Fatal bus error status
    pub mod FBES {
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

    /// Early receive status
    pub mod ERS {
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

    /// Abnormal interrupt summary
    pub mod AIS {
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

    /// Normal interrupt summary
    pub mod NIS {
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

    /// Receive process state
    pub mod RPS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (3 bits: 0b111 << 17)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values
        pub mod R {

            /// 0b000: Stopped, reset or Stop Receive command issued
            pub const Stopped: u32 = 0b000;

            /// 0b001: Running, fetching receive transfer descriptor
            pub const RunningFetching: u32 = 0b001;

            /// 0b011: Running, waiting for receive packet
            pub const RunningWaiting: u32 = 0b011;

            /// 0b100: Suspended, receive descriptor unavailable
            pub const Suspended: u32 = 0b100;

            /// 0b111: Running, writing data to host memory buffer
            pub const RunningWriting: u32 = 0b111;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transmit process state
    pub mod TPS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values
        pub mod R {

            /// 0b000: Stopped, Reset or Stop Transmit command issued
            pub const Stopped: u32 = 0b000;

            /// 0b001: Running, fetching transmit transfer descriptor
            pub const RunningFetching: u32 = 0b001;

            /// 0b010: Running, waiting for status
            pub const RunningWaiting: u32 = 0b010;

            /// 0b011: Running, reading data from host memory buffer
            pub const RunningReading: u32 = 0b011;

            /// 0b110: Suspended, transmit descriptor unavailable or transmit buffer underflow
            pub const Suspended: u32 = 0b110;

            /// 0b111: Running, closing transmit descriptor
            pub const Running: u32 = 0b111;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Error bits status
    pub mod EBS {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (3 bits: 0b111 << 23)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MMC status
    pub mod MMCS {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PMT status
    pub mod PMTS {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Time stamp trigger status
    pub mod TSTS {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Ethernet DMA operation mode register
pub mod DMAOMR {

    /// Start/stop receive
    pub mod SR {
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

            /// 0b0: Reception is stopped after transfer of the current frame
            pub const Stopped: u32 = 0b0;

            /// 0b1: Reception is placed in the Running state
            pub const Started: u32 = 0b1;
        }
    }

    /// Operate on second frame
    pub mod OSF {
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

    /// Receive threshold control
    pub mod RTC {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 64 bytes
            pub const RTC64: u32 = 0b00;

            /// 0b01: 32 bytes
            pub const RTC32: u32 = 0b01;

            /// 0b10: 96 bytes
            pub const RTC96: u32 = 0b10;

            /// 0b11: 128 bytes
            pub const RTC128: u32 = 0b11;
        }
    }

    /// Forward undersized good frames
    pub mod FUGF {
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

            /// 0b0: Rx FIFO drops all frames of less than 64 bytes
            pub const Drop: u32 = 0b0;

            /// 0b1: Rx FIFO forwards undersized frames
            pub const Forward: u32 = 0b1;
        }
    }

    /// Forward error frames
    pub mod FEF {
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

            /// 0b0: Rx FIFO drops frames with error status
            pub const Drop: u32 = 0b0;

            /// 0b1: All frames except runt error frames are forwarded to the DMA
            pub const Forward: u32 = 0b1;
        }
    }

    /// Start/stop transmission
    pub mod ST {
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

            /// 0b0: Transmission is placed in the Stopped state
            pub const Stopped: u32 = 0b0;

            /// 0b1: Transmission is placed in Running state
            pub const Started: u32 = 0b1;
        }
    }

    /// Transmit threshold control
    pub mod TTC {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (3 bits: 0b111 << 14)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 64 bytes
            pub const TTC64: u32 = 0b000;

            /// 0b001: 128 bytes
            pub const TTC128: u32 = 0b001;

            /// 0b010: 192 bytes
            pub const TTC192: u32 = 0b010;

            /// 0b011: 256 bytes
            pub const TTC256: u32 = 0b011;

            /// 0b100: 40 bytes
            pub const TTC40: u32 = 0b100;

            /// 0b101: 32 bytes
            pub const TTC32: u32 = 0b101;

            /// 0b110: 24 bytes
            pub const TTC24: u32 = 0b110;

            /// 0b111: 16 bytes
            pub const TTC16: u32 = 0b111;
        }
    }

    /// Flush transmit FIFO
    pub mod FTF {
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

            /// 0b1: Transmit FIFO controller logic is reset to its default values. Cleared automatically
            pub const Flush: u32 = 0b1;
        }
    }

    /// Transmit store and forward
    pub mod TSF {
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

            /// 0b0: Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold
            pub const CutThrough: u32 = 0b0;

            /// 0b1: Transmission starts when a full frame is in the Tx FIFO
            pub const StoreForward: u32 = 0b1;
        }
    }

    /// Disable flushing of received frames
    pub mod DFRF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive store and forward
    pub mod RSF {
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

            /// 0b0: Rx FIFO operates in cut-through mode, subject to RTC bits
            pub const CutThrough: u32 = 0b0;

            /// 0b1: Frames are read from Rx FIFO after complete frame has been written
            pub const StoreForward: u32 = 0b1;
        }
    }

    /// Dropping of TCP/IP checksum error frames disable
    pub mod DTCEFD {
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

            /// 0b0: Drop frames with errors only in the receive checksum offload engine
            pub const Enabled: u32 = 0b0;

            /// 0b1: Do not drop frames that only have errors in the receive checksum offload engine
            pub const Disabled: u32 = 0b1;
        }
    }
}

/// Ethernet DMA interrupt enable register
pub mod DMAIER {

    /// Transmit interrupt enable
    pub mod TIE {
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

    /// Transmit process stopped interrupt enable
    pub mod TPSIE {
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

    /// Transmit buffer unavailable interrupt enable
    pub mod TBUIE {
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

    /// Transmit jabber timeout interrupt enable
    pub mod TJTIE {
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

    /// Receive overflow interrupt enable
    pub mod ROIE {
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

    /// Transmit underflow interrupt enable
    pub mod TUIE {
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

    /// Receive interrupt enable
    pub mod RIE {
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

    /// Receive buffer unavailable interrupt enable
    pub mod RBUIE {
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

    /// Receive process stopped interrupt enable
    pub mod RPSIE {
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

    /// Receive watchdog timeout interrupt enable
    pub mod RWTIE {
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

    /// Early transmit interrupt enable
    pub mod ETIE {
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

    /// Fatal bus error interrupt enable
    pub mod FBEIE {
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

    /// Early receive interrupt enable
    pub mod ERIE {
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

    /// Abnormal interrupt summary enable
    pub mod AISE {
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

    /// Normal interrupt summary enable
    pub mod NISE {
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

/// Ethernet DMA missed frame and buffer overflow counter register
pub mod DMAMFBOCR {

    /// Missed frames by the controller
    pub mod MFC {
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

    /// Overflow bit for missed frame counter
    pub mod OMFC {
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

    /// Missed frames by the application
    pub mod MFA {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (11 bits: 0x7ff << 17)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overflow bit for FIFO overflow counter
    pub mod OFOC {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Ethernet DMA current host transmit descriptor register
pub mod DMACHTDR {

    /// Host transmit descriptor address pointer
    pub mod HTDAP {
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

/// Ethernet DMA current host receive descriptor register
pub mod DMACHRDR {

    /// Host receive descriptor address pointer
    pub mod HRDAP {
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

/// Ethernet DMA current host transmit buffer address register
pub mod DMACHTBAR {

    /// Host transmit buffer address pointer
    pub mod HTBAP {
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

/// Ethernet DMA current host receive buffer address register
pub mod DMACHRBAR {

    /// Host receive buffer address pointer
    pub mod HRBAP {
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
#[repr(C)]
pub struct RegisterBlock {
    /// Ethernet DMA bus mode register
    pub DMABMR: RWRegister<u32>,

    /// Ethernet DMA transmit poll demand register
    pub DMATPDR: RWRegister<u32>,

    /// EHERNET DMA receive poll demand register
    pub DMARPDR: RWRegister<u32>,

    /// Ethernet DMA receive descriptor list address register
    pub DMARDLAR: UnsafeRWRegister<u32>,

    /// Ethernet DMA transmit descriptor list address register
    pub DMATDLAR: UnsafeRWRegister<u32>,

    /// Ethernet DMA status register
    pub DMASR: RWRegister<u32>,

    /// Ethernet DMA operation mode register
    pub DMAOMR: RWRegister<u32>,

    /// Ethernet DMA interrupt enable register
    pub DMAIER: RWRegister<u32>,

    /// Ethernet DMA missed frame and buffer overflow counter register
    pub DMAMFBOCR: RORegister<u32>,

    _reserved1: [u8; 36],

    /// Ethernet DMA current host transmit descriptor register
    pub DMACHTDR: RORegister<u32>,

    /// Ethernet DMA current host receive descriptor register
    pub DMACHRDR: RORegister<u32>,

    /// Ethernet DMA current host transmit buffer address register
    pub DMACHTBAR: RORegister<u32>,

    /// Ethernet DMA current host receive buffer address register
    pub DMACHRBAR: RORegister<u32>,
}
pub struct ResetValues {
    pub DMABMR: u32,
    pub DMATPDR: u32,
    pub DMARPDR: u32,
    pub DMARDLAR: u32,
    pub DMATDLAR: u32,
    pub DMASR: u32,
    pub DMAOMR: u32,
    pub DMAIER: u32,
    pub DMAMFBOCR: u32,
    pub DMACHTDR: u32,
    pub DMACHRDR: u32,
    pub DMACHTBAR: u32,
    pub DMACHRBAR: u32,
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

/// Access functions for the Ethernet_DMA peripheral instance
pub mod Ethernet_DMA {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40029000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in Ethernet_DMA
    pub const reset: ResetValues = ResetValues {
        DMABMR: 0x00020101,
        DMATPDR: 0x00000000,
        DMARPDR: 0x00000000,
        DMARDLAR: 0x00000000,
        DMATDLAR: 0x00000000,
        DMASR: 0x00000000,
        DMAOMR: 0x00000000,
        DMAIER: 0x00000000,
        DMAMFBOCR: 0x00000000,
        DMACHTDR: 0x00000000,
        DMACHRDR: 0x00000000,
        DMACHTBAR: 0x00000000,
        DMACHRBAR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut Ethernet_DMA_TAKEN: bool = false;

    /// Safe access to Ethernet_DMA
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
            if Ethernet_DMA_TAKEN {
                None
            } else {
                Ethernet_DMA_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to Ethernet_DMA
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if Ethernet_DMA_TAKEN && inst.addr == INSTANCE.addr {
                Ethernet_DMA_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal Ethernet_DMA
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        Ethernet_DMA_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to Ethernet_DMA
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const Ethernet_DMA: *const RegisterBlock = 0x40029000 as *const _;
