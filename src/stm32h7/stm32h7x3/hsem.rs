#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! HSEM

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister};

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R0 {

    /// Semaphore ProcessID
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

    /// Semaphore MasterID
    pub mod MASTERID {
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

    /// Lock indication
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

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R1 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R2 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R3 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R4 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R5 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R6 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R7 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R8 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R9 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R10 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R11 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R12 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R13 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R14 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R15 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R16 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R17 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R18 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R19 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R20 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R21 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R22 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R23 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R24 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R25 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R26 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R27 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R28 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R29 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R30 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM register HSEM_R0 HSEM_R31
pub mod HSEM_R31 {
    pub use super::HSEM_R0::LOCK;
    pub use super::HSEM_R0::MASTERID;
    pub use super::HSEM_R0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR0 {

    /// Semaphore ProcessID
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

    /// Semaphore MasterID
    pub mod MASTERID {
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

    /// Lock indication
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

/// HSEM Read lock register
pub mod HSEM_RLR1 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR2 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR3 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR4 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR5 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR6 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR7 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR8 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR9 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR10 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR11 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR12 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR13 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR14 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR15 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR16 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR17 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR18 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR19 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR20 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR21 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR22 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR23 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR24 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR25 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR26 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR27 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR28 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR29 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR30 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Read lock register
pub mod HSEM_RLR31 {
    pub use super::HSEM_RLR0::LOCK;
    pub use super::HSEM_RLR0::MASTERID;
    pub use super::HSEM_RLR0::PROCID;
}

/// HSEM Interrupt enable register
pub mod HSEM_IER {

    /// Interrupt semaphore n enable bit
    pub mod ISEM0 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM1 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM2 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM3 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM4 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt semaphore n enable bit
    pub mod ISEM6 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM7 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM8 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM9 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM10 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM11 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM12 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM13 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM14 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM15 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM16 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM17 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM18 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM19 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt semaphore n enable bit
    pub mod ISEM21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt semaphore n enable bit
    pub mod ISEM22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt semaphore n enable bit
    pub mod ISEM23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt semaphore n enable bit
    pub mod ISEM24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt semaphore n enable bit
    pub mod ISEM25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt semaphore n enable bit
    pub mod ISEM26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt semaphore n enable bit
    pub mod ISEM27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt semaphore n enable bit
    pub mod ISEM28 {
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

    /// Interrupt semaphore n enable bit
    pub mod ISEM29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt semaphore n enable bit
    pub mod ISEM30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n enable bit.
    pub mod ISEM31 {
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

/// HSEM Interrupt clear register
pub mod HSEM_ICR {

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM0 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM1 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM2 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM3 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM4 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM6 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM7 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM8 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM9 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM10 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM11 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM12 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM13 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM14 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM15 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM16 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM17 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM18 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM19 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM28 {
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

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISEM31 {
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

/// HSEM Interrupt status register
pub mod HSEM_ISR {
    pub use super::HSEM_ICR::ISEM0;
    pub use super::HSEM_ICR::ISEM1;
    pub use super::HSEM_ICR::ISEM10;
    pub use super::HSEM_ICR::ISEM11;
    pub use super::HSEM_ICR::ISEM12;
    pub use super::HSEM_ICR::ISEM13;
    pub use super::HSEM_ICR::ISEM14;
    pub use super::HSEM_ICR::ISEM15;
    pub use super::HSEM_ICR::ISEM16;
    pub use super::HSEM_ICR::ISEM17;
    pub use super::HSEM_ICR::ISEM18;
    pub use super::HSEM_ICR::ISEM19;
    pub use super::HSEM_ICR::ISEM2;
    pub use super::HSEM_ICR::ISEM20;
    pub use super::HSEM_ICR::ISEM21;
    pub use super::HSEM_ICR::ISEM22;
    pub use super::HSEM_ICR::ISEM23;
    pub use super::HSEM_ICR::ISEM24;
    pub use super::HSEM_ICR::ISEM25;
    pub use super::HSEM_ICR::ISEM26;
    pub use super::HSEM_ICR::ISEM27;
    pub use super::HSEM_ICR::ISEM28;
    pub use super::HSEM_ICR::ISEM29;
    pub use super::HSEM_ICR::ISEM3;
    pub use super::HSEM_ICR::ISEM30;
    pub use super::HSEM_ICR::ISEM31;
    pub use super::HSEM_ICR::ISEM4;
    pub use super::HSEM_ICR::ISEM5;
    pub use super::HSEM_ICR::ISEM6;
    pub use super::HSEM_ICR::ISEM7;
    pub use super::HSEM_ICR::ISEM8;
    pub use super::HSEM_ICR::ISEM9;
}

/// HSEM Masked interrupt status register
pub mod HSEM_MISR {
    pub use super::HSEM_ICR::ISEM0;
    pub use super::HSEM_ICR::ISEM1;
    pub use super::HSEM_ICR::ISEM10;
    pub use super::HSEM_ICR::ISEM11;
    pub use super::HSEM_ICR::ISEM12;
    pub use super::HSEM_ICR::ISEM13;
    pub use super::HSEM_ICR::ISEM14;
    pub use super::HSEM_ICR::ISEM15;
    pub use super::HSEM_ICR::ISEM16;
    pub use super::HSEM_ICR::ISEM17;
    pub use super::HSEM_ICR::ISEM18;
    pub use super::HSEM_ICR::ISEM19;
    pub use super::HSEM_ICR::ISEM2;
    pub use super::HSEM_ICR::ISEM20;
    pub use super::HSEM_ICR::ISEM21;
    pub use super::HSEM_ICR::ISEM22;
    pub use super::HSEM_ICR::ISEM23;
    pub use super::HSEM_ICR::ISEM24;
    pub use super::HSEM_ICR::ISEM25;
    pub use super::HSEM_ICR::ISEM26;
    pub use super::HSEM_ICR::ISEM27;
    pub use super::HSEM_ICR::ISEM28;
    pub use super::HSEM_ICR::ISEM29;
    pub use super::HSEM_ICR::ISEM3;
    pub use super::HSEM_ICR::ISEM30;
    pub use super::HSEM_ICR::ISEM31;
    pub use super::HSEM_ICR::ISEM4;
    pub use super::HSEM_ICR::ISEM5;
    pub use super::HSEM_ICR::ISEM6;
    pub use super::HSEM_ICR::ISEM7;
    pub use super::HSEM_ICR::ISEM8;
    pub use super::HSEM_ICR::ISEM9;
}

/// HSEM Clear register
pub mod HSEM_CR {

    /// MasterID of semaphores to be cleared
    pub mod MASTERID {
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

    /// Semaphore clear Key
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

/// HSEM Interrupt clear register
pub mod HSEM_KEYR {

    /// Semaphore Clear Key
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
pub struct RegisterBlock {
    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R0: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R1: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R2: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R3: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R4: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R5: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R6: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R7: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R8: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R9: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R10: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R11: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R12: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R13: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R14: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R15: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R16: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R17: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R18: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R19: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R20: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R21: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R22: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R23: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R24: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R25: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R26: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R27: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R28: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R29: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R30: RWRegister<u32>,

    /// HSEM register HSEM_R0 HSEM_R31
    pub HSEM_R31: RWRegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR0: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR1: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR2: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR3: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR4: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR5: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR6: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR7: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR8: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR9: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR10: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR11: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR12: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR13: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR14: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR15: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR16: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR17: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR18: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR19: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR20: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR21: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR22: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR23: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR24: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR25: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR26: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR27: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR28: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR29: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR30: RORegister<u32>,

    /// HSEM Read lock register
    pub HSEM_RLR31: RORegister<u32>,

    /// HSEM Interrupt enable register
    pub HSEM_IER: RWRegister<u32>,

    /// HSEM Interrupt clear register
    pub HSEM_ICR: RORegister<u32>,

    /// HSEM Interrupt status register
    pub HSEM_ISR: RORegister<u32>,

    /// HSEM Masked interrupt status register
    pub HSEM_MISR: RORegister<u32>,

    _reserved1: [u32; 12],

    /// HSEM Clear register
    pub HSEM_CR: RWRegister<u32>,

    /// HSEM Interrupt clear register
    pub HSEM_KEYR: RWRegister<u32>,
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
    pub HSEM_IER: u32,
    pub HSEM_ICR: u32,
    pub HSEM_ISR: u32,
    pub HSEM_MISR: u32,
    pub HSEM_CR: u32,
    pub HSEM_KEYR: u32,
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

/// Access functions for the HSEM peripheral instance
pub mod HSEM {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58026400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HSEM
    pub const reset: ResetValues = ResetValues {
        HSEM_R0: 0x00000000,
        HSEM_R1: 0x00000000,
        HSEM_R2: 0x00000000,
        HSEM_R3: 0x00000000,
        HSEM_R4: 0x00000000,
        HSEM_R5: 0x00000000,
        HSEM_R6: 0x00000000,
        HSEM_R7: 0x00000000,
        HSEM_R8: 0x00000000,
        HSEM_R9: 0x00000000,
        HSEM_R10: 0x00000000,
        HSEM_R11: 0x00000000,
        HSEM_R12: 0x00000000,
        HSEM_R13: 0x00000000,
        HSEM_R14: 0x00000000,
        HSEM_R15: 0x00000000,
        HSEM_R16: 0x00000000,
        HSEM_R17: 0x00000000,
        HSEM_R18: 0x00000000,
        HSEM_R19: 0x00000000,
        HSEM_R20: 0x00000000,
        HSEM_R21: 0x00000000,
        HSEM_R22: 0x00000000,
        HSEM_R23: 0x00000000,
        HSEM_R24: 0x00000000,
        HSEM_R25: 0x00000000,
        HSEM_R26: 0x00000000,
        HSEM_R27: 0x00000000,
        HSEM_R28: 0x00000000,
        HSEM_R29: 0x00000000,
        HSEM_R30: 0x00000000,
        HSEM_R31: 0x00000000,
        HSEM_RLR0: 0x00000000,
        HSEM_RLR1: 0x00000000,
        HSEM_RLR2: 0x00000000,
        HSEM_RLR3: 0x00000000,
        HSEM_RLR4: 0x00000000,
        HSEM_RLR5: 0x00000000,
        HSEM_RLR6: 0x00000000,
        HSEM_RLR7: 0x00000000,
        HSEM_RLR8: 0x00000000,
        HSEM_RLR9: 0x00000000,
        HSEM_RLR10: 0x00000000,
        HSEM_RLR11: 0x00000000,
        HSEM_RLR12: 0x00000000,
        HSEM_RLR13: 0x00000000,
        HSEM_RLR14: 0x00000000,
        HSEM_RLR15: 0x00000000,
        HSEM_RLR16: 0x00000000,
        HSEM_RLR17: 0x00000000,
        HSEM_RLR18: 0x00000000,
        HSEM_RLR19: 0x00000000,
        HSEM_RLR20: 0x00000000,
        HSEM_RLR21: 0x00000000,
        HSEM_RLR22: 0x00000000,
        HSEM_RLR23: 0x00000000,
        HSEM_RLR24: 0x00000000,
        HSEM_RLR25: 0x00000000,
        HSEM_RLR26: 0x00000000,
        HSEM_RLR27: 0x00000000,
        HSEM_RLR28: 0x00000000,
        HSEM_RLR29: 0x00000000,
        HSEM_RLR30: 0x00000000,
        HSEM_RLR31: 0x00000000,
        HSEM_IER: 0x00000000,
        HSEM_ICR: 0x00000000,
        HSEM_ISR: 0x00000000,
        HSEM_MISR: 0x00000000,
        HSEM_CR: 0x00000000,
        HSEM_KEYR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut HSEM_TAKEN: bool = false;

    /// Safe access to HSEM
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
            if HSEM_TAKEN {
                None
            } else {
                HSEM_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HSEM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HSEM_TAKEN && inst.addr == INSTANCE.addr {
                HSEM_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to HSEM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HSEM: *const RegisterBlock = 0x58026400 as *const _;
