#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Universal synchronous asynchronous receiver transmitter
//!
//! Used by: stm32l4x1, stm32l4x2, stm32l4x3, stm32l4x5, stm32l4x6

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Control register 1
pub mod CR1 {

    /// Word length
    pub mod M1 {
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

            /// 0b0: Use M0 to set the data bits
            pub const M0: u32 = 0b0;

            /// 0b1: 1 start bit, 7 data bits, n stop bits
            pub const Bit7: u32 = 0b1;
        }
    }

    /// Character match interrupt enable
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

            /// 0b0: Interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt is generated when the CMF bit is set in the ISR register
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Mute mode enable
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
            pub const Disabled: u32 = 0b0;

            /// 0b1: Receiver can switch between mute mode and active mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Word length
    pub mod M0 {
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

            /// 0b0: 1 start bit, 8 data bits, n stop bits
            pub const Bit8: u32 = 0b0;

            /// 0b1: 1 start bit, 9 data bits, n stop bits
            pub const Bit9: u32 = 0b1;
        }
    }

    /// Receiver wakeup method
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
            pub const Idle: u32 = 0b0;

            /// 0b1: Address mask
            pub const Address: u32 = 0b1;
        }
    }

    /// Parity control enable
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
            pub const Disabled: u32 = 0b0;

            /// 0b1: Parity control enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Parity selection
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
            pub const Even: u32 = 0b0;

            /// 0b1: Odd parity
            pub const Odd: u32 = 0b1;
        }
    }

    /// PE interrupt enable
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

            /// 0b0: Interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt is generated whenever PE=1 in the ISR register
            pub const Enabled: u32 = 0b1;
        }
    }

    /// interrupt enable
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

            /// 0b0: Interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt is generated whenever TXE=1 in the ISR register
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Transmission complete interrupt enable
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

            /// 0b0: Interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt is generated whenever TC=1 in the ISR register
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RXNE interrupt enable
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

            /// 0b0: Interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register
            pub const Enabled: u32 = 0b1;
        }
    }

    /// IDLE interrupt enable
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

            /// 0b0: Interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt is generated whenever IDLE=1 in the ISR register
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Transmitter enable
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
            pub const Disabled: u32 = 0b0;

            /// 0b1: Transmitter is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Receiver enable
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
            pub const Disabled: u32 = 0b0;

            /// 0b1: Receiver is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// USART enable in Stop mode
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

            /// 0b0: USART not able to wake up the MCU from Stop mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: USART able to wake up the MCU from Stop mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// USART enable
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

            /// 0b0: UART is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: UART is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Driver Enable assertion time
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

    /// Driver Enable de-assertion time
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
}

/// Control register 2
pub mod CR2 {

    /// Most significant bit first
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

            /// 0b0: data is transmitted/received with data bit 0 first, following the start bit
            pub const LSB: u32 = 0b0;

            /// 0b1: data is transmitted/received with MSB (bit 7/8/9) first, following the start bit
            pub const MSB: u32 = 0b1;
        }
    }

    /// Binary data inversion
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

            /// 0b0: Logical data from the data register are send/received in positive/direct logic
            pub const Positive: u32 = 0b0;

            /// 0b1: Logical data from the data register are send/received in negative/inverse logic
            pub const Negative: u32 = 0b1;
        }
    }

    /// TX pin active level inversion
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

            /// 0b0: TX pin signal works using the standard logic levels
            pub const Standard: u32 = 0b0;

            /// 0b1: TX pin signal values are inverted
            pub const Inverted: u32 = 0b1;
        }
    }

    /// RX pin active level inversion
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

            /// 0b0: RX pin signal works using the standard logic levels
            pub const Standard: u32 = 0b0;

            /// 0b1: RX pin signal values are inverted
            pub const Inverted: u32 = 0b1;
        }
    }

    /// Swap TX/RX pins
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
            pub const Standard: u32 = 0b0;

            /// 0b1: The TX and RX pins functions are swapped
            pub const Swapped: u32 = 0b1;
        }
    }

    /// STOP bits
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
            pub const Stop1: u32 = 0b00;

            /// 0b01: 0.5 stop bit
            pub const Stop0p5: u32 = 0b01;

            /// 0b10: 2 stop bit
            pub const Stop2: u32 = 0b10;

            /// 0b11: 1.5 stop bit
            pub const Stop1p5: u32 = 0b11;
        }
    }

    /// Clock enable
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

            /// 0b0: CK pin disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CK pin enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// 7-bit Address Detection/4-bit Address Detection
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
            pub const Bit4: u32 = 0b0;

            /// 0b1: 7-bit address detection
            pub const Bit7: u32 = 0b1;
        }
    }

    /// Address of the USART node
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

    /// Wakeup from Stop mode interrupt enable
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

            /// 0b0: Interrupt is inhibited
            pub const Disabled: u32 = 0b0;

            /// 0b1: An USART interrupt is generated whenever WUF=1 in the ISR register
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Wakeup from Stop mode interrupt flag selection
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

            /// 0b00: WUF active on address match
            pub const Address: u32 = 0b00;

            /// 0b10: WuF active on Start bit detection
            pub const Start: u32 = 0b10;

            /// 0b11: WUF active on RXNE
            pub const RXNE: u32 = 0b11;
        }
    }

    /// Driver enable polarity selection
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

            /// 0b0: DE signal is active high
            pub const High: u32 = 0b0;

            /// 0b1: DE signal is active low
            pub const Low: u32 = 0b1;
        }
    }

    /// Driver enable mode
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

            /// 0b0: DE function is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: The DE signal is output on the RTS pin
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMA Disable on Reception Error
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

            /// 0b0: DMA is not disabled in case of reception error
            pub const NotDisabled: u32 = 0b0;

            /// 0b1: DMA is disabled following a reception error
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Overrun Disable
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

            /// 0b0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data
            pub const Enabled: u32 = 0b0;

            /// 0b1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register
            pub const Disabled: u32 = 0b1;
        }
    }

    /// CTS interrupt enable
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
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt is generated whenever CTSIF=1 in the ISR register
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CTS enable
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
            pub const Disabled: u32 = 0b0;

            /// 0b1: CTS mode enabled, data is only transmitted when the CTS input is asserted
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RTS enable
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
            pub const Disabled: u32 = 0b0;

            /// 0b1: RTS output enabled, data is only requested when there is space in the receive buffer
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMA enable transmitter
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

            /// 0b0: DMA mode is disabled for transmission
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA mode is enabled for transmission
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMA enable receiver
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

            /// 0b0: DMA mode is disabled for reception
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA mode is enabled for reception
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Half-duplex selection
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
            pub const NotSelected: u32 = 0b0;

            /// 0b1: Half duplex mode is selected
            pub const Selected: u32 = 0b1;
        }
    }

    /// Error interrupt enable
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

            /// 0b0: Interrupt is inhibited
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Baud rate register
pub mod BRR {

