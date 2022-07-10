#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Comparator
//!
//! Used by: stm32g0b1, stm32g0c1

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Comparator 1 control and status register
pub mod COMP1_CSR {

    /// Comparator 1 enable bit This bit is controlled by software (if not locked). It enables the comparator 1:
    pub mod EN {
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

    /// Comparator 1 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP1_INM of the comparator 1: > 1000: 1/4 VREFINT
    pub mod INMSEL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Comparator 1 signal selector for non-inverting input This bitfield is controlled by software (if not locked). It selects the signal for the non-inverting input COMP1_INP of the comparator 1 (also see the WINMODE bit):
    pub mod INPSEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Comparator 1 non-inverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP1_INP input of the comparator 1:
    pub mod WINMODE {
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

    /// Comparator 1 output selector This bit is controlled by software (if not locked). It selects the comparator 1 output:
    pub mod WINOUT {
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

    /// Comparator 1 polarity selector This bit is controlled by software (if not locked). It selects the comparator 1 output polarity:
    pub mod POLARITY {
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

    /// Comparator 1 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 1:
    pub mod HYST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Comparator 1 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 1: others: Reserved
    pub mod PWRMODE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Comparator 1 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: xxxx1: TIM1 OC4 xxx1x: TIM1 OC5 xx1xx: TIM2 OC3 x1xxx: TIM3 OC3 1xxxx: TIM15 OC2
    pub mod BLANKSEL {
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

    /// Comparator 1 output status This bit is read-only. It reflects the level of the comparator 1 output after the polarity selector and blanking, as indicated in .
    pub mod VALUE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// COMP1_CSR register lock This bit is set by software and cleared by a system reset. It locks the whole content of the comparator 1 control register COMP1_CSR\[31:0\]:
    pub mod LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Comparator 2 control and status register
pub mod COMP2_CSR {
    pub use super::COMP1_CSR::BLANKSEL;
    pub use super::COMP1_CSR::EN;
    pub use super::COMP1_CSR::HYST;
    pub use super::COMP1_CSR::INMSEL;
    pub use super::COMP1_CSR::INPSEL;
    pub use super::COMP1_CSR::LOCK;
    pub use super::COMP1_CSR::POLARITY;
    pub use super::COMP1_CSR::PWRMODE;
    pub use super::COMP1_CSR::VALUE;
    pub use super::COMP1_CSR::WINMODE;
    pub use super::COMP1_CSR::WINOUT;
}

/// Comparator 2 control and status register
pub mod COMP3_CSR {
    pub use super::COMP1_CSR::BLANKSEL;
    pub use super::COMP1_CSR::EN;
    pub use super::COMP1_CSR::HYST;
    pub use super::COMP1_CSR::INMSEL;
    pub use super::COMP1_CSR::INPSEL;
    pub use super::COMP1_CSR::LOCK;
    pub use super::COMP1_CSR::POLARITY;
    pub use super::COMP1_CSR::PWRMODE;
    pub use super::COMP1_CSR::VALUE;
    pub use super::COMP1_CSR::WINMODE;
    pub use super::COMP1_CSR::WINOUT;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Comparator 1 control and status register
    pub COMP1_CSR: RWRegister<u32>,

    /// Comparator 2 control and status register
    pub COMP2_CSR: RWRegister<u32>,

    /// Comparator 2 control and status register
    pub COMP3_CSR: RWRegister<u32>,
}
pub struct ResetValues {
    pub COMP1_CSR: u32,
    pub COMP2_CSR: u32,
    pub COMP3_CSR: u32,
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
