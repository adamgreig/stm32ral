#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! HSEM

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Semaphore %s register
pub mod R0 {

    /// lock indication
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

    /// Semaphore CoreID
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
}

/// Semaphore %s register
pub mod R1 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R2 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R3 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R4 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R5 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R6 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R7 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R8 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R9 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R10 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R11 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R12 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R13 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R14 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R15 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R16 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R17 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R18 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R19 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R20 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R21 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R22 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R23 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R24 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R25 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R26 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R27 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R28 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R29 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R30 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s register
pub mod R31 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR0 {

    /// lock indication
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

    /// Semaphore CoreID
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
}

/// Semaphore %s read lock register
pub mod RLR1 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR2 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR3 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR4 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR5 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR6 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR7 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR8 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR9 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR10 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR11 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR12 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR13 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR14 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR15 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR16 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR17 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR18 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR19 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR20 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR21 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR22 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR23 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR24 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR25 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR26 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR27 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR28 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR29 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR30 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore %s read lock register
pub mod RLR31 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// Semaphore Clear register
pub mod CR {

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

    /// CoreID of semaphore to be cleared
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
}

/// Interrupt clear register
pub mod KEYR {

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

/// Semaphore hardware configuration register 2
pub mod HWCFGR2 {

    /// Hardware Configuration valid bus masters ID4
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

    /// Hardware Configuration valid bus masters ID3
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

    /// Hardware Configuration valid bus masters ID2
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

    /// Hardware Configuration valid bus masters ID1
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
}

/// Semaphore hardware configuration register 1
pub mod HWCFGR1 {

    /// Hardware Configuration number of interrupts supported number of master IDs
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

    /// Hardware Configuration number of semaphores
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
}

/// HSEM version register
pub mod VERR {

    /// Major Revision
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

    /// Minor Revision
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
}

/// HSEM indentification register
pub mod IPIDR {

    /// Identification Code
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

/// HSEM size indentification register
pub mod SIDR {

    /// Size Identification Code
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

/// HSEM Interrupt enable register
pub mod C1IER {

    /// CPU(n) semaphore m enable bit
    pub mod ISEm {
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

/// HSEM Interrupt clear register
pub mod C1ICR {

    /// CPU(n) semaphore m clear bit
    pub mod ISCm {
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

/// HSEM Interrupt status register
pub mod C1ISR {

    /// CPU(n) semaphore m status bit before enable (mask)
    pub mod ISFm {
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

/// HSEM Masked interrupt status register
pub mod C1MISR {

    /// masked CPU(n) semaphore m status bit after enable (mask).
    pub mod MISFm {
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

/// HSEM Interrupt enable register
pub mod C2IER {
    pub use super::C1IER::ISEm;
}

/// HSEM Interrupt clear register
pub mod C2ICR {
    pub use super::C1ICR::ISCm;
}

/// HSEM Interrupt status register
pub mod C2ISR {
    pub use super::C1ISR::ISFm;
}

/// HSEM Masked interrupt status register
pub mod C2MISR {
    pub use super::C1MISR::MISFm;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Semaphore %s register
    pub R0: RWRegister<u32>,

    /// Semaphore %s register
    pub R1: RWRegister<u32>,

    /// Semaphore %s register
    pub R2: RWRegister<u32>,

    /// Semaphore %s register
    pub R3: RWRegister<u32>,

    /// Semaphore %s register
    pub R4: RWRegister<u32>,

    /// Semaphore %s register
    pub R5: RWRegister<u32>,

    /// Semaphore %s register
    pub R6: RWRegister<u32>,

    /// Semaphore %s register
    pub R7: RWRegister<u32>,

    /// Semaphore %s register
    pub R8: RWRegister<u32>,

    /// Semaphore %s register
    pub R9: RWRegister<u32>,

    /// Semaphore %s register
    pub R10: RWRegister<u32>,

    /// Semaphore %s register
    pub R11: RWRegister<u32>,

    /// Semaphore %s register
    pub R12: RWRegister<u32>,

    /// Semaphore %s register
    pub R13: RWRegister<u32>,

    /// Semaphore %s register
    pub R14: RWRegister<u32>,

