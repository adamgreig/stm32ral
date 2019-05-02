#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CRC calculation unit
//!
//! Used by: stm32f100, stm32f101, stm32f102, stm32f103, stm32f107

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RWRegister, WORegister};

/// Data register
pub mod DR {

    /// Data Register
    pub mod DR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Independent Data register
pub mod IDR {

    /// Independent Data register
    pub mod IDR {
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

/// Control register
pub mod CR {

    /// Reset bit
    pub mod RESET {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF
            pub const Reset: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}
pub struct RegisterBlock {
    /// Data register
    pub DR: RWRegister<u32>,

    /// Independent Data register
    pub IDR: RWRegister<u32>,

    /// Control register
    pub CR: WORegister<u32>,
}
pub struct ResetValues {
    pub DR: u32,
    pub IDR: u32,
    pub CR: u32,
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
