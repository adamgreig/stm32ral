#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Hardware semaphore

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// HSEM register HSEM_R%s HSEM_R31
pub mod R0 {

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

    /// Semaphore MASTERID
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

    /// Lock indication
    pub mod LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Semaphore is free
            pub const Free: u32 = 0b0;

            /// 0b1: Semaphore is locked
            pub const Locked: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: Free semaphore
            pub const Free: u32 = 0b0;

            /// 0b1: Try to lock semaphore
            pub const TryLock: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R1 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R2 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R3 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R4 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R5 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R6 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R7 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R8 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R9 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R10 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R11 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R12 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R13 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R14 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// HSEM register HSEM_R%s HSEM_R31
pub mod R15 {
    pub use super::R0::COREID;
    pub use super::R0::LOCK;
    pub use super::R0::PROCID;
}

/// HSEM Read lock register
pub mod RLR0 {

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

    /// Semaphore MASTERID
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

    /// Lock indication
    pub mod LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Semaphore is free
            pub const Free: u32 = 0b0;

            /// 0b1: Semaphore is locked
            pub const Locked: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// HSEM Read lock register
pub mod RLR1 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR2 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR3 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR4 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR5 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR6 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR7 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR8 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR9 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR10 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR11 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR12 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR13 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR14 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// HSEM Read lock register
pub mod RLR15 {
    pub use super::RLR0::COREID;
    pub use super::RLR0::LOCK;
    pub use super::RLR0::PROCID;
}

/// HSEM Interrupt enable register
pub mod C1IER {

    /// Interrupt semaphore n enable bit
    pub mod ISE0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt generation disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt generation enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Interrupt semaphore n enable bit
    pub mod ISE1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ISE0::RW;
    }

    /// Interrupt semaphore n enable bit
    pub mod ISE2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ISE0::RW;
    }

    /// Interrupt semaphore n enable bit
    pub mod ISE3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ISE0::RW;
    }

    /// Interrupt semaphore n enable bit
    pub mod ISE4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ISE0::RW;
    }

    /// Interrupt semaphore n enable bit
    pub mod ISE5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ISE0::RW;
    }

    /// Interrupt semaphore n enable bit
    pub mod ISE6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ISE0::RW;
    }

    /// Interrupt semaphore n enable bit
    pub mod ISE7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ISE0::RW;
    }

    /// Interrupt semaphore n enable bit
    pub mod ISE8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ISE0::RW;
    }

    /// Interrupt semaphore n enable bit
    pub mod ISE9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ISE0::RW;
    }

    /// Interrupt semaphore n enable bit
    pub mod ISE10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ISE0::RW;
    }

    /// Interrupt semaphore n enable bit
    pub mod ISE11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ISE0::RW;
    }

    /// Interrupt semaphore n enable bit
    pub mod ISE12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ISE0::RW;
    }

    /// Interrupt semaphore n enable bit
    pub mod ISE13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ISE0::RW;
    }

    /// Interrupt semaphore n enable bit
    pub mod ISE14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ISE0::RW;
    }

    /// Interrupt semaphore n enable bit
    pub mod ISE15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ISE0::RW;
    }
}

/// HSEM Interrupt clear register
pub mod C1ICR {

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Always reads 0
            pub const NoEffect: u32 = 0b0;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: Interrupt semaphore x status ISFx and masked status MISFx not affected
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Interrupt semaphore x status ISFx and masked status MISFx cleared
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::ISC0::R;
        pub use super::ISC0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::ISC0::R;
        pub use super::ISC0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::ISC0::R;
        pub use super::ISC0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::ISC0::R;
        pub use super::ISC0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        pub use super::ISC0::R;
        pub use super::ISC0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        pub use super::ISC0::R;
        pub use super::ISC0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        pub use super::ISC0::R;
        pub use super::ISC0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        pub use super::ISC0::R;
        pub use super::ISC0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        pub use super::ISC0::R;
        pub use super::ISC0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        pub use super::ISC0::R;
        pub use super::ISC0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        pub use super::ISC0::R;
        pub use super::ISC0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        pub use super::ISC0::R;
        pub use super::ISC0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        pub use super::ISC0::R;
        pub use super::ISC0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        pub use super::ISC0::R;
        pub use super::ISC0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n clear bit
    pub mod ISC15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        pub use super::ISC0::R;
        pub use super::ISC0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// HSEM Interrupt status register
pub mod C1ISR {

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No interrupt pending
            pub const NotPending: u32 = 0b0;

            /// 0b1: Interrupt pending
            pub const Pending: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::ISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::ISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::ISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::ISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        pub use super::ISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        pub use super::ISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        pub use super::ISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        pub use super::ISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        pub use super::ISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        pub use super::ISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        pub use super::ISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        pub use super::ISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        pub use super::ISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        pub use super::ISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt(N) semaphore n status bit before enable (mask)
    pub mod ISF15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        pub use super::ISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// HSEM Masked interrupt status register
pub mod C1MISR {

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No interrupt pending after masking
            pub const NotPending: u32 = 0b0;

            /// 0b1: Interrupt pending after masking
            pub const Pending: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::MISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::MISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::MISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::MISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        pub use super::MISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        pub use super::MISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        pub use super::MISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        pub use super::MISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        pub use super::MISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        pub use super::MISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        pub use super::MISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        pub use super::MISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        pub use super::MISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        pub use super::MISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// masked interrupt(N) semaphore n status bit after enable (mask)
    pub mod MISF15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        pub use super::MISF0::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// HSEM Clear register
pub mod CR {

    /// MASTERID
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
#[repr(C)]
pub struct RegisterBlock {
    /// HSEM register HSEM_R%s HSEM_R31
    pub R0: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R1: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R2: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R3: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R4: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R5: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R6: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R7: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R8: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R9: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R10: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R11: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R12: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R13: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R14: RWRegister<u32>,

    /// HSEM register HSEM_R%s HSEM_R31
    pub R15: RWRegister<u32>,

    _reserved1: [u8; 64],

    /// HSEM Read lock register
    pub RLR0: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR1: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR2: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR3: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR4: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR5: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR6: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR7: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR8: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR9: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR10: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR11: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR12: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR13: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR14: RORegister<u32>,

    /// HSEM Read lock register
    pub RLR15: RORegister<u32>,

    _reserved2: [u8; 64],

    /// HSEM Interrupt enable register
    pub C1IER: RWRegister<u32>,

    /// HSEM Interrupt clear register
    pub C1ICR: RWRegister<u32>,

    /// HSEM Interrupt status register
    pub C1ISR: RORegister<u32>,

    /// HSEM Masked interrupt status register
    pub C1MISR: RORegister<u32>,

    _reserved3: [u8; 48],

    /// HSEM Clear register
    pub CR: WORegister<u32>,

    /// HSEM Interrupt clear register
    pub KEYR: RWRegister<u32>,
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
    pub C1IER: u32,
    pub C1ICR: u32,
    pub C1ISR: u32,
    pub C1MISR: u32,
    pub CR: u32,
    pub KEYR: u32,
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
        C1IER: 0x00000000,
        C1ICR: 0x00000000,
        C1ISR: 0x00000000,
        C1MISR: 0x00000000,
        CR: 0x00000000,
        KEYR: 0x00000000,
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
