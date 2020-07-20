#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ADC common registers
//!
//! Used by: stm32f410, stm32f412, stm32f413

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// ADC Common status register
pub mod CSR {

    /// Overrun flag of ADC 1
    pub mod OVR1 {
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

            /// 0b0: No overrun occurred
            pub const NoOverrun: u32 = 0b0;

            /// 0b1: Overrun occurred
            pub const Overrun: u32 = 0b1;
        }
    }

    /// Regular channel Start flag of ADC 1
    pub mod STRT1 {
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

            /// 0b0: No regular channel conversion started
            pub const NotStarted: u32 = 0b0;

            /// 0b1: Regular channel conversion has started
            pub const Started: u32 = 0b1;
        }
    }

    /// Injected channel Start flag of ADC 1
    pub mod JSTRT1 {
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

            /// 0b0: No injected channel conversion started
            pub const NotStarted: u32 = 0b0;

            /// 0b1: Injected channel conversion has started
            pub const Started: u32 = 0b1;
        }
    }

    /// Injected channel end of conversion of ADC 1
    pub mod JEOC1 {
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

            /// 0b0: Conversion is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Conversion complete
            pub const Complete: u32 = 0b1;
        }
    }

    /// End of conversion of ADC 1
    pub mod EOC1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::JEOC1::RW;
    }

    /// Analog watchdog flag of ADC 1
    pub mod AWD1 {
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

            /// 0b0: No analog watchdog event occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Analog watchdog event occurred
            pub const Event: u32 = 0b1;
        }
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// ADC Common status register
    pub CSR: RORegister<u32>,

    /// ADC common control register
    pub CCR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CSR: u32,
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
