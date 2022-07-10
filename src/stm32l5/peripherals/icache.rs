#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ICache
//!
//! Used by: stm32l552, stm32l562

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// ICACHE control register
pub mod ICACHE_CR {

    /// EN
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

    /// CACHEINV
    pub mod CACHEINV {
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

    /// WAYSEL
    pub mod WAYSEL {
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

    /// HITMEN
    pub mod HITMEN {
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

    /// MISSMEN
    pub mod MISSMEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HITMRST
    pub mod HITMRST {
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

    /// MISSMRST
    pub mod MISSMRST {
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

/// ICACHE status register
pub mod ICACHE_SR {

    /// BUSYF
    pub mod BUSYF {
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

    /// BSYENDF
    pub mod BSYENDF {
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

    /// ERRF
    pub mod ERRF {
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

/// ICACHE interrupt enable register
pub mod ICACHE_IER {

    /// BSYENDIE
    pub mod BSYENDIE {
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

    /// ERRIE
    pub mod ERRIE {
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

/// ICACHE flag clear register
pub mod ICACHE_FCR {

    /// CBSYENDF
    pub mod CBSYENDF {
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

    /// CERRF
    pub mod CERRF {
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

/// ICACHE hit monitor register
pub mod ICACHE_HMONR {

    /// HITMON
    pub mod HITMON {
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

/// ICACHE miss monitor register
pub mod ICACHE_MMONR {

    /// MISSMON
    pub mod MISSMON {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ICACHE region configuration register
pub mod ICACHE_CRR0 {

    /// BASEADDR
    pub mod BASEADDR {
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

    /// RSIZE
    pub mod RSIZE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// REN
    pub mod REN {
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

    /// REMAPADDR
    pub mod REMAPADDR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MSTSEL
    pub mod MSTSEL {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HBURST
    pub mod HBURST {
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

/// ICACHE region configuration register
pub mod ICACHE_CRR1 {
    pub use super::ICACHE_CRR0::BASEADDR;
    pub use super::ICACHE_CRR0::HBURST;
    pub use super::ICACHE_CRR0::MSTSEL;
    pub use super::ICACHE_CRR0::REMAPADDR;
    pub use super::ICACHE_CRR0::REN;
    pub use super::ICACHE_CRR0::RSIZE;
}

/// ICACHE region configuration register
pub mod ICACHE_CRR2 {
    pub use super::ICACHE_CRR0::BASEADDR;
    pub use super::ICACHE_CRR0::HBURST;
    pub use super::ICACHE_CRR0::MSTSEL;
    pub use super::ICACHE_CRR0::REMAPADDR;
    pub use super::ICACHE_CRR0::REN;
    pub use super::ICACHE_CRR0::RSIZE;
}

/// ICACHE region configuration register
pub mod ICACHE_CRR3 {
    pub use super::ICACHE_CRR0::BASEADDR;
    pub use super::ICACHE_CRR0::HBURST;
    pub use super::ICACHE_CRR0::MSTSEL;
    pub use super::ICACHE_CRR0::REMAPADDR;
    pub use super::ICACHE_CRR0::REN;
    pub use super::ICACHE_CRR0::RSIZE;
}
#[repr(C)]
pub struct RegisterBlock {
    /// ICACHE control register
    pub ICACHE_CR: RWRegister<u32>,

    /// ICACHE status register
    pub ICACHE_SR: RORegister<u32>,

    /// ICACHE interrupt enable register
    pub ICACHE_IER: RWRegister<u32>,

    /// ICACHE flag clear register
    pub ICACHE_FCR: WORegister<u32>,

    /// ICACHE hit monitor register
    pub ICACHE_HMONR: RORegister<u32>,

    /// ICACHE miss monitor register
    pub ICACHE_MMONR: RORegister<u32>,

    _reserved1: [u8; 8],

    /// ICACHE region configuration register
    pub ICACHE_CRR0: RWRegister<u32>,

    /// ICACHE region configuration register
    pub ICACHE_CRR1: RWRegister<u32>,

    /// ICACHE region configuration register
    pub ICACHE_CRR2: RWRegister<u32>,

    /// ICACHE region configuration register
    pub ICACHE_CRR3: RWRegister<u32>,
}
pub struct ResetValues {
    pub ICACHE_CR: u32,
    pub ICACHE_SR: u32,
    pub ICACHE_IER: u32,
    pub ICACHE_FCR: u32,
    pub ICACHE_HMONR: u32,
    pub ICACHE_MMONR: u32,
    pub ICACHE_CRR0: u32,
    pub ICACHE_CRR1: u32,
    pub ICACHE_CRR2: u32,
    pub ICACHE_CRR3: u32,
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
