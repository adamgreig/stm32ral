#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! External interrupt/event controller
//!
//! Used by: stm32h743, stm32h743v, stm32h753, stm32h753v

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

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
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

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR1::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod TR21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
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

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod SWIER21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EXTI D3 pending mask register
pub mod D3PMR1 {

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR0 {
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

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }
}

/// EXTI D3 pending clear selection register low
pub mod D3PCR1L {

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: DMA ch6 event selected as D3 domain pendclear source
            pub const DMA_CH6: u32 = 0b00;

            /// 0b01: DMA ch7 event selected as D3 domain pendclear source
            pub const DMA_CH7: u32 = 0b01;

            /// 0b10: LPTIM4 out selected as D3 domain pendclear source
            pub const LPTIM4: u32 = 0b10;

            /// 0b11: LPTIM5 out selected as D3 domain pendclear source
            pub const LPTIM5: u32 = 0b11;
        }
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS0::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS0::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS3 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS0::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS0::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS5 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS0::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS6 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS0::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS0::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS8 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS0::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS9 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS0::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS10 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS0::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS11 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS0::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS12 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS0::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS13 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS0::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS14 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS0::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate (n/2)
    pub mod PCS15 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS0::RW;
    }
}

/// EXTI D3 pending clear selection register high
pub mod D3PCR1H {

    /// D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
    pub mod PCS19 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: DMA ch6 event selected as D3 domain pendclear source
            pub const DMA_CH6: u32 = 0b00;

            /// 0b01: DMA ch7 event selected as D3 domain pendclear source
            pub const DMA_CH7: u32 = 0b01;

            /// 0b10: LPTIM4 out selected as D3 domain pendclear source
            pub const LPTIM4: u32 = 0b10;

            /// 0b11: LPTIM5 out selected as D3 domain pendclear source
            pub const LPTIM5: u32 = 0b11;
        }
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
    pub mod PCS20 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS19::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
    pub mod PCS21 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS19::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
    pub mod PCS25 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS19::RW;
    }
}

/// EXTI rising trigger selection register
pub mod RTSR2 {

    /// Rising trigger event configuration bit of Configurable Event input x+32
    pub mod TR49 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
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

    /// Rising trigger event configuration bit of Configurable Event input x+32
    pub mod TR51 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR49::RW;
    }
}

/// EXTI falling trigger selection register
pub mod FTSR2 {

    /// Falling trigger event configuration bit of Configurable Event input x+32
    pub mod TR49 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
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

    /// Falling trigger event configuration bit of Configurable Event input x+32
    pub mod TR51 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
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
}

/// EXTI software interrupt event register
pub mod SWIER2 {

    /// Software interrupt on line x+32
    pub mod SWIER49 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
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

    /// Software interrupt on line x+32
    pub mod SWIER51 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER49::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EXTI D3 pending mask register
pub mod D3PMR2 {

    /// D3 Pending Mask on Event input x+32
    pub mod MR34 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
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

    /// D3 Pending Mask on Event input x+32
    pub mod MR35 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR34::RW;
    }

    /// D3 Pending Mask on Event input x+32
    pub mod MR41 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR34::RW;
    }

    /// D3 Pending Mask on Event input x+32
    pub mod MR48 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR34::RW;
    }

    /// D3 Pending Mask on Event input x+32
    pub mod MR49 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR34::RW;
    }

    /// D3 Pending Mask on Event input x+32
    pub mod MR50 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR34::RW;
    }

    /// D3 Pending Mask on Event input x+32
    pub mod MR51 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR34::RW;
    }

    /// D3 Pending Mask on Event input x+32
    pub mod MR52 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR34::RW;
    }

    /// D3 Pending Mask on Event input x+32
    pub mod MR53 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR34::RW;
    }
}

/// EXTI D3 pending clear selection register low
pub mod D3PCR2L {

    /// D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
    pub mod PCS35 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: DMA ch6 event selected as D3 domain pendclear source
            pub const DMA_CH6: u32 = 0b00;

