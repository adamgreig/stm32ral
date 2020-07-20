#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMAMUX
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR0 {

    /// Input DMA request line selected
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

            /// 0b00000101: Signal `dmamux1_req_gen4` selected as request input
            pub const dmamux1_req_gen4: u32 = 0b00000101;

            /// 0b00000110: Signal `dmamux1_req_gen5` selected as request input
            pub const dmamux1_req_gen5: u32 = 0b00000110;

            /// 0b00000111: Signal `dmamux1_req_gen6` selected as request input
            pub const dmamux1_req_gen6: u32 = 0b00000111;

            /// 0b00001000: Signal `dmamux1_req_gen7` selected as request input
            pub const dmamux1_req_gen7: u32 = 0b00001000;

            /// 0b00001001: Signal `adc1_dma` selected as request input
            pub const adc1_dma: u32 = 0b00001001;

            /// 0b00001010: Signal `adc2_dma` selected as request input
            pub const adc2_dma: u32 = 0b00001010;

            /// 0b00001011: Signal `tim1_ch1` selected as request input
            pub const tim1_ch1: u32 = 0b00001011;

            /// 0b00001100: Signal `tim1_ch2` selected as request input
            pub const tim1_ch2: u32 = 0b00001100;

            /// 0b00001101: Signal `tim1_ch3` selected as request input
            pub const tim1_ch3: u32 = 0b00001101;

            /// 0b00001110: Signal `tim1_ch4` selected as request input
            pub const tim1_ch4: u32 = 0b00001110;

            /// 0b00001111: Signal `tim1_up` selected as request input
            pub const tim1_up: u32 = 0b00001111;

            /// 0b00010000: Signal `tim1_trig` selected as request input
            pub const tim1_trig: u32 = 0b00010000;

            /// 0b00010001: Signal `tim1_com` selected as request input
            pub const tim1_com: u32 = 0b00010001;

            /// 0b00010010: Signal `tim2_ch1` selected as request input
            pub const tim2_ch1: u32 = 0b00010010;

            /// 0b00010011: Signal `tim2_ch2` selected as request input
            pub const tim2_ch2: u32 = 0b00010011;

            /// 0b00010100: Signal `tim2_ch3` selected as request input
            pub const tim2_ch3: u32 = 0b00010100;

            /// 0b00010101: Signal `tim2_ch4` selected as request input
            pub const tim2_ch4: u32 = 0b00010101;

            /// 0b00010110: Signal `tim2_up` selected as request input
            pub const tim2_up: u32 = 0b00010110;

            /// 0b00010111: Signal `tim3_ch1` selected as request input
            pub const tim3_ch1: u32 = 0b00010111;

            /// 0b00011000: Signal `tim3_ch2` selected as request input
            pub const tim3_ch2: u32 = 0b00011000;

            /// 0b00011001: Signal `tim3_ch3` selected as request input
            pub const tim3_ch3: u32 = 0b00011001;

            /// 0b00011010: Signal `tim3_ch4` selected as request input
            pub const tim3_ch4: u32 = 0b00011010;

            /// 0b00011011: Signal `tim3_up` selected as request input
            pub const tim3_up: u32 = 0b00011011;

            /// 0b00011100: Signal `tim3_trig` selected as request input
            pub const tim3_trig: u32 = 0b00011100;

            /// 0b00011101: Signal `tim4_ch1` selected as request input
            pub const tim4_ch1: u32 = 0b00011101;

            /// 0b00011110: Signal `tim4_ch2` selected as request input
            pub const tim4_ch2: u32 = 0b00011110;

            /// 0b00011111: Signal `tim4_ch3` selected as request input
            pub const tim4_ch3: u32 = 0b00011111;

            /// 0b00100000: Signal `tim4_up` selected as request input
            pub const tim4_up: u32 = 0b00100000;

            /// 0b00100001: Signal `i2c1_rx_dma` selected as request input
            pub const i2c1_rx_dma: u32 = 0b00100001;

            /// 0b00100010: Signal `i2c1_tx_dma` selected as request input
            pub const i2c1_tx_dma: u32 = 0b00100010;

            /// 0b00100011: Signal `i2c2_rx_dma` selected as request input
            pub const i2c2_rx_dma: u32 = 0b00100011;

            /// 0b00100100: Signal `i2c2_tx_dma` selected as request input
            pub const i2c2_tx_dma: u32 = 0b00100100;

            /// 0b00100101: Signal `spi1_rx_dma` selected as request input
            pub const spi1_rx_dma: u32 = 0b00100101;

            /// 0b00100110: Signal `spi1_tx_dma` selected as request input
            pub const spi1_tx_dma: u32 = 0b00100110;

            /// 0b00100111: Signal `spi2_rx_dma` selected as request input
            pub const spi2_rx_dma: u32 = 0b00100111;

            /// 0b00101000: Signal `spi2_tx_dma` selected as request input
            pub const spi2_tx_dma: u32 = 0b00101000;

            /// 0b00101001: Signal `usart1_rx_dma` selected as request input
            pub const usart1_rx_dma: u32 = 0b00101001;

            /// 0b00101010: Signal `usart1_tx_dma` selected as request input
            pub const usart1_tx_dma: u32 = 0b00101010;

            /// 0b00101011: Signal `usart2_rx_dma` selected as request input
            pub const usart2_rx_dma: u32 = 0b00101011;

            /// 0b00101100: Signal `usart2_tx_dma` selected as request input
            pub const usart2_tx_dma: u32 = 0b00101100;

            /// 0b00101101: Signal `usart3_rx_dma` selected as request input
            pub const usart3_rx_dma: u32 = 0b00101101;

            /// 0b00101110: Signal `usart3_tx_dma` selected as request input
            pub const usart3_tx_dma: u32 = 0b00101110;

            /// 0b00101111: Signal `tim8_ch1` selected as request input
            pub const tim8_ch1: u32 = 0b00101111;

            /// 0b00110000: Signal `tim8_ch2` selected as request input
            pub const tim8_ch2: u32 = 0b00110000;

            /// 0b00110001: Signal `tim8_ch3` selected as request input
            pub const tim8_ch3: u32 = 0b00110001;

            /// 0b00110010: Signal `tim8_ch4` selected as request input
            pub const tim8_ch4: u32 = 0b00110010;

            /// 0b00110011: Signal `tim8_up` selected as request input
            pub const tim8_up: u32 = 0b00110011;

            /// 0b00110100: Signal `tim8_trig` selected as request input
            pub const tim8_trig: u32 = 0b00110100;

            /// 0b00110101: Signal `tim8_com` selected as request input
            pub const tim8_com: u32 = 0b00110101;

            /// 0b00110111: Signal `tim5_ch1` selected as request input
            pub const tim5_ch1: u32 = 0b00110111;

            /// 0b00111000: Signal `tim5_ch2` selected as request input
            pub const tim5_ch2: u32 = 0b00111000;

            /// 0b00111001: Signal `tim5_ch3` selected as request input
            pub const tim5_ch3: u32 = 0b00111001;

            /// 0b00111010: Signal `tim5_ch4` selected as request input
            pub const tim5_ch4: u32 = 0b00111010;

            /// 0b00111011: Signal `tim5_up` selected as request input
            pub const tim5_up: u32 = 0b00111011;

            /// 0b00111100: Signal `tim5_trig` selected as request input
            pub const tim5_trig: u32 = 0b00111100;

            /// 0b00111101: Signal `spi3_rx_dma` selected as request input
            pub const spi3_rx_dma: u32 = 0b00111101;

            /// 0b00111110: Signal `spi3_tx_dma` selected as request input
            pub const spi3_tx_dma: u32 = 0b00111110;

            /// 0b00111111: Signal `uart4_rx_dma` selected as request input
            pub const uart4_rx_dma: u32 = 0b00111111;

            /// 0b01000000: Signal `uart4_tx_dma` selected as request input
            pub const uart4_tx_dma: u32 = 0b01000000;

            /// 0b01000001: Signal `uart5_rx_dma` selected as request input
            pub const uart5_rx_dma: u32 = 0b01000001;

            /// 0b01000010: Signal `uart5_tx_dma` selected as request input
            pub const uart5_tx_dma: u32 = 0b01000010;

            /// 0b01000011: Signal `dac_ch1_dma` selected as request input
            pub const dac_ch1_dma: u32 = 0b01000011;

            /// 0b01000100: Signal `dac_ch2_dma` selected as request input
            pub const dac_ch2_dma: u32 = 0b01000100;

            /// 0b01000101: Signal `tim6_up` selected as request input
            pub const tim6_up: u32 = 0b01000101;

            /// 0b01000110: Signal `tim7_up` selected as request input
            pub const tim7_up: u32 = 0b01000110;

            /// 0b01000111: Signal `usart6_rx_dma` selected as request input
            pub const usart6_rx_dma: u32 = 0b01000111;

            /// 0b01001000: Signal `usart6_tx_dma` selected as request input
            pub const usart6_tx_dma: u32 = 0b01001000;

            /// 0b01001001: Signal `i2c3_rx_dma` selected as request input
            pub const i2c3_rx_dma: u32 = 0b01001001;

            /// 0b01001010: Signal `i2c3_tx_dma` selected as request input
            pub const i2c3_tx_dma: u32 = 0b01001010;

            /// 0b01001011: Signal `dcmi_dma` selected as request input
            pub const dcmi_dma: u32 = 0b01001011;

            /// 0b01001100: Signal `cryp_in_dma` selected as request input
            pub const cryp_in_dma: u32 = 0b01001100;

            /// 0b01001101: Signal `cryp_out_dma` selected as request input
            pub const cryp_out_dma: u32 = 0b01001101;

            /// 0b01001110: Signal `hash_in_dma` selected as request input
            pub const hash_in_dma: u32 = 0b01001110;

            /// 0b01001111: Signal `uart7_rx_dma` selected as request input
            pub const uart7_rx_dma: u32 = 0b01001111;

            /// 0b01010000: Signal `uart7_tx_dma` selected as request input
            pub const uart7_tx_dma: u32 = 0b01010000;

            /// 0b01010001: Signal `uart8_rx_dma` selected as request input
            pub const uart8_rx_dma: u32 = 0b01010001;

            /// 0b01010010: Signal `uart8_tx_dma` selected as request input
            pub const uart8_tx_dma: u32 = 0b01010010;

            /// 0b01010011: Signal `spi4_rx_dma` selected as request input
            pub const spi4_rx_dma: u32 = 0b01010011;

            /// 0b01010100: Signal `spi4_tx_dma` selected as request input
            pub const spi4_tx_dma: u32 = 0b01010100;

            /// 0b01010101: Signal `spi5_rx_dma` selected as request input
            pub const spi5_rx_dma: u32 = 0b01010101;

            /// 0b01010110: Signal `spi5_tx_dma` selected as request input
            pub const spi5_tx_dma: u32 = 0b01010110;

            /// 0b01010111: Signal `sai1a_dma` selected as request input
            pub const sai1a_dma: u32 = 0b01010111;

            /// 0b01011000: Signal `sai1b_dma` selected as request input
            pub const sai1b_dma: u32 = 0b01011000;

            /// 0b01011001: Signal `sai2a_dma` selected as request input
            pub const sai2a_dma: u32 = 0b01011001;

            /// 0b01011010: Signal `sai2b_dma` selected as request input
            pub const sai2b_dma: u32 = 0b01011010;

            /// 0b01011011: Signal `swpmi_rx_dma` selected as request input
            pub const swpmi_rx_dma: u32 = 0b01011011;

            /// 0b01011100: Signal `swpmi_tx_dma` selected as request input
            pub const swpmi_tx_dma: u32 = 0b01011100;

            /// 0b01011101: Signal `spdifrx_dat_dma` selected as request input
            pub const spdifrx_dat_dma: u32 = 0b01011101;

            /// 0b01011110: Signal `spdifrx_ctrl_dma` selected as request input
            pub const spdifrx_ctrl_dma: u32 = 0b01011110;

            /// 0b01011111: Signal `hr_req(1)` selected as request input
            pub const hr_req1: u32 = 0b01011111;

            /// 0b01100000: Signal `hr_req(2)` selected as request input
            pub const hr_req2: u32 = 0b01100000;

            /// 0b01100001: Signal `hr_req(3)` selected as request input
            pub const hr_req3: u32 = 0b01100001;

            /// 0b01100010: Signal `hr_req(4)` selected as request input
            pub const hr_req4: u32 = 0b01100010;

            /// 0b01100011: Signal `hr_req(5)` selected as request input
            pub const hr_req5: u32 = 0b01100011;

            /// 0b01100100: Signal `hr_req(6)` selected as request input
            pub const hr_req6: u32 = 0b01100100;

            /// 0b01100101: Signal `dfsdm1_dma0` selected as request input
            pub const dfsdm1_dma0: u32 = 0b01100101;

            /// 0b01100110: Signal `dfsdm1_dma1` selected as request input
            pub const dfsdm1_dma1: u32 = 0b01100110;

            /// 0b01100111: Signal `dfsdm1_dma2` selected as request input
            pub const dfsdm1_dma2: u32 = 0b01100111;

            /// 0b01101000: Signal `dfsdm1_dma3` selected as request input
            pub const dfsdm1_dma3: u32 = 0b01101000;

            /// 0b01101001: Signal `tim15_ch1` selected as request input
            pub const tim15_ch1: u32 = 0b01101001;

            /// 0b01101010: Signal `tim15_up` selected as request input
            pub const tim15_up: u32 = 0b01101010;

            /// 0b01101011: Signal `tim15_trig` selected as request input
            pub const tim15_trig: u32 = 0b01101011;

            /// 0b01101100: Signal `tim15_com` selected as request input
            pub const tim15_com: u32 = 0b01101100;

            /// 0b01101101: Signal `tim16_ch1` selected as request input
            pub const tim16_ch1: u32 = 0b01101101;

            /// 0b01101110: Signal `tim16_up` selected as request input
            pub const tim16_up: u32 = 0b01101110;

            /// 0b01101111: Signal `tim17_ch1` selected as request input
            pub const tim17_ch1: u32 = 0b01101111;

            /// 0b01110000: Signal `tim17_up` selected as request input
            pub const tim17_up: u32 = 0b01110000;

            /// 0b01110001: Signal `sai3_a_dma` selected as request input
            pub const sai3_a_dma: u32 = 0b01110001;

            /// 0b01110010: Signal `sai3_b_dma` selected as request input
            pub const sai3_b_dma: u32 = 0b01110010;

            /// 0b01110011: Signal `adc3_dma` selected as request input
            pub const adc3_dma: u32 = 0b01110011;
        }
    }

    /// Interrupt enable at synchronization event overrun
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

    /// Event generation enable/disable
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

    /// Synchronous operating mode enable/disable
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

    /// Synchronization event type selector Defines the synchronization event on the selected synchronization input:
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

    /// Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset.
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

    /// Synchronization input selected
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

            /// 0b00000: Signal `dmamux1_evt0` selected as synchronization input
            pub const dmamux1_evt0: u32 = 0b00000;

            /// 0b00001: Signal `dmamux1_evt1` selected as synchronization input
            pub const dmamux1_evt1: u32 = 0b00001;

            /// 0b00010: Signal `dmamux1_evt2` selected as synchronization input
            pub const dmamux1_evt2: u32 = 0b00010;

            /// 0b00011: Signal `lptim1_out` selected as synchronization input
            pub const lptim1_out: u32 = 0b00011;

            /// 0b00100: Signal `lptim2_out` selected as synchronization input
            pub const lptim2_out: u32 = 0b00100;

            /// 0b00101: Signal `lptim3_out` selected as synchronization input
            pub const lptim3_out: u32 = 0b00101;

            /// 0b00110: Signal `extit0` selected as synchronization input
            pub const extit0: u32 = 0b00110;

            /// 0b00111: Signal `tim12_trgo` selected as synchronization input
            pub const tim12_trgo: u32 = 0b00111;
        }
    }
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR1 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR2 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR3 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR4 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR5 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR6 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR7 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR8 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR9 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR10 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR11 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR12 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR13 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR14 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR15 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RGCR0 {

    /// DMA request trigger input selected
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

            /// 0b00000: Signal `dmamux1_evt0` selected as trigger input
            pub const dmamux1_evt0: u32 = 0b00000;

            /// 0b00001: Signal `dmamux1_evt1` selected as trigger input
            pub const dmamux1_evt1: u32 = 0b00001;

            /// 0b00010: Signal `dmamux1_evt2` selected as trigger input
            pub const dmamux1_evt2: u32 = 0b00010;

            /// 0b00011: Signal `lptim1_out` selected as trigger input
            pub const lptim1_out: u32 = 0b00011;

            /// 0b00100: Signal `lptim2_out` selected as trigger input
            pub const lptim2_out: u32 = 0b00100;

            /// 0b00101: Signal `lptim3_out` selected as trigger input
            pub const lptim3_out: u32 = 0b00101;

            /// 0b00110: Signal `extit0` selected as trigger input
            pub const extit0: u32 = 0b00110;

            /// 0b00111: Signal `tim12_trgo` selected as trigger input
            pub const tim12_trgo: u32 = 0b00111;
        }
    }

    /// Interrupt enable at trigger event overrun
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

    /// DMA request generator channel enable/disable
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

    /// DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input
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

    /// Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset.
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
}

