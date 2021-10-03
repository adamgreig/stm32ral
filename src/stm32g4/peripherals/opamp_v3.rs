#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Operational amplifiers
//!
//! Used by: stm32g491, stm32g4a1

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// OPAMP1 control/status register
pub mod OPAMP1_CSR {

    /// Operational amplifier Enable
    pub mod OPAEN {
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

            /// 0b0: OpAmp disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: OpAmp enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// FORCE_VP
    pub mod FORCE_VP {
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

            /// 0b0: Non-inverting input connected configured inputs
            pub const Normal: u32 = 0b0;

            /// 0b1: Non-inverting input connected to calibration reference voltage
            pub const CalibrationVerification: u32 = 0b1;
        }
    }

    /// VP_SEL
    pub mod VP_SEL {
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

            /// 0b00: VINP0 connected to VINP input
            pub const VINP0: u32 = 0b00;

            /// 0b01: VINP1 connected to VINP input
            pub const VINP1: u32 = 0b01;

            /// 0b10: VINP2 connected to VINP input
            pub const VINP2: u32 = 0b10;

            /// 0b11: DAC3_CH1 connected to VINP input
            pub const DAC3_CH1: u32 = 0b11;
        }
    }

    /// USERTRIM
    pub mod USERTRIM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Factory trim used
            pub const Factory: u32 = 0b0;

            /// 0b1: User trim used
            pub const User: u32 = 0b1;
        }
    }

    /// VM_SEL
    pub mod VM_SEL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: VINM0 connected to VINM input
            pub const VINM0: u32 = 0b00;

            /// 0b01: VINM1 connected to VINM input
            pub const VINM1: u32 = 0b01;

            /// 0b10: Feedback resistor connected to VINM (PGA mode)
            pub const PGA: u32 = 0b10;

            /// 0b11: OpAmp output connected to VINM (Follower mode)
            pub const Output: u32 = 0b11;
        }
    }

    /// OPAHSM
    pub mod OPAHSM {
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

            /// 0b0: OpAmp in normal mode
            pub const Normal: u32 = 0b0;

            /// 0b1: OpAmp in high speed mode
            pub const HighSpeed: u32 = 0b1;
        }
    }

    /// OPAINTOEN
    pub mod OPAINTOEN {
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

            /// 0b0: Output is connected to the output Pin
            pub const OutputPin: u32 = 0b0;

            /// 0b1: Output is connected internally to ADC channel
            pub const ADCChannel: u32 = 0b1;
        }
    }

    /// CALON
    pub mod CALON {
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

            /// 0b0: Calibration mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Calibration mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CALSEL
    pub mod CALSEL {
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

            /// 0b00: 0.033*VDDA applied to OPAMP inputs during calibration
            pub const Percent3_3: u32 = 0b00;

            /// 0b01: 0.1*VDDA applied to OPAMP inputs during calibration
            pub const Percent10: u32 = 0b01;

            /// 0b10: 0.5*VDDA applied to OPAMP inputs during calibration
            pub const Percent50: u32 = 0b10;

            /// 0b11: 0.9*VDDA applied to OPAMP inputs during calibration
            pub const Percent90: u32 = 0b11;
        }
    }

    /// PGA_GAIN
    pub mod PGA_GAIN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (5 bits: 0b11111 << 14)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Gain 2
            pub const Gain2: u32 = 0b00000;

            /// 0b00001: Gain 4
            pub const Gain4: u32 = 0b00001;

            /// 0b00010: Gain 8
            pub const Gain8: u32 = 0b00010;

            /// 0b00011: Gain 16
            pub const Gain16: u32 = 0b00011;

            /// 0b00100: Gain 32
            pub const Gain32: u32 = 0b00100;

            /// 0b00101: Gain 64
            pub const Gain64: u32 = 0b00101;

            /// 0b01000: Gain 2, input/bias connected to VINM0 or inverting gain
            pub const Gain2_InputVINM0: u32 = 0b01000;

            /// 0b01001: Gain 4, input/bias connected to VINM0 or inverting gain
            pub const Gain4_InputVINM0: u32 = 0b01001;

            /// 0b01010: Gain 8, input/bias connected to VINM0 or inverting gain
            pub const Gain8_InputVINM0: u32 = 0b01010;

            /// 0b01011: Gain 16, input/bias connected to VINM0 or inverting gain
            pub const Gain16_InputVINM0: u32 = 0b01011;

            /// 0b01100: Gain 32, input/bias connected to VINM0 or inverting gain
            pub const Gain32_InputVINM0: u32 = 0b01100;

            /// 0b01101: Gain 64, input/bias connected to VINM0 or inverting gain
            pub const Gain64_InputVINM0: u32 = 0b01101;

            /// 0b10000: Gain 2, with filtering on VINM0
            pub const Gain2_FilteringVINM0: u32 = 0b10000;

            /// 0b10001: Gain 4, with filtering on VINM0
            pub const Gain4_FilteringVINM0: u32 = 0b10001;

            /// 0b10010: Gain 8, with filtering on VINM0
            pub const Gain8_FilteringVINM0: u32 = 0b10010;

            /// 0b10011: Gain 16, with filtering on VINM0
            pub const Gain16_FilteringVINM0: u32 = 0b10011;

            /// 0b10100: Gain 32, with filtering on VINM0
            pub const Gain32_FilteringVINM0: u32 = 0b10100;

            /// 0b10101: Gain 64, with filtering on VINM0
            pub const Gain64_FilteringVINM0: u32 = 0b10101;

            /// 0b11000: Gain 2, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain2_InputVINM0FilteringVINM1: u32 = 0b11000;

            /// 0b11001: Gain 4, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain4_InputVINM0FilteringVINM1: u32 = 0b11001;

            /// 0b11010: Gain 8, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain8_InputVINM0FilteringVINM1: u32 = 0b11010;

            /// 0b11011: Gain 16, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain16_InputVINM0FilteringVINM1: u32 = 0b11011;

            /// 0b11100: Gain 32, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain32_InputVINM0FilteringVINM1: u32 = 0b11100;

            /// 0b11101: Gain 64, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain64_InputVINM0FilteringVINM1: u32 = 0b11101;
        }
    }

    /// TRIMOFFSETP
    pub mod TRIMOFFSETP {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (5 bits: 0b11111 << 19)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRIMOFFSETN
    pub mod TRIMOFFSETN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CALOUT
    pub mod CALOUT {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LOCK
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

            /// 0b0: CSR is read-write
            pub const ReadWrite: u32 = 0b0;

            /// 0b1: CSR is read-only, can only be cleared by system reset
            pub const ReadOnly: u32 = 0b1;
        }
    }
}