    /// Semaphore %s register
    pub R15: RWRegister<u32>,

    /// Semaphore %s register
    pub R16: RWRegister<u32>,

    /// Semaphore %s register
    pub R17: RWRegister<u32>,

    /// Semaphore %s register
    pub R18: RWRegister<u32>,

    /// Semaphore %s register
    pub R19: RWRegister<u32>,

    /// Semaphore %s register
    pub R20: RWRegister<u32>,

    /// Semaphore %s register
    pub R21: RWRegister<u32>,

    /// Semaphore %s register
    pub R22: RWRegister<u32>,

    /// Semaphore %s register
    pub R23: RWRegister<u32>,

    /// Semaphore %s register
    pub R24: RWRegister<u32>,

    /// Semaphore %s register
    pub R25: RWRegister<u32>,

    /// Semaphore %s register
    pub R26: RWRegister<u32>,

    /// Semaphore %s register
    pub R27: RWRegister<u32>,

    /// Semaphore %s register
    pub R28: RWRegister<u32>,

    /// Semaphore %s register
    pub R29: RWRegister<u32>,

    /// Semaphore %s register
    pub R30: RWRegister<u32>,

    /// Semaphore %s register
    pub R31: RWRegister<u32>,

    /// Semaphore %s read lock register
    pub RLR0: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR1: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR2: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR3: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR4: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR5: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR6: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR7: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR8: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR9: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR10: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR11: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR12: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR13: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR14: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR15: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR16: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR17: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR18: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR19: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR20: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR21: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR22: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR23: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR24: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR25: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR26: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR27: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR28: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR29: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR30: RORegister<u32>,

    /// Semaphore %s read lock register
    pub RLR31: RORegister<u32>,

    /// HSEM Interrupt enable register
    pub C1IER: RWRegister<u32>,

    /// HSEM Interrupt clear register
    pub C1ICR: RWRegister<u32>,

    /// HSEM Interrupt status register
    pub C1ISR: RORegister<u32>,

    /// HSEM Masked interrupt status register
    pub C1MISR: RORegister<u32>,

    /// HSEM Interrupt enable register
    pub C2IER: RWRegister<u32>,

    /// HSEM Interrupt clear register
    pub C2ICR: RWRegister<u32>,

    /// HSEM Interrupt status register
    pub C2ISR: RORegister<u32>,

    /// HSEM Masked interrupt status register
    pub C2MISR: RORegister<u32>,

    _reserved1: [u8; 32],

    /// Semaphore Clear register
    pub CR: RWRegister<u32>,

    /// Interrupt clear register
    pub KEYR: RWRegister<u32>,

    _reserved2: [u8; 676],

    /// Semaphore hardware configuration register 2
    pub HWCFGR2: RORegister<u32>,

    /// Semaphore hardware configuration register 1
    pub HWCFGR1: RORegister<u32>,

    /// HSEM version register
    pub VERR: RORegister<u32>,

    /// HSEM indentification register
    pub IPIDR: RORegister<u32>,

