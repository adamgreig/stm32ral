#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Inter-integrated circuit
//!
//! Used by: stm32g050, stm32g051, stm32g061, stm32g0b0, stm32g0b1, stm32g0c1

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Control register 1
pub mod I2C_CR1 {

    /// Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles.
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

            /// 0b0: Peripheral disable
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Peripheral enable
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// TX Interrupt enable
    pub mod TXIE {
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

            /// 0b0: Transmit (TXIS) interrupt disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Transmit (TXIS) interrupt enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// RX Interrupt enable
    pub mod RXIE {
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

            /// 0b0: Receive (RXNE) interrupt disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Receive (RXNE) interrupt enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Address match Interrupt enable (slave only)
    pub mod ADDRIE {
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

            /// 0b0: Address match (ADDR) interrupts disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Address match (ADDR) interrupts enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Not acknowledge received Interrupt enable
    pub mod NACKIE {
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

            /// 0b0: Not acknowledge (NACKF) received interrupts disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Not acknowledge (NACKF) received interrupts enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Stop detection Interrupt enable
    pub mod STOPIE {
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

            /// 0b0: Stop detection (STOPF) interrupt disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Stop detection (STOPF) interrupt enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)
    pub mod TCIE {
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

            /// 0b0: Transfer Complete interrupt disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Transfer Complete interrupt enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)
    pub mod ERRIE {
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

            /// 0b0: Error detection interrupts disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Error detection interrupts enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\[3:0\] * tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0).
    pub mod DNF {
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

            /// 0b0000: Digital filter disabled
            pub const B_0x0: u32 = 0b0000;

            /// 0b0001: Digital filter enabled and filtering capability up to 1 tI2CCLK
            pub const B_0x1: u32 = 0b0001;

            /// 0b1111: digital filter enabled and filtering capability up to15 tI2CCLK
            pub const B_0xF: u32 = 0b1111;
        }
    }

    /// Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0).
    pub mod ANFOFF {
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

            /// 0b0: Analog noise filter enabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Analog noise filter disabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DMA transmission requests enable
    pub mod TXDMAEN {
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

            /// 0b0: DMA mode disabled for transmission
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DMA mode enabled for transmission
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DMA reception requests enable
    pub mod RXDMAEN {
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

            /// 0b0: DMA mode disabled for reception
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DMA mode enabled for reception
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Slave byte control This bit is used to enable hardware byte control in slave mode.
    pub mod SBC {
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

            /// 0b0: Slave byte control disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Slave byte control enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0).
    pub mod NOSTRETCH {
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

            /// 0b0: Clock stretching enabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Clock stretching disabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to . Note: WUPEN can be set only when DNF = '0000â
    pub mod WUPEN {
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

            /// 0b0: Wakeup from Stop mode disable.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Wakeup from Stop mode enable.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// General call enable
    pub mod GCEN {
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

            /// 0b0: General call disabled. Address 0b00000000 is NACKed.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: General call enabled. Address 0b00000000 is ACKed.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// SMBus Host Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
    pub mod SMBHEN {
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

            /// 0b0: Host Address disabled. Address 0b0001000x is NACKed.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Host Address enabled. Address 0b0001000x is ACKed.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// SMBus Device Default Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
    pub mod SMBDEN {
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

            /// 0b0: Device Default Address disabled. Address 0b1100001x is NACKed.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Device Default Address enabled. Address 0b1100001x is ACKed.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
    pub mod ALERTEN {
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

            /// 0b0: The SMBus alert pin (SMBA) is not supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is released and the Alert Response Address header is disabled (0001100x followed by NACK).
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The SMBus alert pin is supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is driven low and the Alert Response Address header is enabled (0001100x followed by ACK).
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
    pub mod PECEN {
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

            /// 0b0: PEC calculation disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: PEC calculation enabled
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// Control register 2
pub mod I2C_CR2 {

    /// Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\[7:1\] should be written with the 7-bit slave address to be sent. The bits SADD\[9\], SADD\[8\] and SADD\[0\] are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\[9:0\] should be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed.
    pub mod SADD {
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

    /// Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed.
    pub mod RD_WRN {
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

            /// 0b0: Master requests a write transfer.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Master requests a read transfer.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed.
    pub mod ADD10 {
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

            /// 0b0: The master operates in 7-bit addressing mode,
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The master operates in 10-bit addressing mode
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed.
    pub mod HEAD10R {
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

