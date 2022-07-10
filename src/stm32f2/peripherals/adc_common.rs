#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Common ADC registers
//!
//! Used by: stm32f215, stm32f217

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// ADC Common status register
pub mod CSR {

    /// Overrun flag of ADC3
    pub mod OVR3 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No overrun occurred
            pub const NoOverrun: u32 = 0b0;

            /// 0b1: Overrun occurred
            pub const Overrun: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Regular channel Start flag of ADC 3
    pub mod STRT3 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No regular channel conversion started
            pub const NotStarted: u32 = 0b0;

            /// 0b1: Regular channel conversion has started
            pub const Started: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected channel Start flag of ADC 3
    pub mod JSTRT3 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No injected channel conversion started
            pub const NotStarted: u32 = 0b0;

            /// 0b1: Injected channel conversion has started
            pub const Started: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected channel end of conversion of ADC 3
    pub mod JEOC3 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Conversion is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Conversion complete
            pub const Complete: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// End of conversion of ADC 3
    pub mod EOC3 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        pub use super::JEOC3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog flag of ADC 3
    pub mod AWD3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No analog watchdog event occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Analog watchdog event occurred
            pub const Event: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overrun flag of ADC 2
    pub mod OVR2 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        pub use super::OVR3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Regular channel Start flag of ADC 2
    pub mod STRT2 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        pub use super::STRT3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected channel Start flag of ADC 2
    pub mod JSTRT2 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        pub use super::JSTRT3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected channel end of conversion of ADC 2
    pub mod JEOC2 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        pub use super::JEOC3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// End of conversion of ADC 2
    pub mod EOC2 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        pub use super::JEOC3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog flag of ADC 2
    pub mod AWD2 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        pub use super::AWD3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overrun flag of ADC 1
    pub mod OVR1 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        pub use super::OVR3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Regular channel Start flag of ADC 1
    pub mod STRT1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::STRT3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected channel Start flag of ADC 1
    pub mod JSTRT1 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::JSTRT3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected channel end of conversion of ADC 1
    pub mod JEOC1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::JEOC3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// End of conversion of ADC 1
    pub mod EOC1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::JEOC3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog flag of ADC 1
    pub mod AWD1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        pub use super::AWD3::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ADC common control register
pub mod CCR {

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

    /// VBAT enable
    pub mod VBATE {
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

            /// 0b0: V_BAT channel disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: V_BAT channel enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADC prescaler
    pub mod ADCPRE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: PCLK2 divided by 2
            pub const Div2: u32 = 0b00;

            /// 0b01: PCLK2 divided by 4
            pub const Div4: u32 = 0b01;

            /// 0b10: PCLK2 divided by 6
            pub const Div6: u32 = 0b10;

            /// 0b11: PCLK2 divided by 8
            pub const Div8: u32 = 0b11;
        }
    }

    /// Direct memory access mode for multi ADC mode
    pub mod DMA {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: DMA mode disabled
            pub const Disabled: u32 = 0b00;

            /// 0b01: DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)
            pub const Mode1: u32 = 0b01;

            /// 0b10: DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)
            pub const Mode2: u32 = 0b10;

            /// 0b11: DMA mode 3 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)
            pub const Mode3: u32 = 0b11;
        }
    }

    /// DMA disable selection for multi-ADC mode
    pub mod DDS {
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

            /// 0b0: No new DMA request is issued after the last transfer
            pub const Single: u32 = 0b0;

            /// 0b1: DMA requests are issued as long as data are converted and DMA=01, 10 or 11
            pub const Continuous: u32 = 0b1;
        }
    }

    /// Delay between 2 sampling phases
    pub mod DELAY {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Multi ADC mode selection
    pub mod MULTI {
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

            /// 0b00000: All the ADCs independent: independent mode
            pub const Independent: u32 = 0b00000;

            /// 0b00001: Dual ADC1 and ADC2, combined regular and injected simultaneous mode
            pub const DualRJ: u32 = 0b00001;

            /// 0b00010: Dual ADC1 and ADC2, combined regular and alternate trigger mode
            pub const DualRA: u32 = 0b00010;

            /// 0b00101: Dual ADC1 and ADC2, injected simultaneous mode only
            pub const DualJ: u32 = 0b00101;

            /// 0b00110: Dual ADC1 and ADC2, regular simultaneous mode only
            pub const DualR: u32 = 0b00110;

            /// 0b00111: Dual ADC1 and ADC2, interleaved mode only
            pub const DualI: u32 = 0b00111;

            /// 0b01001: Dual ADC1 and ADC2, alternate trigger mode only
            pub const DualA: u32 = 0b01001;

            /// 0b10001: Triple ADC, regular and injected simultaneous mode
            pub const TripleRJ: u32 = 0b10001;

            /// 0b10010: Triple ADC, regular and alternate trigger mode
            pub const TripleRA: u32 = 0b10010;

            /// 0b10101: Triple ADC, injected simultaneous mode only
            pub const TripleJ: u32 = 0b10101;

            /// 0b10110: Triple ADC, regular simultaneous mode only
            pub const TripleR: u32 = 0b10110;

            /// 0b10111: Triple ADC, interleaved mode only
            pub const TripleI: u32 = 0b10111;

            /// 0b11000: Triple ADC, alternate trigger mode only
            pub const TripleA: u32 = 0b11000;
        }
    }
}

/// ADC common regular data register for dual and triple modes
pub mod CDR {

    /// 2nd data item of a pair of regular conversions
    pub mod DATA2 {
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

    /// 1st data item of a pair of regular conversions
    pub mod DATA1 {
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
#[repr(C)]
pub struct RegisterBlock {
    /// ADC Common status register
    pub CSR: RORegister<u32>,

    /// ADC common control register
    pub CCR: RWRegister<u32>,

    /// ADC common regular data register for dual and triple modes
    pub CDR: RORegister<u32>,
}
pub struct ResetValues {
    pub CSR: u32,
    pub CCR: u32,
    pub CDR: u32,
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
