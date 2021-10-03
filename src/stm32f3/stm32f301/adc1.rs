#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Analog to digital converter

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// ADC interrupt and status register
pub mod ISR {

    /// Injected context queue overflow
    pub mod JQOVF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No injected context queue overflow has occurred
            pub const NoOverflow: u32 = 0b0;

            /// 0b1: Injected context queue overflow has occurred
            pub const Overflow: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear injected context queue overflow flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog 3 flag
    pub mod AWD3 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
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

            /// 0b1: Clear analog watchdog event occurred flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog 2 flag
    pub mod AWD2 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        pub use super::AWD3::R;
        pub use super::AWD3::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog 1 flag
    pub mod AWD1 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        pub use super::AWD3::R;
        pub use super::AWD3::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected channel end of sequence flag
    pub mod JEOS {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Injected sequence is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Injected sequence complete
            pub const Complete: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear Injected sequence complete flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected channel end of conversion flag
    pub mod JEOC {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Injected conversion is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Injected conversion complete
            pub const Complete: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear injected conversion complete flag
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

            /// 0b1: Clear overrun occurred flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// End of regular sequence flag
    pub mod EOS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Regular sequence is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Regular sequence complete
            pub const Complete: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear regular sequence complete flag
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

            /// 0b0: Regular conversion is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Regular conversion complete
            pub const Complete: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear regular conversion complete flag
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

            /// 0b0: End of sampling phase no yet reached
            pub const NotEnded: u32 = 0b0;

            /// 0b1: End of sampling phase reached
            pub const Ended: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear end of sampling phase reached flag
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

            /// 0b0: ADC is not ready to start conversion
            pub const NotReady: u32 = 0b0;

            /// 0b1: ADC is ready to start conversion
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear ADC is ready to start conversion flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ADC interrupt enable register
pub mod IER {

    /// Injected context queue overflow interrupt enable
    pub mod JQOVFIE {
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

            /// 0b0: Injected context queue overflow interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Injected context queue overflow interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Analog watchdog 3 interrupt enable
    pub mod AWD3IE {
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

            /// 0b0: Analog watchdog interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Analog watchdog interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Analog watchdog 2 interrupt enable
    pub mod AWD2IE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3IE::RW;
    }

    /// Analog watchdog 1 interrupt enable
    pub mod AWD1IE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3IE::RW;
    }

    /// End of injected sequence of conversions interrupt enable
    pub mod JEOSIE {
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

            /// 0b0: End of injected sequence interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: End of injected sequence interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// End of injected conversion interrupt enable
    pub mod JEOCIE {
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

            /// 0b0: End of injected conversion interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: End of injected conversion interrupt enabled
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

    /// End of regular sequence of conversions interrupt enable
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

            /// 0b0: End of regular sequence interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: End of regular sequence interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// End of regular conversion interrupt enable
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

            /// 0b0: End of regular conversion interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: End of regular conversion interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// End of sampling flag interrupt enable for regular conversions
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

            /// 0b0: End of regular conversion sampling phase interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: End of regular conversion sampling phase interrupt enabled
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

/// ADC control register
pub mod CR {

    /// ADC calibration
    pub mod ADCAL {
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

            /// 0b0: Calibration complete
            pub const Complete: u32 = 0b0;

            /// 0b1: Start the calibration of the ADC
            pub const Calibration: u32 = 0b1;
        }
    }

    /// Differential mode for calibration
    pub mod ADCALDIF {
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

            /// 0b0: Calibration for single-ended mode
            pub const SingleEnded: u32 = 0b0;

            /// 0b1: Calibration for differential mode
            pub const Differential: u32 = 0b1;
        }
    }

    /// ADC voltage regulator enable
    pub mod ADVREGEN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Intermediate state required when moving the ADC voltage regulator between states
            pub const Intermediate: u32 = 0b00;

            /// 0b01: ADC voltage regulator enabled
            pub const Enabled: u32 = 0b01;

            /// 0b10: ADC voltage regulator disabled
            pub const Disabled: u32 = 0b10;
        }
    }

    /// ADC stop of injected conversion command
    pub mod JADST {
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