            /// 0b0: The master sends the complete 10 bit slave address read sequence: Start + 2 bytes 10bit address in write direction + Restart + 1st 7 bits of the 10 bit address in read direction.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing '1â to the ADDRCF bit in the I2C_ICR register. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit generates a START condition once the bus is free. Note: Writing '0â to this bit has no effect. The START bit can be set even if the bus is BUSY or I2C is in slave mode. This bit has no effect when RELOAD is set.
    pub mod START {
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

            /// 0b0: No Start generation.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Restart/Start generation:
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In Master Mode: Note: Writing '0â to this bit has no effect.
    pub mod STOP {
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

            /// 0b0: No Stop generation.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Stop generation after current byte transfer.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. Note: Writing '0â to this bit has no effect. This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated whatever the NACK bit value. When hardware PEC checking is enabled (PECBYTE=1), the PEC acknowledge value does not depend on the NACK value.
    pub mod NACK {
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

            /// 0b0: an ACK is sent after current received byte.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: a NACK is sent after current received byte.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Number of bytes The number of bytes to be transmitted/received is programmed there. This field is donât care in slave mode with SBC=0. Note: Changing these bits when the START bit is set is not allowed.
    pub mod NBYTES {
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

    /// NBYTES reload mode This bit is set and cleared by software.
    pub mod RELOAD {
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

