#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMAMUX2
//!
//! Used by: stm32h735, stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v, stm32h7b3

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

            /// 0b00000001: Signal `dmamux2_req_gen0` selected as request input
            pub const dmamux2_req_gen0: u32 = 0b00000001;

            /// 0b00000010: Signal `dmamux2_req_gen1` selected as request input
            pub const dmamux2_req_gen1: u32 = 0b00000010;

            /// 0b00000011: Signal `dmamux2_req_gen2` selected as request input
            pub const dmamux2_req_gen2: u32 = 0b00000011;

            /// 0b00000100: Signal `dmamux2_req_gen3` selected as request input
            pub const dmamux2_req_gen3: u32 = 0b00000100;

            /// 0b00000101: Signal `dmamux2_req_gen4` selected as request input
            pub const dmamux2_req_gen4: u32 = 0b00000101;

            /// 0b00000110: Signal `dmamux2_req_gen5` selected as request input
            pub const dmamux2_req_gen5: u32 = 0b00000110;

            /// 0b00000111: Signal `dmamux2_req_gen6` selected as request input
            pub const dmamux2_req_gen6: u32 = 0b00000111;

            /// 0b00001000: Signal `dmamux2_req_gen7` selected as request input
            pub const dmamux2_req_gen7: u32 = 0b00001000;

            /// 0b00001001: Signal `lpuart1_rx_dma` selected as request input
            pub const lpuart1_rx_dma: u32 = 0b00001001;

            /// 0b00001010: Signal `lpuart1_tx_dma` selected as request input
            pub const lpuart1_tx_dma: u32 = 0b00001010;

            /// 0b00001011: Signal `spi6_rx_dma` selected as request input
            pub const spi6_rx_dma: u32 = 0b00001011;

            /// 0b00001100: Signal `spi6_tx_dma` selected as request input
            pub const spi6_tx_dma: u32 = 0b00001100;

            /// 0b00001101: Signal `i2c4_rx_dma` selected as request input
            pub const i2c4_rx_dma: u32 = 0b00001101;

            /// 0b00001110: Signal `i2c4_tx_dma` selected as request input
            pub const i2c4_tx_dma: u32 = 0b00001110;

            /// 0b00001111: Signal `sai4_a_dma` selected as request input
            pub const sai4_a_dma: u32 = 0b00001111;

            /// 0b00010000: Signal `sai4_b_dma` selected as request input
            pub const sai4_b_dma: u32 = 0b00010000;

            /// 0b00010001: Signal `adc3_dma` selected as request input
            pub const adc3_dma: u32 = 0b00010001;
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

            /// 0b00000: Signal `dmamux2_evt0` selected as synchronization input
            pub const dmamux2_evt0: u32 = 0b00000;

            /// 0b00001: Signal `dmamux2_evt1` selected as synchronization input
            pub const dmamux2_evt1: u32 = 0b00001;

            /// 0b00010: Signal `dmamux2_evt2` selected as synchronization input
            pub const dmamux2_evt2: u32 = 0b00010;

            /// 0b00011: Signal `dmamux2_evt3` selected as synchronization input
            pub const dmamux2_evt3: u32 = 0b00011;

            /// 0b00100: Signal `dmamux2_evt4` selected as synchronization input
            pub const dmamux2_evt4: u32 = 0b00100;

            /// 0b00101: Signal `dmamux2_evt5` selected as synchronization input
            pub const dmamux2_evt5: u32 = 0b00101;

            /// 0b00110: Signal `lpuart1_rx_wkup` selected as synchronization input
            pub const lpuart1_rx_wkup: u32 = 0b00110;

            /// 0b00111: Signal `lpuart1_tx_wkup` selected as synchronization input
            pub const lpuart1_tx_wkup: u32 = 0b00111;

            /// 0b01000: Signal `lptim2_out` selected as synchronization input
            pub const lptim2_out: u32 = 0b01000;

            /// 0b01001: Signal `lptim3_out` selected as synchronization input
            pub const lptim3_out: u32 = 0b01001;

            /// 0b01010: Signal `i2c4_wkup` selected as synchronization input
            pub const i2c4_wkup: u32 = 0b01010;

            /// 0b01011: Signal `spi6_wkup` selected as synchronization input
            pub const spi6_wkup: u32 = 0b01011;

            /// 0b01100: Signal `comp1_out` selected as synchronization input
            pub const comp1_out: u32 = 0b01100;

            /// 0b01101: Signal `rtc_wkup` selected as synchronization input
            pub const rtc_wkup: u32 = 0b01101;

            /// 0b01110: Signal `syscfg_exti0_mux` selected as synchronization input
            pub const syscfg_exti0_mux: u32 = 0b01110;

            /// 0b01111: Signal `syscfg_exti2_mux` selected as synchronization input
            pub const syscfg_exti2_mux: u32 = 0b01111;
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

            /// 0b00000: Signal `dmamux2_evt0` selected as trigger input
            pub const dmamux2_evt0: u32 = 0b00000;

            /// 0b00001: Signal `dmamux2_evt1` selected as trigger input
            pub const dmamux2_evt1: u32 = 0b00001;

            /// 0b00010: Signal `dmamux2_evt2` selected as trigger input
            pub const dmamux2_evt2: u32 = 0b00010;

            /// 0b00011: Signal `dmamux2_evt3` selected as trigger input
            pub const dmamux2_evt3: u32 = 0b00011;

            /// 0b00100: Signal `dmamux2_evt4` selected as trigger input
            pub const dmamux2_evt4: u32 = 0b00100;

            /// 0b00101: Signal `dmamux2_evt5` selected as trigger input
            pub const dmamux2_evt5: u32 = 0b00101;

            /// 0b00110: Signal `dmamux2_evt6` selected as trigger input
            pub const dmamux2_evt6: u32 = 0b00110;

            /// 0b00111: Signal `lpuart_rx_wkup` selected as trigger input
            pub const lpuart_rx_wkup: u32 = 0b00111;

            /// 0b01000: Signal `lpuart_tx_wkup` selected as trigger input
            pub const lpuart_tx_wkup: u32 = 0b01000;

            /// 0b01001: Signal `lptim2_wkup` selected as trigger input
            pub const lptim2_wkup: u32 = 0b01001;

            /// 0b01010: Signal `lptim2_out` selected as trigger input
            pub const lptim2_out: u32 = 0b01010;

            /// 0b01011: Signal `lptim3_wkup` selected as trigger input
            pub const lptim3_wkup: u32 = 0b01011;

            /// 0b01100: Signal `lptim3_out` selected as trigger input
            pub const lptim3_out: u32 = 0b01100;

            /// 0b01101: Signal `lptim4_ait` selected as trigger input
            pub const lptim4_ait: u32 = 0b01101;

            /// 0b01110: Signal `lptim5_ait` selected as trigger input
            pub const lptim5_ait: u32 = 0b01110;

            /// 0b01111: Signal `i2c4_wkup` selected as trigger input
            pub const i2c4_wkup: u32 = 0b01111;

            /// 0b10000: Signal `spi6_wkup` selected as trigger input
            pub const spi6_wkup: u32 = 0b10000;

            /// 0b10001: Signal `comp1_out` selected as trigger input
            pub const comp1_out: u32 = 0b10001;

            /// 0b10010: Signal `comp2_out` selected as trigger input
            pub const comp2_out: u32 = 0b10010;

            /// 0b10011: Signal `rtc_wkup` selected as trigger input
            pub const rtc_wkup: u32 = 0b10011;

            /// 0b10100: Signal `syscfg_exti0_mux` selected as trigger input
            pub const syscfg_exti0_mux: u32 = 0b10100;

            /// 0b10101: Signal `syscfg_exti2_mux` selected as trigger input
            pub const syscfg_exti2_mux: u32 = 0b10101;

            /// 0b10110: Signal `i2c4_event_it` selected as trigger input
            pub const i2c4_event_it: u32 = 0b10110;

            /// 0b10111: Signal `spi6_it` selected as trigger input
            pub const spi6_it: u32 = 0b10111;

            /// 0b11000: Signal `lpuart1_it_t` selected as trigger input
            pub const lpuart1_it_t: u32 = 0b11000;

            /// 0b11001: Signal `lpuart1_it_r` selected as trigger input
            pub const lpuart1_it_r: u32 = 0b11001;

            /// 0b11010: Signal `adc3_it` selected as trigger input
            pub const adc3_it: u32 = 0b11010;

            /// 0b11011: Signal `adc3_awd1` selected as trigger input
            pub const adc3_awd1: u32 = 0b11011;

            /// 0b11100: Signal `bdma_ch0_it` selected as trigger input
            pub const bdma_ch0_it: u32 = 0b11100;

            /// 0b11101: Signal `bdma_ch1_it` selected as trigger input
            pub const bdma_ch1_it: u32 = 0b11101;
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

    _reserved1: [u8; 96],

    /// DMAMUX request line multiplexer interrupt channel status register
    pub CSR: RORegister<u32>,

    /// DMAMUX request line multiplexer interrupt clear flag register
    pub CFR: WORegister<u32>,

    _reserved2: [u8; 120],

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

    _reserved3: [u8; 32],

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