    /// ADC stop of regular conversion command
    pub mod ADSTP {
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

            /// 0b1: Stop conversion of channel
            pub const Stop: u32 = 0b1;
        }
    }

    /// ADC start of injected conversion
    pub mod JADSTART {
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

            /// 0b1: Starts conversion of channel
            pub const Start: u32 = 0b1;
        }
    }

    /// ADC start of regular conversion
    pub mod ADSTART {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::JADSTART::RW;
    }

    /// ADC disable command
    pub mod ADDIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: Disable ADC conversion and go to power down mode
            pub const Disable: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADC enable control
    pub mod ADEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Enable ADC
            pub const Enable: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ADC configuration register
pub mod CFGR {

    /// Analog watchdog 1 channel selection
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

    /// Automatic injected group conversion
    pub mod JAUTO {
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

            /// 0b0: Automatic injected group conversion disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Automatic injected group conversion enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Analog watchdog 1 enable on injected channels
    pub mod JAWD1EN {
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

            /// 0b0: Analog watchdog 1 disabled on injected channels
            pub const Disabled: u32 = 0b0;

            /// 0b1: Analog watchdog 1 enabled on injected channels
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Analog watchdog 1 enable on regular channels
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

            /// 0b0: Analog watchdog 1 disabled on regular channels
            pub const Disabled: u32 = 0b0;

            /// 0b1: Analog watchdog 1 enabled on regular channels
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Enable the watchdog 1 on a single channel or on all channels
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
            pub const All: u32 = 0b0;

            /// 0b1: Analog watchdog 1 enabled on single channel selected in AWD1CH
            pub const Single: u32 = 0b1;
        }
    }

    /// JSQR queue mode
    pub mod JQM {
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

            /// 0b0: JSQR Mode 0: Queue maintains the last written configuration into JSQR
            pub const Mode0: u32 = 0b0;

            /// 0b1: JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence
            pub const Mode1: u32 = 0b1;
        }
    }

    /// Discontinuous mode on injected channels
    pub mod JDISCEN {
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

            /// 0b0: Discontinuous mode on injected channels disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Discontinuous mode on injected channels enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Discontinuous mode channel count
    pub mod DISCNUM {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (3 bits: 0b111 << 17)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Discontinuous mode for regular channels
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

    /// Delayed conversion mode
    pub mod AUTDLY {
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

            /// 0b0: Auto delayed conversion mode off
            pub const Off: u32 = 0b0;

            /// 0b1: Auto delayed conversion mode on
            pub const On: u32 = 0b1;
        }
    }

    /// Single / continuous conversion mode for regular conversions
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

    /// Overrun Mode
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

            /// 0b0: Preserve DR register when an overrun is detected
            pub const Preserve: u32 = 0b0;

            /// 0b1: Overwrite DR register when an overrun is detected
            pub const Overwrite: u32 = 0b1;
        }
    }

    /// External trigger enable and polarity selection for regular channels
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

    /// External trigger selection for regular group
    pub mod EXTSEL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (4 bits: 0b1111 << 6)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0111: HRTIM_ADCTRG1 event
            pub const HRTIM_ADCTRG1: u32 = 0b0111;

            /// 0b1000: HRTIM_ADCTRG3 event
            pub const HRTIM_ADCTRG3: u32 = 0b1000;

            /// 0b0000: Timer 1 CC1 event
            pub const TIM1_CC1: u32 = 0b0000;

            /// 0b0001: Timer 1 CC2 event
            pub const TIM1_CC2: u32 = 0b0001;

            /// 0b0010: Timer 1 CC3 event
            pub const TIM1_CC3: u32 = 0b0010;

            /// 0b0011: Timer 2 CC2 event
            pub const TIM2_CC2: u32 = 0b0011;

            /// 0b0100: Timer 3 TRGO event
            pub const TIM3_TRGO: u32 = 0b0100;

            /// 0b0110: EXTI line 11
            pub const EXTI11: u32 = 0b0110;

            /// 0b1001: Timer 1 TRGO event
            pub const TIM1_TRGO: u32 = 0b1001;

