#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! External interrupt/event controller
//!
//! Used by: stm32wl5x_cm0p, stm32wl5x_cm4

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// rising trigger selection register
pub mod RTSR1 {

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
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
    pub mod RT22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT21::RW;
    }
}

/// falling trigger selection register
pub mod FTSR1 {

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
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

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT21::RW;
    }
}

/// software interrupt event register
pub mod SWIER1 {

    /// Software interrupt on event
    pub mod SWI21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
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

    /// Software interrupt on event
    pub mod SWI22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EXTI pending register
pub mod PR1 {

    /// Configurable event inputs Pending bit
    pub mod PIF21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
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

    /// Configurable event inputs Pending bit
    pub mod PIF22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF21::R;
        pub use super::PIF21::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// rising trigger selection register
pub mod RTSR2 {

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT34 {
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

            /// 0b0: Rising edge trigger is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Rising edge trigger is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT40 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT34::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT41 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT34::RW;
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT45 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RT34::RW;
    }
}

/// falling trigger selection register
pub mod FTSR2 {

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT34 {
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

            /// 0b0: Falling edge trigger is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Falling edge trigger is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT40 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT34::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT41 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT34::RW;
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT45 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FT34::RW;
    }
}

/// software interrupt event register
pub mod SWIER2 {

    /// Software interrupt on event
    pub mod SWI34 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
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

    /// Software interrupt on event
    pub mod SWI40 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI34::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI41 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI34::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event 45
    pub mod SWI45 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWI34::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// pending register
pub mod PR2 {

    /// Configurable event inputs 33 Pending bit.
    pub mod PIF34 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
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

    /// Configurable event inputs 40_41 Pending bit.
    pub mod PIF40 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF34::R;
        pub use super::PIF34::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs 40_41 Pending bit.
    pub mod PIF41 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF34::R;
        pub use super::PIF34::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs 45 Pending bit.
    pub mod PIF45 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        pub use super::PIF34::R;
        pub use super::PIF34::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// interrupt mask register
pub mod C1IMR1 {

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
    pub mod IM17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// wakeup with interrupt Mask on event input
    pub mod IM18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
    pub mod IM27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM0::RW;
    }

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

    /// wakeup with interrupt Mask on event input
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

/// event mask register
pub mod C1EMR1 {

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
    pub mod EM17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// Wakeup with event generation Mask on Event input
    pub mod EM18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
    pub mod EM20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }

    /// Wakeup with event generation Mask on Event input
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

    /// Wakeup with event generation Mask on Event input
    pub mod EM22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM0::RW;
    }
}

/// wakeup with interrupt mask register
pub mod C1IMR2 {

    /// wakeup with interrupt mask on event input
    pub mod IM34 {
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

    /// wakeup with interrupt mask on event input
    pub mod IM36 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM34::RW;
    }

    /// wakeup with interrupt mask on event input
    pub mod IM37 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM34::RW;
    }

    /// wakeup with interrupt mask on event input
    pub mod IM38 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM34::RW;
    }

    /// wakeup with interrupt mask on event input
    pub mod IM39 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM34::RW;
    }

    /// wakeup with interrupt mask on event input
    pub mod IM40 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM34::RW;
    }

    /// wakeup with interrupt mask on event input
    pub mod IM41 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM34::RW;
    }

    /// wakeup with interrupt mask on event input
    pub mod IM42 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM34::RW;
    }

    /// wakeup with interrupt mask on event input
    pub mod IM43 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM34::RW;
    }

    /// wakeup with interrupt mask on event input
    pub mod IM44 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM34::RW;
    }

    /// wakeup with interrupt mask on event input
    pub mod IM45 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM34::RW;
    }

    /// wakeup with interrupt mask on event input
    pub mod IM46 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IM34::RW;
    }
}

/// wakeup with event mask register
pub mod C1EMR2 {

    /// Wakeup with event generation Mask on Event input
    pub mod EM40 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
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

    /// Wakeup with event generation Mask on Event input
    pub mod EM41 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EM40::RW;
    }
}

/// interrupt mask register
pub mod C2IMR1 {