    /// BRR
    pub mod BRR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (20 bits: 0xfffff << 0)
        pub const mask: u32 = 0xfffff << offset;
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

    /// Receive data flush request
    pub mod RXFRQ {
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

            /// 0b1: clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition
            pub const Discard: u32 = 0b1;
        }
    }

    /// Mute mode request
    pub mod MMRQ {
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

            /// 0b1: Puts the USART in mute mode and sets the RWU flag
            pub const Mute: u32 = 0b1;
        }
    }

    /// Send break request
    pub mod SBKRQ {
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

            /// 0b1: sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available
            pub const Break: u32 = 0b1;
        }
    }
}

/// Interrupt & status register
pub mod ISR {

    /// REACK
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

    /// TEACK
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

    /// WUF
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

    /// RWU
    pub mod RWU {
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

    /// SBKF
    pub mod SBKF {
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

    /// CMF
    pub mod CMF {
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

    /// BUSY
    pub mod BUSY {
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

    /// CTS
    pub mod CTS {
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

    /// CTSIF
    pub mod CTSIF {
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

    /// TXE
    pub mod TXE {
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

    /// TC
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

    /// RXNE
    pub mod RXNE {
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

    /// IDLE
    pub mod IDLE {
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

    /// ORE
    pub mod ORE {
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

    /// NF
    pub mod NF {
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

    /// FE
    pub mod FE {
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

    /// PE
    pub mod PE {
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

/// Interrupt flag clear register
pub mod ICR {

    /// Wakeup from Stop mode clear flag
    pub mod WUCF {
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

            /// 0b1: Clears the WUF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Character match clear flag
    pub mod CMCF {
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

            /// 0b1: Clears the CMF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// CTS clear flag
    pub mod CTSCF {
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

            /// 0b1: Clears the CTSIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Transmission complete clear flag
    pub mod TCCF {
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

            /// 0b1: Clears the TC flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Idle line detected clear flag
    pub mod IDLECF {
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

            /// 0b1: Clears the IDLE flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Overrun error clear flag
    pub mod ORECF {
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

            /// 0b1: Clears the ORE flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Noise detected clear flag
    pub mod NCF {
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

            /// 0b1: Clears the NF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Framing error clear flag
    pub mod FECF {
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

            /// 0b1: Clears the FE flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Parity error clear flag
    pub mod PECF {
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

            /// 0b1: Clears the PE flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
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
#[repr(C)]
pub struct RegisterBlock {
    /// Control register 1
    pub CR1: RWRegister<u32>,

    /// Control register 2
    pub CR2: RWRegister<u32>,

    /// Control register 3
    pub CR3: RWRegister<u32>,

    /// Baud rate register
    pub BRR: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// Request register
    pub RQR: WORegister<u32>,

    /// Interrupt & status register
    pub ISR: RORegister<u32>,

    /// Interrupt flag clear register
    pub ICR: WORegister<u32>,

    /// Receive data register
    pub RDR: RORegister<u32>,

    /// Transmit data register
    pub TDR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1: u32,
    pub CR2: u32,
    pub CR3: u32,
    pub BRR: u32,
    pub RQR: u32,
    pub ISR: u32,
    pub ICR: u32,
    pub RDR: u32,
    pub TDR: u32,
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