            /// 0b1010: Timer 1 TRGO2 event
            pub const TIM1_TRGO2: u32 = 0b1010;

            /// 0b1011: Timer 2 TRGO event
            pub const TIM2_TRGO: u32 = 0b1011;

            /// 0b1101: Timer 6 TRGO event
            pub const TIM6_TRGO: u32 = 0b1101;

            /// 0b1110: Timer 15 TRGO event
            pub const TIM15_TRGO: u32 = 0b1110;

            /// 0b1111: Timer 3 CC4 event
            pub const TIM3_CC4: u32 = 0b1111;
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

            /// 0b00: 12-bit
            pub const Bits12: u32 = 0b00;

            /// 0b01: 10-bit
            pub const Bits10: u32 = 0b01;

            /// 0b10: 8-bit
            pub const Bits8: u32 = 0b10;

            /// 0b11: 6-bit
            pub const Bits6: u32 = 0b11;
        }
    }

    /// Direct memory access configuration
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

            /// 0b0: DMA One Shot Mode selected
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

/// ADC sample time register 1
pub mod SMPR1 {

    /// Channel 1 sampling time selection
    pub mod SMP1 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 1.5 ADC clock cycles
            pub const Cycles1_5: u32 = 0b000;

            /// 0b001: 2.5 ADC clock cycles
            pub const Cycles2_5: u32 = 0b001;

            /// 0b010: 4.5 ADC clock cycles
            pub const Cycles4_5: u32 = 0b010;

            /// 0b011: 7.5 ADC clock cycles
            pub const Cycles7_5: u32 = 0b011;

            /// 0b100: 19.5 ADC clock cycles
            pub const Cycles19_5: u32 = 0b100;

            /// 0b101: 61.5 ADC clock cycles
            pub const Cycles61_5: u32 = 0b101;

            /// 0b110: 181.5 ADC clock cycles
            pub const Cycles181_5: u32 = 0b110;

            /// 0b111: 601.5 ADC clock cycles
            pub const Cycles601_5: u32 = 0b111;
        }
    }

    /// Channel 2 sampling time selection
    pub mod SMP2 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP1::RW;
    }

    /// Channel 3 sampling time selection
    pub mod SMP3 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP1::RW;
    }

    /// Channel 4 sampling time selection
    pub mod SMP4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP1::RW;
    }

    /// Channel 5 sampling time selection
    pub mod SMP5 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (3 bits: 0b111 << 15)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP1::RW;
    }

    /// Channel 6 sampling time selection
    pub mod SMP6 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP1::RW;
    }

    /// Channel 7 sampling time selection
    pub mod SMP7 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (3 bits: 0b111 << 21)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP1::RW;
    }

    /// Channel 8 sampling time selection
    pub mod SMP8 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP1::RW;
    }

    /// Channel 9 sampling time selection
    pub mod SMP9 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (3 bits: 0b111 << 27)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP1::RW;
    }
}

/// ADC sample time register 2
pub mod SMPR2 {

    /// Channel 10 sampling time selection
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

            /// 0b001: 2.5 ADC clock cycles
            pub const Cycles2_5: u32 = 0b001;

            /// 0b010: 4.5 ADC clock cycles
            pub const Cycles4_5: u32 = 0b010;

            /// 0b011: 7.5 ADC clock cycles
            pub const Cycles7_5: u32 = 0b011;

            /// 0b100: 19.5 ADC clock cycles
            pub const Cycles19_5: u32 = 0b100;

            /// 0b101: 61.5 ADC clock cycles
            pub const Cycles61_5: u32 = 0b101;

            /// 0b110: 181.5 ADC clock cycles
            pub const Cycles181_5: u32 = 0b110;

            /// 0b111: 601.5 ADC clock cycles
            pub const Cycles601_5: u32 = 0b111;
        }
    }

    /// Channel 11 sampling time selection
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

    /// Channel 12 sampling time selection
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

    /// Channel 13 sampling time selection
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

    /// Channel 14 sampling time selection
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

    /// Channel 15 sampling time selection
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

    /// Channel 16 sampling time selection
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

    /// Channel 17 sampling time selection
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

    /// Channel 18 sampling time selection
    pub mod SMP18 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SMP10::RW;
    }
}