    /// wakeup with interrupt Mask on Event input
    pub mod IM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: Interrupt request line is masked
            pub const Masked: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: Interrupt request line is unmasked
            pub const Unmasked: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// event mask register
pub mod C2EMR1 {
    pub use super::C1EMR1::EM0;
    pub use super::C1EMR1::EM1;
    pub use super::C1EMR1::EM10;
    pub use super::C1EMR1::EM11;
    pub use super::C1EMR1::EM12;
    pub use super::C1EMR1::EM13;
    pub use super::C1EMR1::EM14;
    pub use super::C1EMR1::EM15;
    pub use super::C1EMR1::EM17;
    pub use super::C1EMR1::EM18;
    pub use super::C1EMR1::EM19;
    pub use super::C1EMR1::EM2;
    pub use super::C1EMR1::EM20;
    pub use super::C1EMR1::EM21;
    pub use super::C1EMR1::EM22;
    pub use super::C1EMR1::EM3;
    pub use super::C1EMR1::EM4;
    pub use super::C1EMR1::EM5;
    pub use super::C1EMR1::EM6;
    pub use super::C1EMR1::EM7;
    pub use super::C1EMR1::EM8;
    pub use super::C1EMR1::EM9;
}

/// wakeup with interrupt mask register
pub mod C2IMR2 {
    pub use super::C1IMR2::IM34;
    pub use super::C1IMR2::IM36;
    pub use super::C1IMR2::IM37;
    pub use super::C1IMR2::IM38;
    pub use super::C1IMR2::IM39;
    pub use super::C1IMR2::IM40;
    pub use super::C1IMR2::IM41;
    pub use super::C1IMR2::IM42;
    pub use super::C1IMR2::IM43;
    pub use super::C1IMR2::IM44;
    pub use super::C1IMR2::IM45;
    pub use super::C1IMR2::IM46;
}

/// wakeup with event mask register
pub mod C2EMR2 {
    pub use super::C1EMR2::EM40;
    pub use super::C1EMR2::EM41;
}
#[repr(C)]
pub struct RegisterBlock {
    /// rising trigger selection register
    pub RTSR1: RWRegister<u32>,

    /// falling trigger selection register
    pub FTSR1: RWRegister<u32>,

    /// software interrupt event register
    pub SWIER1: RWRegister<u32>,

    /// EXTI pending register
    pub PR1: RWRegister<u32>,

    _reserved1: [u8; 16],

    /// rising trigger selection register
    pub RTSR2: RWRegister<u32>,

    /// falling trigger selection register
    pub FTSR2: RWRegister<u32>,

    /// software interrupt event register
    pub SWIER2: RWRegister<u32>,

    /// pending register
    pub PR2: RWRegister<u32>,

    _reserved2: [u8; 80],

    /// interrupt mask register
    pub C1IMR1: RWRegister<u32>,

    /// event mask register
    pub C1EMR1: RWRegister<u32>,

    _reserved3: [u8; 8],

    /// wakeup with interrupt mask register
    pub C1IMR2: RWRegister<u32>,

    /// wakeup with event mask register
    pub C1EMR2: RWRegister<u32>,

    _reserved4: [u8; 40],

    /// interrupt mask register
    pub C2IMR1: RWRegister<u32>,

    /// event mask register
    pub C2EMR1: RWRegister<u32>,

    _reserved5: [u8; 8],

    /// wakeup with interrupt mask register
    pub C2IMR2: RWRegister<u32>,

    /// wakeup with event mask register
    pub C2EMR2: RWRegister<u32>,
}
pub struct ResetValues {
    pub RTSR1: u32,
    pub FTSR1: u32,
    pub SWIER1: u32,
    pub PR1: u32,
    pub RTSR2: u32,
    pub FTSR2: u32,
    pub SWIER2: u32,
    pub PR2: u32,
    pub C1IMR1: u32,
    pub C1EMR1: u32,
    pub C1IMR2: u32,
    pub C1EMR2: u32,
    pub C2IMR1: u32,
    pub C2EMR1: u32,
    pub C2IMR2: u32,
    pub C2EMR2: u32,
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
