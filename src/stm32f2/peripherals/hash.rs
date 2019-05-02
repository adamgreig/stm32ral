#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Hash processor
//!
//! Used by: stm32f215, stm32f217

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister, WORegister};

/// control register
pub mod CR {

    /// Long key selection
    pub mod LKEY {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DIN not empty
    pub mod DINNE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of words already pushed
    pub mod NBW {
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

    /// Algorithm selection
    pub mod ALGO {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Mode selection
    pub mod MODE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data type selection
    pub mod DATATYPE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA enable
    pub mod DMAE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Initialize message digest calculation
    pub mod INIT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// data input register
pub mod DIN {

    /// Data input
    pub mod DATAIN {
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

/// start register
pub mod STR {

    /// Digest calculation
    pub mod DCAL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of valid bits in the last word of the message
    pub mod NBLW {
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

/// digest registers
pub mod HR0 {

    /// H0
    pub mod H {
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

/// digest registers
pub mod HR1 {
    pub use super::HR0::H;
}

/// digest registers
pub mod HR2 {
    pub use super::HR0::H;
}

/// digest registers
pub mod HR3 {
    pub use super::HR0::H;
}

/// digest registers
pub mod HR4 {
    pub use super::HR0::H;
}

/// interrupt enable register
pub mod IMR {

    /// Digest calculation completion interrupt enable
    pub mod DCIE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data input interrupt enable
    pub mod DINIE {
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

/// status register
pub mod SR {

    /// Busy bit
    pub mod BUSY {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Status
    pub mod DMAS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Digest calculation completion interrupt status
    pub mod DCIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data input interrupt status
    pub mod DINIS {
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

/// context swap registers
pub mod CSR0 {

    /// CSR0
    pub mod CSR {
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

/// context swap registers
pub mod CSR1 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR2 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR3 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR4 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR5 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR6 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR7 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR8 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR9 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR10 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR11 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR12 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR13 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR14 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR15 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR16 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR17 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR18 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR19 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR20 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR21 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR22 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR23 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR24 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR25 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR26 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR27 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR28 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR29 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR30 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR31 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR32 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR33 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR34 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR35 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR36 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR37 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR38 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR39 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR40 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR41 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR42 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR43 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR44 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR45 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR46 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR47 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR48 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR49 {
    pub use super::CSR0::CSR;
}

/// context swap registers
pub mod CSR50 {
    pub use super::CSR0::CSR;
}
pub struct RegisterBlock {
    /// control register
    pub CR: RWRegister<u32>,

    /// data input register
    pub DIN: RWRegister<u32>,

    /// start register
    pub STR: WORegister<u32>,

    /// digest registers
    pub HR0: RORegister<u32>,

    /// digest registers
    pub HR1: RORegister<u32>,

    /// digest registers
    pub HR2: RORegister<u32>,

    /// digest registers
    pub HR3: RORegister<u32>,

    /// digest registers
    pub HR4: RORegister<u32>,

    /// interrupt enable register
    pub IMR: RWRegister<u32>,

    /// status register
    pub SR: RWRegister<u32>,

    _reserved1: [u32; 52],

    /// context swap registers
    pub CSR0: RWRegister<u32>,

    /// context swap registers
    pub CSR1: RWRegister<u32>,

    /// context swap registers
    pub CSR2: RWRegister<u32>,

    /// context swap registers
    pub CSR3: RWRegister<u32>,

    /// context swap registers
    pub CSR4: RWRegister<u32>,

    /// context swap registers
    pub CSR5: RWRegister<u32>,

    /// context swap registers
    pub CSR6: RWRegister<u32>,

    /// context swap registers
    pub CSR7: RWRegister<u32>,

    /// context swap registers
    pub CSR8: RWRegister<u32>,

    /// context swap registers
    pub CSR9: RWRegister<u32>,

    /// context swap registers
    pub CSR10: RWRegister<u32>,

    /// context swap registers
    pub CSR11: RWRegister<u32>,

    /// context swap registers
    pub CSR12: RWRegister<u32>,

    /// context swap registers
    pub CSR13: RWRegister<u32>,

    /// context swap registers
    pub CSR14: RWRegister<u32>,

    /// context swap registers
    pub CSR15: RWRegister<u32>,