/// OPAMP2 control/status register
pub mod OPAMP2_CSR {

    /// Operational amplifier Enable
    pub mod OPAEN {
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

            /// 0b0: OpAmp disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: OpAmp enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// FORCE_VP
    pub mod FORCE_VP {
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

            /// 0b0: Non-inverting input connected configured inputs
            pub const Normal: u32 = 0b0;

            /// 0b1: Non-inverting input connected to calibration reference voltage
            pub const CalibrationVerification: u32 = 0b1;
        }
    }

    /// VP_SEL
    pub mod VP_SEL {
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

            /// 0b00: VINP0 connected to VINP input
            pub const VINP0: u32 = 0b00;

            /// 0b01: VINP1 connected to VINP input
            pub const VINP1: u32 = 0b01;

            /// 0b10: VINP2 connected to VINP input
            pub const VINP2: u32 = 0b10;

            /// 0b11: VINP3 connected to VINP input
            pub const VINP3: u32 = 0b11;
        }
    }

    /// USERTRIM
    pub mod USERTRIM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Factory trim used
            pub const Factory: u32 = 0b0;

            /// 0b1: User trim used
            pub const User: u32 = 0b1;
        }
    }

    /// VM_SEL
    pub mod VM_SEL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: VINM0 connected to VINM input
            pub const VINM0: u32 = 0b00;

            /// 0b01: VINM1 connected to VINM input
            pub const VINM1: u32 = 0b01;

            /// 0b10: Feedback resistor connected to VINM (PGA mode)
            pub const PGA: u32 = 0b10;

            /// 0b11: OpAmp output connected to VINM (Follower mode)
            pub const Output: u32 = 0b11;
        }
    }

    /// OPAHSM
    pub mod OPAHSM {
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

            /// 0b0: OpAmp in normal mode
            pub const Normal: u32 = 0b0;

            /// 0b1: OpAmp in high speed mode
            pub const HighSpeed: u32 = 0b1;
        }
    }

    /// OPAINTOEN
    pub mod OPAINTOEN {
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

            /// 0b0: Output is connected to the output Pin
            pub const OutputPin: u32 = 0b0;

            /// 0b1: Output is connected internally to ADC channel
            pub const ADCChannel: u32 = 0b1;
        }
    }

    /// CALON
    pub mod CALON {
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

            /// 0b0: Calibration mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Calibration mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CALSEL
    pub mod CALSEL {
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

            /// 0b00: 0.033*VDDA applied to OPAMP inputs during calibration
            pub const Percent3_3: u32 = 0b00;

            /// 0b01: 0.1*VDDA applied to OPAMP inputs during calibration
            pub const Percent10: u32 = 0b01;

            /// 0b10: 0.5*VDDA applied to OPAMP inputs during calibration
            pub const Percent50: u32 = 0b10;

            /// 0b11: 0.9*VDDA applied to OPAMP inputs during calibration
            pub const Percent90: u32 = 0b11;
        }
    }

    /// PGA_GAIN
    pub mod PGA_GAIN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (5 bits: 0b11111 << 14)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Gain 2
            pub const Gain2: u32 = 0b00000;

            /// 0b00001: Gain 4
            pub const Gain4: u32 = 0b00001;

            /// 0b00010: Gain 8
            pub const Gain8: u32 = 0b00010;

            /// 0b00011: Gain 16
            pub const Gain16: u32 = 0b00011;

            /// 0b00100: Gain 32
            pub const Gain32: u32 = 0b00100;

            /// 0b00101: Gain 64
            pub const Gain64: u32 = 0b00101;

            /// 0b01000: Gain 2, input/bias connected to VINM0 or inverting gain
            pub const Gain2_InputVINM0: u32 = 0b01000;

            /// 0b01001: Gain 4, input/bias connected to VINM0 or inverting gain
            pub const Gain4_InputVINM0: u32 = 0b01001;

            /// 0b01010: Gain 8, input/bias connected to VINM0 or inverting gain
            pub const Gain8_InputVINM0: u32 = 0b01010;

            /// 0b01011: Gain 16, input/bias connected to VINM0 or inverting gain
            pub const Gain16_InputVINM0: u32 = 0b01011;

            /// 0b01100: Gain 32, input/bias connected to VINM0 or inverting gain
            pub const Gain32_InputVINM0: u32 = 0b01100;

            /// 0b01101: Gain 64, input/bias connected to VINM0 or inverting gain
            pub const Gain64_InputVINM0: u32 = 0b01101;

            /// 0b10000: Gain 2, with filtering on VINM0
            pub const Gain2_FilteringVINM0: u32 = 0b10000;

            /// 0b10001: Gain 4, with filtering on VINM0
            pub const Gain4_FilteringVINM0: u32 = 0b10001;

            /// 0b10010: Gain 8, with filtering on VINM0
            pub const Gain8_FilteringVINM0: u32 = 0b10010;

            /// 0b10011: Gain 16, with filtering on VINM0
            pub const Gain16_FilteringVINM0: u32 = 0b10011;

            /// 0b10100: Gain 32, with filtering on VINM0
            pub const Gain32_FilteringVINM0: u32 = 0b10100;

            /// 0b10101: Gain 64, with filtering on VINM0
            pub const Gain64_FilteringVINM0: u32 = 0b10101;

            /// 0b11000: Gain 2, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain2_InputVINM0FilteringVINM1: u32 = 0b11000;

            /// 0b11001: Gain 4, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain4_InputVINM0FilteringVINM1: u32 = 0b11001;

            /// 0b11010: Gain 8, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain8_InputVINM0FilteringVINM1: u32 = 0b11010;

            /// 0b11011: Gain 16, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain16_InputVINM0FilteringVINM1: u32 = 0b11011;

            /// 0b11100: Gain 32, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain32_InputVINM0FilteringVINM1: u32 = 0b11100;

            /// 0b11101: Gain 64, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain64_InputVINM0FilteringVINM1: u32 = 0b11101;
        }
    }

    /// TRIMOFFSETP
    pub mod TRIMOFFSETP {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (5 bits: 0b11111 << 19)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRIMOFFSETN
    pub mod TRIMOFFSETN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CALOUT
    pub mod CALOUT {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LOCK
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

            /// 0b0: CSR is read-write
            pub const ReadWrite: u32 = 0b0;

            /// 0b1: CSR is read-only, can only be cleared by system reset
            pub const ReadOnly: u32 = 0b1;
        }
    }
}

