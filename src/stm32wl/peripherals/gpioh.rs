#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! General-purpose I/Os
//!
//! Used by: stm32wl5x_cm0p, stm32wl5x_cm4, stm32wle5

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// GPIO port mode register
pub mod MODER {

    /// Port x configuration bits (y = 0..15)
    pub mod MODER3 {
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

            /// 0b00: Input mode (reset state)
            pub const Input: u32 = 0b00;

            /// 0b01: General purpose output mode
            pub const Output: u32 = 0b01;

            /// 0b10: Alternate function mode
            pub const Alternate: u32 = 0b10;

            /// 0b11: Analog mode
            pub const Analog: u32 = 0b11;
        }
    }
}

/// GPIO port output type register
pub mod OTYPER {

    /// Port x configuration bits (y = 0..15)
    pub mod OT3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Output push-pull (reset state)
            pub const PushPull: u32 = 0b0;

            /// 0b1: Output open-drain
            pub const OpenDrain: u32 = 0b1;
        }
    }
}

/// GPIO port output speed register
pub mod OSPEEDR {

    /// Port x configuration bits (y = 0..15)
    pub mod OSPEEDR3 {
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

            /// 0b00: Low speed
            pub const LowSpeed: u32 = 0b00;

            /// 0b01: Medium speed
            pub const MediumSpeed: u32 = 0b01;

            /// 0b10: High speed
            pub const HighSpeed: u32 = 0b10;

            /// 0b11: Very high speed
            pub const VeryHighSpeed: u32 = 0b11;
        }
    }
}

/// GPIO port pull-up/pull-down register
pub mod PUPDR {

    /// Port x configuration bits (y = 0..15)
    pub mod PUPDR3 {
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

            /// 0b00: No pull-up, pull-down
            pub const Floating: u32 = 0b00;

            /// 0b01: Pull-up
            pub const PullUp: u32 = 0b01;

            /// 0b10: Pull-down
            pub const PullDown: u32 = 0b10;
        }
    }
}

/// GPIO port input data register
pub mod IDR {

    /// Port input data (y = 0..15)
    pub mod IDR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: Input is logic high
            pub const High: u32 = 0b1;

            /// 0b0: Input is logic low
            pub const Low: u32 = 0b0;
        }
    }
}

/// GPIO port output data register
pub mod ODR {

    /// Port output data (y = 0..15)
    pub mod ODR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: Set output to logic high
            pub const High: u32 = 0b1;

            /// 0b0: Set output to logic low
            pub const Low: u32 = 0b0;
        }
    }
}

/// GPIO port bit set/reset register
pub mod BSRR {

    /// Port x reset bit y (y = 0..15)
    pub mod BR3 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Resets the corresponding ODRx bit
            pub const Reset: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15)
    pub mod BS3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Sets the corresponding ODRx bit
            pub const Set: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPIO port configuration lock register
pub mod LCKR {

    /// Port x lock bit y (y= 0..15)
    pub mod LCKK {
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

    /// Port x lock bit y (y= 0..15)
    pub mod LCK3 {
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
}

/// GPIO alternate function low register
pub mod AFRL {

    /// Alternate function selection for port x bit y (y = 0..7)
    pub mod AFRL3 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: AF0
            pub const AF0: u32 = 0b0000;

            /// 0b0001: AF1
            pub const AF1: u32 = 0b0001;

            /// 0b0010: AF2
            pub const AF2: u32 = 0b0010;

            /// 0b0011: AF3
            pub const AF3: u32 = 0b0011;

            /// 0b0100: AF4
            pub const AF4: u32 = 0b0100;

            /// 0b0101: AF5
            pub const AF5: u32 = 0b0101;

            /// 0b0110: AF6
            pub const AF6: u32 = 0b0110;

            /// 0b0111: AF7
            pub const AF7: u32 = 0b0111;

            /// 0b1000: AF8
            pub const AF8: u32 = 0b1000;

            /// 0b1001: AF9
            pub const AF9: u32 = 0b1001;

            /// 0b1010: AF10
            pub const AF10: u32 = 0b1010;

            /// 0b1011: AF11
            pub const AF11: u32 = 0b1011;

