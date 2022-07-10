#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Comparator
//!
//! Used by: stm32wl5x_cm0p, stm32wl5x_cm4, stm32wle5

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// COMP1_CSR
pub mod COMP1_CSR {

    /// COMP1_CSR register lock bit
    pub mod LOCK {
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

            /// 0b0: Comparator CSR bits are read-write
            pub const Unlocked: u32 = 0b0;

            /// 0b1: Comparator CSR bits are read-only
            pub const Locked: u32 = 0b1;
        }
    }

    /// Comparator 1 output status bit
    pub mod VALUE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Comparator output is low
            pub const Low: u32 = 0b0;

            /// 0b1: Comparator output is high
            pub const High: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// comparator 1 input minus extended selection bits.
    pub mod INMESEL {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (2 bits: 0b11 << 25)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: PA10 connected to input minus
            pub const PA10: u32 = 0b00;

            /// 0b01: PA11 connected to input minus
            pub const PA11: u32 = 0b01;

            /// 0b10: PA15 connected to input minus
            pub const PA15: u32 = 0b10;
        }
    }

    /// Voltage scaler enable bit
    pub mod SCALEN {
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

            /// 0b0: Voltage scaler disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Voltage scaler enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Scaler bridge enable
    pub mod BRGEN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Scaler resistor bridge disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Scaler resistor bridge enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Comparator 1 blanking source selection bits
    pub mod BLANKING {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No blanking
            pub const NoBlanking: u32 = 0b000;

            /// 0b001: TIM1 OC5 selected as blanking source
            pub const TIM1OC5: u32 = 0b001;

            /// 0b010: TIM2 OC3 selected as blanking source
            pub const TIM2OC3: u32 = 0b010;
        }
    }

    /// Comparator 1 hysteresis selection bits
    pub mod HYST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
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

    /// Comparator 1 polarity selection bit
    pub mod POLARITY {
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

            /// 0b0: Output is not inverted
            pub const NotInverted: u32 = 0b0;

            /// 0b1: Output is inverted
            pub const Inverted: u32 = 0b1;
        }
    }

    /// Comparator1 input plus selection bit
    pub mod INPSEL {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (2 bits: 0b11 << 7)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: PB4 connected to input plus
            pub const PB4: u32 = 0b00;

            /// 0b01: PB2 connected to input plus
            pub const PB2: u32 = 0b01;
        }
    }

    /// Comparator 1 input minus selection bits
    pub mod INMSEL {
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

            /// 0b100: DAC Channel 1
            pub const DAC_CH1: u32 = 0b100;

            /// 0b110: PB3
            pub const PB3: u32 = 0b110;

            /// 0b111: GPIO pin selected by INMESEL
            pub const GPIO: u32 = 0b111;
        }
    }

    /// Power Mode of the comparator 1
    pub mod PWRMODE {
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

    /// Comparator 1 enable bit
    pub mod EN {
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
}

/// COMP2_CSR
pub mod COMP2_CSR {

    /// CSR register lock bit
    pub mod LOCK {
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

            /// 0b0: Comparator CSR bits are read-write
            pub const Unlocked: u32 = 0b0;

            /// 0b1: Comparator CSR bits are read-only
            pub const Locked: u32 = 0b1;
        }
    }

    /// Comparator 2 output status bit
    pub mod VALUE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Comparator output is low
            pub const Low: u32 = 0b0;

            /// 0b1: Comparator output is high
            pub const High: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// comparator 2 input minus extended selection bits.
    pub mod INMESEL {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (2 bits: 0b11 << 25)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: PB2 connected to input minus
            pub const PB2: u32 = 0b00;

            /// 0b01: PA10 connected to input minus
            pub const PA10: u32 = 0b01;

            /// 0b10: PA11 connected to input minus
            pub const PA11: u32 = 0b10;
        }
    }

    /// Voltage scaler enable bit
    pub mod SCALEN {
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

            /// 0b0: Voltage scaler disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Voltage scaler enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Scaler bridge enable
    pub mod BRGEN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Scaler resistor bridge disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Scaler resistor bridge enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Comparator 2 blanking source selection bits
    pub mod BLANKING {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No blanking
            pub const NoBlanking: u32 = 0b000;

            /// 0b001: TIM1 OC5 selected as blanking source
            pub const TIM1OC5: u32 = 0b001;

            /// 0b010: TIM2 OC3 selected as blanking source
            pub const TIM2OC3: u32 = 0b010;
        }
    }

    /// Comparator 2 hysteresis selection bits
    pub mod HYST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
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

    /// Comparator 2 polarity selection bit
    pub mod POLARITY {
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

            /// 0b0: Output is not inverted
            pub const NotInverted: u32 = 0b0;

            /// 0b1: Output is inverted
            pub const Inverted: u32 = 0b1;
        }
    }

    /// Windows mode selection bit
    pub mod WINMODE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: COMP2 input plus is not connected to COMP1
            pub const Disabled: u32 = 0b0;

            /// 0b1: COMP2 input plus is connected to COMP1
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Comparator 1 input plus selection bit
    pub mod INPSEL {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (2 bits: 0b11 << 7)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: PB4 connected to input plus
            pub const PB4: u32 = 0b00;

            /// 0b01: PB1 connected to input plus
            pub const PB1: u32 = 0b01;

            /// 0b10: PA15 connected to input plus
            pub const PA15: u32 = 0b10;
        }
    }

    /// Comparator 2 input minus selection bits
    pub mod INMSEL {
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

            /// 0b100: DAC Channel 1
            pub const DAC_CH1: u32 = 0b100;

            /// 0b110: PB3
            pub const PB3: u32 = 0b110;

            /// 0b111: GPIO pin selected by INMESEL
            pub const GPIO: u32 = 0b111;
        }
    }

    /// Power Mode of the comparator 2
    pub mod PWRMODE {
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

    /// Comparator 2 enable bit
    pub mod EN {
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// COMP1_CSR
    pub COMP1_CSR: RWRegister<u32>,

    /// COMP2_CSR
    pub COMP2_CSR: RWRegister<u32>,
}
pub struct ResetValues {
    pub COMP1_CSR: u32,
    pub COMP2_CSR: u32,
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
