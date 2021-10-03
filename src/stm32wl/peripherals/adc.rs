#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Analog to digital convertor
//!
//! Used by: stm32wl5x_cm4, stm32wle5

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// ADC interrupt and status register
pub mod ISR {

    /// ADRDY
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

    /// EOSMP
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

    /// EOC
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

    /// EOS
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

    /// OVR
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

    /// AWD1
    pub mod AWD1 {
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

    /// AWD2
    pub mod AWD2 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        pub use super::AWD1::R;
        pub use super::AWD1::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AWD3
    pub mod AWD3 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        pub use super::AWD1::R;
        pub use super::AWD1::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EOCAL
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

    /// CCRDY
    pub mod CCRDY {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Channel configuration update not applied
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Channel configuration update is applied
            pub const Complete: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the channel configuration flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ADC interrupt enable register
pub mod IER {

    /// ADRDYIE
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

    /// EOSMPIE
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

    /// EOCIE
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

    /// EOSIE
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

    /// OVRIE
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

    /// AWD1IE
    pub mod AWD1IE {
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

    /// AWD2IE
    pub mod AWD2IE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD1IE::RW;
    }

    /// AWD3IE
    pub mod AWD3IE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD1IE::RW;
    }

    /// EOCALIE
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

    /// CCRDYIE
    pub mod CCRDYIE {
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

            /// 0b0: Channel configuration ready interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Channel configuration ready interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// ADC control register
pub mod CR {

    /// ADEN
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

    /// ADDIS
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

    /// ADSTART
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

    /// ADSTP
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

    /// ADVREGEN
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

    /// ADCAL
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

/// ADC configuration register 1
pub mod CFGR1 {

    /// DMAEN
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

    /// DMACFG
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

    /// SCANDIR
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

            /// 0b0: Upward scan (from CHSEL0 to CHSEL17)
            pub const Upward: u32 = 0b0;

            /// 0b1: Backward scan (from CHSEL17 to CHSEL0)
            pub const Backward: u32 = 0b1;
        }
    }

    /// RES
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
            pub const Bits12: u32 = 0b00;

            /// 0b01: 10 bits
            pub const Bits10: u32 = 0b01;

            /// 0b10: 8 bits
            pub const Bits8: u32 = 0b10;

            /// 0b11: 6 bits
            pub const Bits6: u32 = 0b11;
        }
    }

    /// ALIGN
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

    /// EXTSEL
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

            /// 0b000: Timer 1 TRGO event
            pub const TIM1_TRGO: u32 = 0b000;

            /// 0b001: Timer 1 CC4 event
            pub const TIM1_CC4: u32 = 0b001;

            /// 0b010: Timer 2 TRGO event
            pub const TIM2_TRGO: u32 = 0b010;

            /// 0b011: Timer 2 CH4 event
            pub const TIM2_CH4: u32 = 0b011;

            /// 0b101: Timer 2 CH3 event
            pub const TIM2_CH3: u32 = 0b101;

