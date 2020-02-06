#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Analog-to-digital converter
//!
//! Used by: stm32l0x1, stm32l0x3

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// interrupt and status register
pub mod ISR {

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

    /// End of sequence flag
    pub mod EOS {
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

    /// End Of Calibration flag
    pub mod EOCAL {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Calibration is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Calibration complete
            pub const Complete: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the calibration flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// interrupt enable register
pub mod IER {

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

            /// 0b0: ADRDY interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set.
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

            /// 0b0: EOSMP interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set.
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

            /// 0b0: EOC interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: EOC interrupt enabled. An interrupt is generated when the EOC bit is set.
            pub const Enabled: u32 = 0b1;
        }
    }

    /// End of conversion sequence interrupt enable
    pub mod EOSIE {
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

            /// 0b0: EOS interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: EOS interrupt enabled. An interrupt is generated when the EOS bit is set.
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

            /// 0b1: Overrun interrupt enabled. An interrupt is generated when the OVR bit is set.
            pub const Enabled: u32 = 0b1;
        }
    }

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

    /// End of calibration interrupt enable
    pub mod EOCALIE {
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

            /// 0b0: End of calibration interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: End of calibration interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// control register
pub mod CR {

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

    /// ADC Voltage Regulator Enable
    pub mod ADVREGEN {
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

            /// 0b0: ADC voltage regulator disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: ADC voltage regulator enabled
            pub const Enabled: u32 = 0b1;
        }
    }

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

            /// 0b0: Analog watchdog disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Analog watchdog enabled
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

            /// 0b0: Discontinuous mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Discontinuous mode enabled
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
            pub const Preserve: u32 = 0b0;

            /// 0b1: ADC_DR register is overwritten with the last conversion result when an overrun is detected
            pub const Overwrite: u32 = 0b1;
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

            /// 0b00: Hardware trigger detection disabled
            pub const Disabled: u32 = 0b00;

            /// 0b01: Hardware trigger detection on the rising edge
            pub const RisingEdge: u32 = 0b01;

            /// 0b10: Hardware trigger detection on the falling edge
            pub const FallingEdge: u32 = 0b10;

            /// 0b11: Hardware trigger detection on both the rising and falling edges
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

            /// 0b000: Timer 6 TRGO event
            pub const TIM6_TRGO: u32 = 0b000;

            /// 0b001: Timer 21 CH2 event
            pub const TIM21_CH2: u32 = 0b001;

            /// 0b010: Timer 2 TRGO event
            pub const TIM2_TRGO: u32 = 0b010;

            /// 0b011: Timer 2 CH4 event
            pub const TIM2_CH4: u32 = 0b011;

            /// 0b100: Timer 22 TRGO, Timer 21 TRGO event
            pub const TIM22_TRGO: u32 = 0b100;

            /// 0b101: Timer 2 CH3 event
            pub const TIM2_CH3: u32 = 0b101;

            /// 0b110: Timer 3 TRGO event
            pub const TIM3_TRGO: u32 = 0b110;

            /// 0b111: EXTI line 11 event
            pub const EXTI_LINE11: u32 = 0b111;
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

            /// 0b00: 12 bits
            pub const TwelveBit: u32 = 0b00;

            /// 0b01: 10 bits
            pub const TenBit: u32 = 0b01;

            /// 0b10: 8 bits
            pub const EightBit: u32 = 0b10;

            /// 0b11: 6 bits
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

            /// 0b0: DMA one shot mode selected
            pub const OneShot: u32 = 0b0;

            /// 0b1: DMA circular mode selected
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

            /// 0b0: DMA disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// configuration register 2
pub mod CFGR2 {

    /// Oversampler Enable
    pub mod OVSE {
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

            /// 0b0: Oversampler disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Oversampler enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Oversampling ratio
    pub mod OVSR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (3 bits: 0b111 << 2)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 2x
            pub const Mul2: u32 = 0b000;

            /// 0b001: 4x
            pub const Mul4: u32 = 0b001;

            /// 0b010: 8x
            pub const Mul8: u32 = 0b010;

            /// 0b011: 16x
            pub const Mul16: u32 = 0b011;

            /// 0b100: 32x
            pub const Mul32: u32 = 0b100;

