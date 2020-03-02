#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Analog-to-digital converter

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// interrupt and status register
pub mod ISR {

    /// Analog watchdog flag
    pub mod AWD {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No analog watchdog event occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Analog watchdog event occurred
            pub const Event: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the analog watchdog event flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADC overrun
    pub mod OVR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No overrun occurred
            pub const NoOverrun: u32 = 0b0;

            /// 0b1: Overrun occurred
            pub const Overrun: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the overrun flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// End of sequence flag
    pub mod EOSEQ {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Conversion sequence is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Conversion sequence complete
            pub const Complete: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the conversion sequence flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// End of conversion flag
    pub mod EOC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Channel conversion is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Channel conversion complete
            pub const Complete: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the channel conversion flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// End of sampling flag
    pub mod EOSMP {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Not at the end of the samplings phase
            pub const NotAtEnd: u32 = 0b0;

            /// 0b1: End of sampling phase reached
            pub const AtEnd: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the sampling phase flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADC ready
    pub mod ADRDY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: ADC not yet ready to start conversion
            pub const NotReady: u32 = 0b0;

            /// 0b1: ADC ready to start conversion
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the ADC ready flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// interrupt enable register
pub mod IER {

    /// Analog watchdog interrupt enable
    pub mod AWDIE {
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

            /// 0b0: Analog watchdog interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Analog watchdog interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Overrun interrupt enable
    pub mod OVRIE {
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

            /// 0b0: Overrun interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Overrun interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// End of conversion sequence interrupt enable
    pub mod EOSEQIE {
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

            /// 0b0: End of conversion sequence interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: End of conversion sequence interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// End of conversion interrupt enable
    pub mod EOCIE {
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

            /// 0b0: End of conversion interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: End of conversion interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// End of sampling flag interrupt enable
    pub mod EOSMPIE {
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

            /// 0b0: End of sampling interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: End of sampling interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADC ready interrupt enable
    pub mod ADRDYIE {
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

            /// 0b0: ADC ready interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: ADC ready interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// control register
pub mod CR {

    /// ADC calibration
    pub mod ADCAL {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: ADC calibration either not yet performed or completed
            pub const NotCalibrating: u32 = 0b0;

            /// 0b1: ADC calibration in progress
            pub const Calibrating: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Start the ADC calibration sequence
            pub const StartCalibration: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADC stop conversion command
    pub mod ADSTP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No stop command active
            pub const NotStopping: u32 = 0b0;

            /// 0b1: ADC stopping conversion
            pub const Stopping: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Stop the active conversion
            pub const StopConversion: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADC start conversion command
    pub mod ADSTART {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No conversion ongoing
            pub const NotActive: u32 = 0b0;

            /// 0b1: ADC operating and may be converting
            pub const Active: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Start the ADC conversion (may be delayed for hardware triggers)
            pub const StartConversion: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADC disable command
    pub mod ADDIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No disable command active
            pub const NotDisabling: u32 = 0b0;

            /// 0b1: ADC disabling
            pub const Disabling: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Disable the ADC
            pub const Disable: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADC enable command
    pub mod ADEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: ADC disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: ADC enabled
            pub const Enabled: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Enable the ADC
            pub const Enabled: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// configuration register 1
pub mod CFGR1 {

    /// Analog watchdog channel selection
    pub mod AWDCH {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (5 bits: 0b11111 << 26)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog enable
    pub mod AWDEN {
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

            /// 0b0: Analog watchdog disabled on regular channels
            pub const Disabled: u32 = 0b0;

            /// 0b1: Analog watchdog enabled on regular channels
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Enable the watchdog on a single channel or on all channels
    pub mod AWDSGL {
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

            /// 0b0: Analog watchdog enabled on all channels
            pub const AllChannels: u32 = 0b0;

            /// 0b1: Analog watchdog enabled on a single channel
            pub const SingleChannel: u32 = 0b1;
        }
    }