            /// 0b01: DMA ch7 event selected as D3 domain pendclear source
            pub const DMA_CH7: u32 = 0b01;

            /// 0b10: LPTIM4 out selected as D3 domain pendclear source
            pub const LPTIM4: u32 = 0b10;

            /// 0b11: LPTIM5 out selected as D3 domain pendclear source
            pub const LPTIM5: u32 = 0b11;
        }
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
    pub mod PCS34 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS35::RW;
    }

    /// D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
    pub mod PCS41 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS35::RW;
    }
}

/// EXTI D3 pending clear selection register high
pub mod D3PCR2H {

    /// Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    pub mod PCS48 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: DMA ch6 event selected as D3 domain pendclear source
            pub const DMA_CH6: u32 = 0b00;

            /// 0b01: DMA ch7 event selected as D3 domain pendclear source
            pub const DMA_CH7: u32 = 0b01;

            /// 0b10: LPTIM4 out selected as D3 domain pendclear source
            pub const LPTIM4: u32 = 0b10;

            /// 0b11: LPTIM5 out selected as D3 domain pendclear source
            pub const LPTIM5: u32 = 0b11;
        }
    }

    /// Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    pub mod PCS49 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS48::RW;
    }

    /// Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    pub mod PCS50 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS48::RW;
    }

    /// Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    pub mod PCS51 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS48::RW;
    }

    /// Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    pub mod PCS52 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS48::RW;
    }

    /// Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    pub mod PCS53 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PCS48::RW;
    }
}

/// EXTI rising trigger selection register
pub mod RTSR3 {

    /// Rising trigger event configuration bit of Configurable Event input x+64
    pub mod TR82 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
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

    /// Rising trigger event configuration bit of Configurable Event input x+64
    pub mod TR84 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR82::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input x+64
    pub mod TR85 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR82::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input x+64
    pub mod TR86 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR82::RW;
    }
}

/// EXTI falling trigger selection register
pub mod FTSR3 {

    /// Falling trigger event configuration bit of Configurable Event input x+64
    pub mod TR82 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
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

    /// Falling trigger event configuration bit of Configurable Event input x+64
    pub mod TR84 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
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

    /// Falling trigger event configuration bit of Configurable Event input x+64
    pub mod TR85 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR84::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input x+64
    pub mod TR86 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TR84::RW;
    }
}

/// EXTI software interrupt event register
pub mod SWIER3 {

    /// Software interrupt on line x+64
    pub mod SWIER82 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
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

    /// Software interrupt on line x+64
    pub mod SWIER84 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER82::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on line x+64
    pub mod SWIER85 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER82::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on line x+64
    pub mod SWIER86 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWIER82::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EXTI D3 pending mask register
pub mod D3PMR3 {

    /// D3 Pending Mask on Event input x+64
    pub mod MR88 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
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
}

/// EXTI D3 pending clear selection register high
pub mod D3PCR3H {

    /// D3 Pending request clear input signal selection on Event input x= truncate N+160/2
    pub mod PCS88 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: DMA ch6 event selected as D3 domain pendclear source
            pub const DMA_CH6: u32 = 0b00;

            /// 0b01: DMA ch7 event selected as D3 domain pendclear source
            pub const DMA_CH7: u32 = 0b01;

            /// 0b10: LPTIM4 out selected as D3 domain pendclear source
            pub const LPTIM4: u32 = 0b10;

            /// 0b11: LPTIM5 out selected as D3 domain pendclear source
            pub const LPTIM5: u32 = 0b11;
        }
    }
}

/// EXTI interrupt mask register
pub mod CPUIMR1 {

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR0 {
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

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod MR31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }
}

