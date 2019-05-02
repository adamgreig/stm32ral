#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Reset and clock control

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use RWRegister;

/// Clock control register
pub mod CR {

    /// Internal High Speed clock enable
    pub mod HSION {
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

            /// 0b0: Clock Off
            pub const Off: u32 = 0b0;

            /// 0b1: Clock On
            pub const On: u32 = 0b1;
        }
    }

    /// Internal High Speed clock ready flag
    pub mod HSIRDY {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Clock not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: Clock ready
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Internal High Speed clock trimming
    pub mod HSITRIM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (5 bits: 0b11111 << 3)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Internal High Speed clock Calibration
    pub mod HSICAL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External High Speed clock enable
    pub mod HSEON {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSION::RW;
    }

    /// External High Speed clock ready flag
    pub mod HSERDY {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        pub use super::HSIRDY::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External High Speed clock Bypass
    pub mod HSEBYP {
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

            /// 0b0: HSE crystal oscillator not bypassed
            pub const NotBypassed: u32 = 0b0;

            /// 0b1: HSE crystal oscillator bypassed with external clock
            pub const Bypassed: u32 = 0b1;
        }
    }

    /// Clock Security System enable
    pub mod CSSON {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSION::RW;
    }

    /// PLL enable
    pub mod PLLON {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSION::RW;
    }

    /// PLL clock ready flag
    pub mod PLLRDY {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        pub use super::HSIRDY::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Clock configuration register (RCC_CFGR)
pub mod CFGR {

    /// System clock Switch
    pub mod SW {
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

            /// 0b00: HSI selected as system clock
            pub const HSI: u32 = 0b00;

            /// 0b01: HSE selected as system clock
            pub const HSE: u32 = 0b01;

            /// 0b10: PLL selected as system clock
            pub const PLL: u32 = 0b10;
        }
    }

    /// System Clock Switch Status
    pub mod SWS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values
        pub mod R {

            /// 0b00: HSE oscillator used as system clock
            pub const HSI: u32 = 0b00;

            /// 0b01: HSI oscillator used as system clock
            pub const HSE: u32 = 0b01;

            /// 0b10: PLL used as system clock
            pub const PLL: u32 = 0b10;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AHB prescaler
    pub mod HPRE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: SYSCLK not divided
            pub const Div1: u32 = 0b0000;

            /// 0b1000: SYSCLK divided by 2
            pub const Div2: u32 = 0b1000;

            /// 0b1001: SYSCLK divided by 4
            pub const Div4: u32 = 0b1001;

            /// 0b1010: SYSCLK divided by 8
            pub const Div8: u32 = 0b1010;

            /// 0b1011: SYSCLK divided by 16
            pub const Div16: u32 = 0b1011;

            /// 0b1100: SYSCLK divided by 64
            pub const Div64: u32 = 0b1100;

            /// 0b1101: SYSCLK divided by 128
            pub const Div128: u32 = 0b1101;

            /// 0b1110: SYSCLK divided by 256
            pub const Div256: u32 = 0b1110;

            /// 0b1111: SYSCLK divided by 512
            pub const Div512: u32 = 0b1111;
        }
    }

    /// APB Low speed prescaler (APB1)
    pub mod PPRE1 {
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

            /// 0b000: HCLK not divided
            pub const Div1: u32 = 0b000;

            /// 0b100: HCLK divided by 2
            pub const Div2: u32 = 0b100;

            /// 0b101: HCLK divided by 4
            pub const Div4: u32 = 0b101;

            /// 0b110: HCLK divided by 8
            pub const Div8: u32 = 0b110;

            /// 0b111: HCLK divided by 16
            pub const Div16: u32 = 0b111;
        }
    }

    /// APB high speed prescaler (APB2)
    pub mod PPRE2 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (3 bits: 0b111 << 11)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PPRE1::RW;
    }

    /// PLL entry clock source
    pub mod PLLSRC {
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

            /// 0b0: HSI divided by 2 selected as PLL input clock
            pub const HSI_Div2: u32 = 0b0;

            /// 0b1: HSE divided by PREDIV selected as PLL input clock
            pub const HSE_Div_PREDIV: u32 = 0b1;
        }
    }

    /// HSE divider for PLL entry
    pub mod PLLXTPRE {
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

            /// 0b0: HSE clock not divided
            pub const Div1: u32 = 0b0;

            /// 0b1: HSE clock divided by 2
            pub const Div2: u32 = 0b1;
        }
    }

    /// PLL Multiplication Factor
    pub mod PLLMUL {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (4 bits: 0b1111 << 18)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PLL input clock x2
            pub const Mul2: u32 = 0b0000;

            /// 0b0001: PLL input clock x3
            pub const Mul3: u32 = 0b0001;

            /// 0b0010: PLL input clock x4
            pub const Mul4: u32 = 0b0010;

            /// 0b0011: PLL input clock x5
            pub const Mul5: u32 = 0b0011;

            /// 0b0100: PLL input clock x6
            pub const Mul6: u32 = 0b0100;

            /// 0b0101: PLL input clock x7
            pub const Mul7: u32 = 0b0101;

            /// 0b0110: PLL input clock x8
            pub const Mul8: u32 = 0b0110;

            /// 0b0111: PLL input clock x9
            pub const Mul9: u32 = 0b0111;

            /// 0b1000: PLL input clock x10
            pub const Mul10: u32 = 0b1000;

            /// 0b1001: PLL input clock x11
            pub const Mul11: u32 = 0b1001;

            /// 0b1010: PLL input clock x12
            pub const Mul12: u32 = 0b1010;

            /// 0b1011: PLL input clock x13
            pub const Mul13: u32 = 0b1011;

            /// 0b1100: PLL input clock x14
            pub const Mul14: u32 = 0b1100;

            /// 0b1101: PLL input clock x15
            pub const Mul15: u32 = 0b1101;

            /// 0b1111: PLL input clock x16
            pub const Mul16: u32 = 0b1111;
        }
    }

    /// I2S external clock source selection
    pub mod I2SSRC {
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

            /// 0b0: System clock used as I2S clock source
            pub const SYSCLK: u32 = 0b0;

            /// 0b1: External clock mapped on the I2S_CKIN pin used as I2S clock source
            pub const CKIN: u32 = 0b1;
        }
    }

    /// Microcontroller clock output
    pub mod MCO {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: MCO output disabled, no clock on MCO
            pub const NoMCO: u32 = 0b000;

            /// 0b010: Internal low speed (LSI) oscillator clock selected
            pub const LSI: u32 = 0b010;

            /// 0b011: External low speed (LSE) oscillator clock selected
            pub const LSE: u32 = 0b011;

            /// 0b100: System clock selected
            pub const SYSCLK: u32 = 0b100;

            /// 0b101: Internal RC 8 MHz (HSI) oscillator clock selected
            pub const HSI: u32 = 0b101;

            /// 0b110: External 4-32 MHz (HSE) oscillator clock selected
            pub const HSE: u32 = 0b110;

            /// 0b111: PLL clock selected (divided by 1 or 2, depending en PLLNODIV)
            pub const PLL: u32 = 0b111;
        }
    }

    /// Microcontroller Clock Output Prescaler
    pub mod MCOPRE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (3 bits: 0b111 << 28)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: MCO is divided by 1
            pub const Div1: u32 = 0b000;

            /// 0b001: MCO is divided by 2
            pub const Div2: u32 = 0b001;

            /// 0b010: MCO is divided by 4
            pub const Div4: u32 = 0b010;

            /// 0b011: MCO is divided by 8
            pub const Div8: u32 = 0b011;

            /// 0b100: MCO is divided by 16
            pub const Div16: u32 = 0b100;

            /// 0b101: MCO is divided by 32
            pub const Div32: u32 = 0b101;

            /// 0b110: MCO is divided by 64
            pub const Div64: u32 = 0b110;

            /// 0b111: MCO is divided by 128
            pub const Div128: u32 = 0b111;
        }
    }

    /// Do not divide PLL to MCO
    pub mod PLLNODIV {
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

            /// 0b0: PLL is divided by 2 for MCO
            pub const Div2: u32 = 0b0;

            /// 0b1: PLL is not divided for MCO
            pub const Div1: u32 = 0b1;
        }
    }
}

