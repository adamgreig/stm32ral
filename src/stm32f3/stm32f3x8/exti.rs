#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! External interrupt/event controller

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Interrupt mask register (EXTI_IMR)
pub mod IMR {

    /// Interrupt Mask on line 0
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

    /// Interrupt Mask on line 1
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

    /// Interrupt Mask on line 2
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

    /// Interrupt Mask on line 3
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

    /// Interrupt Mask on line 4
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

    /// Interrupt Mask on line 5
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

    /// Interrupt Mask on line 6
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

    /// Interrupt Mask on line 7
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

    /// Interrupt Mask on line 8
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

    /// Interrupt Mask on line 9
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

    /// Interrupt Mask on line 10
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

    /// Interrupt Mask on line 11
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

    /// Interrupt Mask on line 12
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

    /// Interrupt Mask on line 13
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

    /// Interrupt Mask on line 14
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

    /// Interrupt Mask on line 15
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

    /// Interrupt Mask on line 16
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

    /// Interrupt Mask on line 17
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

    /// Interrupt Mask on line 18
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

    /// Interrupt Mask on line 19
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

    /// Interrupt Mask on line 20
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

    /// Interrupt Mask on line 21
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

    /// Interrupt Mask on line 22
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

    /// Interrupt Mask on line 23
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

    /// Interrupt Mask on line 24
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

    /// Interrupt Mask on line 25
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

    /// Interrupt Mask on line 26
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

    /// Interrupt Mask on line 27
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

    /// Interrupt Mask on line 28
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
}

/// Event mask register (EXTI_EMR)
pub mod EMR {
    pub use super::IMR::MR0;
    pub use super::IMR::MR1;
    pub use super::IMR::MR10;
    pub use super::IMR::MR11;
    pub use super::IMR::MR12;
    pub use super::IMR::MR13;
    pub use super::IMR::MR14;
    pub use super::IMR::MR15;
    pub use super::IMR::MR16;
    pub use super::IMR::MR17;
    pub use super::IMR::MR18;
    pub use super::IMR::MR19;
    pub use super::IMR::MR2;
    pub use super::IMR::MR20;
    pub use super::IMR::MR21;
    pub use super::IMR::MR22;
    pub use super::IMR::MR23;
    pub use super::IMR::MR24;
    pub use super::IMR::MR25;
    pub use super::IMR::MR26;
    pub use super::IMR::MR27;
    pub use super::IMR::MR28;
    pub use super::IMR::MR3;
    pub use super::IMR::MR4;
    pub use super::IMR::MR5;
    pub use super::IMR::MR6;
    pub use super::IMR::MR7;
    pub use super::IMR::MR8;
    pub use super::IMR::MR9;
}

/// Rising Trigger selection register (EXTI_RTSR)
pub mod RTSR {

    /// Rising trigger event configuration of line 0
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

    /// Rising trigger event configuration of line 1
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

    /// Rising trigger event configuration of line 2
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

    /// Rising trigger event configuration of line 3
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

    /// Rising trigger event configuration of line 4
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

    /// Rising trigger event configuration of line 5
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

    /// Rising trigger event configuration of line 6
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

    /// Rising trigger event configuration of line 7
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

    /// Rising trigger event configuration of line 8
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

    /// Rising trigger event configuration of line 9
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

    /// Rising trigger event configuration of line 10
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

    /// Rising trigger event configuration of line 11
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

    /// Rising trigger event configuration of line 12
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

    /// Rising trigger event configuration of line 13
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

    /// Rising trigger event configuration of line 14
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

    /// Rising trigger event configuration of line 15
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

    /// Rising trigger event configuration of line 16
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

    /// Rising trigger event configuration of line 17
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

    /// Rising trigger event configuration of line 19
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
}

/// Falling Trigger selection register (EXTI_FTSR)
pub mod FTSR {

    /// Falling trigger event configuration of line 0
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

    /// Falling trigger event configuration of line 1
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