/// EXTI event mask register
pub mod CPUEMR1 {
    pub use super::CPUIMR1::MR0;
    pub use super::CPUIMR1::MR1;
    pub use super::CPUIMR1::MR10;
    pub use super::CPUIMR1::MR11;
    pub use super::CPUIMR1::MR12;
    pub use super::CPUIMR1::MR13;
    pub use super::CPUIMR1::MR14;
    pub use super::CPUIMR1::MR15;
    pub use super::CPUIMR1::MR16;
    pub use super::CPUIMR1::MR17;
    pub use super::CPUIMR1::MR18;
    pub use super::CPUIMR1::MR19;
    pub use super::CPUIMR1::MR2;
    pub use super::CPUIMR1::MR20;
    pub use super::CPUIMR1::MR21;
    pub use super::CPUIMR1::MR22;
    pub use super::CPUIMR1::MR23;
    pub use super::CPUIMR1::MR24;
    pub use super::CPUIMR1::MR25;
    pub use super::CPUIMR1::MR26;
    pub use super::CPUIMR1::MR27;
    pub use super::CPUIMR1::MR28;
    pub use super::CPUIMR1::MR29;
    pub use super::CPUIMR1::MR3;
    pub use super::CPUIMR1::MR30;
    pub use super::CPUIMR1::MR31;
    pub use super::CPUIMR1::MR4;
    pub use super::CPUIMR1::MR5;
    pub use super::CPUIMR1::MR6;
    pub use super::CPUIMR1::MR7;
    pub use super::CPUIMR1::MR8;
    pub use super::CPUIMR1::MR9;
}

/// EXTI pending register
pub mod CPUPR1 {

    /// CPU Event mask on Event input x
    pub mod PR0 {
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

    /// CPU Event mask on Event input x
    pub mod PR1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU Event mask on Event input x
    pub mod PR21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        pub use super::PR0::R;
        pub use super::PR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EXTI interrupt mask register
pub mod CPUIMR2 {

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR0 {
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

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR0::RW;
    }
}

/// EXTI event mask register
pub mod CPUEMR2 {

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR32 {
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

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR33 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR34 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR35 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR36 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR37 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR38 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR39 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR40 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR41 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR42 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR43 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR44 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR46 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR47 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR48 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR49 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR50 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR51 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR52 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR53 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR54 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR55 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR56 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR57 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR58 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR59 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR60 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR61 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR62 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+32
    pub mod MR63 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR32::RW;
    }
}

/// EXTI pending register
pub mod CPUPR2 {

    /// Configurable event inputs x+32 Pending bit
    pub mod PR49 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
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

    /// Configurable event inputs x+32 Pending bit
    pub mod PR51 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        pub use super::PR49::R;
        pub use super::PR49::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EXTI interrupt mask register
pub mod CPUIMR3 {

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR64 {
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

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR65 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR66 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR67 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR68 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR69 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR70 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR71 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR72 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR73 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR74 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR75 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR76 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR77 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR78 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR79 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR80 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR82 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR84 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR85 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR86 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR87 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }

    /// CPU Interrupt Mask on Direct Event input x+64
    pub mod MR88 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MR64::RW;
    }
}

/// EXTI event mask register
pub mod CPUEMR3 {
    pub use super::CPUIMR3::MR64;
    pub use super::CPUIMR3::MR65;
    pub use super::CPUIMR3::MR66;
    pub use super::CPUIMR3::MR67;
    pub use super::CPUIMR3::MR68;
    pub use super::CPUIMR3::MR69;
    pub use super::CPUIMR3::MR70;
    pub use super::CPUIMR3::MR71;
    pub use super::CPUIMR3::MR72;
    pub use super::CPUIMR3::MR73;
    pub use super::CPUIMR3::MR74;
    pub use super::CPUIMR3::MR75;
    pub use super::CPUIMR3::MR76;
    pub use super::CPUIMR3::MR77;
    pub use super::CPUIMR3::MR78;
    pub use super::CPUIMR3::MR79;
    pub use super::CPUIMR3::MR80;
    pub use super::CPUIMR3::MR82;
    pub use super::CPUIMR3::MR84;
    pub use super::CPUIMR3::MR85;
    pub use super::CPUIMR3::MR86;
    pub use super::CPUIMR3::MR87;
    pub use super::CPUIMR3::MR88;
}

/// EXTI pending register
pub mod CPUPR3 {