    /// Discontinuous mode
    pub mod DISCEN {
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

            /// 0b0: Discontinuous mode on regular channels disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Discontinuous mode on regular channels enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Auto-off mode
    pub mod AUTOFF {
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

            /// 0b0: Auto-off mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Auto-off mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Auto-delayed conversion mode
    pub mod AUTDLY {
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

    /// Single / continuous conversion mode
    pub mod CONT {
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

            /// 0b0: Single conversion mode
            pub const Single: u32 = 0b0;

            /// 0b1: Continuous conversion mode
            pub const Continuous: u32 = 0b1;
        }
    }

    /// Overrun management mode
    pub mod OVRMOD {
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

            /// 0b0: ADC_DR register is preserved with the old data when an overrun is detected
            pub const Preserved: u32 = 0b0;

            /// 0b1: ADC_DR register is overwritten with the last conversion result when an overrun is detected
            pub const Overwritten: u32 = 0b1;
        }
    }

    /// External trigger enable and polarity selection
    pub mod EXTEN {
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

            /// 0b00: Trigger detection disabled
            pub const Disabled: u32 = 0b00;

            /// 0b01: Trigger detection on the rising edge
            pub const RisingEdge: u32 = 0b01;

            /// 0b10: Trigger detection on the falling edge
            pub const FallingEdge: u32 = 0b10;

            /// 0b11: Trigger detection on both the rising and falling edges
            pub const BothEdges: u32 = 0b11;
        }
    }

    /// External trigger selection
    pub mod EXTSEL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Timer 1 TRGO Event
            pub const TIM1_TRGO: u32 = 0b000;

            /// 0b001: Timer 1 CC4 event
            pub const TIM1_CC4: u32 = 0b001;

            /// 0b011: Timer 3 TRGO event
            pub const TIM3_TRGO: u32 = 0b011;

            /// 0b100: Timer 15 TRGO event
            pub const TIM15_TRGO: u32 = 0b100;
        }
    }

    /// Data alignment
    pub mod ALIGN {
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

            /// 0b0: Right alignment
            pub const Right: u32 = 0b0;

            /// 0b1: Left alignment
            pub const Left: u32 = 0b1;
        }
    }

    /// Data resolution
    pub mod RES {
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

            /// 0b00: 12-bit (14 ADCCLK cycles)
            pub const TwelveBit: u32 = 0b00;

            /// 0b01: 10-bit (13 ADCCLK cycles)
            pub const TenBit: u32 = 0b01;

            /// 0b10: 8-bit (11 ADCCLK cycles)
            pub const EightBit: u32 = 0b10;

            /// 0b11: 6-bit (9 ADCCLK cycles)
            pub const SixBit: u32 = 0b11;
        }
    }

    /// Scan sequence direction
    pub mod SCANDIR {
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

            /// 0b0: Upward scan (from CHSEL0 to CHSEL18)
            pub const Upward: u32 = 0b0;

            /// 0b1: Backward scan (from CHSEL18 to CHSEL0)
            pub const Backward: u32 = 0b1;
        }
    }

    /// Direct memery access configuration
    pub mod DMACFG {
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

            /// 0b0: DMA one shot mode
            pub const OneShot: u32 = 0b0;

            /// 0b1: DMA circular mode
            pub const Circular: u32 = 0b1;
        }
    }

    /// Direct memory access enable
    pub mod DMAEN {
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

            /// 0b0: DMA mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Wait conversion mode
    pub mod WAIT {
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

            /// 0b0: Wait conversion mode off
            pub const Disabled: u32 = 0b0;

            /// 0b1: Wait conversion mode on
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// configuration register 2
pub mod CFGR2 {

    /// ADC clock mode
    pub mod CKMODE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Asynchronous clock mode
            pub const ADCCLK: u32 = 0b00;