            /// 0b111: EXTI line 11 event
            pub const EXTI_LINE11: u32 = 0b111;
        }
    }

    /// EXTEN
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

    /// OVRMOD
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

    /// CONT
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

    /// WAIT
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

    /// AUTOFF
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

    /// DISCEN
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

    /// CHSELRMOD
    pub mod CHSELRMOD {
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

            /// 0b0: Each bit of the ADC_CHSELR register enables an input
            pub const BitPerInput: u32 = 0b0;

            /// 0b1: ADC_CHSELR register is able to sequence up to 8 channels
            pub const Sequence: u32 = 0b1;
        }
    }

    /// AWD1SGL
    pub mod AWD1SGL {
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

            /// 0b0: Analog watchdog 1 enabled on all channels
            pub const AllChannels: u32 = 0b0;

            /// 0b1: Analog watchdog 1 enabled on a single channel
            pub const SingleChannel: u32 = 0b1;
        }
    }

    /// AWD1EN
    pub mod AWD1EN {
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

            /// 0b0: Analog watchdog 1 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Analog watchdog 1 enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// AWD1CH
    pub mod AWD1CH {
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
}

/// ADC configuration register 2
pub mod CFGR2 {

    /// OVSE
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

    /// TOVS
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

    /// LFTRIG
    pub mod LFTRIG {
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

            /// 0b0: Low Frequency Trigger Mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Low Frequency Trigger Mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CKMODE
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

    /// OVSS0
    pub mod OVSS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (4 bits: 0b1111 << 5)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No shift
            pub const NoShift: u32 = 0b0000;

            /// 0b0001: Shift 1-bit
            pub const Shift1: u32 = 0b0001;

            /// 0b0010: Shift 2-bits
            pub const Shift2: u32 = 0b0010;

            /// 0b0011: Shift 3-bits
            pub const Shift3: u32 = 0b0011;

            /// 0b0100: Shift 4-bits
            pub const Shift4: u32 = 0b0100;

            /// 0b0101: Shift 5-bits
            pub const Shift5: u32 = 0b0101;

            /// 0b0110: Shift 6-bits
            pub const Shift6: u32 = 0b0110;

            /// 0b0111: Shift 7-bits
            pub const Shift7: u32 = 0b0111;

            /// 0b1000: Shift 8-bits
            pub const Shift8: u32 = 0b1000;
        }
    }

    /// OVSR0
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
}

/// ADC sampling time register
pub mod SMPR {

    /// SMP1
    pub mod SMP1 {
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

    /// SMP2
    pub mod SMP2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP1::RW;
    }

    /// SMPSEL
    pub mod SMPSEL0 {
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

            /// 0b0: Sampling time of CHANNELx use the setting of SMP1 register
            pub const Smp1: u32 = 0b0;

            /// 0b1: Sampling time of CHANNELx use the setting of SMP2 register
            pub const Smp2: u32 = 0b1;
        }
    }

    /// SMPSEL
    pub mod SMPSEL1 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL2 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL3 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL5 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL6 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL7 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL8 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL9 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL10 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL11 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL12 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL13 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL14 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL15 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL16 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }

    /// SMPSEL
    pub mod SMPSEL17 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMPSEL0::RW;
    }
}

/// ADC watchdog threshold register
pub mod AWD1TR {

    /// LT1
    pub mod LT1 {
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

    /// HT1
    pub mod HT1 {
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
}

/// ADC watchdog threshold register
pub mod AWD2TR {

    /// LT2
    pub mod LT2 {
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

    /// HT2
    pub mod HT2 {
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
}

/// CHSELR0 and CHSELR1
/// CHSELR0: channel selection register
/// CHSELR1: channel selection register
pub mod CHSELR {

    /// CHSEL
    pub mod CHSEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (18 bits: 0x3ffff << 0)
        pub const mask: u32 = 0x3ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000000000000000: Input Channel is not selected for conversion
            pub const NotSelected: u32 = 0b000000000000000000;

            /// 0b000000000000000001: Input Channel is selected for conversion
            pub const Selected: u32 = 0b000000000000000001;
        }
    }

    /// SQ1
    pub mod SQ1 {
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

            /// 0b0000: Channel 0 selected for the Nth conversion
            pub const Ch0: u32 = 0b0000;

            /// 0b0001: Channel 1 selected for the Nth conversion
            pub const Ch1: u32 = 0b0001;

            /// 0b0010: Channel 2 selected for the Nth conversion
            pub const Ch2: u32 = 0b0010;

            /// 0b0011: Channel 3 selected for the Nth conversion
            pub const Ch3: u32 = 0b0011;

            /// 0b0100: Channel 4 selected for the Nth conversion
            pub const Ch4: u32 = 0b0100;

            /// 0b0101: Channel 5 selected for the Nth conversion
            pub const Ch5: u32 = 0b0101;

            /// 0b0110: Channel 6 selected for the Nth conversion
            pub const Ch6: u32 = 0b0110;

            /// 0b0111: Channel 7 selected for the Nth conversion
            pub const Ch7: u32 = 0b0111;

