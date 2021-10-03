#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Universal synchronous asynchronous receiver transmitter
//!
//! Used by: stm32g050, stm32g051, stm32g061, stm32g0b0, stm32g0b1, stm32g0c1

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// CR1_FIFO_ENABLED and CR1_FIFO_DISABLED
/// CR1_FIFO_ENABLED: Control register 1
/// CR1_FIFO_DISABLED: Control register 1
pub mod CR1_FIFO {

    /// USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. In Smartcard mode, (SCEN = 1), the SCLK is always available when CLKEN = 1, regardless of the UE bit value.
    pub mod UE {
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

            /// 0b0: USART prescaler and outputs disabled, low-power mode
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .
    pub mod UESM {
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

            /// 0b0: USART not able to wake up the MCU from low-power mode.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART able to wake up the MCU from low-power mode.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Receiver enable This bit enables the receiver. It is set and cleared by software.
    pub mod RE {
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

            /// 0b0: Receiver is disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Receiver is enabled and begins searching for a start bit
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ('0â followed by '1â) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to '1â. To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
    pub mod TE {
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

            /// 0b0: Transmitter is disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Transmitter is enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// IDLE interrupt enable This bit is set and cleared by software.
    pub mod IDLEIE {
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated whenever IDLE = 1 in the USART_ISR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// RXFIFO not empty interrupt enable This bit is set and cleared by software.
    pub mod RXFNEIE {
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated whenever ORE = 1 or RXFNE = 1 in the USART_ISR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Transmission complete interrupt enable This bit is set and cleared by software.
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated whenever TC = 1 in the USART_ISR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// TXFIFO not full interrupt enable This bit is set and cleared by software.
    pub mod TXFNFIE {
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated whenever TXFNF =1 in the USART_ISR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// PE interrupt enable This bit is set and cleared by software.
    pub mod PEIE {
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated whenever PE = 1 in the USART_ISR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
    pub mod PS {
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

            /// 0b0: Even parity
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Odd parity
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if MÂ =Â 1; 8th bit if MÂ =Â 0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
    pub mod PCE {
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

            /// 0b0: Parity control disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Parity control enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
    pub mod WAKE {
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

            /// 0b0: Idle line
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Address mark
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UEÂ =Â 0).
    pub mod M0 {
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

    /// Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.
    pub mod MME {
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

            /// 0b0: Receiver in active mode permanently
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Receiver can switch between Mute mode and active mode.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Character match interrupt enable This bit is set and cleared by software.
    pub mod CMIE {
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated when the CMF bit is set in the USART_ISR register.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Oversampling mode This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.
    pub mod OVER8 {
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

