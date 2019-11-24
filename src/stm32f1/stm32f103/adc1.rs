#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Analog to digital converter

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// status register
pub mod SR {

    /// Regular channel start flag
    pub mod STRT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No regular channel conversion started
            pub const NotStarted: u32 = 0b0;

            /// 0b1: Regular channel conversion has started
            pub const Started: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: Clear the Regular channel Start flag
            pub const Clear: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected channel start flag
    pub mod JSTRT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No injected group conversion started
            pub const NotStarted: u32 = 0b0;

            /// 0b1: Injected group conversion has started
            pub const Started: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: Clear Injected channel Start flag
            pub const Clear: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected channel end of conversion
    pub mod JEOC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Conversion is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Conversion complete
            pub const Complete: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: Clear Injected channel end of conversion flag
            pub const Clear: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Regular channel end of conversion
    pub mod EOC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::JEOC::R;
        /// Write-only values
        pub mod W {

            /// 0b0: Clear End of conversion flag
            pub const Clear: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog flag
    pub mod AWD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
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

            /// 0b0: Clear the analog watchdog event flag
            pub const Clear: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// control register 1
pub mod CR1 {

    /// Analog watchdog enable on regular channels
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

    /// Analog watchdog enable on injected channels
    pub mod JAWDEN {
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

            /// 0b0: Analog watchdog disabled on injected channels
            pub const Disabled: u32 = 0b0;

            /// 0b1: Analog watchdog enabled on injected channels
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Dual mode selection
    pub mod DUALMOD {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Independent mode
            pub const Independent: u32 = 0b0000;

            /// 0b0001: Combined regular simultaneous + injected simultaneous mode
            pub const RegularInjected: u32 = 0b0001;

            /// 0b0010: Combined regular simultaneous + alternate trigger mode
            pub const RegularAlternateTrigger: u32 = 0b0010;

            /// 0b0011: Combined injected simultaneous + fast interleaved mode
            pub const InjectedFastInterleaved: u32 = 0b0011;

            /// 0b0100: Combined injected simultaneous + slow interleaved mode
            pub const InjectedSlowInterleaved: u32 = 0b0100;

            /// 0b0101: Injected simultaneous mode only
            pub const Injected: u32 = 0b0101;

            /// 0b0110: Regular simultaneous mode only
            pub const Regular: u32 = 0b0110;

            /// 0b0111: Fast interleaved mode only
            pub const FastInterleaved: u32 = 0b0111;

            /// 0b1000: Slow interleaved mode only
            pub const SlowInterleaved: u32 = 0b1000;

            /// 0b1001: Alternate trigger mode only
            pub const AlternateTrigger: u32 = 0b1001;
        }
    }

    /// Discontinuous mode channel count
    pub mod DISCNUM {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Discontinuous mode on injected channels
    pub mod JDISCEN {
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

            /// 0b0: Discontinuous mode on injected channels disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Discontinuous mode on injected channels enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Discontinuous mode on regular channels
    pub mod DISCEN {
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

            /// 0b0: Discontinuous mode on regular channels disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Discontinuous mode on regular channels enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Automatic injected group conversion
    pub mod JAUTO {
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

            /// 0b0: Automatic injected group conversion disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Automatic injected group conversion enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Enable the watchdog on a single channel in scan mode
    pub mod AWDSGL {
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

            /// 0b0: Analog watchdog enabled on all channels
            pub const All: u32 = 0b0;

            /// 0b1: Analog watchdog enabled on a single channel
            pub const Single: u32 = 0b1;
        }
    }

    /// Scan mode
    pub mod SCAN {
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

            /// 0b0: Scan mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Scan mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Interrupt enable for injected channels
    pub mod JEOCIE {
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

            /// 0b0: JEOC interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: JEOC interrupt enabled. An interrupt is generated when the JEOC bit is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Analog watchdog interrupt enable
    pub mod AWDIE {
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

            /// 0b0: EOC interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: EOC interrupt enabled. An interrupt is generated when the EOC bit is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Interrupt enable for EOC
    pub mod EOCIE {
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

    /// Analog watchdog channel select bits
    pub mod AWDCH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// control register 2
pub mod CR2 {

    /// Temperature sensor and VREFINT enable
    pub mod TSVREFE {
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

            /// 0b0: Temperature sensor and V_REFINT channel disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Temperature sensor and V_REFINT channel enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Start conversion of regular channels
    pub mod SWSTART {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Reset state
            pub const Started: u32 = 0b0;

