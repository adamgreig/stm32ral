#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Cryptographic processor
//!
//! Used by: stm32f7x2, stm32f7x3

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister};

/// control register
pub mod CR {

    /// Key size selection
    pub mod KEYSIZE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Used only for GCM, GMAC and CMAC algorithms and has no effect when other
    pub mod GCMPH {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable DMA management of data output phase
    pub mod DMAOUTEN {
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

    /// Enable DMA management of data input phase
    pub mod DMAINEN {
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

    /// Error interrupt enable
    pub mod ERRIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CCF flag interrupt enable
    pub mod CCFIE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Error clear
    pub mod ERRC {
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

    /// Computation complete flag clear
    pub mod CCFC {
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

    /// AES chaining mode
    pub mod CHMOD {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AES operating mode
    pub mod MODE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data type selection (for data in and data out to/from the cryptographic block)
    pub mod DATATYPE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AES enable
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
}

/// status register
pub mod SR {

    /// Busy flag
    pub mod Busy {
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

    /// Write error flag
    pub mod WRERR {
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

    /// Read error flag
    pub mod RDERR {
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

    /// Computation complete flag
    pub mod CCF {
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
pub mod DINR {

    /// Data input
    pub mod DINR {
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
pub mod DOUTR {

    /// Data output
    pub mod DOUTR {
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

/// key register
pub mod KEYR0 {

    /// Data output register
    pub mod KEYR0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// key register
pub mod KEYR1 {

    /// Data output register
    pub mod KEYR1 {
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

/// key register
pub mod KEYR2 {

    /// Data output register
    pub mod KEYR2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// key register
pub mod KEYR3 {

    /// Data output register
    pub mod KEYR3 {
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

/// initialization vector register
pub mod IVR0 {

    /// initialization vector register
    pub mod IVR0 {
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

/// initialization vector register
pub mod IVR1 {

    /// Initialization vector register
    pub mod IVR1 {
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

/// initialization vector register
pub mod IVR2 {

    /// Initialization vector register
    pub mod IVR2 {
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

/// initialization vector register
pub mod IVR3 {

    /// Initialization vector register
    pub mod IVR3 {
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
pub mod KEYR4 {

    /// Data output register
    pub mod KEYR4 {
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
pub mod KEYR5 {

    /// Data output register
    pub mod KEYR5 {
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
pub mod KEYR6 {

    /// Data output register
    pub mod KEYR6 {
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
pub mod KEYR7 {

    /// Data output register
    pub mod KEYR7 {
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

/// Suspend registers
pub mod SUSP0R {

    /// AES Suspend
    pub mod SUSP0R {
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

/// Suspend registers
pub mod SUSP1R {

    /// AES Suspend
    pub mod SUSP1R {
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

/// Suspend registers
pub mod SUSP2R {

    /// AES Suspend
    pub mod SUSP2R {
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

/// Suspend registers
pub mod SUSP3R {

    /// IV127
    pub mod SUSP3R {
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

/// Suspend registers
pub mod SUSP4R {

    /// AES Suspend
    pub mod SUSP4R {
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

/// Suspend registers
pub mod SUSP5R {

    /// AES Suspend
    pub mod SUSP5R {
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

/// Suspend registers
pub mod SUSP6R {

    /// AES Suspend
    pub mod SUSP6R {
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

/// Suspend registers
pub mod SUSP7R {

    /// AES Suspend
    pub mod SUSP7R {
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
    pub DINR: RWRegister<u32>,

    /// data output register
    pub DOUTR: RORegister<u32>,

    /// key register
    pub KEYR0: RWRegister<u32>,

    /// key register
    pub KEYR1: RWRegister<u32>,

    /// key register
    pub KEYR2: RORegister<u32>,

    /// key register
    pub KEYR3: RORegister<u32>,

    /// initialization vector register
    pub IVR0: RWRegister<u32>,

    /// initialization vector register
    pub IVR1: RWRegister<u32>,

    /// initialization vector register
    pub IVR2: RWRegister<u32>,

    /// initialization vector register
    pub IVR3: RWRegister<u32>,

    /// key registers
    pub KEYR4: RWRegister<u32>,

    /// key registers
    pub KEYR5: RWRegister<u32>,

    /// key registers
    pub KEYR6: RWRegister<u32>,

    /// key registers
    pub KEYR7: RWRegister<u32>,

    /// Suspend registers
    pub SUSP0R: RWRegister<u32>,

    /// Suspend registers
    pub SUSP1R: RWRegister<u32>,

    /// Suspend registers
    pub SUSP2R: RWRegister<u32>,

    /// Suspend registers
    pub SUSP3R: RWRegister<u32>,

    /// Suspend registers
    pub SUSP4R: RWRegister<u32>,

    /// Suspend registers
    pub SUSP5R: RWRegister<u32>,

    /// Suspend registers
    pub SUSP6R: RWRegister<u32>,

    /// Suspend registers
    pub SUSP7R: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SR: u32,
    pub DINR: u32,
    pub DOUTR: u32,
    pub KEYR0: u32,
    pub KEYR1: u32,
    pub KEYR2: u32,
    pub KEYR3: u32,
    pub IVR0: u32,
    pub IVR1: u32,
    pub IVR2: u32,
    pub IVR3: u32,
    pub KEYR4: u32,
    pub KEYR5: u32,
    pub KEYR6: u32,
    pub KEYR7: u32,
    pub SUSP0R: u32,
    pub SUSP1R: u32,
    pub SUSP2R: u32,
    pub SUSP3R: u32,
    pub SUSP4R: u32,
    pub SUSP5R: u32,
    pub SUSP6R: u32,
    pub SUSP7R: u32,
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
