#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Flexible static memory controller
//!
//! Used by: stm32l100, stm32l151, stm32l162

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use RWRegister;

/// BCR1
pub mod BCR1 {

    /// CBURSTRW
    pub mod CBURSTRW {
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

    /// ASYNCWAIT
    pub mod ASYNCWAIT {
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

    /// EXTMOD
    pub mod EXTMOD {
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

    /// WAITEN
    pub mod WAITEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WREN
    pub mod WREN {
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

    /// WAITCFG
    pub mod WAITCFG {
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

    /// WRAPMOD
    pub mod WRAPMOD {
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

    /// WAITPOL
    pub mod WAITPOL {
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

    /// BURSTEN
    pub mod BURSTEN {
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

    /// FACCEN
    pub mod FACCEN {
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

    /// MWID
    pub mod MWID {
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

    /// MTYP
    pub mod MTYP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MUXEN
    pub mod MUXEN {
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

    /// MBKEN
    pub mod MBKEN {
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

/// BCR1
pub mod BCR2 {
    pub use super::BCR1::ASYNCWAIT;
    pub use super::BCR1::BURSTEN;
    pub use super::BCR1::CBURSTRW;
    pub use super::BCR1::EXTMOD;
    pub use super::BCR1::FACCEN;
    pub use super::BCR1::MBKEN;
    pub use super::BCR1::MTYP;
    pub use super::BCR1::MUXEN;
    pub use super::BCR1::MWID;
    pub use super::BCR1::WAITCFG;
    pub use super::BCR1::WAITEN;
    pub use super::BCR1::WAITPOL;
    pub use super::BCR1::WRAPMOD;
    pub use super::BCR1::WREN;
}

/// BCR1
pub mod BCR3 {
    pub use super::BCR1::ASYNCWAIT;
    pub use super::BCR1::BURSTEN;
    pub use super::BCR1::CBURSTRW;
    pub use super::BCR1::EXTMOD;
    pub use super::BCR1::FACCEN;
    pub use super::BCR1::MBKEN;
    pub use super::BCR1::MTYP;
    pub use super::BCR1::MUXEN;
    pub use super::BCR1::MWID;
    pub use super::BCR1::WAITCFG;
    pub use super::BCR1::WAITEN;
    pub use super::BCR1::WAITPOL;
    pub use super::BCR1::WRAPMOD;
    pub use super::BCR1::WREN;
}

/// BCR1
pub mod BCR4 {
    pub use super::BCR1::ASYNCWAIT;
    pub use super::BCR1::BURSTEN;
    pub use super::BCR1::CBURSTRW;
    pub use super::BCR1::EXTMOD;
    pub use super::BCR1::FACCEN;
    pub use super::BCR1::MBKEN;
    pub use super::BCR1::MTYP;
    pub use super::BCR1::MUXEN;
    pub use super::BCR1::MWID;
    pub use super::BCR1::WAITCFG;
    pub use super::BCR1::WAITEN;
    pub use super::BCR1::WAITPOL;
    pub use super::BCR1::WRAPMOD;
    pub use super::BCR1::WREN;
}

/// BTR1
pub mod BTR1 {

    /// ACCMOD
    pub mod ACCMOD {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DATLAT
    pub mod DATLAT {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CLKDIV
    pub mod CLKDIV {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BUSTURN
    pub mod BUSTURN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DATAST
    pub mod DATAST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADDHLD
    pub mod ADDHLD {
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

    /// ADDSET
    pub mod ADDSET {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// BTR1
pub mod BTR2 {
    pub use super::BTR1::ACCMOD;
    pub use super::BTR1::ADDHLD;
    pub use super::BTR1::ADDSET;
    pub use super::BTR1::BUSTURN;
    pub use super::BTR1::CLKDIV;
    pub use super::BTR1::DATAST;
    pub use super::BTR1::DATLAT;
}

/// BTR1
pub mod BTR3 {
    pub use super::BTR1::ACCMOD;
    pub use super::BTR1::ADDHLD;
    pub use super::BTR1::ADDSET;
    pub use super::BTR1::BUSTURN;
    pub use super::BTR1::CLKDIV;
    pub use super::BTR1::DATAST;
    pub use super::BTR1::DATLAT;
}

/// BTR1
pub mod BTR4 {
    pub use super::BTR1::ACCMOD;
    pub use super::BTR1::ADDHLD;
    pub use super::BTR1::ADDSET;
    pub use super::BTR1::BUSTURN;
    pub use super::BTR1::CLKDIV;
    pub use super::BTR1::DATAST;
    pub use super::BTR1::DATLAT;
}

/// BWTR1
pub mod BWTR1 {

    /// ACCMOD
    pub mod ACCMOD {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DATLAT
    pub mod DATLAT {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CLKDIV
    pub mod CLKDIV {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DATAST
    pub mod DATAST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADDHLD
    pub mod ADDHLD {
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

    /// ADDSET
    pub mod ADDSET {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// BWTR1
pub mod BWTR2 {
    pub use super::BWTR1::ACCMOD;
    pub use super::BWTR1::ADDHLD;
    pub use super::BWTR1::ADDSET;
    pub use super::BWTR1::CLKDIV;
    pub use super::BWTR1::DATAST;
    pub use super::BWTR1::DATLAT;
}

/// BWTR1
pub mod BWTR3 {
    pub use super::BWTR1::ACCMOD;
    pub use super::BWTR1::ADDHLD;
    pub use super::BWTR1::ADDSET;
    pub use super::BWTR1::CLKDIV;
    pub use super::BWTR1::DATAST;
    pub use super::BWTR1::DATLAT;
}

/// BWTR1
pub mod BWTR4 {
    pub use super::BWTR1::ACCMOD;
    pub use super::BWTR1::ADDHLD;
    pub use super::BWTR1::ADDSET;
    pub use super::BWTR1::CLKDIV;
    pub use super::BWTR1::DATAST;
    pub use super::BWTR1::DATLAT;
}
pub struct RegisterBlock {
    /// BCR1
    pub BCR1: RWRegister<u32>,

    /// BTR1
    pub BTR1: RWRegister<u32>,

    /// BCR1
    pub BCR2: RWRegister<u32>,

    /// BTR1
    pub BTR2: RWRegister<u32>,

    /// BCR1
    pub BCR3: RWRegister<u32>,

    /// BTR1
    pub BTR3: RWRegister<u32>,

    /// BCR1
    pub BCR4: RWRegister<u32>,

    /// BTR1
    pub BTR4: RWRegister<u32>,

    _reserved1: [u32; 57],

    /// BWTR1
    pub BWTR1: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// BWTR1
    pub BWTR2: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// BWTR1
    pub BWTR3: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// BWTR1
    pub BWTR4: RWRegister<u32>,
}
pub struct ResetValues {
    pub BCR1: u32,
    pub BTR1: u32,
    pub BCR2: u32,
    pub BTR2: u32,
    pub BCR3: u32,
    pub BTR3: u32,
    pub BCR4: u32,
    pub BTR4: u32,
    pub BWTR1: u32,
    pub BWTR2: u32,
    pub BWTR3: u32,
    pub BWTR4: u32,
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