    /// Configurable event inputs x+64 Pending bit
    pub mod PR82 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
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

    /// Configurable event inputs x+64 Pending bit
    pub mod PR84 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        pub use super::PR82::R;
        pub use super::PR82::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs x+64 Pending bit
    pub mod PR85 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        pub use super::PR82::R;
        pub use super::PR82::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs x+64 Pending bit
    pub mod PR86 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        pub use super::PR82::R;
        pub use super::PR82::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// EXTI rising trigger selection register
    pub RTSR1: RWRegister<u32>,

    /// EXTI falling trigger selection register
    pub FTSR1: RWRegister<u32>,

    /// EXTI software interrupt event register
    pub SWIER1: RWRegister<u32>,

    /// EXTI D3 pending mask register
    pub D3PMR1: RWRegister<u32>,

    /// EXTI D3 pending clear selection register low
    pub D3PCR1L: RWRegister<u32>,

    /// EXTI D3 pending clear selection register high
    pub D3PCR1H: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// EXTI rising trigger selection register
    pub RTSR2: RWRegister<u32>,

    /// EXTI falling trigger selection register
    pub FTSR2: RWRegister<u32>,

    /// EXTI software interrupt event register
    pub SWIER2: RWRegister<u32>,

    /// EXTI D3 pending mask register
    pub D3PMR2: RWRegister<u32>,

    /// EXTI D3 pending clear selection register low
    pub D3PCR2L: RWRegister<u32>,

    /// EXTI D3 pending clear selection register high
    pub D3PCR2H: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// EXTI rising trigger selection register
    pub RTSR3: RWRegister<u32>,

    /// EXTI falling trigger selection register
    pub FTSR3: RWRegister<u32>,

    /// EXTI software interrupt event register
    pub SWIER3: RWRegister<u32>,

    /// EXTI D3 pending mask register
    pub D3PMR3: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// EXTI D3 pending clear selection register high
    pub D3PCR3H: RWRegister<u32>,

    _reserved4: [u32; 10],

    /// EXTI interrupt mask register
    pub CPUIMR1: RWRegister<u32>,

    /// EXTI event mask register
    pub CPUEMR1: RWRegister<u32>,

    /// EXTI pending register
    pub CPUPR1: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// EXTI interrupt mask register
    pub CPUIMR2: RWRegister<u32>,

    /// EXTI event mask register
    pub CPUEMR2: RWRegister<u32>,

    /// EXTI pending register
    pub CPUPR2: RWRegister<u32>,

    _reserved6: [u32; 1],

    /// EXTI interrupt mask register
    pub CPUIMR3: RWRegister<u32>,

    /// EXTI event mask register
    pub CPUEMR3: RWRegister<u32>,

    /// EXTI pending register
    pub CPUPR3: RWRegister<u32>,
}
pub struct ResetValues {
    pub RTSR1: u32,
    pub FTSR1: u32,
    pub SWIER1: u32,
    pub D3PMR1: u32,
    pub D3PCR1L: u32,
    pub D3PCR1H: u32,
    pub RTSR2: u32,
    pub FTSR2: u32,
    pub SWIER2: u32,
    pub D3PMR2: u32,
    pub D3PCR2L: u32,
    pub D3PCR2H: u32,
    pub RTSR3: u32,
    pub FTSR3: u32,
    pub SWIER3: u32,
    pub D3PMR3: u32,
    pub D3PCR3H: u32,
    pub CPUIMR1: u32,
    pub CPUEMR1: u32,
    pub CPUPR1: u32,
    pub CPUIMR2: u32,
    pub CPUEMR2: u32,
    pub CPUPR2: u32,
    pub CPUIMR3: u32,
    pub CPUEMR3: u32,
    pub CPUPR3: u32,
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
