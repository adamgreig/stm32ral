#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Inter-integrated circuit
//!
//! Used by: stm32f215, stm32f217

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Control register 1
pub mod CR1 {

    /// Software reset
    pub mod SWRST {
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

            /// 0b0: I2C peripheral not under reset
            pub const NotReset: u32 = 0b0;

            /// 0b1: I2C peripheral under reset
            pub const Reset: u32 = 0b1;
        }
    }

    /// SMBus alert
    pub mod ALERT {
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

            /// 0b0: SMBA pin released high
            pub const Release: u32 = 0b0;

            /// 0b1: SMBA pin driven low
            pub const Drive: u32 = 0b1;
        }
    }

    /// Packet error checking
    pub mod PEC {
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

            /// 0b0: No PEC transfer
            pub const Disabled: u32 = 0b0;

            /// 0b1: PEC transfer
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Acknowledge/PEC Position (for data reception)
    pub mod POS {
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

            /// 0b0: ACK bit controls the (N)ACK of the current byte being received
            pub const Current: u32 = 0b0;

            /// 0b1: ACK bit controls the (N)ACK of the next byte to be received
            pub const Next: u32 = 0b1;
        }
    }

    /// Acknowledge enable
    pub mod ACK {
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

            /// 0b0: No acknowledge returned
            pub const NAK: u32 = 0b0;

            /// 0b1: Acknowledge returned after a byte is received
            pub const ACK: u32 = 0b1;
        }
    }

    /// Stop generation
    pub mod STOP {
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

            /// 0b0: No Stop generation
            pub const NoStop: u32 = 0b0;

            /// 0b1: In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte
            pub const Stop: u32 = 0b1;
        }
    }

    /// Start generation
    pub mod START {
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

            /// 0b0: No Start generation
            pub const NoStart: u32 = 0b0;

            /// 0b1: In master mode: repeated start generation, in slave mode: start generation when bus is free
            pub const Start: u32 = 0b1;
        }
    }

    /// Clock stretching disable (Slave mode)
    pub mod NOSTRETCH {
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

            /// 0b0: Clock stretching enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: Clock stretching disabled
            pub const Disabled: u32 = 0b1;
        }
    }

    /// General call enable
    pub mod ENGC {
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

            /// 0b0: General call disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: General call enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PEC enable
    pub mod ENPEC {
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

            /// 0b0: PEC calculation disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: PEC calculation enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ARP enable
    pub mod ENARP {
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

            /// 0b0: ARP disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: ARP enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SMBus type
    pub mod SMBTYPE {
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

            /// 0b0: SMBus Device
            pub const Device: u32 = 0b0;

            /// 0b1: SMBus Host
            pub const Host: u32 = 0b1;
        }
    }

    /// SMBus mode
    pub mod SMBUS {
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

            /// 0b0: I2C Mode
            pub const I2C: u32 = 0b0;

            /// 0b1: SMBus
            pub const SMBus: u32 = 0b1;
        }
    }

    /// Peripheral enable
    pub mod PE {
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

/// Control register 2
pub mod CR2 {

    /// DMA last transfer
    pub mod LAST {
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

            /// 0b0: Next DMA EOT is not the last transfer
            pub const NotLast: u32 = 0b0;

            /// 0b1: Next DMA EOT is the last transfer
            pub const Last: u32 = 0b1;
        }
    }

    /// DMA requests enable
    pub mod DMAEN {
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

            /// 0b0: DMA requests disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA request enabled when TxE=1 or RxNE=1
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Buffer interrupt enable
    pub mod ITBUFEN {
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

            /// 0b0: TxE=1 or RxNE=1 does not generate any interrupt
            pub const Disabled: u32 = 0b0;

            /// 0b1: TxE=1 or RxNE=1 generates Event interrupt
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Event interrupt enable
    pub mod ITEVTEN {
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

            /// 0b0: Event interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Event interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Error interrupt enable
    pub mod ITERREN {
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

            /// 0b0: Error interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Error interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Peripheral clock frequency
    pub mod FREQ {
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
}

/// Own address register 1
pub mod OAR1 {

    /// Addressing mode (slave mode)
    pub mod ADDMODE {
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

            /// 0b0: 7-bit slave address
            pub const ADD7: u32 = 0b0;

            /// 0b1: 10-bit slave address
            pub const ADD10: u32 = 0b1;
        }
    }

    /// Interface address
    pub mod ADD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Own address register 2
pub mod OAR2 {

    /// Interface address
    pub mod ADD2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (7 bits: 0x7f << 1)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Dual addressing mode enable
    pub mod ENDUAL {
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

            /// 0b0: Single addressing mode
            pub const Single: u32 = 0b0;

            /// 0b1: Dual addressing mode
            pub const Dual: u32 = 0b1;
        }
    }
}

/// Data register
pub mod DR {

    /// 8-bit data register
    pub mod DR {
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

/// Status register 1
pub mod SR1 {

    /// SMBus alert
    pub mod SMBALERT {
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

            /// 0b0: No SMBALERT occured
            pub const NoAlert: u32 = 0b0;

            /// 0b1: SMBALERT occurred
            pub const Alert: u32 = 0b1;
        }
    }

    /// Timeout or Tlow error
    pub mod TIMEOUT {
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

            /// 0b0: No Timeout error
            pub const NoTimeout: u32 = 0b0;

            /// 0b1: SCL remained LOW for 25 ms
            pub const Timeout: u32 = 0b1;
        }
    }

    /// PEC Error in reception
    pub mod PECERR {
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

            /// 0b0: no PEC error: receiver returns ACK after PEC reception (if ACK=1)
            pub const NoError: u32 = 0b0;

            /// 0b1: PEC error: receiver returns NACK after PEC reception (whatever ACK)
            pub const Error: u32 = 0b1;
        }
    }

    /// Overrun/Underrun
    pub mod OVR {
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

            /// 0b0: No overrun/underrun occured
            pub const NoOverrun: u32 = 0b0;

            /// 0b1: Overrun/underrun occured
            pub const Overrun: u32 = 0b1;
        }
    }

    /// Acknowledge failure
    pub mod AF {
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

            /// 0b0: No acknowledge failure
            pub const NoFailure: u32 = 0b0;

            /// 0b1: Acknowledge failure
            pub const Failure: u32 = 0b1;
        }
    }

    /// Arbitration lost (master mode)
    pub mod ARLO {
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

            /// 0b0: No Arbitration Lost detected
            pub const NoLost: u32 = 0b0;

            /// 0b1: Arbitration Lost detected
            pub const Lost: u32 = 0b1;
        }
    }

    /// Bus error
    pub mod BERR {
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

            /// 0b0: No misplaced Start or Stop condition
            pub const NoError: u32 = 0b0;

            /// 0b1: Misplaced Start or Stop condition
            pub const Error: u32 = 0b1;
        }
    }

    /// Data register empty (transmitters)
    pub mod TxE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Data register not empty
            pub const NotEmpty: u32 = 0b0;

            /// 0b1: Data register empty
            pub const Empty: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data register not empty (receivers)
    pub mod RxNE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Data register empty
            pub const Empty: u32 = 0b0;

            /// 0b1: Data register not empty
            pub const NotEmpty: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Stop detection (slave mode)
    pub mod STOPF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No Stop condition detected
            pub const NoStop: u32 = 0b0;

            /// 0b1: Stop condition detected
            pub const Stop: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 10-bit header sent (Master mode)
    pub mod ADD10 {
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

    /// Byte transfer finished
    pub mod BTF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Data byte transfer not done
            pub const NotFinished: u32 = 0b0;

            /// 0b1: Data byte transfer successful
            pub const Finished: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Address sent (master mode)/matched (slave mode)
    pub mod ADDR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Adress mismatched or not received
            pub const NotMatch: u32 = 0b0;

            /// 0b1: Received slave address matched with one of the enabled slave addresses
            pub const Match: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Start bit (Master mode)
    pub mod SB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No Start condition
            pub const NoStart: u32 = 0b0;

            /// 0b1: Start condition generated
            pub const Start: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Status register 2
pub mod SR2 {

    /// acket error checking register
    pub mod PEC {
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

    /// Dual flag (Slave mode)
    pub mod DUALF {
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

    /// SMBus host header (Slave mode)
    pub mod SMBHOST {
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

    /// SMBus device default address (Slave mode)
    pub mod SMBDEFAULT {
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

    /// General call address (Slave mode)
    pub mod GENCALL {
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

    /// Transmitter/receiver
    pub mod TRA {
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

    /// Bus busy
    pub mod BUSY {
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

    /// Master/slave
    pub mod MSL {
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
}

/// Clock control register
pub mod CCR {

    /// I2C master mode selection
    pub mod F_S {
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

            /// 0b0: Standard mode I2C
            pub const Standard: u32 = 0b0;

            /// 0b1: Fast mode I2C
            pub const Fast: u32 = 0b1;
        }
    }

    /// Fast mode duty cycle
    pub mod DUTY {
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

            /// 0b0: Duty cycle t_low/t_high = 2/1
            pub const Duty2_1: u32 = 0b0;

            /// 0b1: Duty cycle t_low/t_high = 16/9
            pub const Duty16_9: u32 = 0b1;
        }
    }

    /// Clock control register in Fast/Standard mode (Master mode)
    pub mod CCR {
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

/// TRISE register
pub mod TRISE {

    /// Maximum rise time in Fast/Standard mode (Master mode)
    pub mod TRISE {
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// Control register 1
    pub CR1: RWRegister<u32>,

    /// Control register 2
    pub CR2: RWRegister<u32>,

    /// Own address register 1
    pub OAR1: RWRegister<u32>,

    /// Own address register 2
    pub OAR2: RWRegister<u32>,

    /// Data register
    pub DR: RWRegister<u32>,

    /// Status register 1
    pub SR1: RWRegister<u32>,

    /// Status register 2
    pub SR2: RORegister<u32>,

    /// Clock control register
    pub CCR: RWRegister<u32>,

    /// TRISE register
    pub TRISE: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1: u32,
    pub CR2: u32,
    pub OAR1: u32,
    pub OAR2: u32,
    pub DR: u32,
    pub SR1: u32,
    pub SR2: u32,
    pub CCR: u32,
    pub TRISE: u32,
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
