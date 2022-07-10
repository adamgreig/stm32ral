#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Firewall
//!
//! Used by: stm32l0x0, stm32l0x1, stm32l0x2, stm32l0x3

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Code segment start address
pub mod CSSA {

    /// code segment start address
    pub mod ADD {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (16 bits: 0xffff << 8)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Code segment length
pub mod CSL {

    /// code segment length
    pub mod LENG {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (14 bits: 0x3fff << 8)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Non-volatile data segment start address
pub mod NVDSSA {
    pub use super::CSSA::ADD;
}

/// Non-volatile data segment length
pub mod NVDSL {
    pub use super::CSL::LENG;
}

/// Volatile data segment start address
pub mod VDSSA {

    /// Volatile data segment start address
    pub mod ADD {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (10 bits: 0x3ff << 6)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Volatile data segment length
pub mod VDSL {

    /// Non-volatile data segment length
    pub mod LENG {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (10 bits: 0x3ff << 6)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Configuration register
pub mod CR {

    /// Volatile data execution
    pub mod VDE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Volatile data segment cannot be executed if VDS = 0
            pub const NotExecutable: u32 = 0b0;

            /// 0b1: Volatile data segment is declared executable whatever VDS bit value
            pub const Executable: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: Resets volatile data execution bit
            pub const Reset: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Volatile data shared
    pub mod VDS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Volatile data segment is not shared and cannot be hit by a non protected executable code when the Firewall is closed
            pub const NotShared: u32 = 0b0;

            /// 0b1: Volatile data segment is shared with non protected application code
            pub const Shared: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: Resets volatile data shared bit
            pub const Reset: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Firewall pre alarm
    pub mod FPA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: Any code executed outside the protected segment when the Firewall is opened will generate a system reset
            pub const PreArmReset: u32 = 0b0;

            /// 0b1: Any code executed outside the protected segment will close the Firewall
            pub const PreArmSet: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Code segment start address
    pub CSSA: RWRegister<u32>,

    /// Code segment length
    pub CSL: RWRegister<u32>,

    /// Non-volatile data segment start address
    pub NVDSSA: RWRegister<u32>,

    /// Non-volatile data segment length
    pub NVDSL: RWRegister<u32>,

    /// Volatile data segment start address
    pub VDSSA: RWRegister<u32>,

    /// Volatile data segment length
    pub VDSL: RWRegister<u32>,

    _reserved1: [u8; 8],

    /// Configuration register
    pub CR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CSSA: u32,
    pub CSL: u32,
    pub NVDSSA: u32,
    pub NVDSL: u32,
    pub VDSSA: u32,
    pub VDSL: u32,
    pub CR: u32,
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
