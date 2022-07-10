#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DDRPERFM
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Write-only register. A read request returns all zeros.
pub mod DDRPERFM_CTL {

    /// START
    pub mod START {
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

    /// STOP
    pub mod STOP {
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
}

/// DDRPERFM configurationl register
pub mod DDRPERFM_CFG {

    /// EN
    pub mod EN {
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

    /// SEL
    pub mod SEL {
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
}

/// DDRPERFM status register
pub mod DDRPERFM_STATUS {

    /// COVF
    pub mod COVF {
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

    /// BUSY
    pub mod BUSY {
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

    /// TOVF
    pub mod TOVF {
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

/// Write-only register. A read request returns all zeros
pub mod DDRPERFM_CCR {

    /// CCLR
    pub mod CCLR {
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

    /// TCLR
    pub mod TCLR {
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

/// DDRPERFM interrupt enable register
pub mod DDRPERFM_IER {

    /// OVFIE
    pub mod OVFIE {
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

/// DDRPERFM interrupt status register
pub mod DDRPERFM_ISR {

    /// OVFF
    pub mod OVFF {
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

/// Write-only register. A read request returns all zeros
pub mod DDRPERFM_ICR {

    /// OVF
    pub mod OVF {
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

/// DDRPERFM time counter register
pub mod DDRPERFM_TCNT {

    /// CNT
    pub mod CNT {
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

/// DDRPERFM event counter 0 register
pub mod DDRPERFM_CNT0 {
    pub use super::DDRPERFM_TCNT::CNT;
}

/// DDRPERFM event counter 1 register
pub mod DDRPERFM_CNT1 {
    pub use super::DDRPERFM_TCNT::CNT;
}

/// DDRPERFM event counter 2 register
pub mod DDRPERFM_CNT2 {
    pub use super::DDRPERFM_TCNT::CNT;
}

/// DDRPERFM event counter 3 register
pub mod DDRPERFM_CNT3 {
    pub use super::DDRPERFM_TCNT::CNT;
}

/// DDRPERFM hardware configuration register
pub mod DDRPERFM_HWCFG {

    /// NCNT
    pub mod NCNT {
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

/// DDRPERFM version register
pub mod DDRPERFM_VER {

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

/// DDRPERFM ID register
pub mod DDRPERFM_ID {

    /// ID
    pub mod ID {
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

/// DDRPERFM magic ID register
pub mod DDRPERFM_SID {

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
    /// Write-only register. A read request returns all zeros.
    pub DDRPERFM_CTL: WORegister<u32>,

    /// DDRPERFM configurationl register
    pub DDRPERFM_CFG: RWRegister<u32>,

    /// DDRPERFM status register
    pub DDRPERFM_STATUS: RORegister<u32>,

    /// Write-only register. A read request returns all zeros
    pub DDRPERFM_CCR: WORegister<u32>,

    /// DDRPERFM interrupt enable register
    pub DDRPERFM_IER: RWRegister<u32>,

    /// DDRPERFM interrupt status register
    pub DDRPERFM_ISR: RORegister<u32>,

    /// Write-only register. A read request returns all zeros
    pub DDRPERFM_ICR: WORegister<u32>,

    _reserved1: [u8; 4],

    /// DDRPERFM time counter register
    pub DDRPERFM_TCNT: RORegister<u32>,

    _reserved2: [u8; 60],

    /// DDRPERFM event counter 0 register
    pub DDRPERFM_CNT0: RORegister<u32>,

    _reserved3: [u8; 4],

    /// DDRPERFM event counter 1 register
    pub DDRPERFM_CNT1: RORegister<u32>,

    _reserved4: [u8; 4],

    /// DDRPERFM event counter 2 register
    pub DDRPERFM_CNT2: RORegister<u32>,

    _reserved5: [u8; 4],

    /// DDRPERFM event counter 3 register
    pub DDRPERFM_CNT3: RORegister<u32>,

    _reserved6: [u8; 884],

    /// DDRPERFM hardware configuration register
    pub DDRPERFM_HWCFG: RORegister<u32>,

    /// DDRPERFM version register
    pub DDRPERFM_VER: RORegister<u32>,

    /// DDRPERFM ID register
    pub DDRPERFM_ID: RORegister<u32>,

    /// DDRPERFM magic ID register
    pub DDRPERFM_SID: RORegister<u32>,
}
pub struct ResetValues {
    pub DDRPERFM_CTL: u32,
    pub DDRPERFM_CFG: u32,
    pub DDRPERFM_STATUS: u32,
    pub DDRPERFM_CCR: u32,
    pub DDRPERFM_IER: u32,
    pub DDRPERFM_ISR: u32,
    pub DDRPERFM_ICR: u32,
    pub DDRPERFM_TCNT: u32,
    pub DDRPERFM_CNT0: u32,
    pub DDRPERFM_CNT1: u32,
    pub DDRPERFM_CNT2: u32,
    pub DDRPERFM_CNT3: u32,
    pub DDRPERFM_HWCFG: u32,
    pub DDRPERFM_VER: u32,
    pub DDRPERFM_ID: u32,
    pub DDRPERFM_SID: u32,
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