            /// 0b1000: Channel 8 selected for the Nth conversion
            pub const Ch8: u32 = 0b1000;

            /// 0b1001: Channel 9 selected for the Nth conversion
            pub const Ch9: u32 = 0b1001;

            /// 0b1010: Channel 10 selected for the Nth conversion
            pub const Ch10: u32 = 0b1010;

            /// 0b1011: Channel 11 selected for the Nth conversion
            pub const Ch11: u32 = 0b1011;

            /// 0b1100: Channel 12 selected for the Nth conversion
            pub const Ch12: u32 = 0b1100;

            /// 0b1101: Channel 13 selected for the Nth conversion
            pub const Ch13: u32 = 0b1101;

            /// 0b1110: Channel 14 selected for the Nth conversion
            pub const Ch14: u32 = 0b1110;

            /// 0b1111: End of sequence
            pub const EOS: u32 = 0b1111;
        }
    }

    /// SQ2
    pub mod SQ2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SQ1::RW;
    }

    /// SQ3
    pub mod SQ3 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SQ1::RW;
    }

    /// SQ4
    pub mod SQ4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SQ1::RW;
    }

    /// SQ5
    pub mod SQ5 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SQ1::RW;
    }

    /// SQ6
    pub mod SQ6 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SQ1::RW;
    }

    /// SQ7
    pub mod SQ7 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SQ1::RW;
    }

    /// SQ8
    pub mod SQ8 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SQ1::RW;
    }
}

/// ADC watchdog threshold register
pub mod AWD3TR {

    /// LT3
    pub mod LT3 {
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

    /// HT3
    pub mod HT3 {
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
}

/// ADC data register
pub mod DR {

    /// DATA
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

/// ADC Analog Watchdog 2 Configuration register
pub mod AWD2CR {

    /// AWD2CH
    pub mod AWD2CH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (18 bits: 0x3ffff << 0)
        pub const mask: u32 = 0x3ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ADC Analog Watchdog 3 Configuration register
pub mod AWD3CR {

    /// AWD3CH
    pub mod AWD3CH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (18 bits: 0x3ffff << 0)
        pub const mask: u32 = 0x3ffff << offset;
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

    /// CALFACT
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

    /// VREFEN
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

    /// TSEN
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

    /// VBATEN
    pub mod VBATEN {
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

            /// 0b0: VBAT channel disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: VBAT channel enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PRESC0
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// ADC interrupt and status register
    pub ISR: RWRegister<u32>,

    /// ADC interrupt enable register
    pub IER: RWRegister<u32>,

    /// ADC control register
    pub CR: RWRegister<u32>,

    /// ADC configuration register 1
    pub CFGR1: RWRegister<u32>,

    /// ADC configuration register 2
    pub CFGR2: RWRegister<u32>,

    /// ADC sampling time register
    pub SMPR: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// ADC watchdog threshold register
    pub AWD1TR: RWRegister<u32>,

    /// ADC watchdog threshold register
    pub AWD2TR: RWRegister<u32>,

    /// CHSELR0 and CHSELR1
    /// CHSELR0: channel selection register
    /// CHSELR1: channel selection register
    pub CHSELR: RWRegister<u32>,

    /// ADC watchdog threshold register
    pub AWD3TR: RWRegister<u32>,

    _reserved2: [u32; 4],

    /// ADC data register
    pub DR: RORegister<u32>,

    _reserved3: [u32; 23],

    /// ADC Analog Watchdog 2 Configuration register
    pub AWD2CR: RWRegister<u32>,

    /// ADC Analog Watchdog 3 Configuration register
    pub AWD3CR: RWRegister<u32>,

    _reserved4: [u32; 3],

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
    pub AWD1TR: u32,
    pub AWD2TR: u32,
    pub CHSELR: u32,
    pub AWD3TR: u32,
    pub DR: u32,
    pub AWD2CR: u32,
    pub AWD3CR: u32,
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
#[cfg(feature = "rtic")]
unsafe impl Send for Instance {}