/// OPAMP3 control/status register
pub mod OPAMP3_CSR {

    /// Operational amplifier Enable
    pub mod OPAEN {
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

            /// 0b0: OpAmp disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: OpAmp enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// FORCE_VP
    pub mod FORCE_VP {
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

            /// 0b0: Non-inverting input connected configured inputs
            pub const Normal: u32 = 0b0;

            /// 0b1: Non-inverting input connected to calibration reference voltage
            pub const CalibrationVerification: u32 = 0b1;
        }
    }

    /// VP_SEL
    pub mod VP_SEL {
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

            /// 0b00: VINP0 connected to VINP input
            pub const VINP0: u32 = 0b00;

            /// 0b01: VINP1 connected to VINP input
            pub const VINP1: u32 = 0b01;

            /// 0b10: VINP2 connected to VINP input
            pub const VINP2: u32 = 0b10;

            /// 0b11: DAC3_CH2 connected to VINP input
            pub const DAC3_CH2: u32 = 0b11;
        }
    }

    /// USERTRIM
    pub mod USERTRIM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Factory trim used
            pub const Factory: u32 = 0b0;

            /// 0b1: User trim used
            pub const User: u32 = 0b1;
        }
    }

    /// VM_SEL
    pub mod VM_SEL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: VINM0 connected to VINM input
            pub const VINM0: u32 = 0b00;

            /// 0b01: VINM1 connected to VINM input
            pub const VINM1: u32 = 0b01;

            /// 0b10: Feedback resistor connected to VINM (PGA mode)
            pub const PGA: u32 = 0b10;

            /// 0b11: OpAmp output connected to VINM (Follower mode)
            pub const Output: u32 = 0b11;
        }
    }

    /// OPAHSM
    pub mod OPAHSM {
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

            /// 0b0: OpAmp in normal mode
            pub const Normal: u32 = 0b0;

            /// 0b1: OpAmp in high speed mode
            pub const HighSpeed: u32 = 0b1;
        }
    }

    /// OPAINTOEN
    pub mod OPAINTOEN {
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

            /// 0b0: Output is connected to the output Pin
            pub const OutputPin: u32 = 0b0;

            /// 0b1: Output is connected internally to ADC channel
            pub const ADCChannel: u32 = 0b1;
        }
    }

    /// CALON
    pub mod CALON {
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

            /// 0b0: Calibration mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Calibration mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CALSEL
    pub mod CALSEL {
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

            /// 0b00: 0.033*VDDA applied to OPAMP inputs during calibration
            pub const Percent3_3: u32 = 0b00;

            /// 0b01: 0.1*VDDA applied to OPAMP inputs during calibration
            pub const Percent10: u32 = 0b01;

            /// 0b10: 0.5*VDDA applied to OPAMP inputs during calibration
            pub const Percent50: u32 = 0b10;

            /// 0b11: 0.9*VDDA applied to OPAMP inputs during calibration
            pub const Percent90: u32 = 0b11;
        }
    }

    /// PGA_GAIN
    pub mod PGA_GAIN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (5 bits: 0b11111 << 14)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Gain 2
            pub const Gain2: u32 = 0b00000;

            /// 0b00001: Gain 4
            pub const Gain4: u32 = 0b00001;

            /// 0b00010: Gain 8
            pub const Gain8: u32 = 0b00010;

            /// 0b00011: Gain 16
            pub const Gain16: u32 = 0b00011;

            /// 0b00100: Gain 32
            pub const Gain32: u32 = 0b00100;

            /// 0b00101: Gain 64
            pub const Gain64: u32 = 0b00101;

            /// 0b01000: Gain 2, input/bias connected to VINM0 or inverting gain
            pub const Gain2_InputVINM0: u32 = 0b01000;

            /// 0b01001: Gain 4, input/bias connected to VINM0 or inverting gain
            pub const Gain4_InputVINM0: u32 = 0b01001;

            /// 0b01010: Gain 8, input/bias connected to VINM0 or inverting gain
            pub const Gain8_InputVINM0: u32 = 0b01010;

            /// 0b01011: Gain 16, input/bias connected to VINM0 or inverting gain
            pub const Gain16_InputVINM0: u32 = 0b01011;

            /// 0b01100: Gain 32, input/bias connected to VINM0 or inverting gain
            pub const Gain32_InputVINM0: u32 = 0b01100;

            /// 0b01101: Gain 64, input/bias connected to VINM0 or inverting gain
            pub const Gain64_InputVINM0: u32 = 0b01101;

            /// 0b10000: Gain 2, with filtering on VINM0
            pub const Gain2_FilteringVINM0: u32 = 0b10000;

            /// 0b10001: Gain 4, with filtering on VINM0
            pub const Gain4_FilteringVINM0: u32 = 0b10001;

            /// 0b10010: Gain 8, with filtering on VINM0
            pub const Gain8_FilteringVINM0: u32 = 0b10010;

            /// 0b10011: Gain 16, with filtering on VINM0
            pub const Gain16_FilteringVINM0: u32 = 0b10011;

            /// 0b10100: Gain 32, with filtering on VINM0
            pub const Gain32_FilteringVINM0: u32 = 0b10100;

            /// 0b10101: Gain 64, with filtering on VINM0
            pub const Gain64_FilteringVINM0: u32 = 0b10101;

            /// 0b11000: Gain 2, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain2_InputVINM0FilteringVINM1: u32 = 0b11000;

            /// 0b11001: Gain 4, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain4_InputVINM0FilteringVINM1: u32 = 0b11001;

            /// 0b11010: Gain 8, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain8_InputVINM0FilteringVINM1: u32 = 0b11010;

            /// 0b11011: Gain 16, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain16_InputVINM0FilteringVINM1: u32 = 0b11011;

            /// 0b11100: Gain 32, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain32_InputVINM0FilteringVINM1: u32 = 0b11100;

            /// 0b11101: Gain 64, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
            pub const Gain64_InputVINM0FilteringVINM1: u32 = 0b11101;
        }
    }

    /// TRIMOFFSETP
    pub mod TRIMOFFSETP {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (5 bits: 0b11111 << 19)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRIMOFFSETN
    pub mod TRIMOFFSETN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CALOUT
    pub mod CALOUT {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LOCK
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

            /// 0b0: CSR is read-write
            pub const ReadWrite: u32 = 0b0;

            /// 0b1: CSR is read-only, can only be cleared by system reset
            pub const ReadOnly: u32 = 0b1;
        }
    }
}