/// Clock interrupt register (RCC_CIR)
pub mod CIR {

    /// LSI Ready Interrupt flag
    pub mod LSIRDYF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No clock ready interrupt
            pub const NotInterrupted: u32 = 0b0;

            /// 0b1: Clock ready interrupt
            pub const Interrupted: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSE Ready Interrupt flag
    pub mod LSERDYF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::LSIRDYF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSI Ready Interrupt flag
    pub mod HSIRDYF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::LSIRDYF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSE Ready Interrupt flag
    pub mod HSERDYF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::LSIRDYF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL Ready Interrupt flag
    pub mod PLLRDYF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::LSIRDYF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clock Security System Interrupt flag
    pub mod CSSF {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No clock security interrupt caused by HSE clock failure
            pub const NotInterrupted: u32 = 0b0;

            /// 0b1: Clock security interrupt caused by HSE clock failure
            pub const Interrupted: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSI Ready Interrupt Enable
    pub mod LSIRDYIE {
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

            /// 0b0: Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LSE Ready Interrupt Enable
    pub mod LSERDYIE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYIE::RW;
    }

    /// HSI Ready Interrupt Enable
    pub mod HSIRDYIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYIE::RW;
    }

    /// HSE Ready Interrupt Enable
    pub mod HSERDYIE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYIE::RW;
    }

    /// PLL Ready Interrupt Enable
    pub mod PLLRDYIE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYIE::RW;
    }

    /// LSI Ready Interrupt Clear
    pub mod LSIRDYC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear interrupt flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSE Ready Interrupt Clear
    pub mod LSERDYC {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LSIRDYC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSI Ready Interrupt Clear
    pub mod HSIRDYC {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LSIRDYC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSE Ready Interrupt Clear
    pub mod HSERDYC {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LSIRDYC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL Ready Interrupt Clear
    pub mod PLLRDYC {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LSIRDYC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clock security system interrupt clear
    pub mod CSSC {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear CSSF flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// APB2 peripheral reset register (RCC_APB2RSTR)
pub mod APB2RSTR {

    /// SYSCFG and COMP reset
    pub mod SYSCFGRST {
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

            /// 0b1: Reset the selected module
            pub const Reset: u32 = 0b1;
        }
    }

    /// TIM1 timer reset
    pub mod TIM1RST {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGRST::RW;
    }

    /// USART1 reset
    pub mod USART1RST {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGRST::RW;
    }

    /// TIM15 timer reset
    pub mod TIM15RST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGRST::RW;
    }

    /// TIM16 timer reset
    pub mod TIM16RST {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGRST::RW;
    }

    /// TIM17 timer reset
    pub mod TIM17RST {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGRST::RW;
    }
}

/// APB1 peripheral reset register (RCC_APB1RSTR)
pub mod APB1RSTR {

    /// Timer 2 reset
    pub mod TIM2RST {
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

            /// 0b1: Reset the selected module
            pub const Reset: u32 = 0b1;
        }
    }

    /// Timer 6 reset
    pub mod TIM6RST {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// Window watchdog reset
    pub mod WWDGRST {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// SPI2 reset
    pub mod SPI2RST {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// SPI3 reset
    pub mod SPI3RST {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// USART 2 reset
    pub mod USART2RST {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// USART3 reset
    pub mod USART3RST {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// I2C1 reset
    pub mod I2C1RST {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// I2C2 reset
    pub mod I2C2RST {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// Power interface reset
    pub mod PWRRST {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// DAC interface reset
    pub mod DAC1RST {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// I2C3 reset
    pub mod I2C3RST {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }
}

/// AHB Peripheral Clock enable register (RCC_AHBENR)
pub mod AHBENR {

    /// DMA1 clock enable
    pub mod DMA1EN {
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

            /// 0b0: The selected clock is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SRAM interface clock enable
    pub mod SRAMEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// FLITF clock enable
    pub mod FLITFEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// CRC clock enable
    pub mod CRCEN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// I/O port A clock enable
    pub mod IOPAEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// I/O port B clock enable
    pub mod IOPBEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// I/O port C clock enable
    pub mod IOPCEN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// I/O port D clock enable
    pub mod IOPDEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// I/O port F clock enable
    pub mod IOPFEN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// Touch sensing controller clock enable
    pub mod TSCEN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// ADC 1
    pub mod ADC1EN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }
}

/// APB2 peripheral clock enable register (RCC_APB2ENR)
pub mod APB2ENR {

    /// SYSCFG clock enable
    pub mod SYSCFGEN {
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

            /// 0b0: The selected clock is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TIM1 Timer clock enable
    pub mod TIM1EN {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGEN::RW;
    }

    /// USART1 clock enable
    pub mod USART1EN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGEN::RW;
    }

    /// TIM15 timer clock enable
    pub mod TIM15EN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGEN::RW;
    }

    /// TIM16 timer clock enable
    pub mod TIM16EN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGEN::RW;
    }

    /// TIM17 timer clock enable
    pub mod TIM17EN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGEN::RW;
    }
}

/// APB1 peripheral clock enable register (RCC_APB1ENR)
pub mod APB1ENR {

    /// Timer 2 clock enable
    pub mod TIM2EN {
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

            /// 0b0: The selected clock is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Timer 6 clock enable
    pub mod TIM6EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// Window watchdog clock enable
    pub mod WWDGEN {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// SPI 2 clock enable
    pub mod SPI2EN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// SPI 3 clock enable
    pub mod SPI3EN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// USART 2 clock enable
    pub mod USART2EN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// USART 3 clock enable
    pub mod USART3EN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// I2C 1 clock enable
    pub mod I2C1EN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// I2C 2 clock enable
    pub mod I2C2EN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// Power interface clock enable
    pub mod PWREN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// DAC interface clock enable
    pub mod DAC1EN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// I2C3 clock enable
    pub mod I2C3EN {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }
}

/// Backup domain control register (RCC_BDCR)
pub mod BDCR {

    /// External Low Speed oscillator enable
    pub mod LSEON {
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

            /// 0b0: LSE oscillator Off
            pub const Off: u32 = 0b0;

            /// 0b1: LSE oscillator On
            pub const On: u32 = 0b1;
        }
    }

    /// External Low Speed oscillator ready
    pub mod LSERDY {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: LSE oscillator not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: LSE oscillator ready
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Low Speed oscillator bypass
    pub mod LSEBYP {
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

            /// 0b0: LSE crystal oscillator not bypassed
            pub const NotBypassed: u32 = 0b0;

            /// 0b1: LSE crystal oscillator bypassed with external clock
            pub const Bypassed: u32 = 0b1;
        }
    }

    /// LSE oscillator drive capability
    pub mod LSEDRV {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Low drive capacity
            pub const Low: u32 = 0b00;

            /// 0b01: Medium-high drive capacity
            pub const MediumHigh: u32 = 0b01;

            /// 0b10: Medium-low drive capacity
            pub const MediumLow: u32 = 0b10;

            /// 0b11: High drive capacity
            pub const High: u32 = 0b11;
        }
    }

    /// RTC clock source selection
    pub mod RTCSEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No clock
            pub const NoClock: u32 = 0b00;

            /// 0b01: LSE oscillator clock used as RTC clock
            pub const LSE: u32 = 0b01;

            /// 0b10: LSI oscillator clock used as RTC clock
            pub const LSI: u32 = 0b10;

            /// 0b11: HSE oscillator clock divided by a prescaler used as RTC clock
            pub const HSE: u32 = 0b11;
        }
    }

    /// RTC clock enable
    pub mod RTCEN {
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

            /// 0b0: RTC clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: RTC clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Backup domain software reset
    pub mod BDRST {
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

            /// 0b0: Reset not activated
            pub const Disabled: u32 = 0b0;

            /// 0b1: Reset the entire RTC domain
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Control/status register (RCC_CSR)
pub mod CSR {

    /// Internal low speed oscillator enable
    pub mod LSION {
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

            /// 0b0: LSI oscillator Off
            pub const Off: u32 = 0b0;

            /// 0b1: LSI oscillator On
            pub const On: u32 = 0b1;
        }
    }

    /// Internal low speed oscillator ready
    pub mod LSIRDY {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: LSI oscillator not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: LSI oscillator ready
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Remove reset flag
    pub mod RMVF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears the reset flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Option byte loader reset flag
    pub mod OBLRSTF {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No reset has occured
            pub const NoReset: u32 = 0b0;

            /// 0b1: A reset has occured
            pub const Reset: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PIN reset flag
    pub mod PINRSTF {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        pub use super::OBLRSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// POR/PDR reset flag
    pub mod PORRSTF {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        pub use super::OBLRSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software reset flag
    pub mod SFTRSTF {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        pub use super::OBLRSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Independent watchdog reset flag
    pub mod IWDGRSTF {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        pub use super::OBLRSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Window watchdog reset flag
    pub mod WWDGRSTF {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        pub use super::OBLRSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Low-power reset flag
    pub mod LPWRRSTF {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        pub use super::OBLRSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reset flag of the 1.8 V domain
    pub mod V18PWRRSTF {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        pub use super::OBLRSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// AHB peripheral reset register
pub mod AHBRSTR {

    /// I/O port A reset
    pub mod IOPARST {
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

            /// 0b1: Reset the selected module
            pub const Reset: u32 = 0b1;
        }
    }

    /// I/O port B reset
    pub mod IOPBRST {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPARST::RW;
    }

    /// I/O port C reset
    pub mod IOPCRST {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPARST::RW;
    }

    /// I/O port D reset
    pub mod IOPDRST {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPARST::RW;
    }

    /// I/O port F reset
    pub mod IOPFRST {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPARST::RW;
    }

    /// Touch sensing controller reset
    pub mod TSCRST {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPARST::RW;
    }

    /// ADC1 reset
    pub mod ADC1RST {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPARST::RW;
    }
}

/// Clock configuration register 2
pub mod CFGR2 {

    /// PREDIV division factor
    pub mod PREDIV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PREDIV input clock not divided
            pub const Div1: u32 = 0b0000;

            /// 0b0001: PREDIV input clock divided by 2
            pub const Div2: u32 = 0b0001;

            /// 0b0010: PREDIV input clock divided by 3
            pub const Div3: u32 = 0b0010;

            /// 0b0011: PREDIV input clock divided by 4
            pub const Div4: u32 = 0b0011;

            /// 0b0100: PREDIV input clock divided by 5
            pub const Div5: u32 = 0b0100;

            /// 0b0101: PREDIV input clock divided by 6
            pub const Div6: u32 = 0b0101;

            /// 0b0110: PREDIV input clock divided by 7
            pub const Div7: u32 = 0b0110;

            /// 0b0111: PREDIV input clock divided by 8
            pub const Div8: u32 = 0b0111;

            /// 0b1000: PREDIV input clock divided by 9
            pub const Div9: u32 = 0b1000;

            /// 0b1001: PREDIV input clock divided by 10
            pub const Div10: u32 = 0b1001;

            /// 0b1010: PREDIV input clock divided by 11
            pub const Div11: u32 = 0b1010;

            /// 0b1011: PREDIV input clock divided by 12
            pub const Div12: u32 = 0b1011;

            /// 0b1100: PREDIV input clock divided by 13
            pub const Div13: u32 = 0b1100;

            /// 0b1101: PREDIV input clock divided by 14
            pub const Div14: u32 = 0b1101;

            /// 0b1110: PREDIV input clock divided by 15
            pub const Div15: u32 = 0b1110;

            /// 0b1111: PREDIV input clock divided by 16
            pub const Div16: u32 = 0b1111;
        }
    }

    /// ADC1 prescaler
    pub mod ADC1PRES {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No clock
            pub const NoClock: u32 = 0b0000;

            /// 0b10000: PLL clock not divided
            pub const Div1: u32 = 0b10000;

            /// 0b10001: PLL clock divided by 2
            pub const Div2: u32 = 0b10001;

            /// 0b10010: PLL clock divided by 4
            pub const Div4: u32 = 0b10010;

            /// 0b10011: PLL clock divided by 6
            pub const Div6: u32 = 0b10011;

            /// 0b10100: PLL clock divided by 8
            pub const Div8: u32 = 0b10100;

            /// 0b10101: PLL clock divided by 10
            pub const Div10: u32 = 0b10101;

            /// 0b10110: PLL clock divided by 12
            pub const Div12: u32 = 0b10110;

            /// 0b10111: PLL clock divided by 16
            pub const Div16: u32 = 0b10111;

            /// 0b11000: PLL clock divided by 32
            pub const Div32: u32 = 0b11000;

            /// 0b11001: PLL clock divided by 64
            pub const Div64: u32 = 0b11001;

            /// 0b11010: PLL clock divided by 128
            pub const Div128: u32 = 0b11010;

            /// 0b11011: PLL clock divided by 256
            pub const Div256: u32 = 0b11011;
        }
    }
}

/// Clock configuration register 3
pub mod CFGR3 {

    /// USART1 clock source selection
    pub mod USART1SW {
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

            /// 0b00: PCLK selected as USART clock source
            pub const PCLK: u32 = 0b00;

            /// 0b01: SYSCLK selected as USART clock source
            pub const SYSCLK: u32 = 0b01;

            /// 0b10: LSE selected as USART clock source
            pub const LSE: u32 = 0b10;

            /// 0b11: HSI selected as USART clock source
            pub const HSI: u32 = 0b11;
        }
    }

    /// I2C1 clock source selection
    pub mod I2C1SW {
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

            /// 0b0: HSI clock selected as I2C clock source
            pub const HSI: u32 = 0b0;

            /// 0b1: SYSCLK clock selected as I2C clock source
            pub const SYSCLK: u32 = 0b1;
        }
    }

    /// I2C2 clock source selection
    pub mod I2C2SW {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C1SW::RW;
    }

    /// Timer1 clock source selection
    pub mod TIM1SW {
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

            /// 0b0: PCLK2 clock (doubled frequency when prescaled)
            pub const PCLK2: u32 = 0b0;

            /// 0b1: PLL vco output (running up to 144 MHz)
            pub const PLL: u32 = 0b1;
        }
    }

    /// Timer15 clock source selection
    pub mod TIM15SW {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1SW::RW;
    }

    /// Timer16 clock source selection
    pub mod TIM16SW {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1SW::RW;
    }

    /// Timer17 clock source selection
    pub mod TIM17SW {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1SW::RW;
    }

    /// I2C3 clock source selection
    pub mod I2C3SW {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C1SW::RW;
    }
}
pub struct RegisterBlock {
    /// Clock control register
    pub CR: RWRegister<u32>,

    /// Clock configuration register (RCC_CFGR)
    pub CFGR: RWRegister<u32>,

    /// Clock interrupt register (RCC_CIR)
    pub CIR: RWRegister<u32>,

    /// APB2 peripheral reset register (RCC_APB2RSTR)
    pub APB2RSTR: RWRegister<u32>,

    /// APB1 peripheral reset register (RCC_APB1RSTR)
    pub APB1RSTR: RWRegister<u32>,

    /// AHB Peripheral Clock enable register (RCC_AHBENR)
    pub AHBENR: RWRegister<u32>,

    /// APB2 peripheral clock enable register (RCC_APB2ENR)
    pub APB2ENR: RWRegister<u32>,

    /// APB1 peripheral clock enable register (RCC_APB1ENR)
    pub APB1ENR: RWRegister<u32>,

    /// Backup domain control register (RCC_BDCR)
    pub BDCR: RWRegister<u32>,

    /// Control/status register (RCC_CSR)
    pub CSR: RWRegister<u32>,

    /// AHB peripheral reset register
    pub AHBRSTR: RWRegister<u32>,

    /// Clock configuration register 2
    pub CFGR2: RWRegister<u32>,

    /// Clock configuration register 3
    pub CFGR3: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub CFGR: u32,
    pub CIR: u32,
    pub APB2RSTR: u32,
    pub APB1RSTR: u32,
    pub AHBENR: u32,
    pub APB2ENR: u32,
    pub APB1ENR: u32,
    pub BDCR: u32,
    pub CSR: u32,
    pub AHBRSTR: u32,
    pub CFGR2: u32,
    pub CFGR3: u32,
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

/// Access functions for the RCC peripheral instance
pub mod RCC {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40021000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RCC
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000083,
        CFGR: 0x00000000,
        CIR: 0x00000000,
        APB2RSTR: 0x00000000,
        APB1RSTR: 0x00000000,
        AHBENR: 0x00000014,
        APB2ENR: 0x00000000,
        APB1ENR: 0x00000000,
        BDCR: 0x00000000,
        CSR: 0x0C000000,
        AHBRSTR: 0x00000000,
        CFGR2: 0x00000000,
        CFGR3: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RCC_TAKEN: bool = false;

    /// Safe access to RCC
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
            if RCC_TAKEN {
                None
            } else {
                RCC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RCC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RCC_TAKEN && inst.addr == INSTANCE.addr {
                RCC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal RCC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RCC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to RCC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RCC: *const RegisterBlock = 0x40021000 as *const _;
