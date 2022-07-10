#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA request multiplexer
//!
//! Used by: stm32wl5x_cm0p, stm32wl5x_cm4, stm32wle5

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// request line multiplexer channel x configuration register
pub mod C0CR {

    /// Synchronization identification
    pub mod SYNC_ID {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Signal `EXTIx` selected as synchronization input
            pub const exti0: u32 = 0b00000;

            /// 0b00001: Signal `EXTIx` selected as synchronization input
            pub const exti1: u32 = 0b00001;

            /// 0b00010: Signal `EXTIx` selected as synchronization input
            pub const exti2: u32 = 0b00010;

            /// 0b00011: Signal `EXTIx` selected as synchronization input
            pub const exti3: u32 = 0b00011;

            /// 0b00100: Signal `EXTIx` selected as synchronization input
            pub const exti4: u32 = 0b00100;

            /// 0b00101: Signal `EXTIx` selected as synchronization input
            pub const exti5: u32 = 0b00101;

            /// 0b00110: Signal `EXTIx` selected as synchronization input
            pub const exti6: u32 = 0b00110;

            /// 0b00111: Signal `EXTIx` selected as synchronization input
            pub const exti7: u32 = 0b00111;

            /// 0b01000: Signal `EXTIx` selected as synchronization input
            pub const exti8: u32 = 0b01000;

            /// 0b01001: Signal `EXTIx` selected as synchronization input
            pub const exti9: u32 = 0b01001;

            /// 0b01010: Signal `EXTIx` selected as synchronization input
            pub const exti10: u32 = 0b01010;

            /// 0b01011: Signal `EXTIx` selected as synchronization input
            pub const exti11: u32 = 0b01011;

            /// 0b01100: Signal `EXTIx` selected as synchronization input
            pub const exti12: u32 = 0b01100;

            /// 0b01101: Signal `EXTIx` selected as synchronization input
            pub const exti13: u32 = 0b01101;

            /// 0b01110: Signal `EXTIx` selected as synchronization input
            pub const exti14: u32 = 0b01110;

            /// 0b01111: Signal `EXTIx` selected as synchronization input
            pub const exti15: u32 = 0b01111;

            /// 0b10000: Signal `dmamux1_evt0` selected as synchronization input
            pub const dmamux1_evt0: u32 = 0b10000;

            /// 0b10001: Signal `dmamux1_evt1` selected as synchronization input
            pub const dmamux1_evt1: u32 = 0b10001;

            /// 0b10010: Signal `lptim1_out` selected as synchronization input
            pub const lptim1_out: u32 = 0b10010;

            /// 0b10011: Signal `lptim2_out` selected as synchronization input
            pub const lptim2_out: u32 = 0b10011;

