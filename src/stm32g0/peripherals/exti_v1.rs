#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! External interrupt/event controller
//!
//! Used by: stm32g030, stm32g031, stm32g041

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// EXTI rising trigger selection register
pub mod RTSR1 {

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR0 {
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

            /// 0b0: Rising edge trigger is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Rising edge trigger is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }
}

/// EXTI falling trigger selection register
pub mod FTSR1 {

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR0 {
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

            /// 0b0: Falling edge trigger is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Falling edge trigger is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Rising edge trigger is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Rising edge trigger is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }
}

/// EXTI software interrupt event register
pub mod SWIER1 {

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Generates an interrupt request
            pub const Pend: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EXTI rising edge pending register
pub mod RPR1 {

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No trigger request occurred
            pub const NotPending: u32 = 0b0;

            /// 0b1: Selected trigger request occurred
            pub const Pending: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clears pending bit
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit
    pub mod RPIF5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x rising edge Pending bit.
    pub mod RPIF16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        pub use super::RPIF0::R;
        pub use super::RPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EXTI falling edge pending register
pub mod FPR1 {

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No trigger request occurred
            pub const NotPending: u32 = 0b0;

            /// 0b1: Selected trigger request occurred
            pub const Pending: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clears pending bit
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// configurable event inputs x falling edge pending bit.
    pub mod FPIF16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        pub use super::FPIF0::R;
        pub use super::FPIF0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EXTI external interrupt selection register
pub mod EXTICR1 {

    /// GPIO port selection
    pub mod EXTI0_7 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: GPIO port A selected
            pub const PA: u32 = 0b00000000;

            /// 0b00000001: GPIO port B selected
            pub const PB: u32 = 0b00000001;

            /// 0b00000010: GPIO port C selected
            pub const PC: u32 = 0b00000010;

            /// 0b00000011: GPIO port D selected
            pub const PD: u32 = 0b00000011;

            /// 0b00000101: GPIO port F selected
            pub const PF: u32 = 0b00000101;
        }
    }

    /// GPIO port selection
    pub mod EXTI8_15 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTI0_7::RW;
    }

    /// GPIO port selection
    pub mod EXTI16_23 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTI0_7::RW;
    }

    /// GPIO port selection
    pub mod EXTI24_31 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTI0_7::RW;
    }
}

/// EXTI external interrupt selection register
pub mod EXTICR2 {
    pub use super::EXTICR1::EXTI0_7;
    pub use super::EXTICR1::EXTI16_23;
    pub use super::EXTICR1::EXTI24_31;
    pub use super::EXTICR1::EXTI8_15;
}

/// EXTI external interrupt selection register
pub mod EXTICR3 {
    pub use super::EXTICR1::EXTI0_7;
    pub use super::EXTICR1::EXTI16_23;
    pub use super::EXTICR1::EXTI24_31;
    pub use super::EXTICR1::EXTI8_15;
}

/// EXTI external interrupt selection register
pub mod EXTICR4 {
    pub use super::EXTICR1::EXTI0_7;
    pub use super::EXTICR1::EXTI16_23;
    pub use super::EXTICR1::EXTI24_31;
    pub use super::EXTICR1::EXTI8_15;
}

/// EXTI CPU wakeup with interrupt mask register
pub mod IMR1 {

    /// CPU wakeup with interrupt mask on event input
    pub mod IM0 {
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

            /// 0b0: Interrupt request line is masked
            pub const Masked: u32 = 0b0;

            /// 0b1: Interrupt request line is unmasked
            pub const Unmasked: u32 = 0b1;
        }
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// CPU wakeup with interrupt mask on event input
    pub mod IM31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }
}

/// EXTI CPU wakeup with event mask register
pub mod EMR1 {

    /// CPU wakeup with event mask on event input
    pub mod EM0 {
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

            /// 0b0: Interrupt request line is masked
            pub const Masked: u32 = 0b0;

            /// 0b1: Interrupt request line is unmasked
            pub const Unmasked: u32 = 0b1;
        }
    }

    /// CPU wakeup with event mask on event input
    pub mod EM1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// CPU wakeup with event mask on event input
    pub mod EM31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }
}
pub struct RegisterBlock {
    /// EXTI rising trigger selection register
    pub RTSR1: RWRegister<u32>,

    /// EXTI falling trigger selection register
    pub FTSR1: RWRegister<u32>,

    /// EXTI software interrupt event register
    pub SWIER1: RWRegister<u32>,

    /// EXTI rising edge pending register
    pub RPR1: RWRegister<u32>,

    /// EXTI falling edge pending register
    pub FPR1: RWRegister<u32>,

    _reserved1: [u32; 19],

    /// EXTI external interrupt selection register
    pub EXTICR1: RWRegister<u32>,

    /// EXTI external interrupt selection register
    pub EXTICR2: RWRegister<u32>,

    /// EXTI external interrupt selection register
    pub EXTICR3: RWRegister<u32>,

    /// EXTI external interrupt selection register
    pub EXTICR4: RWRegister<u32>,

    _reserved2: [u32; 4],

    /// EXTI CPU wakeup with interrupt mask register
    pub IMR1: RWRegister<u32>,

    /// EXTI CPU wakeup with event mask register
    pub EMR1: RWRegister<u32>,
}
pub struct ResetValues {
    pub RTSR1: u32,
    pub FTSR1: u32,
    pub SWIER1: u32,
    pub RPR1: u32,
    pub FPR1: u32,
    pub EXTICR1: u32,
    pub EXTICR2: u32,
    pub EXTICR3: u32,
    pub EXTICR4: u32,
    pub IMR1: u32,
    pub EMR1: u32,
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