            /// 0b0: The transfer is completed after the NBYTES data transfer (STOP or RESTART follows).
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The transfer is not completed after the NBYTES data transfer (NBYTES is reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set.
    pub mod AUTOEND {
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

            /// 0b0: software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. Note: Writing '0â to this bit has no effect. This bit has no effect when RELOAD is set. This bit has no effect is slave mode when SBC=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
    pub mod PECBYTE {
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

            /// 0b0: No PEC transfer.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: PEC transmission/reception is requested
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// Own address register 1
pub mod I2C_OAR1 {

    /// Interface own slave address 7-bit addressing mode: OA1\[7:1\] contains the 7-bit own slave address. The bits OA1\[9\], OA1\[8\] and OA1\[0\] are don't care. 10-bit addressing mode: OA1\[9:0\] contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0.
    pub mod OA1 {
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

    /// Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0.
    pub mod OA1MODE {
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

            /// 0b0: Own address 1 is a 7-bit address.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Own address 1 is a 10-bit address.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Own Address 1 enable
    pub mod OA1EN {
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

            /// 0b0: Own address 1 disabled. The received slave address OA1 is NACKed.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Own address 1 enabled. The received slave address OA1 is ACKed.
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// Own address register 2
pub mod I2C_OAR2 {

    /// Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN=0.
    pub mod OA2 {
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

    /// Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches.
    pub mod OA2MSK {
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

            /// 0b000: No mask
            pub const B_0x0: u32 = 0b000;

            /// 0b001: OA2\[1\] is masked and donât care. Only OA2\[7:2\] are compared.
            pub const B_0x1: u32 = 0b001;

            /// 0b010: OA2\[2:1\] are masked and donât care. Only OA2\[7:3\] are compared.
            pub const B_0x2: u32 = 0b010;

            /// 0b011: OA2\[3:1\] are masked and donât care. Only OA2\[7:4\] are compared.
            pub const B_0x3: u32 = 0b011;

            /// 0b100: OA2\[4:1\] are masked and donât care. Only OA2\[7:5\] are compared.
            pub const B_0x4: u32 = 0b100;

            /// 0b101: OA2\[5:1\] are masked and donât care. Only OA2\[7:6\] are compared.
            pub const B_0x5: u32 = 0b101;

            /// 0b110: OA2\[6:1\] are masked and donât care. Only OA2\[7\] is compared.
            pub const B_0x6: u32 = 0b110;

            /// 0b111: OA2\[7:1\] are masked and donât care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged.
            pub const B_0x7: u32 = 0b111;
        }
    }

    /// Own Address 2 enable
    pub mod OA2EN {
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

            /// 0b0: Own address 2 disabled. The received slave address OA2 is NACKed.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Own address 2 enabled. The received slave address OA2 is ACKed.
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// Timing register
pub mod I2C_TIMINGR {

    /// SCL low period (master mode)
    pub mod SCLL {
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

    /// SCL high period (master mode)
    pub mod SCLH {
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

    /// Data hold time
    pub mod SDADEL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data setup time
    pub mod SCLDEL {
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

    /// Timing prescaler
    pub mod PRESC {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Status register 1
pub mod I2C_TIMEOUTR {

    /// Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0.
    pub mod TIMEOUTA {
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

    /// Idle clock timeout detection Note: This bit can be written only when TIMOUTEN=0.
    pub mod TIDLE {
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

            /// 0b0: TIMEOUTA is used to detect SCL low timeout
            pub const B_0x0: u32 = 0b0;

            /// 0b1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Clock timeout enable
    pub mod TIMOUTEN {
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

            /// 0b0: SCL timeout detection is disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: SCL timeout detection is enabled: when SCL is low for more than tTIMEOUT (TIDLE=0) or high for more than tIDLE (TIDLE=1), a timeout error is detected (TIMEOUT=1).
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0.
    pub mod TIMEOUTB {
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

    /// Extended clock timeout enable
    pub mod TEXTEN {
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

            /// 0b0: Extended clock timeout detection is disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Extended clock timeout detection is enabled. When a cumulative SCL stretch for more than tLOW:EXT is done by the I2C interface, a timeout error is detected (TIMEOUT=1).
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// Interrupt and Status register
pub mod I2C_ISR {

    /// Address match code (Slave mode)
    pub mod ADDCODE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (7 bits: 0x7f << 17)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer direction (Slave mode) This flag is updated when an address match event occurs (ADDR=1).
    pub mod DIR {
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

            /// 0b0: Write transfer, slave enters receiver mode.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Read transfer, slave enters transmitter mode.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Bus busy
    pub mod BUSY {
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
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timeout or t_low detection flag
    pub mod TIMEOUT {
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

    /// PEC Error in reception
    pub mod PECERR {
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

    /// Overrun/Underrun (slave mode)
    pub mod OVR {
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

    /// Arbitration lost
    pub mod ARLO {
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
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transfer Complete Reload
    pub mod TCR {
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

    /// Transfer Complete (master mode)
    pub mod TC {
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

    /// Stop detection flag
    pub mod STOPF {
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

    /// Not acknowledge received flag
    pub mod NACKF {
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

    /// Address matched (slave mode)
    pub mod ADDR {
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

    /// Receive data register not empty (receivers)
    pub mod RXNE {
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

    /// Transmit interrupt status (transmitters)
    pub mod TXIS {
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

    /// Transmit data register empty (transmitters)
    pub mod TXE {
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

/// Interrupt clear register
pub mod I2C_ICR {

    /// Alert flag clear
    pub mod ALERTCF {
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

    /// Timeout detection flag clear
    pub mod TIMOUTCF {
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

    /// PEC Error flag clear
    pub mod PECCF {
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

    /// Overrun/Underrun flag clear
    pub mod OVRCF {
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

    /// Arbitration lost flag clear
    pub mod ARLOCF {
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

    /// Bus error flag clear
    pub mod BERRCF {
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

    /// Stop detection flag clear
    pub mod STOPCF {
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

    /// Not Acknowledge flag clear
    pub mod NACKCF {
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

    /// Address Matched flag clear
    pub mod ADDRCF {
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
}

/// PEC register
pub mod I2C_PECR {

    /// Packet error checking register
    pub mod PEC {
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

/// Receive data register
pub mod I2C_RXDR {

    /// 8-bit receive data
    pub mod RXDATA {
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

/// Transmit data register
pub mod I2C_TXDR {

    /// 8-bit transmit data
    pub mod TXDATA {
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
    /// Control register 1
    pub I2C_CR1: RWRegister<u32>,

    /// Control register 2
    pub I2C_CR2: RWRegister<u32>,

    /// Own address register 1
    pub I2C_OAR1: RWRegister<u32>,

    /// Own address register 2
    pub I2C_OAR2: RWRegister<u32>,

    /// Timing register
    pub I2C_TIMINGR: RWRegister<u32>,

    /// Status register 1
    pub I2C_TIMEOUTR: RWRegister<u32>,

    /// Interrupt and Status register
    pub I2C_ISR: RWRegister<u32>,

    /// Interrupt clear register
    pub I2C_ICR: WORegister<u32>,

    /// PEC register
    pub I2C_PECR: RORegister<u32>,

    /// Receive data register
    pub I2C_RXDR: RORegister<u32>,

    /// Transmit data register
    pub I2C_TXDR: RWRegister<u32>,
}
pub struct ResetValues {
    pub I2C_CR1: u32,
    pub I2C_CR2: u32,
    pub I2C_OAR1: u32,
    pub I2C_OAR2: u32,
    pub I2C_TIMINGR: u32,
    pub I2C_TIMEOUTR: u32,
    pub I2C_ISR: u32,
    pub I2C_ICR: u32,
    pub I2C_PECR: u32,
    pub I2C_RXDR: u32,
    pub I2C_TXDR: u32,
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