/// OPAMP1 control/status register
pub mod OPAMP1_TCMR {

    /// VMS_SEL
    pub mod VMS_SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VPS_SEL
    pub mod VPS_SEL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: VINP0 connected to VINP input
            pub const VINP0: u32 = 0b00;

            /// 0b01: VINP1 connected to VINP input
            pub const VINP1: u32 = 0b01;

            /// 0b10: VINP2 connected to VINP input
            pub const VINP2: u32 = 0b10;

            /// 0b11: DAC3_CH1 connected to VINP input
            pub const DAC3_CH1: u32 = 0b11;
        }
    }

    /// T1CM_EN
    pub mod T1CM_EN {
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

            /// 0b0: Automatic input switch triggered by TIM1 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Automatic input switch triggered by TIM1 enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// T8CM_EN
    pub mod T8CM_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Automatic input switch triggered by TIM8 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Automatic input switch triggered by TIM8 enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// T20CM_EN
    pub mod T20CM_EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Automatic input switch triggered by TIM20 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Automatic input switch triggered by TIM20 enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LOCK
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

            /// 0b0: TCMR is read-write
            pub const ReadWrite: u32 = 0b0;

            /// 0b1: TCMR is read-only, can only be cleared by system reset
            pub const ReadOnly: u32 = 0b1;
        }
    }
}

