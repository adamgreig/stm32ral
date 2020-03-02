#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Cyclic redundancy check calculation unit
//!
//! Used by: stm32l0x1, stm32l0x2, stm32l0x3

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Data register
pub mod DR {

    /// Data register bits
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

/// Independent data register
pub mod IDR {

    /// General-purpose 8-bit data register bits
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

    /// Reverse output data
    pub mod REV_OUT {
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

            /// 0b0: Bit order not affected
            pub const Forward: u32 = 0b0;

            /// 0b1: Bit-reversed output format
            pub const Reverse: u32 = 0b1;
        }
    }

    /// Reverse input data
    pub mod REV_IN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Bit order not affected
            pub const Forward: u32 = 0b00;

            /// 0b01: Bit reversal done by byte
            pub const ReverseByte: u32 = 0b01;

            /// 0b10: Bit reversal done by half-word
            pub const ReverseHalfWord: u32 = 0b10;

            /// 0b11: Bit reversal done by word
            pub const ReverseWord: u32 = 0b11;
        }
    }

    /// Polynomial size
    pub mod POLYSIZE {
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

            /// 0b00: 32 bit polynomial
            pub const Polysize32: u32 = 0b00;

            /// 0b01: 16 bit polynomial
            pub const Polysize16: u32 = 0b01;

            /// 0b10: 8 bit polynomial
            pub const Polysize8: u32 = 0b10;

            /// 0b11: 7 bit polynomial
            pub const Polysize7: u32 = 0b11;
        }
    }

    /// RESET bit
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

/// Initial CRC value
pub mod INIT {

    /// Programmable initial CRC value
    pub mod CRC_INIT {
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

/// polynomial
pub mod POL {

    /// Programmable polynomial
    pub mod POL {
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
#[repr(C)]
pub struct RegisterBlock {
    /// Data register
    pub DR: RWRegister<u32>,

    /// Independent data register
    pub IDR: RWRegister<u32>,

    /// Control register
    pub CR: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// Initial CRC value
    pub INIT: RWRegister<u32>,

    /// polynomial
    pub POL: RWRegister<u32>,
}
pub struct ResetValues {
    pub DR: u32,
    pub IDR: u32,
    pub CR: u32,
    pub INIT: u32,
    pub POL: u32,
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