    /// HSEM size indentification register
    pub SIDR: RORegister<u32>,
}
pub struct ResetValues {
    pub R0: u32,
    pub R1: u32,
    pub R2: u32,
    pub R3: u32,
    pub R4: u32,
    pub R5: u32,
    pub R6: u32,
    pub R7: u32,
    pub R8: u32,
    pub R9: u32,
    pub R10: u32,
    pub R11: u32,
    pub R12: u32,
    pub R13: u32,
    pub R14: u32,
    pub R15: u32,
    pub R16: u32,
    pub R17: u32,
    pub R18: u32,
    pub R19: u32,
    pub R20: u32,
    pub R21: u32,
    pub R22: u32,
    pub R23: u32,
    pub R24: u32,
    pub R25: u32,
    pub R26: u32,
    pub R27: u32,
    pub R28: u32,
    pub R29: u32,
    pub R30: u32,
    pub R31: u32,
    pub RLR0: u32,
    pub RLR1: u32,
    pub RLR2: u32,
    pub RLR3: u32,
    pub RLR4: u32,
    pub RLR5: u32,
    pub RLR6: u32,
    pub RLR7: u32,
    pub RLR8: u32,
    pub RLR9: u32,
    pub RLR10: u32,
    pub RLR11: u32,
    pub RLR12: u32,
    pub RLR13: u32,
    pub RLR14: u32,
    pub RLR15: u32,
    pub RLR16: u32,
    pub RLR17: u32,
    pub RLR18: u32,
    pub RLR19: u32,
    pub RLR20: u32,
    pub RLR21: u32,
    pub RLR22: u32,
    pub RLR23: u32,
    pub RLR24: u32,
    pub RLR25: u32,
    pub RLR26: u32,
    pub RLR27: u32,
    pub RLR28: u32,
    pub RLR29: u32,
    pub RLR30: u32,
    pub RLR31: u32,
    pub C1IER: u32,
    pub C1ICR: u32,
    pub C1ISR: u32,
    pub C1MISR: u32,
    pub C2IER: u32,
    pub C2ICR: u32,
    pub C2ISR: u32,
    pub C2MISR: u32,
    pub CR: u32,
    pub KEYR: u32,
    pub HWCFGR2: u32,
    pub HWCFGR1: u32,
    pub VERR: u32,
    pub IPIDR: u32,
    pub SIDR: u32,
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

/// Access functions for the HSEM peripheral instance
pub mod HSEM {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58001400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HSEM
    pub const reset: ResetValues = ResetValues {
        R0: 0x00000000,
        R1: 0x00000000,
        R2: 0x00000000,
        R3: 0x00000000,
        R4: 0x00000000,
        R5: 0x00000000,
        R6: 0x00000000,
        R7: 0x00000000,
        R8: 0x00000000,
        R9: 0x00000000,
        R10: 0x00000000,
        R11: 0x00000000,
        R12: 0x00000000,
        R13: 0x00000000,
        R14: 0x00000000,
        R15: 0x00000000,
        R16: 0x00000000,
        R17: 0x00000000,
        R18: 0x00000000,
        R19: 0x00000000,
        R20: 0x00000000,
        R21: 0x00000000,
        R22: 0x00000000,
        R23: 0x00000000,
        R24: 0x00000000,
        R25: 0x00000000,
        R26: 0x00000000,
        R27: 0x00000000,
        R28: 0x00000000,
        R29: 0x00000000,
        R30: 0x00000000,
        R31: 0x00000000,
        RLR0: 0x00000000,
        RLR1: 0x00000000,
        RLR2: 0x00000000,
        RLR3: 0x00000000,
        RLR4: 0x00000000,
        RLR5: 0x00000000,
        RLR6: 0x00000000,
        RLR7: 0x00000000,
        RLR8: 0x00000000,
        RLR9: 0x00000000,
        RLR10: 0x00000000,
        RLR11: 0x00000000,
        RLR12: 0x00000000,
        RLR13: 0x00000000,
        RLR14: 0x00000000,
        RLR15: 0x00000000,
        RLR16: 0x00000000,
        RLR17: 0x00000000,
        RLR18: 0x00000000,
        RLR19: 0x00000000,
        RLR20: 0x00000000,
        RLR21: 0x00000000,
        RLR22: 0x00000000,
        RLR23: 0x00000000,
        RLR24: 0x00000000,
        RLR25: 0x00000000,
        RLR26: 0x00000000,
        RLR27: 0x00000000,
        RLR28: 0x00000000,
        RLR29: 0x00000000,
        RLR30: 0x00000000,
        RLR31: 0x00000000,
        CR: 0x00000000,
        KEYR: 0x00000000,
        HWCFGR2: 0x00000084,
        HWCFGR1: 0x00000220,
        VERR: 0x00000020,
        IPIDR: 0x00100072,
        SIDR: 0xA3C5DD01,
        C1IER: 0x00000000,
        C1ICR: 0x00000000,
        C1ISR: 0x00000000,
        C1MISR: 0x00000000,
        C2IER: 0x00000000,
        C2ICR: 0x00000000,
        C2ISR: 0x00000000,
        C2MISR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
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

    /// Unsafely steal HSEM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HSEM_TAKEN = true;
        INSTANCE
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
pub const HSEM: *const RegisterBlock = 0x58001400 as *const _;
