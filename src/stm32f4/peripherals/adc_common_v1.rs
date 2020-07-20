#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ADC common registers
//!
//! Used by: stm32f401, stm32f411

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

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
    _reserved1: [u32; 1],

    /// ADC common control register
    pub CCR: RWRegister<u32>,
}
pub struct ResetValues {
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