            /// 0b01: Synchronous clock mode (PCLK/2)
            pub const PCLK_Div2: u32 = 0b01;

            /// 0b10: Sychronous clock mode (PCLK/4)
            pub const PCLK_Div4: u32 = 0b10;
        }
    }
}

/// sampling time register
pub mod SMPR {

    /// Sampling time selection
    pub mod SMP {
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

            /// 0b000: 1.5 cycles
            pub const Cycles1_5: u32 = 0b000;

            /// 0b001: 7.5 cycles
            pub const Cycles7_5: u32 = 0b001;

            /// 0b010: 13.5 cycles
            pub const Cycles13_5: u32 = 0b010;

            /// 0b011: 28.5 cycles
            pub const Cycles28_5: u32 = 0b011;

            /// 0b100: 41.5 cycles
            pub const Cycles41_5: u32 = 0b100;

            /// 0b101: 55.5 cycles
            pub const Cycles55_5: u32 = 0b101;

            /// 0b110: 71.5 cycles
            pub const Cycles71_5: u32 = 0b110;

            /// 0b111: 239.5 cycles
            pub const Cycles239_5: u32 = 0b111;
        }
    }
}

/// watchdog threshold register
pub mod TR {

    /// Analog watchdog higher threshold
    pub mod HT {
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

    /// Analog watchdog lower threshold
    pub mod LT {
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

/// channel selection register
pub mod CHSELR {

    /// Channel-x selection
    pub mod CHSEL18 {
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

            /// 0b0: Input Channel is not selected for conversion
            pub const NotSelected: u32 = 0b0;

            /// 0b1: Input Channel is selected for conversion
            pub const Selected: u32 = 0b1;
        }
    }

    /// Channel-x selection
    pub mod CHSEL17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }

    /// Channel-x selection
    pub mod CHSEL0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHSEL18::RW;
    }
}

/// data register
pub mod DR {

    /// Converted data
    pub mod DATA {
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

/// common configuration register
pub mod CCR {

    /// Temperature sensor enable
    pub mod TSEN {
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

            /// 0b0: Temperature sensor disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Temperature sensor enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Temperature sensor and VREFINT enable
    pub mod VREFEN {
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

            /// 0b0: V_REFINT channel disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: V_REFINT channel enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// interrupt and status register
    pub ISR: RWRegister<u32>,

    /// interrupt enable register
    pub IER: RWRegister<u32>,

    /// control register
    pub CR: RWRegister<u32>,

    /// configuration register 1
    pub CFGR1: RWRegister<u32>,

    /// configuration register 2
    pub CFGR2: RWRegister<u32>,

    /// sampling time register
    pub SMPR: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// watchdog threshold register
    pub TR: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// channel selection register
    pub CHSELR: RWRegister<u32>,

    _reserved3: [u32; 5],

    /// data register
    pub DR: RORegister<u32>,

    _reserved4: [u32; 177],

    /// common configuration register
    pub CCR: RWRegister<u32>,
}
pub struct ResetValues {
    pub ISR: u32,
    pub IER: u32,
    pub CR: u32,
    pub CFGR1: u32,
    pub CFGR2: u32,
    pub SMPR: u32,
    pub TR: u32,
    pub CHSELR: u32,
    pub DR: u32,
    pub CCR: u32,
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
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}

/// Access functions for the ADC peripheral instance
pub mod ADC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40012400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IER: 0x00000000,
        CR: 0x00000000,
        CFGR1: 0x00000000,
        CFGR2: 0x00008000,
        SMPR: 0x00000000,
        TR: 0x00000FFF,
        CHSELR: 0x00000000,
        DR: 0x00000000,
        CCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ADC_TAKEN: bool = false;

    /// Safe access to ADC
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
            if ADC_TAKEN {
                None
            } else {
                ADC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC_TAKEN && inst.addr == INSTANCE.addr {
                ADC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ADC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ADC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ADC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC: *const RegisterBlock = 0x40012400 as *const _;
