#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Cryptographic processor
//!
//! Used by: stm32h747cm4, stm32h747cm7, stm32h757cm7

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register
pub mod CR {

    /// Algorithm direction
    pub mod ALGODIR {
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

    /// Algorithm mode
    pub mod ALGOMODE0 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data type selection
    pub mod DATATYPE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Key size selection (AES mode only)
    pub mod KEYSIZE {
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

    /// FIFO flush
    pub mod FFLUSH {
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

    /// Cryptographic processor enable
    pub mod CRYPEN {
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

    /// GCM_CCMPH
    pub mod GCM_CCMPH {
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

    /// ALGOMODE
    pub mod ALGOMODE3 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
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
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output FIFO full
    pub mod OFFU {
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

    /// Output FIFO not empty
    pub mod OFNE {
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

    /// Input FIFO not full
    pub mod IFNF {
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

    /// Input FIFO empty
    pub mod IFEM {
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

/// data output register
pub mod DOUT {

    /// Data output
    pub mod DATAOUT {
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

/// DMA control register
pub mod DMACR {

    /// DMA output enable
    pub mod DOEN {
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

    /// DMA input enable
    pub mod DIEN {
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

/// interrupt mask set/clear register
pub mod IMSCR {

    /// Output FIFO service interrupt mask
    pub mod OUTIM {
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

    /// Input FIFO service interrupt mask
    pub mod INIM {
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

/// raw interrupt status register
pub mod RISR {

    /// Output FIFO service raw interrupt status
    pub mod OUTRIS {
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

    /// Input FIFO service raw interrupt status
    pub mod INRIS {
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

/// masked interrupt status register
pub mod MISR {

    /// Output FIFO service masked interrupt status
    pub mod OUTMIS {
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

    /// Input FIFO service masked interrupt status
    pub mod INMIS {
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

/// key registers
pub mod K0LR {

    /// b224
    pub mod b2 {
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

/// key registers
pub mod K0RR {

    /// b192
    pub mod b {
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

/// key registers
pub mod K1LR {

    /// b160
    pub mod b1 {
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

/// key registers
pub mod K1RR {
    pub use super::K1LR::b1;
}

/// key registers
pub mod K2LR {
    pub use super::K0RR::b;
}

/// key registers
pub mod K2RR {
    pub use super::K0RR::b;
}

/// key registers
pub mod K3LR {
    pub use super::K0RR::b;
}

/// key registers
pub mod K3RR {
    pub use super::K0RR::b;
}

/// initialization vector registers
pub mod IV0LR {

    /// IV31
    pub mod IV {
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

/// initialization vector registers
pub mod IV0RR {
    pub use super::IV0LR::IV;
}

/// initialization vector registers
pub mod IV1LR {
    pub use super::IV0LR::IV;
}

/// initialization vector registers
pub mod IV1RR {
    pub use super::IV0LR::IV;
}

/// context swap register
pub mod CSGCMCCM0R {

    /// CSGCMCCM0R
    pub mod CSGCMCCM0R {
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

/// context swap register
pub mod CSGCMCCM1R {

    /// CSGCMCCM1R
    pub mod CSGCMCCM1R {
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

/// context swap register
pub mod CSGCMCCM2R {

    /// CSGCMCCM2R
    pub mod CSGCMCCM2R {
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

/// context swap register
pub mod CSGCMCCM3R {

    /// CSGCMCCM3R
    pub mod CSGCMCCM3R {
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

/// context swap register
pub mod CSGCMCCM4R {

    /// CSGCMCCM4R
    pub mod CSGCMCCM4R {
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

/// context swap register
pub mod CSGCMCCM5R {

    /// CSGCMCCM5R
    pub mod CSGCMCCM5R {
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

/// context swap register
pub mod CSGCMCCM6R {

    /// CSGCMCCM6R
    pub mod CSGCMCCM6R {
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

/// context swap register
pub mod CSGCMCCM7R {

    /// CSGCMCCM7R
    pub mod CSGCMCCM7R {
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

/// context swap register
pub mod CSGCM0R {

    /// CSGCM0R
    pub mod CSGCM0R {
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

/// context swap register
pub mod CSGCM1R {

    /// CSGCM1R
    pub mod CSGCM1R {
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

/// context swap register
pub mod CSGCM2R {

    /// CSGCM2R
    pub mod CSGCM2R {
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

/// context swap register
pub mod CSGCM3R {