/// OPAMP2 control/status register
pub mod OPAMP2_TCMR {

    /// VMS_SEL
    pub mod VMS_SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VPS_SEL
    pub mod VPS_SEL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: VINP0 connected to VINP input
            pub const VINP0: u32 = 0b00;

            /// 0b01: VINP1 connected to VINP input
            pub const VINP1: u32 = 0b01;

            /// 0b10: VINP2 connected to VINP input
            pub const VINP2: u32 = 0b10;

            /// 0b11: VINP3 connected to VINP input
            pub const VINP3: u32 = 0b11;
        }
    }

    /// T1CM_EN
    pub mod T1CM_EN {
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

            /// 0b0: Automatic input switch triggered by TIM1 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Automatic input switch triggered by TIM1 enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// T8CM_EN
    pub mod T8CM_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Automatic input switch triggered by TIM8 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Automatic input switch triggered by TIM8 enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// T20CM_EN
    pub mod T20CM_EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Automatic input switch triggered by TIM20 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Automatic input switch triggered by TIM20 enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LOCK
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

            /// 0b0: TCMR is read-write
            pub const ReadWrite: u32 = 0b0;

            /// 0b1: TCMR is read-only, can only be cleared by system reset
            pub const ReadOnly: u32 = 0b1;
        }
    }
}