    /// context swap registers
    pub CSR16: RWRegister<u32>,

    /// context swap registers
    pub CSR17: RWRegister<u32>,

    /// context swap registers
    pub CSR18: RWRegister<u32>,

    /// context swap registers
    pub CSR19: RWRegister<u32>,

    /// context swap registers
    pub CSR20: RWRegister<u32>,

    /// context swap registers
    pub CSR21: RWRegister<u32>,

    /// context swap registers
    pub CSR22: RWRegister<u32>,

    /// context swap registers
    pub CSR23: RWRegister<u32>,

    /// context swap registers
    pub CSR24: RWRegister<u32>,

    /// context swap registers
    pub CSR25: RWRegister<u32>,

    /// context swap registers
    pub CSR26: RWRegister<u32>,

    /// context swap registers
    pub CSR27: RWRegister<u32>,

    /// context swap registers
    pub CSR28: RWRegister<u32>,

    /// context swap registers
    pub CSR29: RWRegister<u32>,

    /// context swap registers
    pub CSR30: RWRegister<u32>,

    /// context swap registers
    pub CSR31: RWRegister<u32>,

    /// context swap registers
    pub CSR32: RWRegister<u32>,

    /// context swap registers
    pub CSR33: RWRegister<u32>,

    /// context swap registers
    pub CSR34: RWRegister<u32>,

    /// context swap registers
    pub CSR35: RWRegister<u32>,

    /// context swap registers
    pub CSR36: RWRegister<u32>,

    /// context swap registers
    pub CSR37: RWRegister<u32>,

    /// context swap registers
    pub CSR38: RWRegister<u32>,

    /// context swap registers
    pub CSR39: RWRegister<u32>,

    /// context swap registers
    pub CSR40: RWRegister<u32>,

    /// context swap registers
    pub CSR41: RWRegister<u32>,

    /// context swap registers
    pub CSR42: RWRegister<u32>,

    /// context swap registers
    pub CSR43: RWRegister<u32>,

    /// context swap registers
    pub CSR44: RWRegister<u32>,

    /// context swap registers
    pub CSR45: RWRegister<u32>,

    /// context swap registers
    pub CSR46: RWRegister<u32>,

    /// context swap registers
    pub CSR47: RWRegister<u32>,

    /// context swap registers
    pub CSR48: RWRegister<u32>,

    /// context swap registers
    pub CSR49: RWRegister<u32>,

    /// context swap registers
    pub CSR50: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub DIN: u32,
    pub STR: u32,
    pub HR0: u32,
    pub HR1: u32,
    pub HR2: u32,
    pub HR3: u32,
    pub HR4: u32,
    pub IMR: u32,
    pub SR: u32,
    pub CSR0: u32,
    pub CSR1: u32,
    pub CSR2: u32,
    pub CSR3: u32,
    pub CSR4: u32,
    pub CSR5: u32,
    pub CSR6: u32,
    pub CSR7: u32,
    pub CSR8: u32,
    pub CSR9: u32,
    pub CSR10: u32,
    pub CSR11: u32,
    pub CSR12: u32,
    pub CSR13: u32,
    pub CSR14: u32,
    pub CSR15: u32,
    pub CSR16: u32,
    pub CSR17: u32,
    pub CSR18: u32,
    pub CSR19: u32,
    pub CSR20: u32,
    pub CSR21: u32,
    pub CSR22: u32,
    pub CSR23: u32,
    pub CSR24: u32,
    pub CSR25: u32,
    pub CSR26: u32,
    pub CSR27: u32,
    pub CSR28: u32,
    pub CSR29: u32,
    pub CSR30: u32,
    pub CSR31: u32,
    pub CSR32: u32,
    pub CSR33: u32,
    pub CSR34: u32,
    pub CSR35: u32,
    pub CSR36: u32,
    pub CSR37: u32,
    pub CSR38: u32,
    pub CSR39: u32,
    pub CSR40: u32,
    pub CSR41: u32,
    pub CSR42: u32,
    pub CSR43: u32,
    pub CSR44: u32,
    pub CSR45: u32,
    pub CSR46: u32,
    pub CSR47: u32,
    pub CSR48: u32,
    pub CSR49: u32,
    pub CSR50: u32,
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
