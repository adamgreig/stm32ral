#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! HSEM
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R0 {

    /// PROCID
    pub mod PROCID {
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

    /// COREID
    pub mod COREID {
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

    /// LOCK
    pub mod LOCK {
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

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R1 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R2 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R3 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R4 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R5 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R6 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R7 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R8 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R9 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R10 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R11 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R12 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R13 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R14 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R15 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R16 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R17 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R18 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R19 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R20 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R21 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R22 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R23 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R24 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R25 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R26 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R27 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R28 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R29 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R30 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_R31 {
    pub use super::HSEM_R0::COREID;
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR0 {

    /// PROCID
    pub mod PROCID {
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

    /// COREID
    pub mod COREID {
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

    /// LOCK
    pub mod LOCK {
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

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR1 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR2 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR3 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR4 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR5 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR6 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR7 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR8 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR9 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR10 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR11 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR12 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR13 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR14 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR15 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR16 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR17 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR18 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR19 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR20 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR21 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR22 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR23 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR24 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR25 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR26 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR27 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR28 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR29 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR30 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_RLR31 {
    pub use super::HSEM_RLR0::COREID;
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM i1terrupt enable register
pub mod HSEM_C1IER {

    /// ISE
    pub mod ISE {
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

/// HSEM i1terrupt clear register
pub mod HSEM_C1ICR {

    /// ISC
    pub mod ISC {
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

/// HSEM i1terrupt status register
pub mod HSEM_C1ISR {

    /// ISF
    pub mod ISF {
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

/// HSEM i1terrupt status register
pub mod HSEM_C1MISR {

    /// MISF
    pub mod MISF {
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

/// HSEM i2terrupt enable register
pub mod HSEM_C2IER {
    pub use super::HSEM_C1IER::ISE;
}

/// HSEM i2terrupt clear register
pub mod HSEM_C2ICR {
    pub use super::HSEM_C1ICR::ISC;
}

/// HSEM i2terrupt status register
pub mod HSEM_C2ISR {
    pub use super::HSEM_C1ISR::ISF;
}

/// HSEM i2terrupt status register
pub mod HSEM_C2MISR {
    pub use super::HSEM_C1MISR::MISF;
}

/// Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod HSEM_CR {

    /// COREID
    pub mod COREID {
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

    /// KEY
    pub mod KEY {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// HSEM interrupt clear register
pub mod HSEM_KEYR {

    /// KEY
    pub mod KEY {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// HSEM hardware configuration register 2
pub mod HSEM_HWCFGR2 {

    /// MASTERID1
    pub mod MASTERID1 {
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

    /// MASTERID2
    pub mod MASTERID2 {
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

    /// MASTERID3
    pub mod MASTERID3 {
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

    /// MASTERID4
    pub mod MASTERID4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// HSEM hardware configuration register 1
pub mod HSEM_HWCFGR1 {

    /// NBSEM
    pub mod NBSEM {
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

    /// NBINT
    pub mod NBINT {
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
}

/// HSEM IP version register
pub mod HSEM_VERR {

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

/// HSEM IP identification register
pub mod HSEM_IPIDR {

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

/// HSEM size identification register
pub mod HSEM_SIDR {

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
    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R0: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R1: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R2: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R3: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R4: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R5: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R6: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R7: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R8: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R9: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R10: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R11: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R12: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R13: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R14: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R15: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R16: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R17: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R18: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R19: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R20: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R21: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R22: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R23: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R24: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R25: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R26: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R27: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R28: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R29: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R30: RWRegister<u32>,

    /// The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_R31: RWRegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR0: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR1: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR2: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR3: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR4: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR5: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR6: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR7: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR8: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR9: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR10: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR11: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR12: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR13: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR14: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR15: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR16: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR17: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR18: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR19: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR20: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR21: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR22: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR23: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR24: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR25: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR26: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR27: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR28: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR29: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR30: RORegister<u32>,

    /// Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_RLR31: RORegister<u32>,

    /// HSEM i1terrupt enable register
    pub HSEM_C1IER: RWRegister<u32>,

    /// HSEM i1terrupt clear register
    pub HSEM_C1ICR: RWRegister<u32>,

    /// HSEM i1terrupt status register
    pub HSEM_C1ISR: RORegister<u32>,

    /// HSEM i1terrupt status register
    pub HSEM_C1MISR: RORegister<u32>,

    /// HSEM i2terrupt enable register
    pub HSEM_C2IER: RWRegister<u32>,

    /// HSEM i2terrupt clear register
    pub HSEM_C2ICR: RWRegister<u32>,

    /// HSEM i2terrupt status register
    pub HSEM_C2ISR: RORegister<u32>,

    /// HSEM i2terrupt status register
    pub HSEM_C2MISR: RORegister<u32>,

    _reserved1: [u32; 8],

    /// Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub HSEM_CR: WORegister<u32>,

    /// HSEM interrupt clear register
    pub HSEM_KEYR: RWRegister<u32>,

    _reserved2: [u32; 169],

    /// HSEM hardware configuration register 2
    pub HSEM_HWCFGR2: RORegister<u32>,

    /// HSEM hardware configuration register 1
    pub HSEM_HWCFGR1: RORegister<u32>,

    /// HSEM IP version register
    pub HSEM_VERR: RORegister<u32>,

    /// HSEM IP identification register
    pub HSEM_IPIDR: RORegister<u32>,

    /// HSEM size identification register
    pub HSEM_SIDR: RORegister<u32>,
}
pub struct ResetValues {
    pub HSEM_R0: u32,
    pub HSEM_R1: u32,
    pub HSEM_R2: u32,
    pub HSEM_R3: u32,
    pub HSEM_R4: u32,
    pub HSEM_R5: u32,
    pub HSEM_R6: u32,
    pub HSEM_R7: u32,
    pub HSEM_R8: u32,
    pub HSEM_R9: u32,
    pub HSEM_R10: u32,
    pub HSEM_R11: u32,
    pub HSEM_R12: u32,
    pub HSEM_R13: u32,
    pub HSEM_R14: u32,
    pub HSEM_R15: u32,
    pub HSEM_R16: u32,
    pub HSEM_R17: u32,
    pub HSEM_R18: u32,
    pub HSEM_R19: u32,
    pub HSEM_R20: u32,
    pub HSEM_R21: u32,
    pub HSEM_R22: u32,
    pub HSEM_R23: u32,
    pub HSEM_R24: u32,
    pub HSEM_R25: u32,
    pub HSEM_R26: u32,
    pub HSEM_R27: u32,
    pub HSEM_R28: u32,
    pub HSEM_R29: u32,
    pub HSEM_R30: u32,
    pub HSEM_R31: u32,
    pub HSEM_RLR0: u32,
    pub HSEM_RLR1: u32,
    pub HSEM_RLR2: u32,
    pub HSEM_RLR3: u32,
    pub HSEM_RLR4: u32,
    pub HSEM_RLR5: u32,
    pub HSEM_RLR6: u32,
    pub HSEM_RLR7: u32,
    pub HSEM_RLR8: u32,
    pub HSEM_RLR9: u32,
    pub HSEM_RLR10: u32,
    pub HSEM_RLR11: u32,
    pub HSEM_RLR12: u32,
    pub HSEM_RLR13: u32,
    pub HSEM_RLR14: u32,
    pub HSEM_RLR15: u32,
    pub HSEM_RLR16: u32,
    pub HSEM_RLR17: u32,
    pub HSEM_RLR18: u32,
    pub HSEM_RLR19: u32,
    pub HSEM_RLR20: u32,
    pub HSEM_RLR21: u32,
    pub HSEM_RLR22: u32,
    pub HSEM_RLR23: u32,
    pub HSEM_RLR24: u32,
    pub HSEM_RLR25: u32,
    pub HSEM_RLR26: u32,
    pub HSEM_RLR27: u32,
    pub HSEM_RLR28: u32,
    pub HSEM_RLR29: u32,
    pub HSEM_RLR30: u32,
    pub HSEM_RLR31: u32,
    pub HSEM_C1IER: u32,
    pub HSEM_C1ICR: u32,
    pub HSEM_C1ISR: u32,
    pub HSEM_C1MISR: u32,
    pub HSEM_C2IER: u32,
    pub HSEM_C2ICR: u32,
    pub HSEM_C2ISR: u32,
    pub HSEM_C2MISR: u32,
    pub HSEM_CR: u32,
    pub HSEM_KEYR: u32,
    pub HSEM_HWCFGR2: u32,
    pub HSEM_HWCFGR1: u32,
    pub HSEM_VERR: u32,
    pub HSEM_IPIDR: u32,
    pub HSEM_SIDR: u32,
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
