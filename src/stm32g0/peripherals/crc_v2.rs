#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Cyclic redundancy check calculation unit
//!
//! Used by: stm32g050, stm32g051, stm32g061, stm32g0b0, stm32g0b1, stm32g0c1

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Data register
pub mod CRC_DR {

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
pub mod CRC_IDR {

    /// General-purpose 32-bit data register bits
    pub mod IDR {
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

/// Control register
pub mod CRC_CR {

    /// Reverse output data This bit controls the reversal of the bit order of the output data.
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
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Bit-reversed output format
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Reverse input data These bits control the reversal of the bit order of the input data
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
            pub const B_0x0: u32 = 0b00;

            /// 0b01: Bit reversal done by byte
            pub const B_0x1: u32 = 0b01;

            /// 0b10: Bit reversal done by half-word
            pub const B_0x2: u32 = 0b10;

            /// 0b11: Bit reversal done by word
            pub const B_0x3: u32 = 0b11;
        }
    }

    /// Polynomial size These bits control the size of the polynomial.
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
            pub const B_0x0: u32 = 0b00;

            /// 0b01: 16 bit polynomial
            pub const B_0x1: u32 = 0b01;

            /// 0b10: 8 bit polynomial
            pub const B_0x2: u32 = 0b10;

            /// 0b11: 7 bit polynomial
            pub const B_0x3: u32 = 0b11;
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
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Initial CRC value
pub mod CRC_INIT {

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
pub mod CRC_POL {

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
    pub CRC_DR: RWRegister<u32>,

    /// Independent data register
    pub CRC_IDR: RWRegister<u32>,

    /// Control register
    pub CRC_CR: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// Initial CRC value
    pub CRC_INIT: RWRegister<u32>,

    /// polynomial
    pub CRC_POL: RWRegister<u32>,
}
pub struct ResetValues {
    pub CRC_DR: u32,
    pub CRC_IDR: u32,
    pub CRC_CR: u32,
    pub CRC_INIT: u32,
    pub CRC_POL: u32,
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