            /// 0b1: Starting conversion of regular channels
            pub const NotStarted: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Start conversion of regular channels
            pub const Start: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Start conversion of injected channels
    pub mod JSWSTART {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Reset state
            pub const Started: u32 = 0b0;

            /// 0b1: Starting conversion of injected channels
            pub const NotStarted: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Start conversion of injected channels
            pub const Start: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External trigger conversion mode for regular channels
    pub mod EXTTRIG {
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

            /// 0b0: Conversion on external event disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Conversion on external event enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// External event select for regular group
    pub mod EXTSEL {
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

            /// 0b000: Timer 1 CC1 event
            pub const Tim1Cc1: u32 = 0b000;

            /// 0b001: Timer 1 CC2 event
            pub const Tim1Cc2: u32 = 0b001;

            /// 0b010: Timer 1 CC3 event
            pub const Tim1Cc3: u32 = 0b010;

            /// 0b011: Timer 2 CC2 event
            pub const Tim2Cc2: u32 = 0b011;

            /// 0b100: Timer 3 TRGO event
            pub const Tim3Trgo: u32 = 0b100;

            /// 0b101: Timer 4 CC4 event
            pub const Tim4Cc4: u32 = 0b101;

            /// 0b110: EXTI line 11/TIM8_TRGO event (TIM8_TRGO is available only in high-density and XL-density devices)
            pub const Exti11: u32 = 0b110;

            /// 0b111: SWSTART
            pub const Swstart: u32 = 0b111;
        }
    }

    /// External trigger conversion mode for injected channels
    pub mod JEXTTRIG {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTTRIG::RW;
    }

    /// External event select for injected group
    pub mod JEXTSEL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Timer 1 TRGO event
            pub const Tim1Trgo: u32 = 0b000;

            /// 0b001: Timer 1 CC4 event
            pub const Tim1Cc4: u32 = 0b001;

            /// 0b010: Timer 2 TRGO event
            pub const Tim2Trgo: u32 = 0b010;

            /// 0b011: Timer 2 CC1 event
            pub const Tim2Cc1: u32 = 0b011;

            /// 0b100: Timer 3 CC4 event
            pub const Tim3Cc4: u32 = 0b100;

            /// 0b101: Timer 4 TRGO event
            pub const Tim4Trgo: u32 = 0b101;

            /// 0b110: EXTI line15/TIM8_CC4 event (TIM8_CC4 is available only in high-density and XL-density devices)
            pub const Exti15: u32 = 0b110;

            /// 0b111: JSWSTART
            pub const Jswstart: u32 = 0b111;
        }
    }

    /// Data alignment
    pub mod ALIGN {
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

            /// 0b0: Right Alignment
            pub const Right: u32 = 0b0;

            /// 0b1: Left Alignment
            pub const Left: u32 = 0b1;
        }
    }

    /// Direct memory access mode
    pub mod DMA {
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

            /// 0b0: DMA mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Reset calibration
    pub mod RSTCAL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Calibration register initialized
            pub const Initialized: u32 = 0b0;

            /// 0b1: Initializing calibration register
            pub const NotInitialized: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Initialize calibration register
            pub const Initialize: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// A/D calibration
    pub mod CAL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Calibration completed
            pub const Complete: u32 = 0b0;

            /// 0b1: Calibrating
            pub const NotComplete: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Enable calibration
            pub const Start: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Continuous conversion
    pub mod CONT {
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

            /// 0b0: Single conversion mode
            pub const Single: u32 = 0b0;

            /// 0b1: Continuous conversion mode
            pub const Continuous: u32 = 0b1;
        }
    }

    /// A/D converter ON / OFF
    pub mod ADON {
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

            /// 0b0: Disable ADC conversion/calibration and go to power down mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable ADC and to start conversion
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// sample time register 1
pub mod SMPR1 {

    /// Channel 10 sample time selection
    pub mod SMP10 {
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

            /// 0b001: 7.5 ADC clock cycles
            pub const Cycles7_5: u32 = 0b001;

            /// 0b010: 13.5 ADC clock cycles
            pub const Cycles13_5: u32 = 0b010;

            /// 0b011: 28.5 ADC clock cycles
            pub const Cycles28_5: u32 = 0b011;

            /// 0b100: 41.5 ADC clock cycles
            pub const Cycles41_5: u32 = 0b100;

            /// 0b101: 55.5 ADC clock cycles
            pub const Cycles55_5: u32 = 0b101;

            /// 0b110: 71.5 ADC clock cycles
            pub const Cycles71_5: u32 = 0b110;

            /// 0b111: 239.5 ADC clock cycles
            pub const Cycles239_5: u32 = 0b111;
        }
    }