    /// Falling trigger event configuration of line 2
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

    /// Falling trigger event configuration of line 3
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

    /// Falling trigger event configuration of line 4
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

    /// Falling trigger event configuration of line 5
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

    /// Falling trigger event configuration of line 6
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

    /// Falling trigger event configuration of line 7
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

    /// Falling trigger event configuration of line 8
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

    /// Falling trigger event configuration of line 9
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

    /// Falling trigger event configuration of line 10
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

    /// Falling trigger event configuration of line 11
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

    /// Falling trigger event configuration of line 12
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

    /// Falling trigger event configuration of line 13
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

    /// Falling trigger event configuration of line 14
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

    /// Falling trigger event configuration of line 15
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

    /// Falling trigger event configuration of line 16
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

    /// Falling trigger event configuration of line 17
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

    /// Falling trigger event configuration of line 19
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
}

/// Software interrupt event register (EXTI_SWIER)
pub mod SWIER {

    /// Software Interrupt on line 0
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

    /// Software Interrupt on line 1
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

    /// Software Interrupt on line 2
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

    /// Software Interrupt on line 3
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

    /// Software Interrupt on line 4
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

    /// Software Interrupt on line 5
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

    /// Software Interrupt on line 6
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

    /// Software Interrupt on line 7
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

    /// Software Interrupt on line 8
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

    /// Software Interrupt on line 9
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

    /// Software Interrupt on line 10
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

    /// Software Interrupt on line 11
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

    /// Software Interrupt on line 12
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

    /// Software Interrupt on line 13
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

    /// Software Interrupt on line 14
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

    /// Software Interrupt on line 15
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

    /// Software Interrupt on line 16
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

    /// Software Interrupt on line 17
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

    /// Software Interrupt on line 19
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
}

/// Pending register (EXTI_PR)
pub mod PR {

    /// Pending bit 0
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

    /// Pending bit 1
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

    /// Pending bit 2
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

    /// Pending bit 3
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

    /// Pending bit 4
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

    /// Pending bit 5
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

    /// Pending bit 6
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

    /// Pending bit 7
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

    /// Pending bit 8
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

    /// Pending bit 9
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

    /// Pending bit 10
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

    /// Pending bit 11
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

    /// Pending bit 12
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

    /// Pending bit 13
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

    /// Pending bit 14
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

    /// Pending bit 15
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

    /// Pending bit 16
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

    /// Pending bit 17
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

    /// Pending bit 19
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt mask register (EXTI_IMR)
    pub IMR: RWRegister<u32>,

    /// Event mask register (EXTI_EMR)
    pub EMR: RWRegister<u32>,

    /// Rising Trigger selection register (EXTI_RTSR)
    pub RTSR: RWRegister<u32>,

    /// Falling Trigger selection register (EXTI_FTSR)
    pub FTSR: RWRegister<u32>,

    /// Software interrupt event register (EXTI_SWIER)
    pub SWIER: RWRegister<u32>,

    /// Pending register (EXTI_PR)
    pub PR: RWRegister<u32>,
}
pub struct ResetValues {
    pub IMR: u32,
    pub EMR: u32,
    pub RTSR: u32,
    pub FTSR: u32,
    pub SWIER: u32,
    pub PR: u32,
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

/// Access functions for the EXTI peripheral instance
pub mod EXTI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40010400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in EXTI
    pub const reset: ResetValues = ResetValues {
        IMR: 0x1F800000,
        EMR: 0x00000000,
        RTSR: 0x00000000,
        FTSR: 0x00000000,
        SWIER: 0x00000000,
        PR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut EXTI_TAKEN: bool = false;

    /// Safe access to EXTI
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
            if EXTI_TAKEN {
                None
            } else {
                EXTI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to EXTI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if EXTI_TAKEN && inst.addr == INSTANCE.addr {
                EXTI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal EXTI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        EXTI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to EXTI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const EXTI: *const RegisterBlock = 0x40010400 as *const _;
