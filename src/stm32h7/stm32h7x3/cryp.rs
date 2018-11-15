#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Cryptographic processor

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister, WORegister};

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

/// Access functions for the CRYP peripheral instance
pub mod CRYP {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x48021000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CRYP
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SR: 0x00000003,
        DIN: 0x00000000,
        DOUT: 0x00000000,
        DMACR: 0x00000000,
        IMSCR: 0x00000000,
        RISR: 0x00000001,
        MISR: 0x00000000,
        K0LR: 0x00000000,
        K0RR: 0x00000000,
        K1LR: 0x00000000,
        K1RR: 0x00000000,
        K2LR: 0x00000000,
        K2RR: 0x00000000,
        K3LR: 0x00000000,
        K3RR: 0x00000000,
        IV0LR: 0x00000000,
        IV0RR: 0x00000000,
        IV1LR: 0x00000000,
        IV1RR: 0x00000000,
        CSGCMCCM0R: 0x00000000,
        CSGCMCCM1R: 0x00000000,
        CSGCMCCM2R: 0x00000000,
        CSGCMCCM3R: 0x00000000,
        CSGCMCCM4R: 0x00000000,
        CSGCMCCM5R: 0x00000000,
        CSGCMCCM6R: 0x00000000,
        CSGCMCCM7R: 0x00000000,
        CSGCM0R: 0x00000000,
        CSGCM1R: 0x00000000,
        CSGCM2R: 0x00000000,
        CSGCM3R: 0x00000000,
        CSGCM4R: 0x00000000,
        CSGCM5R: 0x00000000,
        CSGCM6R: 0x00000000,
        CSGCM7R: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CRYP_TAKEN: bool = false;

    /// Safe access to CRYP
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CRYP_TAKEN {
                None
            } else {
                CRYP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CRYP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CRYP_TAKEN && inst.addr == INSTANCE.addr {
                CRYP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to CRYP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CRYP: *const RegisterBlock = 0x48021000 as *const _;