    /// Channel 11 sample time selection
    pub mod SMP11 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP10::RW;
    }

    /// Channel 12 sample time selection
    pub mod SMP12 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP10::RW;
    }

    /// Channel 13 sample time selection
    pub mod SMP13 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP10::RW;
    }

    /// Channel 14 sample time selection
    pub mod SMP14 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP10::RW;
    }

    /// Channel 15 sample time selection
    pub mod SMP15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (3 bits: 0b111 << 15)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP10::RW;
    }

    /// Channel 16 sample time selection
    pub mod SMP16 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP10::RW;
    }

    /// Channel 17 sample time selection
    pub mod SMP17 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (3 bits: 0b111 << 21)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP10::RW;
    }
}

/// sample time register 2
pub mod SMPR2 {

    /// Channel 0 sample time selection
    pub mod SMP0 {
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

            /// 0b001: 7.5 ADC clock cycles
            pub const Cycles7_5: u32 = 0b001;

            /// 0b010: 13.5 ADC clock cycles
            pub const Cycles13_5: u32 = 0b010;

            /// 0b011: 28.5 ADC clock cycles
            pub const Cycles28_5: u32 = 0b011;

            /// 0b100: 41.5 ADC clock cycles
            pub const Cycles41_5: u32 = 0b100;

            /// 0b101: 55.5 ADC clock cycles
            pub const Cycles55_5: u32 = 0b101;

            /// 0b110: 71.5 ADC clock cycles
            pub const Cycles71_5: u32 = 0b110;

            /// 0b111: 239.5 ADC clock cycles
            pub const Cycles239_5: u32 = 0b111;
        }
    }

    /// Channel 1 sample time selection
    pub mod SMP1 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP0::RW;
    }

    /// Channel 2 sample time selection
    pub mod SMP2 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP0::RW;
    }

    /// Channel 3 sample time selection
    pub mod SMP3 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP0::RW;
    }

    /// Channel 4 sample time selection
    pub mod SMP4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP0::RW;
    }

    /// Channel 5 sample time selection
    pub mod SMP5 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (3 bits: 0b111 << 15)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP0::RW;
    }

    /// Channel 6 sample time selection
    pub mod SMP6 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP0::RW;
    }

    /// Channel 7 sample time selection
    pub mod SMP7 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (3 bits: 0b111 << 21)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP0::RW;
    }

    /// Channel 8 sample time selection
    pub mod SMP8 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP0::RW;
    }

    /// Channel 9 sample time selection
    pub mod SMP9 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (3 bits: 0b111 << 27)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP0::RW;
    }
}

/// injected channel data offset register x
pub mod JOFR1 {

    /// Data offset for injected channel x
    pub mod JOFFSET {
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

/// injected channel data offset register x
pub mod JOFR2 {
    pub use super::JOFR1::JOFFSET;
}

/// injected channel data offset register x
pub mod JOFR3 {
    pub use super::JOFR1::JOFFSET;
}

/// injected channel data offset register x
pub mod JOFR4 {
    pub use super::JOFR1::JOFFSET;
}

/// watchdog higher threshold register
pub mod HTR {

    /// Analog watchdog higher threshold
    pub mod HT {
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

/// watchdog lower threshold register
pub mod LTR {

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

/// regular sequence register 1
pub mod SQR1 {

    /// Regular channel sequence length
    pub mod L {
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

    /// 16th conversion in regular sequence
    pub mod SQ16 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (5 bits: 0b11111 << 15)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 15th conversion in regular sequence
    pub mod SQ15 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 14th conversion in regular sequence
    pub mod SQ14 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (5 bits: 0b11111 << 5)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 13th conversion in regular sequence
    pub mod SQ13 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// regular sequence register 2
pub mod SQR2 {

    /// 12th conversion in regular sequence
    pub mod SQ12 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (5 bits: 0b11111 << 25)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 11th conversion in regular sequence
    pub mod SQ11 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (5 bits: 0b11111 << 20)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 10th conversion in regular sequence
    pub mod SQ10 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (5 bits: 0b11111 << 15)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 9th conversion in regular sequence
    pub mod SQ9 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 8th conversion in regular sequence
    pub mod SQ8 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (5 bits: 0b11111 << 5)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 7th conversion in regular sequence
    pub mod SQ7 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// regular sequence register 3
pub mod SQR3 {