            /// 0b101: 64x
            pub const Mul64: u32 = 0b101;

            /// 0b110: 128x
            pub const Mul128: u32 = 0b110;

            /// 0b111: 256x
            pub const Mul256: u32 = 0b111;
        }
    }

    /// Oversampling shift
    pub mod OVSS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (4 bits: 0b1111 << 5)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Triggered Oversampling
    pub mod TOVS {
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

            /// 0b0: All oversampled conversions for a channel are done consecutively after a trigger
            pub const TriggerAll: u32 = 0b0;

            /// 0b1: Each oversampled conversion for a channel needs a trigger
            pub const TriggerEach: u32 = 0b1;
        }
    }

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

            /// 0b00: ADCCLK (Asynchronous clock mode)
            pub const ADCLK: u32 = 0b00;

            /// 0b01: PCLK/2 (Synchronous clock mode)
            pub const PCLK_Div2: u32 = 0b01;

            /// 0b10: PCLK/4 (Synchronous clock mode)
            pub const PCLK_Div4: u32 = 0b10;

            /// 0b11: PCLK (Synchronous clock mode)
            pub const PCLK: u32 = 0b11;
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

            /// 0b000: 1.5 ADC clock cycles
            pub const Cycles1_5: u32 = 0b000;

            /// 0b001: 3.5 ADC clock cycles
            pub const Cycles3_5: u32 = 0b001;

            /// 0b010: 7.5 ADC clock cycles
            pub const Cycles7_5: u32 = 0b010;

            /// 0b011: 12.5 ADC clock cycles
            pub const Cycles12_5: u32 = 0b011;

            /// 0b100: 19.5 ADC clock cycles
            pub const Cycles19_5: u32 = 0b100;

            /// 0b101: 39.5 ADC clock cycles
            pub const Cycles39_5: u32 = 0b101;

            /// 0b110: 79.5 ADC clock cycles
            pub const Cycles79_5: u32 = 0b110;

            /// 0b111: 160.5 ADC clock cycles
            pub const Cycles160_5: u32 = 0b111;
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

/// ADC Calibration factor
pub mod CALFACT {

    /// Calibration factor
    pub mod CALFACT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ADC common configuration register
pub mod CCR {

    /// ADC prescaler
    pub mod PRESC {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (4 bits: 0b1111 << 18)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Input ADC clock not divided
            pub const Div1: u32 = 0b0000;

            /// 0b0001: Input ADC clock divided by 2
            pub const Div2: u32 = 0b0001;

            /// 0b0010: Input ADC clock divided by 4
            pub const Div4: u32 = 0b0010;

            /// 0b0011: Input ADC clock divided by 6
            pub const Div6: u32 = 0b0011;

            /// 0b0100: Input ADC clock divided by 8
            pub const Div8: u32 = 0b0100;

            /// 0b0101: Input ADC clock divided by 10
            pub const Div10: u32 = 0b0101;

            /// 0b0110: Input ADC clock divided by 12
            pub const Div12: u32 = 0b0110;

            /// 0b0111: Input ADC clock divided by 16
            pub const Div16: u32 = 0b0111;

            /// 0b1000: Input ADC clock divided by 32
            pub const Div32: u32 = 0b1000;

            /// 0b1001: Input ADC clock divided by 64
            pub const Div64: u32 = 0b1001;

            /// 0b1010: Input ADC clock divided by 128
            pub const Div128: u32 = 0b1010;

            /// 0b1011: Input ADC clock divided by 256
            pub const Div256: u32 = 0b1011;
        }
    }

    /// VREFINT enable
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

            /// 0b0: VREFINT disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: VREFINT enabled
            pub const Enabled: u32 = 0b1;
        }
    }

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

    /// VLCD enable
    pub mod VLCDEN {
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

            /// 0b0: VLCD reading circuitry disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: VLCD reading circuitry enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Low Frequency Mode enable
    pub mod LFMEN {
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

            /// 0b0: Low Frequency Mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Low Frequency Mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}
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

    _reserved4: [u32; 28],

    /// ADC Calibration factor
    pub CALFACT: RWRegister<u32>,

    _reserved5: [u32; 148],

    /// ADC common configuration register
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
    pub CALFACT: u32,
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