            /// 0b10100: Signal `lptim3_out` selected as synchronization input
            pub const lptim3_out: u32 = 0b10100;
        }
    }

    /// Number of DMA requests minus 1 to forward
    pub mod NBREQ {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (5 bits: 0b11111 << 19)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Synchronization polarity
    pub mod SPOL {
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

            /// 0b00: No event, i.e. no synchronization nor detection
            pub const NoEdge: u32 = 0b00;

            /// 0b01: Rising edge
            pub const RisingEdge: u32 = 0b01;

            /// 0b10: Falling edge
            pub const FallingEdge: u32 = 0b10;

            /// 0b11: Rising and falling edges
            pub const BothEdges: u32 = 0b11;
        }
    }

    /// Synchronization enable
    pub mod SE {
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

            /// 0b0: Synchronization disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Synchronization enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Event generation enable
    pub mod EGE {
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

            /// 0b0: Event generation disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Event generation enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Synchronization overrun interrupt enable
    pub mod SOIE {
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

            /// 0b0: Synchronization overrun interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Synchronization overrun interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMA request identification
    pub mod DMAREQ_ID {
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

            /// 0b00000000: No signal selected as request input
            pub const none: u32 = 0b00000000;

            /// 0b00000001: Signal `dmamux1_req_gen0` selected as request input
            pub const dmamux1_req_gen0: u32 = 0b00000001;

            /// 0b00000010: Signal `dmamux1_req_gen1` selected as request input
            pub const dmamux1_req_gen1: u32 = 0b00000010;

            /// 0b00000011: Signal `dmamux1_req_gen2` selected as request input
            pub const dmamux1_req_gen2: u32 = 0b00000011;

            /// 0b00000100: Signal `dmamux1_req_gen3` selected as request input
            pub const dmamux1_req_gen3: u32 = 0b00000100;

            /// 0b00000101: Signal `adc1_dma` selected as request input
            pub const adc: u32 = 0b00000101;

            /// 0b00000110: Signal `dac_out1_dma` selected as request input
            pub const dat_out1: u32 = 0b00000110;

            /// 0b00000111: Signal `spi1_rx_dma` selected as request input
            pub const spi1_rx_dma: u32 = 0b00000111;

            /// 0b00001000: Signal `spi1_tx_dma` selected as request input
            pub const spi1_tx_dma: u32 = 0b00001000;

            /// 0b00001001: Signal `spi2_rx_dma` selected as request input
            pub const spi2_rx_dma: u32 = 0b00001001;

            /// 0b00001010: Signal `spi2_tx_dma` selected as request input
            pub const spi2_tx_dma: u32 = 0b00001010;

            /// 0b00001011: Signal `i2c1_rx_dma` selected as request input
            pub const i2c1_rx_dma: u32 = 0b00001011;

            /// 0b00001100: Signal `i2c1_tx_dma` selected as request input
            pub const i2c1_tx_dma: u32 = 0b00001100;

            /// 0b00001101: Signal `i2c2_rx_dma` selected as request input
            pub const i2c2_rx_dma: u32 = 0b00001101;

            /// 0b00001110: Signal `i2c2_tx_dma` selected as request input
            pub const i2c2_tx_dma: u32 = 0b00001110;

            /// 0b00001111: Signal `i2c3_rx_dma` selected as request input
            pub const i2c3_rx_dma: u32 = 0b00001111;

            /// 0b00010000: Signal `i2c3_tx_dma` selected as request input
            pub const i2c3_tx_dma: u32 = 0b00010000;

            /// 0b00010001: Signal `usart1_rx_dma` selected as request input
            pub const usart1_rx_dma: u32 = 0b00010001;

            /// 0b00010010: Signal `usart1_tx_dma` selected as request input
            pub const usart1_tx_dma: u32 = 0b00010010;

            /// 0b00010011: Signal `usart2_rx_dma` selected as request input
            pub const usart2_rx_dma: u32 = 0b00010011;

            /// 0b00010100: Signal `usart2_tx_dma` selected as request input
            pub const usart2_tx_dma: u32 = 0b00010100;

            /// 0b00010101: Signal `lpuart1_rx_dma` selected as request input
            pub const lpuart1_rx_dma: u32 = 0b00010101;

            /// 0b00010110: Signal `lpuart1_tx_dma` selected as request input
            pub const lpuart1_tx_dma: u32 = 0b00010110;

            /// 0b00010111: Signal `tim1_ch1` selected as request input
            pub const tim1_ch1: u32 = 0b00010111;

            /// 0b00011000: Signal `tim1_ch2` selected as request input
            pub const tim1_ch2: u32 = 0b00011000;

            /// 0b00011001: Signal `tim1_ch3` selected as request input
            pub const tim1_ch3: u32 = 0b00011001;

            /// 0b00011010: Signal `tim1_ch4` selected as request input
            pub const tim1_ch4: u32 = 0b00011010;

            /// 0b00011011: Signal `tim1_up` selected as request input
            pub const tim1_up: u32 = 0b00011011;

            /// 0b00011100: Signal `tim1_trig` selected as request input
            pub const tim1_trig: u32 = 0b00011100;

            /// 0b00011101: Signal `tim1_com` selected as request input
            pub const tim1_com: u32 = 0b00011101;

            /// 0b00011110: Signal `tim2_ch1` selected as request input
            pub const tim2_ch1: u32 = 0b00011110;

            /// 0b00011111: Signal `tim2_ch2` selected as request input
            pub const tim2_ch2: u32 = 0b00011111;

            /// 0b00100000: Signal `tim2_ch3` selected as request input
            pub const tim2_ch3: u32 = 0b00100000;

            /// 0b00100001: Signal `tim2_ch4` selected as request input
            pub const tim2_ch4: u32 = 0b00100001;

            /// 0b00100010: Signal `tim2_up` selected as request input
            pub const tim2_up: u32 = 0b00100010;

            /// 0b00100011: Signal `tim16_ch1` selected as request input
            pub const tim16_ch1: u32 = 0b00100011;

            /// 0b00100100: Signal `tim16_up` selected as request input
            pub const tim16_up: u32 = 0b00100100;

            /// 0b00100101: Signal `tim17_ch1` selected as request input
            pub const tim17_ch1: u32 = 0b00100101;

            /// 0b00100110: Signal `tim17_up` selected as request input
            pub const tim17_up: u32 = 0b00100110;

            /// 0b00100111: Signal `aes_in` selected as request input
            pub const aes_in: u32 = 0b00100111;

            /// 0b00101000: Signal `aes_out` selected as request input
            pub const aes_out: u32 = 0b00101000;

            /// 0b00101001: Signal `subghzspi_rx` selected as request input
            pub const subghzspi_rx: u32 = 0b00101001;

            /// 0b00101010: Signal `subghzspi_tx` selected as request input
            pub const subghzspi_tx: u32 = 0b00101010;
        }
    }
}

/// request line multiplexer channel x configuration register
pub mod C1CR {
    pub use super::C0CR::DMAREQ_ID;
    pub use super::C0CR::EGE;
    pub use super::C0CR::NBREQ;
    pub use super::C0CR::SE;
    pub use super::C0CR::SOIE;
    pub use super::C0CR::SPOL;
    pub use super::C0CR::SYNC_ID;
}

/// request line multiplexer channel x configuration register
pub mod C2CR {
    pub use super::C0CR::DMAREQ_ID;
    pub use super::C0CR::EGE;
    pub use super::C0CR::NBREQ;
    pub use super::C0CR::SE;
    pub use super::C0CR::SOIE;
    pub use super::C0CR::SPOL;
    pub use super::C0CR::SYNC_ID;
}

/// request line multiplexer channel x configuration register
pub mod C3CR {
    pub use super::C0CR::DMAREQ_ID;
    pub use super::C0CR::EGE;
    pub use super::C0CR::NBREQ;
    pub use super::C0CR::SE;
    pub use super::C0CR::SOIE;
    pub use super::C0CR::SPOL;
    pub use super::C0CR::SYNC_ID;
}

/// request line multiplexer channel x configuration register
pub mod C4CR {
    pub use super::C0CR::DMAREQ_ID;
    pub use super::C0CR::EGE;
    pub use super::C0CR::NBREQ;
    pub use super::C0CR::SE;
    pub use super::C0CR::SOIE;
    pub use super::C0CR::SPOL;
    pub use super::C0CR::SYNC_ID;
}

/// request line multiplexer channel x configuration register
pub mod C5CR {
    pub use super::C0CR::DMAREQ_ID;
    pub use super::C0CR::EGE;
    pub use super::C0CR::NBREQ;
    pub use super::C0CR::SE;
    pub use super::C0CR::SOIE;
    pub use super::C0CR::SPOL;
    pub use super::C0CR::SYNC_ID;
}

/// request line multiplexer channel x configuration register
pub mod C6CR {
    pub use super::C0CR::DMAREQ_ID;
    pub use super::C0CR::EGE;
    pub use super::C0CR::NBREQ;
    pub use super::C0CR::SE;
    pub use super::C0CR::SOIE;
    pub use super::C0CR::SPOL;
    pub use super::C0CR::SYNC_ID;
}

/// request line multiplexer channel x configuration register
pub mod C7CR {
    pub use super::C0CR::DMAREQ_ID;
    pub use super::C0CR::EGE;
    pub use super::C0CR::NBREQ;
    pub use super::C0CR::SE;
    pub use super::C0CR::SOIE;
    pub use super::C0CR::SPOL;
    pub use super::C0CR::SYNC_ID;
}

/// request line multiplexer channel x configuration register
pub mod C8CR {
    pub use super::C0CR::DMAREQ_ID;
    pub use super::C0CR::EGE;
    pub use super::C0CR::NBREQ;
    pub use super::C0CR::SE;
    pub use super::C0CR::SOIE;
    pub use super::C0CR::SPOL;
    pub use super::C0CR::SYNC_ID;
}

/// request line multiplexer channel x configuration register
pub mod C9CR {
    pub use super::C0CR::DMAREQ_ID;
    pub use super::C0CR::EGE;
    pub use super::C0CR::NBREQ;
    pub use super::C0CR::SE;
    pub use super::C0CR::SOIE;
    pub use super::C0CR::SPOL;
    pub use super::C0CR::SYNC_ID;
}

/// C10CR
pub mod C10CR {

    /// SYNC_ID
    pub mod SYNC_ID {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NBREQ
    pub mod NBREQ {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (5 bits: 0b11111 << 19)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SPOL
    pub mod SPOL {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (2 bits: 0b11 << 17)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SE
    pub mod SE {
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

    /// EGE
    pub mod EGE {
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

    /// SOIE
    pub mod SOIE {
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

    /// DMAREQ_ID
    pub mod DMAREQ_ID {
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

/// C11CR
pub mod C11CR {
    pub use super::C10CR::DMAREQ_ID;
    pub use super::C10CR::EGE;
    pub use super::C10CR::NBREQ;
    pub use super::C10CR::SE;
    pub use super::C10CR::SOIE;
    pub use super::C10CR::SPOL;
    pub use super::C10CR::SYNC_ID;
}

/// C12CR
pub mod C12CR {
    pub use super::C10CR::DMAREQ_ID;
    pub use super::C10CR::EGE;
    pub use super::C10CR::NBREQ;
    pub use super::C10CR::SE;
    pub use super::C10CR::SOIE;
    pub use super::C10CR::SPOL;
    pub use super::C10CR::SYNC_ID;
}

/// C13CR
pub mod C13CR {
    pub use super::C10CR::DMAREQ_ID;
    pub use super::C10CR::EGE;
    pub use super::C10CR::NBREQ;
    pub use super::C10CR::SE;
    pub use super::C10CR::SOIE;
    pub use super::C10CR::SPOL;
    pub use super::C10CR::SYNC_ID;
}

/// request line multiplexer interrupt channel status register
pub mod CSR {

    /// Synchronization overrun event flag
    pub mod SOF13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ
            pub const NoSyncEvent: u32 = 0b0;

            /// 0b1: Synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ
            pub const SyncEvent: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF12
    pub mod SOF12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        pub use super::SOF13::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF11
    pub mod SOF11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        pub use super::SOF13::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF10
    pub mod SOF10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        pub use super::SOF13::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF9
    pub mod SOF9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        pub use super::SOF13::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF8
    pub mod SOF8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        pub use super::SOF13::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF7
    pub mod SOF7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        pub use super::SOF13::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF6
    pub mod SOF6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        pub use super::SOF13::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF5
    pub mod SOF5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        pub use super::SOF13::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF4
    pub mod SOF4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::SOF13::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF3
    pub mod SOF3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::SOF13::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF2
    pub mod SOF2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::SOF13::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF1
    pub mod SOF1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::SOF13::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF0
    pub mod SOF0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        pub use super::SOF13::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// request line multiplexer interrupt channel clear flag register
pub mod CCFR {

    /// CSOF13
    pub mod CSOF13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear synchronization flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSOF12
    pub mod CSOF12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSOF13::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSOF11
    pub mod CSOF11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSOF13::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSOF10
    pub mod CSOF10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSOF13::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSOF9
    pub mod CSOF9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSOF13::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSOF8
    pub mod CSOF8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSOF13::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSOF7
    pub mod CSOF7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSOF13::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSOF6
    pub mod CSOF6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSOF13::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSOF5
    pub mod CSOF5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSOF13::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSOF4
    pub mod CSOF4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSOF13::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSOF3
    pub mod CSOF3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSOF13::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSOF2
    pub mod CSOF2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSOF13::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSOF1
    pub mod CSOF1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSOF13::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSOF0
    pub mod CSOF0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSOF13::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// request generator channel x configuration register
pub mod RG0CR {

    /// Number of DMA requests to be generated (minus 1)
    pub mod GNBREQ {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (5 bits: 0b11111 << 19)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA request generator trigger polarity
    pub mod GPOL {
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

            /// 0b00: No event, i.e. no detection nor generation
            pub const NoEdge: u32 = 0b00;

            /// 0b01: Rising edge
            pub const RisingEdge: u32 = 0b01;

            /// 0b10: Falling edge
            pub const FallingEdge: u32 = 0b10;

            /// 0b11: Rising and falling edges
            pub const BothEdges: u32 = 0b11;
        }
    }

    /// DMA request generator channel x enable
    pub mod GE {
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

            /// 0b0: DMA request generation disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Trigger overrun interrupt enable
    pub mod OIE {
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

            /// 0b0: Trigger overrun interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Trigger overrun interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Signal identification
    pub mod SIG_ID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Signal `EXTIx` selected as synchronization input
            pub const exti0: u32 = 0b00000;

            /// 0b00001: Signal `EXTIx` selected as synchronization input
            pub const exti1: u32 = 0b00001;

            /// 0b00010: Signal `EXTIx` selected as synchronization input
            pub const exti2: u32 = 0b00010;

            /// 0b00011: Signal `EXTIx` selected as synchronization input
            pub const exti3: u32 = 0b00011;

            /// 0b00100: Signal `EXTIx` selected as synchronization input
            pub const exti4: u32 = 0b00100;

            /// 0b00101: Signal `EXTIx` selected as synchronization input
            pub const exti5: u32 = 0b00101;

            /// 0b00110: Signal `EXTIx` selected as synchronization input
            pub const exti6: u32 = 0b00110;

            /// 0b00111: Signal `EXTIx` selected as synchronization input
            pub const exti7: u32 = 0b00111;

            /// 0b01000: Signal `EXTIx` selected as synchronization input
            pub const exti8: u32 = 0b01000;

            /// 0b01001: Signal `EXTIx` selected as synchronization input
            pub const exti9: u32 = 0b01001;

            /// 0b01010: Signal `EXTIx` selected as synchronization input
            pub const exti10: u32 = 0b01010;

            /// 0b01011: Signal `EXTIx` selected as synchronization input
            pub const exti11: u32 = 0b01011;

            /// 0b01100: Signal `EXTIx` selected as synchronization input
            pub const exti12: u32 = 0b01100;

            /// 0b01101: Signal `EXTIx` selected as synchronization input
            pub const exti13: u32 = 0b01101;

            /// 0b01110: Signal `EXTIx` selected as synchronization input
            pub const exti14: u32 = 0b01110;

            /// 0b01111: Signal `EXTIx` selected as synchronization input
            pub const exti15: u32 = 0b01111;

            /// 0b10000: Signal `dmamux1_evt0` selected as synchronization input
            pub const dmamux1_evt0: u32 = 0b10000;

            /// 0b10001: Signal `dmamux1_evt1` selected as synchronization input
            pub const dmamux1_evt1: u32 = 0b10001;

            /// 0b10010: Signal `lptim1_out` selected as synchronization input
            pub const lptim1_out: u32 = 0b10010;

            /// 0b10011: Signal `lptim2_out` selected as synchronization input
            pub const lptim2_out: u32 = 0b10011;

            /// 0b10100: Signal `lptim3_out` selected as synchronization input
            pub const lptim3_out: u32 = 0b10100;
        }
    }
}

/// request generator channel x configuration register
pub mod RG1CR {
    pub use super::RG0CR::GE;
    pub use super::RG0CR::GNBREQ;
    pub use super::RG0CR::GPOL;
    pub use super::RG0CR::OIE;
    pub use super::RG0CR::SIG_ID;
}

/// request generator channel x configuration register
pub mod RG2CR {
    pub use super::RG0CR::GE;
    pub use super::RG0CR::GNBREQ;
    pub use super::RG0CR::GPOL;
    pub use super::RG0CR::OIE;
    pub use super::RG0CR::SIG_ID;
}

/// request generator channel x configuration register
pub mod RG3CR {
    pub use super::RG0CR::GE;
    pub use super::RG0CR::GNBREQ;
    pub use super::RG0CR::GPOL;
    pub use super::RG0CR::OIE;
    pub use super::RG0CR::SIG_ID;
}

/// request generator interrupt status register
pub mod RGSR {

    /// Trigger overrun event flag
    pub mod OF3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No new trigger event occured on DMA request generator channel x, before the request counter underrun
            pub const NoTrigger: u32 = 0b0;

            /// 0b1: New trigger event occured on DMA request generator channel x, before the request counter underrun
            pub const Trigger: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OF2
    pub mod OF2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::OF3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OF1
    pub mod OF1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::OF3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OF0
    pub mod OF0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        pub use super::OF3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// request generator interrupt clear flag register
pub mod RGCFR {

    /// Clear trigger overrun event flag
    pub mod COF3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear overrun flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// COF2
    pub mod COF2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::COF3::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// COF1
    pub mod COF1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::COF3::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// COF0
    pub mod COF0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::COF3::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// request line multiplexer channel x configuration register
    pub C0CR: RWRegister<u32>,

    /// request line multiplexer channel x configuration register
    pub C1CR: RWRegister<u32>,

    /// request line multiplexer channel x configuration register
    pub C2CR: RWRegister<u32>,

    /// request line multiplexer channel x configuration register
    pub C3CR: RWRegister<u32>,

    /// request line multiplexer channel x configuration register
    pub C4CR: RWRegister<u32>,

    /// request line multiplexer channel x configuration register
    pub C5CR: RWRegister<u32>,

    /// request line multiplexer channel x configuration register
    pub C6CR: RWRegister<u32>,

    /// request line multiplexer channel x configuration register
    pub C7CR: RWRegister<u32>,

    /// request line multiplexer channel x configuration register
    pub C8CR: RWRegister<u32>,

    /// request line multiplexer channel x configuration register
    pub C9CR: RWRegister<u32>,

    /// C10CR
    pub C10CR: RWRegister<u32>,

    /// C11CR
    pub C11CR: RWRegister<u32>,

    /// C12CR
    pub C12CR: RWRegister<u32>,

    /// C13CR
    pub C13CR: RWRegister<u32>,

    _reserved1: [u8; 72],

    /// request line multiplexer interrupt channel status register
    pub CSR: RORegister<u32>,

    /// request line multiplexer interrupt channel clear flag register
    pub CCFR: WORegister<u32>,

    _reserved2: [u8; 120],

    /// request generator channel x configuration register
    pub RG0CR: RWRegister<u32>,

    /// request generator channel x configuration register
    pub RG1CR: RWRegister<u32>,

    /// request generator channel x configuration register
    pub RG2CR: RWRegister<u32>,

    /// request generator channel x configuration register
    pub RG3CR: RWRegister<u32>,

    _reserved3: [u8; 48],

    /// request generator interrupt status register
    pub RGSR: RORegister<u32>,

    /// request generator interrupt clear flag register
    pub RGCFR: WORegister<u32>,
}
pub struct ResetValues {
    pub C0CR: u32,
    pub C1CR: u32,
    pub C2CR: u32,
    pub C3CR: u32,
    pub C4CR: u32,
    pub C5CR: u32,
    pub C6CR: u32,
    pub C7CR: u32,
    pub C8CR: u32,
    pub C9CR: u32,
    pub C10CR: u32,
    pub C11CR: u32,
    pub C12CR: u32,
    pub C13CR: u32,
    pub CSR: u32,
    pub CCFR: u32,
    pub RG0CR: u32,
    pub RG1CR: u32,
    pub RG2CR: u32,
    pub RG3CR: u32,
    pub RGSR: u32,
    pub RGCFR: u32,
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
