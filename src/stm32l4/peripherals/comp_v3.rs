#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Comparator
//!
//! Used by: stm32l4x3, stm32l4x5, stm32l4x6

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Comparator 1 control and status register
pub mod COMP1_CSR {

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

            /// 0b11: Low speed / ultra-low power
            pub const LowSpeed: u32 = 0b11;
        }
    }

    /// Comparator 1 Input Minus connection configuration bit
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

            /// 0b101: DAC Channel 2
            pub const DAC_CH2: u32 = 0b101;

            /// 0b110: PC4
            pub const PC4: u32 = 0b110;
        }
    }

    /// Comparator1 input plus selection bit
    pub mod INPSEL {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PC5 connected to input plus
            pub const PC5: u32 = 0b0;

            /// 0b1: PB2 connected to input plus
            pub const PB2: u32 = 0b1;
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

            /// 0b100: TIM15 OC1 selected as blanking source
            pub const TIM1OC5: u32 = 0b100;
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
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Comparator 2 control and status register
pub mod COMP2_CSR {

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

            /// 0b11: Low speed / ultra-low power
            pub const LowSpeed: u32 = 0b11;
        }
    }

    /// Comparator 2 Input Minus connection configuration bit
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

            /// 0b101: DAC Channel 2
            pub const DAC_CH2: u32 = 0b101;

            /// 0b110: PB3
            pub const PB3: u32 = 0b110;

            /// 0b111: PB7
            pub const PB7: u32 = 0b111;
        }
    }

    /// Comparator 2 Input Plus connection configuration bit
    pub mod INPSEL {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PB4 connected to input plus
            pub const PB4: u32 = 0b0;

            /// 0b1: PB6 connected to input plus
            pub const PB6: u32 = 0b1;
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

            /// 0b1: COMP2 input plus is connected to COMP1 plus
            pub const Enabled: u32 = 0b1;
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

            /// 0b100: TIM15 OC1 selected as blanking source
            pub const TIM1OC5: u32 = 0b100;
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

    /// COMP2_CSR register lock bit
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
#[repr(C)]
pub struct RegisterBlock {
    /// Comparator 1 control and status register
    pub COMP1_CSR: RWRegister<u32>,

    /// Comparator 2 control and status register
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
