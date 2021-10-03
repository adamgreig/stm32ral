#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GICV
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// GICV virtual machine control register
pub mod GICV_CTLR {

    /// ENABLEGRP0
    pub mod ENABLEGRP0 {
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

    /// ENABLEGRP1
    pub mod ENABLEGRP1 {
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

    /// ACKCTL
    pub mod ACKCTL {
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

    /// FIQEN
    pub mod FIQEN {
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

    /// CBPR
    pub mod CBPR {
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

    /// EOIMODE
    pub mod EOIMODE {
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
}

/// GICV VM priority mask register
pub mod GICV_PMR {

    /// PRIORITY
    pub mod PRIORITY {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (5 bits: 0b11111 << 3)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GICV VM binary point register
pub mod GICV_BPR {

    /// BINARY_POINT
    pub mod BINARY_POINT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GICV VM interrupt acknowledge register
pub mod GICV_IAR {

    /// INTERRUPT_ID
    pub mod INTERRUPT_ID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPUID
    pub mod CPUID {
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
}

/// GICV VM end of interrupt register
pub mod GICV_EOIR {

    /// EOIINTID
    pub mod EOIINTID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPUID
    pub mod CPUID {
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
}

/// GICV VM running priority register
pub mod GICV_RPR {

    /// PRIORITY
    pub mod PRIORITY {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (5 bits: 0b11111 << 3)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GICV VM highest priority pending interrupt register
pub mod GICV_HPPIR {

    /// PENDINTID
    pub mod PENDINTID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPUID
    pub mod CPUID {
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
}

/// GICV VM aliased binary point register
pub mod GICV_ABPR {
    pub use super::GICV_BPR::BINARY_POINT;
}

/// GICV VM aliased interrupt register
pub mod GICV_AIAR {
    pub use super::GICV_IAR::CPUID;
    pub use super::GICV_IAR::INTERRUPT_ID;
}

/// GICV VM aliased end of interrupt register
pub mod GICV_AEOIR {
    pub use super::GICV_EOIR::CPUID;
    pub use super::GICV_EOIR::EOIINTID;
}

/// GICV VM aliased highest priority pending interrupt register
pub mod GICV_AHPPIR {
    pub use super::GICV_HPPIR::CPUID;
    pub use super::GICV_HPPIR::PENDINTID;
}

/// The GICV_APR0 is an alias of GICH_APR.
pub mod GICV_APR0 {

    /// APR0
    pub mod APR0 {
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

/// The GICV_IIDR is an alias of GICC_IIDR.
pub mod GICV_IIDR {

    /// IIDR
    pub mod IIDR {
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

/// GICV VM deactivate interrupt register
pub mod GICV_DIR {

    /// INTERRUPT_ID
    pub mod INTERRUPT_ID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPUID
    pub mod CPUID {
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// GICV virtual machine control register
    pub GICV_CTLR: RWRegister<u32>,

    /// GICV VM priority mask register
    pub GICV_PMR: RWRegister<u32>,

    /// GICV VM binary point register
    pub GICV_BPR: RWRegister<u32>,

    /// GICV VM interrupt acknowledge register
    pub GICV_IAR: RORegister<u32>,

    /// GICV VM end of interrupt register
    pub GICV_EOIR: WORegister<u32>,

    /// GICV VM running priority register
    pub GICV_RPR: RORegister<u32>,

    /// GICV VM highest priority pending interrupt register
    pub GICV_HPPIR: RORegister<u32>,

    /// GICV VM aliased binary point register
    pub GICV_ABPR: RWRegister<u32>,

    /// GICV VM aliased interrupt register
    pub GICV_AIAR: RORegister<u32>,

    /// GICV VM aliased end of interrupt register
    pub GICV_AEOIR: WORegister<u32>,

    /// GICV VM aliased highest priority pending interrupt register
    pub GICV_AHPPIR: RORegister<u32>,

    _reserved1: [u32; 41],

    /// The GICV_APR0 is an alias of GICH_APR.
    pub GICV_APR0: RWRegister<u32>,

    _reserved2: [u32; 10],

    /// The GICV_IIDR is an alias of GICC_IIDR.
    pub GICV_IIDR: RORegister<u32>,

    _reserved3: [u32; 960],

    /// GICV VM deactivate interrupt register
    pub GICV_DIR: WORegister<u32>,
}
pub struct ResetValues {
    pub GICV_CTLR: u32,
    pub GICV_PMR: u32,
    pub GICV_BPR: u32,
    pub GICV_IAR: u32,
    pub GICV_EOIR: u32,
    pub GICV_RPR: u32,
    pub GICV_HPPIR: u32,
    pub GICV_ABPR: u32,
    pub GICV_AIAR: u32,
    pub GICV_AEOIR: u32,
    pub GICV_AHPPIR: u32,
    pub GICV_APR0: u32,
    pub GICV_IIDR: u32,
    pub GICV_DIR: u32,
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