/// ADC watchdog threshold register 1
pub mod TR1 {

    /// Analog watchdog 1 higher threshold
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

    /// Analog watchdog 1 lower threshold
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
}

/// ADC watchdog threshold register 2
pub mod TR2 {

    /// Analog watchdog 2 higher threshold
    pub mod HT2 {
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

    /// Analog watchdog 2 lower threshold
    pub mod LT2 {
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

/// read-write
pub mod TR3 {

    /// Analog watchdog 3 higher threshold
    pub mod HT3 {
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

    /// Analog watchdog 3 lower threshold
    pub mod LT3 {
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

/// ADC regular sequence register 1
pub mod SQR1 {

    /// 4th conversion in regular sequence
    pub mod SQ4 {
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

    /// 3rd conversion in regular sequence
    pub mod SQ3 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (5 bits: 0b11111 << 18)
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
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (5 bits: 0b11111 << 12)
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
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Regular channel sequence length
    pub mod L {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ADC regular sequence register 2
pub mod SQR2 {

    /// 9th conversion in regular sequence
    pub mod SQ9 {
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

    /// 8th conversion in regular sequence
    pub mod SQ8 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (5 bits: 0b11111 << 18)
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
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (5 bits: 0b11111 << 12)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 6th conversion in regular sequence
    pub mod SQ6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
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

/// ADC regular sequence register 3
pub mod SQR3 {

    /// 14th conversion in regular sequence
    pub mod SQ14 {
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

    /// 13th conversion in regular sequence
    pub mod SQ13 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (5 bits: 0b11111 << 18)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 13th conversion in regular sequence
    pub mod SQ12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (5 bits: 0b11111 << 12)
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
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
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

/// ADC regular sequence register 4
pub mod SQR4 {

    /// 16th conversion in regular sequence
    pub mod SQ16 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
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

/// ADC regular Data Register
pub mod DR {

    /// Regular Data converted
    pub mod RDATA {
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

/// ADC injected sequence register
pub mod JSQR {

    /// 4th conversion in the injected sequence
    pub mod JSQ4 {
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

    /// 3rd conversion in the injected sequence
    pub mod JSQ3 {
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

    /// 2nd conversion in the injected sequence
    pub mod JSQ2 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (5 bits: 0b11111 << 14)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 1st conversion in the injected sequence
    pub mod JSQ1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Trigger Enable and Polarity Selection for injected channels
    pub mod JEXTEN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
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

    /// External Trigger Selection for injected group
    pub mod JEXTSEL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (4 bits: 0b1111 << 2)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1001: HRTIM_ADCTRG2 event
            pub const HRTIM_ADCTRG2: u32 = 0b1001;

            /// 0b1010: HRTIM_ADCTRG4 event
            pub const HRTIM_ADCTRG4: u32 = 0b1010;

            /// 0b0000: Timer 1 TRGO event
            pub const TIM1_TRGO: u32 = 0b0000;

            /// 0b0001: Timer 1 CC4 event
            pub const TIM1_CC4: u32 = 0b0001;

            /// 0b0010: Timer 2 TRGO event
            pub const TIM2_TRGO: u32 = 0b0010;

            /// 0b0011: Timer 2 CC1 event
            pub const TIM2_CC1: u32 = 0b0011;

            /// 0b0100: Timer 3 CC4 event
            pub const TIM3_CC4: u32 = 0b0100;

            /// 0b0110: EXTI line 15
            pub const EXTI15: u32 = 0b0110;

            /// 0b1000: Timer 1 TRGO2 event
            pub const TIM1_TRGO2: u32 = 0b1000;

            /// 0b1011: Timer 3 CC3 event
            pub const TIM3_CC3: u32 = 0b1011;

            /// 0b1100: Timer 3 TRGO event
            pub const TIM3_TRGO: u32 = 0b1100;

            /// 0b1101: Timer 3 CC1 event
            pub const TIM3_CC1: u32 = 0b1101;

            /// 0b1110: Timer 6 TRGO event
            pub const TIM6_TRGO: u32 = 0b1110;

            /// 0b1111: Timer 15 TRGO event
            pub const TIM15_TRGO: u32 = 0b1111;
        }
    }

