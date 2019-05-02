#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! General purpose comparators
//!
//! Used by: stm32f0x1, stm32f0x2, stm32f0x8

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control and status register
pub mod CSR {

    /// Comparator 1 enable
    pub mod COMP1EN {
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

            /// 0b0: Comparator 1 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Comparator 1 enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Comparator 1 mode
    pub mod COMP1MODE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: High speed / full power
            pub const HighSpeed: u32 = 0b00;

            /// 0b01: Medium speed / medium power
            pub const MediumSpeed: u32 = 0b01;

            /// 0b10: Low speed / low power
            pub const LowSpeed: u32 = 0b10;

            /// 0b11: Very-low speed / ultra-low power
            pub const VeryLowSpeed: u32 = 0b11;
        }
    }

    /// Comparator 1 inverting input selection
    pub mod COMP1INSEL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 1/4 of VRefint
            pub const OneQuarterVRef: u32 = 0b000;

            /// 0b001: 1/2 of VRefint
            pub const OneHalfVRef: u32 = 0b001;

            /// 0b010: 3/4 of VRefint
            pub const ThreeQuarterVRef: u32 = 0b010;

            /// 0b011: VRefint
            pub const VRef: u32 = 0b011;

            /// 0b100: COMP1_INM4 (PA4 with DAC_OUT1 if enabled)
            pub const Comp1_INM4: u32 = 0b100;

            /// 0b101: COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)
            pub const Comp1_INM5: u32 = 0b101;

            /// 0b110: COMP1_INM6 (PA0)
            pub const Comp1_INM6: u32 = 0b110;
        }
    }

    /// Comparator 1 output selection
    pub mod COMP1OUTSEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No selection
            pub const NoSelection: u32 = 0b000;

            /// 0b001: Timer 1 break input
            pub const Timer1BreakInput: u32 = 0b001;

            /// 0b010: Timer 1 Input capture 1
            pub const Timer1InputCapture1: u32 = 0b010;

            /// 0b011: Timer 1 OCrefclear input
            pub const Timer1OCRefClearInput: u32 = 0b011;

            /// 0b100: Timer 2 input capture 4
            pub const Timer2InputCapture4: u32 = 0b100;

            /// 0b101: Timer 2 OCrefclear input
            pub const Timer2OCRefClearInput: u32 = 0b101;

            /// 0b110: Timer 3 input capture 1
            pub const Timer3InputCapture1: u32 = 0b110;

            /// 0b111: Timer 3 OCrefclear input
            pub const Timer3OCRefClearInput: u32 = 0b111;
        }
    }

    /// Comparator 1 output polarity
    pub mod COMP1POL {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Output is not inverted
            pub const NotInverted: u32 = 0b0;

            /// 0b1: Output is inverted
            pub const Inverted: u32 = 0b1;
        }
    }

    /// Comparator 1 hysteresis
    pub mod COMP1HYST {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No hysteresis
            pub const NoHysteresis: u32 = 0b00;

            /// 0b01: Low hysteresis
            pub const LowHysteresis: u32 = 0b01;

            /// 0b10: Medium hysteresis
            pub const MediumHysteresis: u32 = 0b10;

            /// 0b11: High hysteresis
            pub const HighHysteresis: u32 = 0b11;
        }
    }

    /// Comparator 1 output
    pub mod COMP1OUT {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Non-inverting input below inverting input
            pub const Low: u32 = 0b0;

            /// 0b1: Non-inverting input above inverting input
            pub const High: u32 = 0b1;
        }
    }

    /// Comparator 1 lock
    pub mod COMP1LOCK {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Comparator 1 CSR bits (CSR\[15:0\]) are read-write
            pub const Unlocked: u32 = 0b0;

            /// 0b1: Comparator 1 CSR bits (CSR\[15:0\]) are read-only
            pub const Locked: u32 = 0b1;
        }
    }

    /// Comparator 2 enable
    pub mod COMP2EN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Comparator 2 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Comparator 2 enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Comparator 2 mode
    pub mod COMP2MODE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::COMP1MODE::RW;
    }

    /// Comparator 2 inverting input selection
    pub mod COMP2INSEL {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 1/4 of VRefint
            pub const OneQuarterVRef: u32 = 0b000;

            /// 0b001: 1/2 of VRefint
            pub const OneHalfVRef: u32 = 0b001;

            /// 0b010: 3/4 of VRefint
            pub const ThreeQuarterVRef: u32 = 0b010;

            /// 0b011: VRefint
            pub const VRef: u32 = 0b011;

            /// 0b100: COMP1_INM4 (PA4 with DAC_OUT1 if enabled)
            pub const Comp2_INM4: u32 = 0b100;

            /// 0b101: COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)
            pub const Comp2_INM5: u32 = 0b101;

            /// 0b110: COMP1_INM6 (PA2)
            pub const Comp2_INM6: u32 = 0b110;
        }
    }

    /// Window mode enable
    pub mod WNDWEN {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Window mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Window mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Comparator 2 output selection
    pub mod COMP2OUTSEL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::COMP1OUTSEL::RW;
    }

    /// Comparator 2 output polarity
    pub mod COMP2POL {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::COMP1POL::RW;
    }

    /// Comparator 2 hysteresis
    pub mod COMP2HYST {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::COMP1HYST::RW;
    }

    /// Comparator 2 output
    pub mod COMP2OUT {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::COMP1OUT::RW;
    }

    /// Comparator 2 lock
    pub mod COMP2LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Comparator 2 CSR bits (CSR\[31:16\]) are read-write
            pub const Unlocked: u32 = 0b0;

            /// 0b1: Comparator 2 CSR bits (CSR\[31:16\]) are read-only
            pub const Locked: u32 = 0b1;
        }
    }

    /// Comparator 1 non inverting input DAC switch
    pub mod COMP1SW1 {
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

            /// 0b0: Switch open
            pub const Open: u32 = 0b0;

            /// 0b1: Switch closed
            pub const Closed: u32 = 0b1;
        }
    }
}
pub struct RegisterBlock {
    _reserved1: [u32; 7],

    /// control and status register
    pub CSR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CSR: u32,
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