    /// CSGCM3R
    pub mod CSGCM3R {
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

/// context swap register
pub mod CSGCM4R {

    /// CSGCM4R
    pub mod CSGCM4R {
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

/// context swap register
pub mod CSGCM5R {

    /// CSGCM5R
    pub mod CSGCM5R {
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

/// context swap register
pub mod CSGCM6R {

    /// CSGCM6R
    pub mod CSGCM6R {
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

/// context swap register
pub mod CSGCM7R {

    /// CSGCM7R
    pub mod CSGCM7R {
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
pub struct RegisterBlock {
    /// control register
    pub CR: RWRegister<u32>,

    /// status register
    pub SR: RORegister<u32>,

    /// data input register
    pub DIN: RWRegister<u32>,

    /// data output register
    pub DOUT: RORegister<u32>,

    /// DMA control register
    pub DMACR: RWRegister<u32>,

    /// interrupt mask set/clear register
    pub IMSCR: RWRegister<u32>,

    /// raw interrupt status register
    pub RISR: RORegister<u32>,

    /// masked interrupt status register
    pub MISR: RORegister<u32>,

    /// key registers
    pub K0LR: WORegister<u32>,

    /// key registers
    pub K0RR: WORegister<u32>,

    /// key registers
    pub K1LR: WORegister<u32>,

    /// key registers
    pub K1RR: WORegister<u32>,

    /// key registers
    pub K2LR: WORegister<u32>,

    /// key registers
    pub K2RR: WORegister<u32>,

    /// key registers
    pub K3LR: WORegister<u32>,

    /// key registers
    pub K3RR: WORegister<u32>,

    /// initialization vector registers
    pub IV0LR: RWRegister<u32>,

    /// initialization vector registers
    pub IV0RR: RWRegister<u32>,

    /// initialization vector registers
    pub IV1LR: RWRegister<u32>,

    /// initialization vector registers
    pub IV1RR: RWRegister<u32>,

    /// context swap register
    pub CSGCMCCM0R: RWRegister<u32>,

    /// context swap register
    pub CSGCMCCM1R: RWRegister<u32>,

    /// context swap register
    pub CSGCMCCM2R: RWRegister<u32>,

    /// context swap register
    pub CSGCMCCM3R: RWRegister<u32>,

    /// context swap register
    pub CSGCMCCM4R: RWRegister<u32>,

    /// context swap register
    pub CSGCMCCM5R: RWRegister<u32>,

    /// context swap register
    pub CSGCMCCM6R: RWRegister<u32>,

    /// context swap register
    pub CSGCMCCM7R: RWRegister<u32>,

    /// context swap register
    pub CSGCM0R: RWRegister<u32>,

    /// context swap register
    pub CSGCM1R: RWRegister<u32>,

    /// context swap register
    pub CSGCM2R: RWRegister<u32>,

    /// context swap register
    pub CSGCM3R: RWRegister<u32>,

    /// context swap register
    pub CSGCM4R: RWRegister<u32>,

    /// context swap register
    pub CSGCM5R: RWRegister<u32>,

    /// context swap register
    pub CSGCM6R: RWRegister<u32>,

    /// context swap register
    pub CSGCM7R: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SR: u32,
    pub DIN: u32,
    pub DOUT: u32,
    pub DMACR: u32,
    pub IMSCR: u32,
    pub RISR: u32,
    pub MISR: u32,
    pub K0LR: u32,
    pub K0RR: u32,
    pub K1LR: u32,
    pub K1RR: u32,
    pub K2LR: u32,
    pub K2RR: u32,
    pub K3LR: u32,
    pub K3RR: u32,
    pub IV0LR: u32,
    pub IV0RR: u32,
    pub IV1LR: u32,
    pub IV1RR: u32,
    pub CSGCMCCM0R: u32,
    pub CSGCMCCM1R: u32,
    pub CSGCMCCM2R: u32,
    pub CSGCMCCM3R: u32,
    pub CSGCMCCM4R: u32,
    pub CSGCMCCM5R: u32,
    pub CSGCMCCM6R: u32,
    pub CSGCMCCM7R: u32,
    pub CSGCM0R: u32,
    pub CSGCM1R: u32,
    pub CSGCM2R: u32,
    pub CSGCM3R: u32,
    pub CSGCM4R: u32,
    pub CSGCM5R: u32,
    pub CSGCM6R: u32,
    pub CSGCM7R: u32,
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