    /// Injected channel sequence length
    pub mod JL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ADC offset register1
pub mod OFR1 {

    /// Offset1 Enable
    pub mod OFFSET1_EN {
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

            /// 0b0: Offset disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Offset enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Channel selection for the Data offset 1
    pub mod OFFSET1_CH {
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

    /// Data offset 1 for the channel programmed into bits OFFSET1_CH
    pub mod OFFSET1 {
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

/// ADC offset register2
pub mod OFR2 {

    /// Offset 2 Enable
    pub mod OFFSET2_EN {
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

            /// 0b0: Offset disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Offset enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Channel selection for the Data offset 2
    pub mod OFFSET2_CH {
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

    /// Data offset 2 for the channel programmed into bits OFFSET2_CH
    pub mod OFFSET2 {
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

/// ADC offset register3
pub mod OFR3 {

    /// Offset y Enable
    pub mod OFFSET3_EN {
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

            /// 0b0: Offset disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Offset enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Channel selection for the Data offset 3
    pub mod OFFSET3_CH {
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

    /// Data offset 3 for the channel programmed into bits OFFSET3_CH
    pub mod OFFSET3 {
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

/// ADC offset register4
pub mod OFR4 {

    /// Offset 4 Enable
    pub mod OFFSET4_EN {
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

            /// 0b0: Offset disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Offset enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Channel selection for the Data offset 4
    pub mod OFFSET4_CH {
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

    /// Data offset 4 for the channel programmed into bits OFFSET4_CH
    pub mod OFFSET4 {
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

/// ADC offset register1
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

/// ADC offset register2
pub mod JDR2 {
    pub use super::JDR1::JDATA;
}

/// ADC offset register3
pub mod JDR3 {
    pub use super::JDR1::JDATA;
}

/// ADC offset register4
pub mod JDR4 {
    pub use super::JDR1::JDATA;
}

/// ADC Analog Watchdog 2 Configuration Register
pub mod AWD2CR {

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH0 {
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

            /// 0b0: Input channel not monitored by AWDx
            pub const NotMonitored: u32 = 0b0;

            /// 0b1: Input channel monitored by AWDx
            pub const Monitored: u32 = 0b1;
        }
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH2 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH3 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH4 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH5 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH6 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH7 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH8 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH9 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH10 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH11 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH12 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH13 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH14 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH15 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH16 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }

    /// Analog watchdog 2 channel selection
    pub mod AWD2CH17 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD2CH0::RW;
    }
}

/// ADC Analog Watchdog 3 Configuration Register
pub mod AWD3CR {

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH0 {
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

            /// 0b0: Input channel not monitored by AWDx
            pub const NotMonitored: u32 = 0b0;

            /// 0b1: Input channel monitored by AWDx
            pub const Monitored: u32 = 0b1;
        }
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH2 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH3 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH4 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH5 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH6 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH7 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH8 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH9 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH10 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH11 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH12 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH13 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH14 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH15 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH16 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }

    /// Analog watchdog 3 channel selection
    pub mod AWD3CH17 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD3CH0::RW;
    }
}

/// ADC Differential Mode Selection Register
pub mod DIFSEL {

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL0 {
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

            /// 0b0: Input channel is configured in single-ended mode
            pub const SingleEnded: u32 = 0b0;

            /// 0b1: Input channel is configured in differential mode
            pub const Differential: u32 = 0b1;
        }
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL2 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL3 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL4 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL5 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL6 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL7 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL8 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL9 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL10 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL11 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL12 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL13 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL14 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL15 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL16 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }

    /// Differential mode for channels 15 to 1
    pub mod DIFSEL17 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIFSEL0::RW;
    }
}

/// ADC Calibration Factors
pub mod CALFACT {

    /// Calibration Factors in differential mode
    pub mod CALFACT_D {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (7 bits: 0x7f << 16)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Calibration Factors In Single-Ended mode
    pub mod CALFACT_S {
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
#[repr(C)]
pub struct RegisterBlock {
    /// ADC interrupt and status register
    pub ISR: RWRegister<u32>,

