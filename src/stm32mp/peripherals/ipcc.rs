#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IPCC
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// IPCC Processor 1 control register
pub mod IPCC_C1CR {

    /// RXOIE
    pub mod RXOIE {
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

    /// TXFIE
    pub mod TXFIE {
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
}

/// IPCC Processor 1 mask register
pub mod IPCC_C1MR {

    /// CHxOM
    pub mod CHxOM {
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

    /// CHxFM
    pub mod CHxFM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reading this register will always return 0x0000 0000.
pub mod IPCC_C1SCR {

    /// CHxC
    pub mod CHxC {
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

    /// CHxS
    pub mod CHxS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPCC processor 1 to processor 2 status register
pub mod IPCC_C1TOC2SR {

    /// CHxF
    pub mod CHxF {
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

/// IPCC Processor 2 control register
pub mod IPCC_C2CR {
    pub use super::IPCC_C1CR::RXOIE;
    pub use super::IPCC_C1CR::TXFIE;
}

/// IPCC Processor 2 mask register
pub mod IPCC_C2MR {
    pub use super::IPCC_C1MR::CHxFM;
    pub use super::IPCC_C1MR::CHxOM;
}

/// Reading this register will always return 0x0000 0000.
pub mod IPCC_C2SCR {
    pub use super::IPCC_C1SCR::CHxC;
    pub use super::IPCC_C1SCR::CHxS;
}

/// IPCC processor 2 to processor 1 status register
pub mod IPCC_C2TOC1SR {
    pub use super::IPCC_C1TOC2SR::CHxF;
}

/// IPCC Hardware configuration register
pub mod IPCC_HWCFGR {

    /// CHANNELS
    pub mod CHANNELS {
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

/// IPCC IP Version register
pub mod IPCC_VER {

    /// MINREV
    pub mod MINREV {
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

    /// MAJREV
    pub mod MAJREV {
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
}

/// IPCC IP Identification register
pub mod IPCC_ID {

    /// IPID
    pub mod IPID {
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

/// IPCC Size ID register
pub mod IPCC_SID {

    /// SID
    pub mod SID {
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
    /// IPCC Processor 1 control register
    pub IPCC_C1CR: RWRegister<u32>,

    /// IPCC Processor 1 mask register
    pub IPCC_C1MR: RWRegister<u32>,

    /// Reading this register will always return 0x0000 0000.
    pub IPCC_C1SCR: RWRegister<u32>,

    /// IPCC processor 1 to processor 2 status register
    pub IPCC_C1TOC2SR: RORegister<u32>,

    /// IPCC Processor 2 control register
    pub IPCC_C2CR: RWRegister<u32>,

    /// IPCC Processor 2 mask register
    pub IPCC_C2MR: RWRegister<u32>,

    /// Reading this register will always return 0x0000 0000.
    pub IPCC_C2SCR: RWRegister<u32>,

    /// IPCC processor 2 to processor 1 status register
    pub IPCC_C2TOC1SR: RORegister<u32>,

    _reserved1: [u8; 976],

    /// IPCC Hardware configuration register
    pub IPCC_HWCFGR: RORegister<u32>,

    /// IPCC IP Version register
    pub IPCC_VER: RORegister<u32>,

    /// IPCC IP Identification register
    pub IPCC_ID: RORegister<u32>,

    /// IPCC Size ID register
    pub IPCC_SID: RORegister<u32>,
}
pub struct ResetValues {
    pub IPCC_C1CR: u32,
    pub IPCC_C1MR: u32,
    pub IPCC_C1SCR: u32,
    pub IPCC_C1TOC2SR: u32,
    pub IPCC_C2CR: u32,
    pub IPCC_C2MR: u32,
    pub IPCC_C2SCR: u32,
    pub IPCC_C2TOC1SR: u32,
    pub IPCC_HWCFGR: u32,
    pub IPCC_VER: u32,
    pub IPCC_ID: u32,
    pub IPCC_SID: u32,
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