    /// 6th conversion in regular sequence
    pub mod SQ6 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (5 bits: 0b11111 << 25)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 5th conversion in regular sequence
    pub mod SQ5 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (5 bits: 0b11111 << 20)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 4th conversion in regular sequence
    pub mod SQ4 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (5 bits: 0b11111 << 15)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 3rd conversion in regular sequence
    pub mod SQ3 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 2nd conversion in regular sequence
    pub mod SQ2 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (5 bits: 0b11111 << 5)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 1st conversion in regular sequence
    pub mod SQ1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// injected sequence register
pub mod JSQR {

    /// Injected sequence length
    pub mod JL {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 4th conversion in injected sequence
    pub mod JSQ4 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (5 bits: 0b11111 << 15)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 3rd conversion in injected sequence
    pub mod JSQ3 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 2nd conversion in injected sequence
    pub mod JSQ2 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (5 bits: 0b11111 << 5)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 1st conversion in injected sequence
    pub mod JSQ1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// injected data register x
pub mod JDR1 {

    /// Injected data
    pub mod JDATA {
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

/// injected data register x
pub mod JDR2 {
    pub use super::JDR1::JDATA;
}

/// injected data register x
pub mod JDR3 {
    pub use super::JDR1::JDATA;
}

/// injected data register x
pub mod JDR4 {
    pub use super::JDR1::JDATA;
}

/// regular data register
pub mod DR {

    /// Regular data
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

    /// ADC2 data
    pub mod ADC2DATA {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
pub struct RegisterBlock {
    /// status register
    pub SR: RWRegister<u32>,

    /// control register 1
    pub CR1: RWRegister<u32>,

    /// control register 2
    pub CR2: RWRegister<u32>,

    /// sample time register 1
    pub SMPR1: RWRegister<u32>,

    /// sample time register 2
    pub SMPR2: RWRegister<u32>,

    /// injected channel data offset register x
    pub JOFR1: RWRegister<u32>,

    /// injected channel data offset register x
    pub JOFR2: RWRegister<u32>,

    /// injected channel data offset register x
    pub JOFR3: RWRegister<u32>,

    /// injected channel data offset register x
    pub JOFR4: RWRegister<u32>,

    /// watchdog higher threshold register
    pub HTR: RWRegister<u32>,

    /// watchdog lower threshold register
    pub LTR: RWRegister<u32>,

    /// regular sequence register 1
    pub SQR1: RWRegister<u32>,

    /// regular sequence register 2
    pub SQR2: RWRegister<u32>,

    /// regular sequence register 3
    pub SQR3: RWRegister<u32>,

    /// injected sequence register
    pub JSQR: RWRegister<u32>,

    /// injected data register x
    pub JDR1: RORegister<u32>,

    /// injected data register x
    pub JDR2: RORegister<u32>,

    /// injected data register x
    pub JDR3: RORegister<u32>,

    /// injected data register x
    pub JDR4: RORegister<u32>,

    /// regular data register
    pub DR: RORegister<u32>,
}
pub struct ResetValues {
    pub SR: u32,
    pub CR1: u32,
    pub CR2: u32,
    pub SMPR1: u32,
    pub SMPR2: u32,
    pub JOFR1: u32,
    pub JOFR2: u32,
    pub JOFR3: u32,
    pub JOFR4: u32,
    pub HTR: u32,
    pub LTR: u32,
    pub SQR1: u32,
    pub SQR2: u32,
    pub SQR3: u32,
    pub JSQR: u32,
    pub JDR1: u32,
    pub JDR2: u32,
    pub JDR3: u32,
    pub JDR4: u32,
    pub DR: u32,
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

/// Access functions for the ADC1 peripheral instance
pub mod ADC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40012400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC1
    pub const reset: ResetValues = ResetValues {
        SR: 0x00000000,
        CR1: 0x00000000,
        CR2: 0x00000000,
        SMPR1: 0x00000000,
        SMPR2: 0x00000000,
        JOFR1: 0x00000000,
        JOFR2: 0x00000000,
        JOFR3: 0x00000000,
        JOFR4: 0x00000000,
        HTR: 0x00000FFF,
        LTR: 0x00000000,
        SQR1: 0x00000000,
        SQR2: 0x00000000,
        SQR3: 0x00000000,
        JSQR: 0x00000000,
        JDR1: 0x00000000,
        JDR2: 0x00000000,
        JDR3: 0x00000000,
        JDR4: 0x00000000,
        DR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ADC1_TAKEN: bool = false;

    /// Safe access to ADC1
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
            if ADC1_TAKEN {
                None
            } else {
                ADC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC1_TAKEN && inst.addr == INSTANCE.addr {
                ADC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ADC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ADC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ADC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC1: *const RegisterBlock = 0x40012400 as *const _;