/// DMAMux - DMA request generator channel x control register
pub mod RGCR1 {
    pub use super::RGCR0::GE;
    pub use super::RGCR0::GNBREQ;
    pub use super::RGCR0::GPOL;
    pub use super::RGCR0::OIE;
    pub use super::RGCR0::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RGCR2 {
    pub use super::RGCR0::GE;
    pub use super::RGCR0::GNBREQ;
    pub use super::RGCR0::GPOL;
    pub use super::RGCR0::OIE;
    pub use super::RGCR0::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RGCR3 {
    pub use super::RGCR0::GE;
    pub use super::RGCR0::GNBREQ;
    pub use super::RGCR0::GPOL;
    pub use super::RGCR0::OIE;
    pub use super::RGCR0::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RGCR4 {
    pub use super::RGCR0::GE;
    pub use super::RGCR0::GNBREQ;
    pub use super::RGCR0::GPOL;
    pub use super::RGCR0::OIE;
    pub use super::RGCR0::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RGCR5 {
    pub use super::RGCR0::GE;
    pub use super::RGCR0::GNBREQ;
    pub use super::RGCR0::GPOL;
    pub use super::RGCR0::OIE;
    pub use super::RGCR0::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RGCR6 {
    pub use super::RGCR0::GE;
    pub use super::RGCR0::GNBREQ;
    pub use super::RGCR0::GPOL;
    pub use super::RGCR0::OIE;
    pub use super::RGCR0::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RGCR7 {
    pub use super::RGCR0::GE;
    pub use super::RGCR0::GNBREQ;
    pub use super::RGCR0::GPOL;
    pub use super::RGCR0::OIE;
    pub use super::RGCR0::SIG_ID;
}

/// DMAMux - DMA request generator status register
pub mod RGSR {

    /// Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register.
    pub mod OF0 {
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

    /// Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register.
    pub mod OF1 {
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

    /// Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register.
    pub mod OF2 {
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

    /// Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register.
    pub mod OF3 {
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

    /// Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register.
    pub mod OF4 {
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

    /// Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register.
    pub mod OF5 {
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

    /// Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register.
    pub mod OF6 {
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

    /// Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register.
    pub mod OF7 {
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
}

/// DMAMux - DMA request generator clear flag register
pub mod RGCFR {

    /// Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    pub mod COF0 {
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

    /// Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    pub mod COF1 {
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

    /// Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    pub mod COF2 {
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

    /// Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    pub mod COF3 {
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

    /// Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    pub mod COF4 {
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

    /// Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    pub mod COF5 {
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

    /// Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    pub mod COF6 {
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

    /// Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    pub mod COF7 {
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
}

/// DMAMUX request line multiplexer interrupt channel status register
pub mod CSR {

    /// Synchronization overrun event flag
    pub mod SOF0 {
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

    /// Synchronization overrun event flag
    pub mod SOF1 {
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

    /// Synchronization overrun event flag
    pub mod SOF2 {
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

    /// Synchronization overrun event flag
    pub mod SOF3 {
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

    /// Synchronization overrun event flag
    pub mod SOF4 {
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

    /// Synchronization overrun event flag
    pub mod SOF5 {
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

    /// Synchronization overrun event flag
    pub mod SOF6 {
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

    /// Synchronization overrun event flag
    pub mod SOF7 {
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

    /// Synchronization overrun event flag
    pub mod SOF8 {
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

    /// Synchronization overrun event flag
    pub mod SOF9 {
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

    /// Synchronization overrun event flag
    pub mod SOF10 {
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

    /// Synchronization overrun event flag
    pub mod SOF11 {
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

    /// Synchronization overrun event flag
    pub mod SOF12 {
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

    /// Synchronization overrun event flag
    pub mod SOF13 {
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

    /// Synchronization overrun event flag
    pub mod SOF14 {
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

    /// Synchronization overrun event flag
    pub mod SOF15 {
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
}

/// DMAMUX request line multiplexer interrupt clear flag register
pub mod CFR {

    /// Clear synchronization overrun event flag
    pub mod CSOF0 {
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

    /// Clear synchronization overrun event flag
    pub mod CSOF1 {
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

    /// Clear synchronization overrun event flag
    pub mod CSOF2 {
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

    /// Clear synchronization overrun event flag
    pub mod CSOF3 {
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

    /// Clear synchronization overrun event flag
    pub mod CSOF4 {
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

    /// Clear synchronization overrun event flag
    pub mod CSOF5 {
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

    /// Clear synchronization overrun event flag
    pub mod CSOF6 {
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

    /// Clear synchronization overrun event flag
    pub mod CSOF7 {
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

    /// Clear synchronization overrun event flag
    pub mod CSOF8 {
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

    /// Clear synchronization overrun event flag
    pub mod CSOF9 {
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

    /// Clear synchronization overrun event flag
    pub mod CSOF10 {
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

    /// Clear synchronization overrun event flag
    pub mod CSOF11 {
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

    /// Clear synchronization overrun event flag
    pub mod CSOF12 {
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

    /// Clear synchronization overrun event flag
    pub mod CSOF13 {
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

    /// Clear synchronization overrun event flag
    pub mod CSOF14 {
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

    /// Clear synchronization overrun event flag
    pub mod CSOF15 {
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR0: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR1: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR2: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR3: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR4: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR5: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR6: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR7: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR8: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR9: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR10: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR11: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR12: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR13: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR14: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR15: RWRegister<u32>,

    _reserved1: [u32; 16],

    /// DMAMUX request line multiplexer interrupt channel status register
    pub CSR: RORegister<u32>,

    /// DMAMUX request line multiplexer interrupt clear flag register
    pub CFR: WORegister<u32>,

    _reserved2: [u32; 30],

    /// DMAMux - DMA request generator channel x control register
    pub RGCR0: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RGCR1: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RGCR2: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RGCR3: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RGCR4: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RGCR5: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RGCR6: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RGCR7: RWRegister<u32>,

    _reserved3: [u32; 8],

    /// DMAMux - DMA request generator status register
    pub RGSR: RORegister<u32>,

    /// DMAMux - DMA request generator clear flag register
    pub RGCFR: WORegister<u32>,
}
pub struct ResetValues {
    pub CCR0: u32,
    pub CCR1: u32,
    pub CCR2: u32,
    pub CCR3: u32,
    pub CCR4: u32,
    pub CCR5: u32,
    pub CCR6: u32,
    pub CCR7: u32,
    pub CCR8: u32,
    pub CCR9: u32,
    pub CCR10: u32,
    pub CCR11: u32,
    pub CCR12: u32,
    pub CCR13: u32,
    pub CCR14: u32,
    pub CCR15: u32,
    pub CSR: u32,
    pub CFR: u32,
    pub RGCR0: u32,
    pub RGCR1: u32,
    pub RGCR2: u32,
    pub RGCR3: u32,
    pub RGCR4: u32,
    pub RGCR5: u32,
    pub RGCR6: u32,
    pub RGCR7: u32,
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