            /// 0b0: Oversampling by 16
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Oversampling by 8
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod DEDT {
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

    /// Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod DEAT {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (5 bits: 0b11111 << 21)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. .
    pub mod RTOIE {
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated when the RTOF bit is set in the USART_ISR register.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// End of Block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
    pub mod EOBIE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated when the EOBF flag is set in the USART_ISR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\] = '00â: 1 start bit, 8 Data bits, n Stop bit M\[1:0\] = '01â: 1 start bit, 9 Data bits, n Stop bit M\[1:0\] = '10â: 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and Auto baud rate (0x7F and 0x55 frames detection) are not supported.
    pub mod M1 {
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

    /// FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.
    pub mod FIFOEN {
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

            /// 0b0: FIFO mode is disabled.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: FIFO mode is enabled.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// TXFIFO empty interrupt enable This bit is set and cleared by software.
    pub mod TXFEIE {
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated when TXFE = 1 in the USART_ISR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// RXFIFO Full interrupt enable This bit is set and cleared by software.
    pub mod RXFFIE {
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated when RXFF = 1 in the USART_ISR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Receive data register not empty This bit is set and cleared by software.
    pub mod RXNEIE {
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated whenever ORE = 1 or RXNE = 1 in the USART_ISR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Transmit data register empty This bit is set and cleared by software.
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated whenever TXE =1 in the USART_ISR register
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// Control register 2
pub mod CR2 {

    /// Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod SLVEN {
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

            /// 0b0: Slave mode disabled.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Slave mode enabled.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// When the DIS_NSS bit is set, the NSS pin input is ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod DIS_NSS {
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

            /// 0b0: SPI slave selection depends on NSS input pin.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: SPI slave is always selected and NSS input pin is ignored.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the USART is disabled (UEÂ =Â 0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\[5:0\] and ADD\[7:0\]) respectively.
    pub mod ADDM7 {
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

            /// 0b0: 4-bit address detection
            pub const B_0x0: u32 = 0b0;

            /// 0b1: 7-bit address detection (in 8-bit data mode)
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod LBDL {
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

            /// 0b0: 10-bit break detection
            pub const B_0x0: u32 = 0b0;

            /// 0b1: 11-bit break detection
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod LBDIE {
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

            /// 0b0: Interrupt is inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: An interrupt is generated whenever LBDF = 1 in the USART_ISR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod LBCL {
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

            /// 0b0: The clock pulse of the last data bit is not output to the SCLK pin
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The clock pulse of the last data bit is output to the SCLK pin
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see and ) This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod CPHA {
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

            /// 0b0: The first clock transition is the first data capture edge
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The second clock transition is the first data capture edge
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Clock polarity This bit enables the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod CPOL {
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

            /// 0b0: Steady low value on SCLK pin outside transmission window
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Steady high value on SCLK pin outside transmission window
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Clock enable This bit enables the user to enable the SCLK pin. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and must be kept at reset value. Refer to . In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: UE = 0 SCEN = 1 GTPR configuration CLKEN= 1 UE = 1
    pub mod CLKEN {
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

            /// 0b0: SCLK pin disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: SCLK pin enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// stop bits These bits are used for programming the stop bits. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
    pub mod STOP {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 1 stop bit
            pub const B_0x0: u32 = 0b00;

            /// 0b01: 0.5 stop bit.
            pub const B_0x1: u32 = 0b01;

            /// 0b10: 2 stop bits
            pub const B_0x2: u32 = 0b10;

            /// 0b11: 1.5 stop bits
            pub const B_0x3: u32 = 0b11;
        }
    }

    /// LIN mode enable This bit is set and cleared by software. The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value. Refer to .
    pub mod LINEN {
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

            /// 0b0: LIN mode disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: LIN mode enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
    pub mod SWAP {
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

            /// 0b0: TX/RX pins are used as defined in standard pinout
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The TX and RX pins functions are swapped. This enables to work in the case of a cross-wired connection to another UART.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
    pub mod RXINV {
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

            /// 0b0: RX pin signal works using the standard logic levels (VDD =1/idle, Gnd = 0/mark)
            pub const B_0x0: u32 = 0b0;

            /// 0b1: RX pin signal values are inverted (VDD =0/mark, Gnd = 1/idle).
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
    pub mod TXINV {
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

            /// 0b0: TX pin signal works using the standard logic levels (VDD =1/idle, Gnd = 0/mark)
            pub const B_0x0: u32 = 0b0;

            /// 0b1: TX pin signal values are inverted (VDD =0/mark, Gnd = 1/idle).
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
    pub mod DATAINV {
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

            /// 0b0: Logical data from the data register are send/received in positive/direct logic. (1 = H, 0 = L)
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Logical data from the data register are send/received in negative/inverse logic. (1 = L, 0 = H). The parity bit is also inverted.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
    pub mod MSBFIRST {
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

            /// 0b0: data is transmitted/received with data bit 0 first, following the start bit.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Auto baud rate enable This bit is set and cleared by software. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to .
    pub mod ABREN {
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

            /// 0b0: Auto baud rate detection is disabled.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Auto baud rate detection is enabled.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Auto baud rate mode These bits are set and cleared by software. This bitfield can only be written when ABREN = 0 or the USART is disabled (UEÂ =Â 0). Note: If DATAINVÂ =Â 1 and/or MSBFIRSTÂ =Â 1 the patterns must be the same on the line, for example 0xAA for MSBFIRST) If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to .
    pub mod ABRMOD {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (2 bits: 0b11 << 21)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Measurement of the start bit is used to detect the baud rate.
            pub const B_0x0: u32 = 0b00;

            /// 0b01: Falling edge to falling edge measurement (the received frame must start with a single bit = 1 and Frame = Start10xxxxxx)
            pub const B_0x1: u32 = 0b01;

            /// 0b10: 0x7F frame detection.
            pub const B_0x2: u32 = 0b10;

            /// 0b11: 0x55 frame detection
            pub const B_0x3: u32 = 0b11;
        }
    }

    /// Receiver timeout enable This bit is set and cleared by software. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register). Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to .
    pub mod RTOEN {
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

            /// 0b0: Receiver timeout feature disabled.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Receiver timeout feature enabled.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Address of the USART node ADD\[7:4\]: These bits give the address of the USART node or a character code to be recognized. They are used to wake up the MCU with 7-bit address mark detection in multiprocessor communication during Mute mode or low-power mode. The MSB of the character sent by the transmitter should be equal to 1. They can also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8-bit) is compared to the ADD\[7:0\] value and CMF flag is set on match. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UEÂ =Â 0). ADD\[3:0\]: These bits give the address of the USART node or a character code to be recognized. They are used for wakeup with address mark detection, in multiprocessor communication during Mute mode or low-power mode. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UEÂ =Â 0).
    pub mod ADD {
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

/// Control register 3
pub mod CR3 {

    /// Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 or UDR = 1 in the USART_ISR register).
    pub mod EIE {
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: interrupt generated when FE = 1 or ORE = 1 or NE = 1 or UDR = 1 (in SPI slave mode) in the USART_ISR register.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod IREN {
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

            /// 0b0: IrDA disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: IrDA enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod IRLP {
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

            /// 0b0: Normal mode
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Low-power mode
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UEÂ =Â 0).
    pub mod HDSEL {
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

            /// 0b0: Half duplex mode is not selected
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Half duplex mode is selected
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Smartcard NACK enable This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
    pub mod NACK {
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

            /// 0b0: NACK transmission in case of parity error is disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: NACK transmission during parity error is enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
    pub mod SCEN {
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

            /// 0b0: Smartcard Mode disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Smartcard Mode enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DMA enable receiver This bit is set/reset by software
    pub mod DMAR {
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

            /// 0b1: DMA mode is enabled for reception
            pub const B_0x1: u32 = 0b1;

            /// 0b0: DMA mode is disabled for reception
            pub const B_0x0: u32 = 0b0;
        }
    }

    /// DMA enable transmitter This bit is set/reset by software
    pub mod DMAT {
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

            /// 0b1: DMA mode is enabled for transmission
            pub const B_0x1: u32 = 0b1;

            /// 0b0: DMA mode is disabled for transmission
            pub const B_0x0: u32 = 0b0;
        }
    }

    /// RTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod RTSE {
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

            /// 0b0: RTS hardware flow control disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// CTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod CTSE {
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

            /// 0b0: CTS hardware flow control disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod CTSIE {
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

            /// 0b0: Interrupt is inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: An interrupt is generated whenever CTSIF = 1 in the USART_ISR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UEÂ =Â 0).
    pub mod ONEBIT {
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

            /// 0b0: Three sample bit method
            pub const B_0x0: u32 = 0b0;

            /// 0b1: One sample bit method
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data
    pub mod OVRDIS {
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

            /// 0b0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error.
    pub mod DDRE {
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

            /// 0b0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data is transferred (used for Smartcard mode).
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE/RXFNE is case FIFO mode is enabled) before clearing the error flag.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. .
    pub mod DEM {
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

            /// 0b0: DE function is disabled.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DE function is enabled. The DE signal is output on the RTS pin.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Driver enable polarity selection This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod DEP {
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

            /// 0b0: DE signal is active high.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DE signal is active low.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UEÂ =Â 0). When the USART is enabled (UEÂ =Â 1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod SCARCNT {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (3 bits: 0b111 << 17)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: retransmission disabled - No automatic retransmission in transmit mode.
            pub const B_0x0: u32 = 0b000;

            /// 0b001: number of automatic retransmission attempts (before signaling error)
            pub const B_0x1: u32 = 0b001;

            /// 0b010: number of automatic retransmission attempts (before signaling error)
            pub const B_0x2: u32 = 0b010;

            /// 0b011: number of automatic retransmission attempts (before signaling error)
            pub const B_0x3: u32 = 0b011;

            /// 0b100: number of automatic retransmission attempts (before signaling error)
            pub const B_0x4: u32 = 0b100;

            /// 0b101: number of automatic retransmission attempts (before signaling error)
            pub const B_0x5: u32 = 0b101;

            /// 0b110: number of automatic retransmission attempts (before signaling error)
            pub const B_0x6: u32 = 0b110;

            /// 0b111: number of automatic retransmission attempts (before signaling error)
            pub const B_0x7: u32 = 0b111;
        }
    }

    /// Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UEÂ =Â 0). If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835.
    pub mod WUS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: WUF active on address match (as defined by ADD\[7:0\] and ADDM7)
            pub const B_0x0: u32 = 0b00;

            /// 0b10: WUF active on start bit detection
            pub const B_0x2: u32 = 0b10;

            /// 0b11: WUF active on RXNE/RXFNE.
            pub const B_0x3: u32 = 0b11;
        }
    }

    /// Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835.
    pub mod WUFIE {
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated whenever WUF = 1 in the USART_ISR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// TXFIFO threshold interrupt enable This bit is set and cleared by software.
    pub mod TXFTIE {
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated when TXFIFO reaches the threshold programmed in TXFTCFG.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
    pub mod TCBGTIE {
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated whenever TCBGT=1 in the USART_ISR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Receive FIFO threshold configuration Remaining combinations: Reserved
    pub mod RXFTCFG {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (3 bits: 0b111 << 25)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Receive FIFO reaches 1/8 of its depth
            pub const B_0x0: u32 = 0b000;

            /// 0b001: Receive FIFO reaches 1/4 of its depth
            pub const B_0x1: u32 = 0b001;

            /// 0b010: Receive FIFO reaches 1/2 of its depth
            pub const B_0x2: u32 = 0b010;

            /// 0b011: Receive FIFO reaches 3/4 of its depth
            pub const B_0x3: u32 = 0b011;

            /// 0b100: Receive FIFO reaches 7/8 of its depth
            pub const B_0x4: u32 = 0b100;

            /// 0b101: Receive FIFO becomes full
            pub const B_0x5: u32 = 0b101;
        }
    }

    /// RXFIFO threshold interrupt enable This bit is set and cleared by software.
    pub mod RXFTIE {
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

            /// 0b0: Interrupt inhibited
            pub const B_0x0: u32 = 0b0;

            /// 0b1: USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// TXFIFO threshold configuration Remaining combinations: Reserved
    pub mod TXFTCFG {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (3 bits: 0b111 << 29)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: TXFIFO reaches 1/8 of its depth
            pub const B_0x0: u32 = 0b000;

            /// 0b001: TXFIFO reaches 1/4 of its depth
            pub const B_0x1: u32 = 0b001;

            /// 0b010: TXFIFO reaches 1/2 of its depth
            pub const B_0x2: u32 = 0b010;

            /// 0b011: TXFIFO reaches 3/4 of its depth
            pub const B_0x3: u32 = 0b011;

            /// 0b100: TXFIFO reaches 7/8 of its depth
            pub const B_0x4: u32 = 0b100;

            /// 0b101: TXFIFO becomes empty
            pub const B_0x5: u32 = 0b101;
        }
    }
}

/// Baud rate register
pub mod BRR {

    /// USART baud rate
    pub mod BRR {
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

/// Guard time and prescaler register
pub mod GTPR {

    /// Prescaler value In IrDA low-power and normal IrDA mode: PSC\[7:0\] = IrDA Normal and Low-Power baud rate PSC\[7:0\] is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\[4:0\]Â =Â Prescaler value PSC\[4:0\] is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 0010Â 0000: Divides the source clock by 32 (IrDA mode) ... 1111Â 1111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: Bits \[7:5\] must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0â when the Smartcard and IrDA modes are not supported. Refer to .
    pub mod PSC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: Reserved - do not program this value
            pub const B_0x0: u32 = 0b00000000;

            /// 0b00000001: Divides the source clock by 1 (IrDA mode) / by 2 (Smarcard mode)
            pub const B_0x1: u32 = 0b00000001;

            /// 0b00000010: Divides the source clock by 2 (IrDA mode) / by 4 (Smartcard mode)
            pub const B_0x2: u32 = 0b00000010;

            /// 0b00000011: Divides the source clock by 3 (IrDA mode) / by 6 (Smartcard mode)
            pub const B_0x3: u32 = 0b00000011;

            /// 0b00011111: Divides the source clock by 31 (IrDA mode) / by 62 (Smartcard mode)
            pub const B_0x1F: u32 = 0b00011111;
        }
    }

    /// Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod GT {
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

/// Receiver timeout register
pub mod RTOR {

    /// Receiver timeout value
    pub mod RTO {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Block Length
    pub mod BLEN {
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

/// Request register
pub mod RQR {

    /// Auto baud rate request Writing 1 to this bit resets the ABRF flag in the USART_ISR and requests an automatic baud rate measurement on the next received data frame. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to .
    pub mod ABRRQ {
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

    /// Send break request Writing 1 to this bit sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available. Note: When the application needs to send the break character following all previously inserted data, including the ones not yet transmitted, the software should wait for the TXE flag assertion before setting the SBKRQ bit.
    pub mod SBKRQ {
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

    /// Mute mode request Writing 1 to this bit puts the USART in Mute mode and resets the RWU flag.
    pub mod MMRQ {
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

    /// Receive data flush request Writing 1 to this bit empties the entire receive FIFO i.e. clears the bit RXFNE. This enables to discard the received data without reading them, and avoid an overrun condition.
    pub mod RXFRQ {
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

    /// Transmit data flush request When FIFO mode is disabled, writing '1â to this bit sets the TXE flag. This enables to discard the transmit data. This bit must be used only in Smartcard mode, when data have not been sent due to errors (NACK) and the FE flag is active in the USART_ISR register. If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. When FIFO is enabled, TXFRQ bit is set to flush the whole FIFO. This sets the TXFE flag (Transmit FIFO empty, bit 23 in the USART_ISR register). Flushing the Transmit FIFO is supported in both UART and Smartcard modes. Note: In FIFO mode, the TXFNF flag is reset during the flush request until TxFIFO is empty in order to ensure that no data are written in the data register.
    pub mod TXFRQ {
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

/// ISR_FIFO_ENABLED and ISR_FIFO_DISABLED
/// ISR_FIFO_ENABLED: Interrupt & status register
/// ISR_FIFO_DISABLED: Interrupt & status register
pub mod ISR_FIFO {

    /// Parity error This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR.
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

            /// 0b0: No parity error
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Parity error
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIEÂ =Â 1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR.
    pub mod FE {
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

            /// 0b0: No Framing error is detected
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Framing error or break character is detected
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NECF bit in the USART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. When the line is noise-free, the NE flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Tolerance of the USART receiver to clock deviation on pageÂ 861). This error is associated with the character in the USART_RDR.
    pub mod NE {
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

            /// 0b0: No noise is detected
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Noise is detected
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USART_RDR register while RXFF = 1. It is cleared by a software, writing 1 to the ORECF, in the USART_ICR register. An interrupt is generated if RXFNEIEÂ =Â 1 or EIE = 1 in the USART_CR1 register. Note: When this bit is set, the USART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the USART_CR3 register.
    pub mod ORE {
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

            /// 0b0: No overrun error
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Overrun error is detected
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIEÂ =Â 1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. Note: The IDLE bit is not set again until the RXFNE bit has been set (i.e. a new idle line occurs). If Mute mode is enabled (MMEÂ =Â 1), IDLE is set if the USART is not mute (RWUÂ =Â 0), whatever the Mute mode selected by the WAKE bit. If RWUÂ =Â 1, IDLE is not set.
    pub mod IDLE {
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

            /// 0b0: No Idle line is detected
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Idle line is detected
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// RXFIFO not empty RXFNE bit is set by hardware when the RXFIFO is not empty, meaning that data can be read from the USART_RDR register. Every read operation from the USART_RDR frees a location in the RXFIFO. RXFNE is cleared when the RXFIFO is empty. The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXFNEIEÂ =Â 1 in the USART_CR1 register.
    pub mod RXFNE {
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

            /// 0b0: Data is not received
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Received data is ready to be read.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Transmission complete This bit indicates that the last data written in the USART_TDR has been transmitted out of the shift register. It is set by hardware when the transmission of a frame containing data is complete and when TXFE is set. An interrupt is generated if TCIEÂ =Â 1 in the USART_CR1 register. TC bit is is cleared by software, by writing 1 to the TCCF in the USART_ICR register or by a write to the USART_TDR register. Note: If TE bit is reset and no transmission is on going, the TC bit is immediately set.
    pub mod TC {
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

            /// 0b0: Transmission is not complete
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Transmission is complete
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// TXFIFO not full TXFNF is set by hardware when TXFIFO is not full meaning that data can be written in the USART_TDR. Every write operation to the USART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the USART_TDR. An interrupt is generated if the TXFNFIE bit =1 in the USART_CR1 register. Note: The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending the flush request (by setting TXFRQ bit), the flag TXFNF should be checked prior to writing in TXFIFO (TXFNF and TXFE are set at the same time). This bit is used during single buffer transmission.
    pub mod TXFNF {
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

            /// 0b0: Transmit FIFO is full
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Transmit FIFO is not full
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value. Refer to .
    pub mod LBDF {
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

            /// 0b0: LIN Break not detected
            pub const B_0x0: u32 = 0b0;

            /// 0b1: LIN break detected
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// CTS interrupt flag This bit is set by hardware when the nCTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIEÂ =Â 1 in the USART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.
    pub mod CTSIF {
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

            /// 0b0: No change occurred on the nCTS status line
            pub const B_0x0: u32 = 0b0;

            /// 0b1: A change occurred on the nCTS status line
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the nCTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.
    pub mod CTS {
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

            /// 0b0: nCTS line set
            pub const B_0x0: u32 = 0b0;

            /// 0b1: nCTS line reset
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIEÂ =Â 1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. Note: If a time equal to the value programmed in RTOR register separates 2 characters, RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8, depending on the oversampling method), RTOF flag is set. The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has already elapsed when RE is set, then RTOF is set. If the USART does not support the Receiver timeout feature, this bit is reserved and kept at reset value.
    pub mod RTOF {
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

            /// 0b0: Timeout value not reached
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Timeout value reached without any data reception
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// End of block flag This bit is set by hardware when a complete block has been received (for example TÂ =Â 1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if the EOBIEÂ =Â 1 in the USART_CR2 register. It is cleared by software, writing 1 to the EOBCF in the USART_ICR register. Note: If Smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to .
    pub mod EOBF {
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

            /// 0b0: End of Block not reached
            pub const B_0x0: u32 = 0b0;

            /// 0b1: End of Block (number of characters) reached
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// SPI slave underrun error flag In slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to .
    pub mod UDR {
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

            /// 0b0: No underrun error
            pub const B_0x0: u32 = 0b0;

            /// 0b1: underrun error
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Auto baud rate error This bit is set by hardware if the baud rate measurement failed (baud rate out of range or character comparison failed) It is cleared by software, by writing 1 to the ABRRQ bit in the USART_CR3 register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value.
    pub mod ABRE {
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

    /// Auto baud rate flag This bit is set by hardware when the automatic baud rate has been set (RXFNE is also set, generating an interrupt if RXFNEIE = 1) or when the auto baud rate operation was completed without success (ABREÂ =Â 1) (ABRE, RXFNE and FE are also set in this case) It is cleared by software, in order to request a new auto baud rate detection, by writing 1 to the ABRRQ in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value.
    pub mod ABRF {
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

    /// Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not).
    pub mod BUSY {
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

            /// 0b0: USART is idle (no reception)
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Reception on going
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Character match flag This bit is set by hardware, when a the character defined by ADD\[7:0\] is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIEÂ =Â 1in the USART_CR1 register.
    pub mod CMF {
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

            /// 0b0: No Character match detected
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Character Match detected
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission.
    pub mod SBKF {
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

            /// 0b0: Break character transmitted
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Break character requested by setting SBKRQ bit in USART_RQR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Receiver wakeup from Mute mode This bit indicates if the USART is in Mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wakeup on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to .
    pub mod RWU {
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

            /// 0b0: Receiver in active mode
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Receiver in Mute mode
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Wakeup from low-power mode flag This bit is set by hardware, when a wakeup event is detected. The event is defined by the WUS bitfield. It is cleared by software, writing a 1 to the WUCF in the USART_ICR register. An interrupt is generated if WUFIEÂ =Â 1 in the USART_CR3 register. Note: When UESM is cleared, WUF flag is also cleared. If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to .
    pub mod WUF {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the USART. It can be used when an idle frame request is generated by writing TEÂ =Â 0, followed by TEÂ =Â 1 in the USART_CR1 register, in order to respect the TEÂ =Â 0 minimum period.
    pub mod TEACK {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the USART. It can be used to verify that the USART is ready for reception before entering low-power mode. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to .
    pub mod REACK {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXFIFO empty This bit is set by hardware when TXFIFO is empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the USART_RQR register. An interrupt is generated if the TXFEIE bit Â =Â 1 (bit 30) in the USART_CR1 register.
    pub mod TXFE {
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

            /// 0b0: TXFIFO not empty.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: TXFIFO empty.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// RXFIFO full This bit is set by hardware when the number of received data corresponds to RXFIFOÂ sizeÂ +Â 1 (RXFIFO full + 1 data in the USART_RDR register. An interrupt is generated if the RXFFIE bit Â =Â 1 in the USART_CR1 register.
    pub mod RXFF {
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

            /// 0b0: RXFIFO not full.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: RXFIFO Full.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Transmission complete before guard time flag This bit is set when the last data written in the USART_TDR has been transmitted correctly out of the shift register. It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if the smartcard did not send back any NACK. An interrupt is generated if TCBGTIEÂ =Â 1 in the USART_CR3 register. This bit is cleared by software, by writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. Note: If the USART does not support the Smartcard mode, this bit is reserved and kept at reset value. If the USART supports the Smartcard mode and the Smartcard mode is enabled, the TCBGT reset value is '1â. Refer to on pageÂ 835.
    pub mod TCBGT {
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

            /// 0b0: Transmission is not complete or transmission is complete unsuccessfully (i.e. a NACK is received from the card)
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Transmission is complete successfully (before Guard time completion and there is no NACK from the smart card).
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// RXFIFO threshold flag This bit is set by hardware when the threshold programmed in RXFTCFG in USART_CR3 register is reached. This means that there are (RXFTCFG - 1) data in the Receive FIFO and one data in the USART_RDR register. An interrupt is generated if the RXFTIE bit Â =Â 1 (bit 27) in the USART_CR3 register. Note: When the RXFTCFG threshold is configured to '101â, RXFT flag is set if 16 data are available i.e. 15 data in the RXFIFO and 1 data in the USART_RDR. Consequently, the 17th received data does not cause an overrun error. The overrun error occurs after receiving the 18th data.
    pub mod RXFT {
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

            /// 0b0: Receive FIFO does not reach the programmed threshold.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Receive FIFO reached the programmed threshold.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the threshold programmed in TXFTCFG of USART_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit Â =Â 1 (bit 31) in the USART_CR3 register.
    pub mod TXFT {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: TXFIFO does not reach the programmed threshold.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: TXFIFO reached the programmed threshold.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Read data register not empty RXNE bit is set by hardware when the content of the USART_RDR shift register has been transferred to the USART_RDR register. It is cleared by reading from the USART_RDR register. The RXNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXNEIEÂ =Â 1 in the USART_CR1 register.
    pub mod RXNE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RXFNE::RW;
    }

    /// Transmit data register empty TXE is set by hardware when the content of the USART_TDR register has been transferred into the shift register. It is cleared by writing to the USART_TDR register. The TXE flag can also be set by writing 1 to the TXFRQ in the USART_RQR register, in order to discard the data (only in Smartcard TÂ =Â 0 mode, in case of transmission failure). An interrupt is generated if the TXEIE bit Â =Â 1 in the USART_CR1 register.
    pub mod TXE {
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

            /// 0b0: Data register full
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Data register not full
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// Interrupt flag clear register
pub mod ICR {

    /// Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register.
    pub mod PECF {
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

    /// Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register.
    pub mod FECF {
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

    /// Noise detected clear flag Writing 1 to this bit clears the NE flag in the USART_ISR register.
    pub mod NECF {
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

    /// Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register.
    pub mod ORECF {
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

    /// Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register.
    pub mod IDLECF {
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

    /// TXFIFO empty clear flag Writing 1 to this bit clears the TXFE flag in the USART_ISR register.
    pub mod TXFECF {
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

    /// Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register.
    pub mod TCCF {
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

    /// Transmission complete before Guard time clear flag Writing 1 to this bit clears the TCBGT flag in the USART_ISR register.
    pub mod TCBGTCF {
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

    /// LIN break detection clear flag Writing 1 to this bit clears the LBDF flag in the USART_ISR register. Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod LBDCF {
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

    /// CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register. Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    pub mod CTSCF {
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

    /// Receiver timeout clear flag Writing 1 to this bit clears the RTOF flag in the USART_ISR register. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835.
    pub mod RTOCF {
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

    /// End of block clear flag Writing 1 to this bit clears the EOBF flag in the USART_ISR register. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
    pub mod EOBCF {
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

    /// SPI slave underrun clear flag Writing 1 to this bit clears the UDRF flag in the USART_ISR register. Note: If the USART does not support SPI slave mode, this bit is reserved and must be kept at reset value. Refer to
    pub mod UDRCF {
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

    /// Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register.
    pub mod CMCF {
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

    /// Wakeup from low-power mode clear flag Writing 1 to this bit clears the WUF flag in the USART_ISR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835.
    pub mod WUCF {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Receive data register
pub mod RDR {

    /// Receive data value
    pub mod RDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmit data register
pub mod TDR {

    /// Transmit data value
    pub mod TDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Prescaler register
pub mod PRESC {

    /// Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.
    pub mod PRESCALER {
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

            /// 0b0000: input clock not divided
            pub const B_0x0: u32 = 0b0000;

            /// 0b0001: input clock divided by 2
            pub const B_0x1: u32 = 0b0001;

            /// 0b0010: input clock divided by 4
            pub const B_0x2: u32 = 0b0010;

            /// 0b0011: input clock divided by 6
            pub const B_0x3: u32 = 0b0011;

            /// 0b0100: input clock divided by 8
            pub const B_0x4: u32 = 0b0100;

            /// 0b0101: input clock divided by 10
            pub const B_0x5: u32 = 0b0101;

            /// 0b0110: input clock divided by 12
            pub const B_0x6: u32 = 0b0110;

            /// 0b0111: input clock divided by 16
            pub const B_0x7: u32 = 0b0111;

            /// 0b1000: input clock divided by 32
            pub const B_0x8: u32 = 0b1000;

            /// 0b1001: input clock divided by 64
            pub const B_0x9: u32 = 0b1001;

            /// 0b1010: input clock divided by 128
            pub const B_0xA: u32 = 0b1010;

            /// 0b1011: input clock divided by 256
            pub const B_0xB: u32 = 0b1011;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// CR1_FIFO_ENABLED and CR1_FIFO_DISABLED
    /// CR1_FIFO_ENABLED: Control register 1
    /// CR1_FIFO_DISABLED: Control register 1
    pub CR1_FIFO: RWRegister<u32>,

    /// Control register 2
    pub CR2: RWRegister<u32>,

    /// Control register 3
    pub CR3: RWRegister<u32>,

    /// Baud rate register
    pub BRR: RWRegister<u32>,

    /// Guard time and prescaler register
    pub GTPR: RWRegister<u32>,

    /// Receiver timeout register
    pub RTOR: RWRegister<u32>,

    /// Request register
    pub RQR: WORegister<u32>,

    /// ISR_FIFO_ENABLED and ISR_FIFO_DISABLED
    /// ISR_FIFO_ENABLED: Interrupt & status register
    /// ISR_FIFO_DISABLED: Interrupt & status register
    pub ISR_FIFO: RWRegister<u32>,

    /// Interrupt flag clear register
    pub ICR: WORegister<u32>,

    /// Receive data register
    pub RDR: RORegister<u32>,

    /// Transmit data register
    pub TDR: RWRegister<u32>,

    /// Prescaler register
    pub PRESC: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1_FIFO: u32,
    pub CR2: u32,
    pub CR3: u32,
    pub BRR: u32,
    pub GTPR: u32,
    pub RTOR: u32,
    pub RQR: u32,
    pub ISR_FIFO: u32,
    pub ICR: u32,
    pub RDR: u32,
    pub TDR: u32,
    pub PRESC: u32,
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