/// OPAMP3 control/status register
pub mod OPAMP3_TCMR {

    /// VMS_SEL
    pub mod VMS_SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VPS_SEL
    pub mod VPS_SEL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: VINP0 connected to VINP input
            pub const VINP0: u32 = 0b00;

            /// 0b01: VINP1 connected to VINP input
            pub const VINP1: u32 = 0b01;

            /// 0b10: VINP2 connected to VINP input
            pub const VINP2: u32 = 0b10;

            /// 0b11: DAC3_CH2 connected to VINP input
            pub const DAC3_CH2: u32 = 0b11;
        }
    }

    /// T1CM_EN
    pub mod T1CM_EN {
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

            /// 0b0: Automatic input switch triggered by TIM1 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Automatic input switch triggered by TIM1 enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// T8CM_EN
    pub mod T8CM_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Automatic input switch triggered by TIM8 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Automatic input switch triggered by TIM8 enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// T20CM_EN
    pub mod T20CM_EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Automatic input switch triggered by TIM20 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Automatic input switch triggered by TIM20 enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LOCK
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

            /// 0b0: TCMR is read-write
            pub const ReadWrite: u32 = 0b0;

            /// 0b1: TCMR is read-only, can only be cleared by system reset
            pub const ReadOnly: u32 = 0b1;
        }
    }
}

/// OPAMP6 control/status register
pub mod OPAMP6_CSR {
    pub use super::OPAMP1_CSR::CALON;
    pub use super::OPAMP1_CSR::CALOUT;
    pub use super::OPAMP1_CSR::CALSEL;
    pub use super::OPAMP1_CSR::FORCE_VP;
    pub use super::OPAMP1_CSR::LOCK;
    pub use super::OPAMP1_CSR::OPAEN;
    pub use super::OPAMP1_CSR::OPAHSM;
    pub use super::OPAMP1_CSR::OPAINTOEN;
    pub use super::OPAMP1_CSR::PGA_GAIN;
    pub use super::OPAMP1_CSR::TRIMOFFSETN;
    pub use super::OPAMP1_CSR::TRIMOFFSETP;
    pub use super::OPAMP1_CSR::USERTRIM;
    pub use super::OPAMP1_CSR::VM_SEL;
    pub use super::OPAMP1_CSR::VP_SEL;
}

/// OPAMP6 control/status register
pub mod OPAMP6_TCMR {
    pub use super::OPAMP1_TCMR::LOCK;
    pub use super::OPAMP1_TCMR::T1CM_EN;
    pub use super::OPAMP1_TCMR::T20CM_EN;
    pub use super::OPAMP1_TCMR::T8CM_EN;
    pub use super::OPAMP1_TCMR::VMS_SEL;
    pub use super::OPAMP1_TCMR::VPS_SEL;
}
#[repr(C)]
pub struct RegisterBlock {
    /// OPAMP1 control/status register
    pub OPAMP1_CSR: RWRegister<u32>,

    /// OPAMP2 control/status register
    pub OPAMP2_CSR: RWRegister<u32>,

    /// OPAMP3 control/status register
    pub OPAMP3_CSR: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// OPAMP6 control/status register
    pub OPAMP6_CSR: RWRegister<u32>,

    /// OPAMP1 control/status register
    pub OPAMP1_TCMR: RWRegister<u32>,

    /// OPAMP2 control/status register
    pub OPAMP2_TCMR: RWRegister<u32>,

    /// OPAMP3 control/status register
    pub OPAMP3_TCMR: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// OPAMP6 control/status register
    pub OPAMP6_TCMR: RWRegister<u32>,
}
pub struct ResetValues {
    pub OPAMP1_CSR: u32,
    pub OPAMP2_CSR: u32,
    pub OPAMP3_CSR: u32,
    pub OPAMP6_CSR: u32,
    pub OPAMP1_TCMR: u32,
    pub OPAMP2_TCMR: u32,
    pub OPAMP3_TCMR: u32,
    pub OPAMP6_TCMR: u32,
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