            /// 0b1100: AF12
            pub const AF12: u32 = 0b1100;

            /// 0b1101: AF13
            pub const AF13: u32 = 0b1101;

            /// 0b1110: AF14
            pub const AF14: u32 = 0b1110;

            /// 0b1111: AF15
            pub const AF15: u32 = 0b1111;
        }
    }
}

/// GPIO alternate function high register
pub mod AFRH {

    /// Alternate function selection for port x bit y (y = 8..15)
    pub mod AFRH15 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: AF0
            pub const AF0: u32 = 0b0000;

            /// 0b0001: AF1
            pub const AF1: u32 = 0b0001;

            /// 0b0010: AF2
            pub const AF2: u32 = 0b0010;

            /// 0b0011: AF3
            pub const AF3: u32 = 0b0011;

            /// 0b0100: AF4
            pub const AF4: u32 = 0b0100;

            /// 0b0101: AF5
            pub const AF5: u32 = 0b0101;

            /// 0b0110: AF6
            pub const AF6: u32 = 0b0110;

            /// 0b0111: AF7
            pub const AF7: u32 = 0b0111;

            /// 0b1000: AF8
            pub const AF8: u32 = 0b1000;

            /// 0b1001: AF9
            pub const AF9: u32 = 0b1001;

            /// 0b1010: AF10
            pub const AF10: u32 = 0b1010;

            /// 0b1011: AF11
            pub const AF11: u32 = 0b1011;

            /// 0b1100: AF12
            pub const AF12: u32 = 0b1100;

            /// 0b1101: AF13
            pub const AF13: u32 = 0b1101;

            /// 0b1110: AF14
            pub const AF14: u32 = 0b1110;

            /// 0b1111: AF15
            pub const AF15: u32 = 0b1111;
        }
    }

    /// Alternate function selection for port x bit y (y = 8..15)
    pub mod AFRH14 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFRH15::RW;
    }

    /// Alternate function selection for port x bit y (y = 8..15)
    pub mod AFRH13 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFRH15::RW;
    }

    /// Alternate function selection for port x bit y (y = 8..15)
    pub mod AFRH12 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFRH15::RW;
    }

    /// Alternate function selection for port x bit y (y = 8..15)
    pub mod AFRH11 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFRH15::RW;
    }

    /// Alternate function selection for port x bit y (y = 8..15)
    pub mod AFRH10 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFRH15::RW;
    }

    /// Alternate function selection for port x bit y (y = 8..15)
    pub mod AFRH9 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFRH15::RW;
    }

    /// Alternate function selection for port x bit y (y = 8..15)
    pub mod AFRH8 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFRH15::RW;
    }
}

/// GPIO port bit reset register
pub mod BRR {

    /// Port Reset bit
    pub mod BR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: No action on the corresponding ODx bit
            pub const NoAction: u32 = 0b0;

            /// 0b1: Reset the ODx bit
            pub const Reset: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// GPIO port mode register
    pub MODER: RWRegister<u32>,

    /// GPIO port output type register
    pub OTYPER: RWRegister<u32>,

    /// GPIO port output speed register
    pub OSPEEDR: RWRegister<u32>,

    /// GPIO port pull-up/pull-down register
    pub PUPDR: RWRegister<u32>,

    /// GPIO port input data register
    pub IDR: RORegister<u32>,

    /// GPIO port output data register
    pub ODR: RWRegister<u32>,

    /// GPIO port bit set/reset register
    pub BSRR: WORegister<u32>,

    /// GPIO port configuration lock register
    pub LCKR: RWRegister<u32>,

    /// GPIO alternate function low register
    pub AFRL: RWRegister<u32>,

    /// GPIO alternate function high register
    pub AFRH: RWRegister<u32>,

    /// GPIO port bit reset register
    pub BRR: RWRegister<u32>,
}
pub struct ResetValues {
    pub MODER: u32,
    pub OTYPER: u32,
    pub OSPEEDR: u32,
    pub PUPDR: u32,
    pub IDR: u32,
    pub ODR: u32,
    pub BSRR: u32,
    pub LCKR: u32,
    pub AFRL: u32,
    pub AFRH: u32,
    pub BRR: u32,
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