    /// ADC interrupt enable register
    pub IER: RWRegister<u32>,

    /// ADC control register
    pub CR: RWRegister<u32>,

    /// ADC configuration register
    pub CFGR: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// ADC sample time register 1
    pub SMPR1: RWRegister<u32>,

    /// ADC sample time register 2
    pub SMPR2: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// ADC watchdog threshold register 1
    pub TR1: RWRegister<u32>,

    /// ADC watchdog threshold register 2
    pub TR2: RWRegister<u32>,

    /// read-write
    pub TR3: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// ADC regular sequence register 1
    pub SQR1: RWRegister<u32>,

    /// ADC regular sequence register 2
    pub SQR2: RWRegister<u32>,

    /// ADC regular sequence register 3
    pub SQR3: RWRegister<u32>,

    /// ADC regular sequence register 4
    pub SQR4: RWRegister<u32>,

    /// ADC regular Data Register
    pub DR: RORegister<u32>,

    _reserved4: [u32; 2],

    /// ADC injected sequence register
    pub JSQR: RWRegister<u32>,

    _reserved5: [u32; 4],

    /// ADC offset register1
    pub OFR1: RWRegister<u32>,

    /// ADC offset register2
    pub OFR2: RWRegister<u32>,

    /// ADC offset register3
    pub OFR3: RWRegister<u32>,

    /// ADC offset register4
    pub OFR4: RWRegister<u32>,

    _reserved6: [u32; 4],

    /// ADC offset register1
    pub JDR1: RORegister<u32>,

    /// ADC offset register2
    pub JDR2: RORegister<u32>,

    /// ADC offset register3
    pub JDR3: RORegister<u32>,

    /// ADC offset register4
    pub JDR4: RORegister<u32>,

    _reserved7: [u32; 4],

    /// ADC Analog Watchdog 2 Configuration Register
    pub AWD2CR: RWRegister<u32>,

    /// ADC Analog Watchdog 3 Configuration Register
    pub AWD3CR: RWRegister<u32>,

    _reserved8: [u32; 2],

    /// ADC Differential Mode Selection Register
    pub DIFSEL: RWRegister<u32>,

    /// ADC Calibration Factors
    pub CALFACT: RWRegister<u32>,
}
pub struct ResetValues {
    pub ISR: u32,
    pub IER: u32,
    pub CR: u32,
    pub CFGR: u32,
    pub SMPR1: u32,
    pub SMPR2: u32,
    pub TR1: u32,
    pub TR2: u32,
    pub TR3: u32,
    pub SQR1: u32,
    pub SQR2: u32,
    pub SQR3: u32,
    pub SQR4: u32,
    pub DR: u32,
    pub JSQR: u32,
    pub OFR1: u32,
    pub OFR2: u32,
    pub OFR3: u32,
    pub OFR4: u32,
    pub JDR1: u32,
    pub JDR2: u32,
    pub JDR3: u32,
    pub JDR4: u32,
    pub AWD2CR: u32,
    pub AWD3CR: u32,
    pub DIFSEL: u32,
    pub CALFACT: u32,
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

/// Access functions for the ADC1 peripheral instance
pub mod ADC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC1
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IER: 0x00000000,
        CR: 0x20000000,
        CFGR: 0x00000000,
        SMPR1: 0x00000000,
        SMPR2: 0x00000000,
        TR1: 0x0FFF0000,
        TR2: 0x0FFF0000,
        TR3: 0x0FFF0000,
        SQR1: 0x00000000,
        SQR2: 0x00000000,
        SQR3: 0x00000000,
        SQR4: 0x00000000,
        DR: 0x00000000,
        JSQR: 0x00000000,
        OFR1: 0x00000000,
        OFR2: 0x00000000,
        OFR3: 0x00000000,
        OFR4: 0x00000000,
        JDR1: 0x00000000,
        JDR2: 0x00000000,
        JDR3: 0x00000000,
        JDR4: 0x00000000,
        AWD2CR: 0x00000000,
        AWD3CR: 0x00000000,
        DIFSEL: 0x00000000,
        CALFACT: 0x00000000,
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
pub const ADC1: *const RegisterBlock = 0x50000000 as *const _;
