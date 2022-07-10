#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Voltage reference buffer
//!
//! Used by: stm32wl5x_cm0p, stm32wl5x_cm4, stm32wle5

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control and status register
pub mod CSR {

    /// Voltage reference buffer ready
    pub mod VRR {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: The voltage reference buffer output is not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: The voltage reference buffer output reached the requested level
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Voltage reference scale
    pub mod VRS {
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

            /// 0b0: Voltage reference set to VREF_OUT1 (around 2.048 V)
            pub const V2_048: u32 = 0b0;

            /// 0b1: Voltage reference set to VREF_OUT2 (around 2.5 V)
            pub const V2_5: u32 = 0b1;
        }
    }

    /// High impedance mode
    pub mod HIZ {
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

            /// 0b0: VREF+ pin is internally connected to the voltage reference buffer output
            pub const Connected: u32 = 0b0;

            /// 0b1: VREF+ pin is high impedance
            pub const HighZ: u32 = 0b1;
        }
    }

    /// Voltage reference buffer mode enable
    pub mod ENVR {
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

            /// 0b0: Internal voltage reference mode disable (external voltage reference mode)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Internal voltage reference mode (reference buffer enable or hold mode) enable
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// calibration control register
pub mod CCR {

    /// Trimming code
    pub mod TRIM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
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
    /// control and status register
    pub CSR: RWRegister<u32>,

    /// calibration control register
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
