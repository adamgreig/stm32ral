#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Reset and clock control
//!
//! Used by: stm32l0x2, stm32l0x3

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Clock control register
pub mod CR {

    /// PLL clock ready flag
    pub mod PLLRDY {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: PLL unlocked
            pub const Unlocked: u32 = 0b0;

            /// 0b1: PLL locked
            pub const Locked: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL enable bit
    pub mod PLLON {
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

            /// 0b0: Clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TC/LCD prescaler
    pub mod RTCPRE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: HSE divided by 2
            pub const Div2: u32 = 0b00;

            /// 0b01: HSE divided by 4
            pub const Div4: u32 = 0b01;

            /// 0b10: HSE divided by 8
            pub const Div8: u32 = 0b10;

            /// 0b11: HSE divided by 16
            pub const Div16: u32 = 0b11;
        }
    }

    /// Clock security system on HSE enable bit
    pub mod CSSHSEON {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLLON::RW;
    }

    /// HSE clock bypass bit
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

            /// 0b0: HSE oscillator not bypassed
            pub const NotBypassed: u32 = 0b0;

            /// 0b1: HSE oscillator bypassed
            pub const Bypassed: u32 = 0b1;
        }
    }

    /// HSE clock ready flag
    pub mod HSERDY {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Oscillator is not stable
            pub const NotReady: u32 = 0b0;

            /// 0b1: Oscillator is stable
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSE clock enable bit
    pub mod HSEON {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLLON::RW;
    }

    /// MSI clock ready flag
    pub mod MSIRDY {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        pub use super::HSERDY::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MSI clock enable bit
    pub mod MSION {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLLON::RW;
    }

    /// HSI16DIVF
    pub mod HSI16DIVF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: 16 MHz HSI clock not divided
            pub const NotDivided: u32 = 0b0;

            /// 0b1: 16 MHz HSI clock divided by 4
            pub const Div4: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSI16DIVEN
    pub mod HSI16DIVEN {
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

            /// 0b0: no 16 MHz HSI division requested
            pub const NotDivided: u32 = 0b0;

            /// 0b1: 16 MHz HSI division by 4 requested
            pub const Div4: u32 = 0b1;
        }
    }

    /// Internal high-speed clock ready flag
    pub mod HSI16RDYF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: HSI 16 MHz oscillator not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: HSI 16 MHz oscillator ready
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// High-speed internal clock enable bit for some IP kernels
    pub mod HSI16KERON {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLLON::RW;
    }

    /// 16 MHz high-speed internal clock enable
    pub mod HSI16ON {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLLON::RW;
    }

    /// 16 MHz high-speed internal clock output enable
    pub mod HSI16OUTEN {
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

            /// 0b0: HSI output clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: HSI output clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Internal clock sources calibration register
pub mod ICSCR {

    /// MSI clock trimming
    pub mod MSITRIM {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MSI clock calibration
    pub mod MSICAL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MSI clock ranges
    pub mod MSIRANGE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: range 0 around 65.536 kHz
            pub const Range0: u32 = 0b000;

            /// 0b001: range 1 around 131.072 kHz
            pub const Range1: u32 = 0b001;

            /// 0b010: range 2 around 262.144 kHz
            pub const Range2: u32 = 0b010;

            /// 0b011: range 3 around 524.288 kHz
            pub const Range3: u32 = 0b011;

            /// 0b100: range 4 around 1.048 MHz
            pub const Range4: u32 = 0b100;

            /// 0b101: range 5 around 2.097 MHz (reset value)
            pub const Range5: u32 = 0b101;

            /// 0b110: range 6 around 4.194 MHz
            pub const Range6: u32 = 0b110;

            /// 0b111: not allowed
            pub const Range7: u32 = 0b111;
        }
    }

    /// High speed internal clock trimming
    pub mod HSI16TRIM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// nternal high speed clock calibration
    pub mod HSI16CAL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Clock recovery RC register
pub mod CRRCR {

    /// 48 MHz HSI clock calibration
    pub mod HSI48CAL {
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

    /// 48MHz HSI clock ready flag
    pub mod HSI48RDY {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 48MHz HSI clock enable bit
    pub mod HSI48ON {
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

    /// 48 MHz HSI clock divided by 6 output enable
    pub mod HSI48DIV6EN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Clock configuration register
pub mod CFGR {

    /// Microcontroller clock output prescaler
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

            /// 0b000: No division
            pub const Div1: u32 = 0b000;

            /// 0b001: Division by 2
            pub const Div2: u32 = 0b001;

            /// 0b010: Division by 4
            pub const Div4: u32 = 0b010;

            /// 0b011: Division by 8
            pub const Div8: u32 = 0b011;

            /// 0b100: Division by 16
            pub const Div16: u32 = 0b100;
        }
    }

    /// Microcontroller clock output selection
    pub mod MCOSEL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No clock
            pub const NoClock: u32 = 0b0000;

            /// 0b0001: SYSCLK clock selected
            pub const SYSCLK: u32 = 0b0001;

            /// 0b0010: HSI oscillator clock selected
            pub const HSI16: u32 = 0b0010;

            /// 0b0011: MSI oscillator clock selected
            pub const MSI: u32 = 0b0011;

            /// 0b0100: HSE oscillator clock selected
            pub const HSE: u32 = 0b0100;

            /// 0b0101: PLL clock selected
            pub const PLL: u32 = 0b0101;

            /// 0b0110: LSI oscillator clock selected
            pub const LSI: u32 = 0b0110;

            /// 0b0111: LSE oscillator clock selected
            pub const LSE: u32 = 0b0111;
        }
    }

    /// PLL output division
    pub mod PLLDIV {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b01: PLLVCO / 2
            pub const Div2: u32 = 0b01;

            /// 0b10: PLLVCO / 3
            pub const Div3: u32 = 0b10;

            /// 0b11: PLLVCO / 4
            pub const Div4: u32 = 0b11;
        }
    }

    /// PLL multiplication factor
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

            /// 0b0000: PLL clock entry x 3
            pub const Mul3: u32 = 0b0000;

            /// 0b0001: PLL clock entry x 4
            pub const Mul4: u32 = 0b0001;

            /// 0b0010: PLL clock entry x 6
            pub const Mul6: u32 = 0b0010;

            /// 0b0011: PLL clock entry x 8
            pub const Mul8: u32 = 0b0011;

            /// 0b0100: PLL clock entry x 12
            pub const Mul12: u32 = 0b0100;

            /// 0b0101: PLL clock entry x 16
            pub const Mul16: u32 = 0b0101;

            /// 0b0110: PLL clock entry x 24
            pub const Mul24: u32 = 0b0110;

            /// 0b0111: PLL clock entry x 32
            pub const Mul32: u32 = 0b0111;

            /// 0b1000: PLL clock entry x 48
            pub const Mul48: u32 = 0b1000;
        }
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

            /// 0b0: HSI selected as PLL input clock
            pub const HSI16: u32 = 0b0;

            /// 0b1: HSE selected as PLL input clock
            pub const HSE: u32 = 0b1;
        }
    }

    /// Wake-up from stop clock selection
    pub mod STOPWUCK {
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

            /// 0b0: Internal 64 KHz to 4 MHz (MSI) oscillator selected as wake-up from Stop clock
            pub const MSI: u32 = 0b0;

            /// 0b1: Internal 16 MHz (HSI) oscillator selected as wake-up from Stop clock (or HSI16/4 if HSI16DIVEN=1)
            pub const HSI16: u32 = 0b1;
        }
    }

    /// APB high-speed prescaler (APB2)
    pub mod PPRE2 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (3 bits: 0b111 << 11)
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

    /// APB low-speed prescaler (APB1)
    pub mod PPRE1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PPRE2::RW;
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

            /// 0b0000: system clock not divided
            pub const Div1: u32 = 0b0000;

            /// 0b1000: system clock divided by 2
            pub const Div2: u32 = 0b1000;

            /// 0b1001: system clock divided by 4
            pub const Div4: u32 = 0b1001;

            /// 0b1010: system clock divided by 8
            pub const Div8: u32 = 0b1010;

            /// 0b1011: system clock divided by 16
            pub const Div16: u32 = 0b1011;

            /// 0b1100: system clock divided by 64
            pub const Div64: u32 = 0b1100;

            /// 0b1101: system clock divided by 128
            pub const Div128: u32 = 0b1101;

            /// 0b1110: system clock divided by 256
            pub const Div256: u32 = 0b1110;

            /// 0b1111: system clock divided by 512
            pub const Div512: u32 = 0b1111;
        }
    }

    /// System clock switch status
    pub mod SWS {
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

            /// 0b00: MSI oscillator used as system clock
            pub const MSI: u32 = 0b00;

            /// 0b01: HSI oscillator used as system clock
            pub const HSI16: u32 = 0b01;

            /// 0b10: HSE oscillator used as system clock
            pub const HSE: u32 = 0b10;

            /// 0b11: PLL used as system clock
            pub const PLL: u32 = 0b11;
        }
    }

    /// System clock switch
    pub mod SW {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SWS::RW;
    }
}

/// Clock interrupt enable register
pub mod CIER {

    /// LSE CSS interrupt flag
    pub mod CSSLSE {
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

            /// 0b0: LSE CSS interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: LSE CSS interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// HSI48 ready interrupt flag
    pub mod HSI48RDYIE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Ready interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Ready interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// MSI ready interrupt flag
    pub mod MSIRDYIE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSI48RDYIE::RW;
    }

    /// PLL ready interrupt flag
    pub mod PLLRDYIE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSI48RDYIE::RW;
    }

    /// HSE ready interrupt flag
    pub mod HSERDYIE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSI48RDYIE::RW;
    }

    /// HSI16 ready interrupt flag
    pub mod HSI16RDYIE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSI48RDYIE::RW;
    }

    /// LSE ready interrupt flag
    pub mod LSERDYIE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSI48RDYIE::RW;
    }

    /// LSI ready interrupt flag
    pub mod LSIRDYIE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSI48RDYIE::RW;
    }
}

/// Clock interrupt flag register
pub mod CIFR {

    /// Clock Security System Interrupt flag
    pub mod CSSHSEF {
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

            /// 0b0: No clock security interrupt caused by HSE clock failure
            pub const NoClock: u32 = 0b0;

            /// 0b1: Clock security interrupt caused by HSE clock failure
            pub const Clock: u32 = 0b1;
        }
    }

    /// LSE Clock Security System Interrupt flag
    pub mod CSSLSEF {
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

            /// 0b0: No failure detected on LSE clock failure
            pub const NoFailure: u32 = 0b0;

            /// 0b1: Failure detected on LSE clock failure
            pub const Failure: u32 = 0b1;
        }
    }

    /// HSI48 ready interrupt flag
    pub mod HSI48RDYF {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
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

    /// MSI ready interrupt flag
    pub mod MSIRDYF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        pub use super::HSI48RDYF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL ready interrupt flag
    pub mod PLLRDYF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::HSI48RDYF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSE ready interrupt flag
    pub mod HSERDYF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::HSI48RDYF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSI16 ready interrupt flag
    pub mod HSI16RDYF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::HSI48RDYF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSE ready interrupt flag
    pub mod LSERDYF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::HSI48RDYF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSI ready interrupt flag
    pub mod LSIRDYF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        pub use super::HSI48RDYF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Clock interrupt clear register
pub mod CICR {

    /// Clock Security System Interrupt clear
    pub mod CSSHSEC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
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

    /// LSE Clock Security System Interrupt clear
    pub mod CSSLSEC {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSSHSEC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSI48 ready Interrupt clear
    pub mod HSI48RDYC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSSHSEC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MSI ready Interrupt clear
    pub mod MSIRDYC {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSSHSEC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL ready Interrupt clear
    pub mod PLLRDYC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSSHSEC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSE ready Interrupt clear
    pub mod HSERDYC {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSSHSEC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSI16 ready Interrupt clear
    pub mod HSI16RDYC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSSHSEC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSE ready Interrupt clear
    pub mod LSERDYC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSSHSEC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSI ready Interrupt clear
    pub mod LSIRDYC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CSSHSEC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPIO reset register
pub mod IOPRSTR {

    /// I/O port H reset
    pub mod IOPHRST {
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

            /// 0b1: Reset I/O port
            pub const Reset: u32 = 0b1;
        }
    }

    /// I/O port D reset
    pub mod IOPDRST {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPHRST::RW;
    }

    /// I/O port A reset
    pub mod IOPCRST {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPHRST::RW;
    }

    /// I/O port B reset
    pub mod IOPBRST {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPHRST::RW;
    }

    /// I/O port A reset
    pub mod IOPARST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPHRST::RW;
    }

    /// I/O port E reset
    pub mod IOPERST {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPHRST::RW;
    }
}

/// AHB peripheral reset register
pub mod AHBRSTR {

    /// Crypto module reset
    pub mod CRYPRST {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Reset the module
            pub const Reset: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Random Number Generator module reset
    pub mod RNGRST {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CRYPRST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Touch Sensing reset
    pub mod TOUCHRST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CRYPRST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Test integration module reset
    pub mod CRCRST {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CRYPRST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Memory interface reset
    pub mod MIFRST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CRYPRST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA reset
    pub mod DMARST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CRYPRST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// APB2 peripheral reset register
pub mod APB2RSTR {

    /// DBG reset
    pub mod DBGRST {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Reset the module
            pub const Reset: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USART1 reset
    pub mod USART1RST {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DBGRST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SPI 1 reset
    pub mod SPI1RST {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DBGRST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADC interface reset
    pub mod ADCRST {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DBGRST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TIM22 timer reset
    pub mod TIM22RST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DBGRST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TIM21 timer reset
    pub mod TIM21RST {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DBGRST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// System configuration controller reset
    pub mod SYSCFGRST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DBGRST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// APB1 peripheral reset register
pub mod APB1RSTR {

    /// Low power timer reset
    pub mod LPTIM1RST {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Reset the module
            pub const Reset: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC interface reset
    pub mod DACRST {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Power interface reset
    pub mod PWRRST {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clock recovery system reset
    pub mod CRSRST {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USB reset
    pub mod USBRST {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C2 reset
    pub mod I2C2RST {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C1 reset
    pub mod I2C1RST {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPUART1 reset
    pub mod LPUART1RST {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// UART2 reset
    pub mod LPUART12RST {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SPI2 reset
    pub mod SPI2RST {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Window watchdog reset
    pub mod WWDRST {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer 6 reset
    pub mod TIM6RST {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer2 reset
    pub mod TIM2RST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer3 reset
    pub mod TIM3RST {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer 7 reset
    pub mod TIM7RST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USART4 reset
    pub mod USART4RST {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USART5 reset
    pub mod USART5RST {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C3 reset
    pub mod I2C3RST {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::LPTIM1RST::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPIO clock enable register
pub mod IOPENR {

    /// I/O port H clock enable bit
    pub mod IOPHEN {
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

            /// 0b0: Port clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Port clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// I/O port D clock enable bit
    pub mod IOPDEN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPHEN::RW;
    }

    /// IO port A clock enable bit
    pub mod IOPCEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPHEN::RW;
    }

    /// IO port B clock enable bit
    pub mod IOPBEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPHEN::RW;
    }

    /// IO port A clock enable bit
    pub mod IOPAEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPHEN::RW;
    }

    /// I/O port E clock enable bit
    pub mod IOPEEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPHEN::RW;
    }
}

/// AHB peripheral clock enable register
pub mod AHBENR {

    /// Crypto clock enable bit
    pub mod CRYPEN {
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

            /// 0b0: Clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Random Number Generator clock enable bit
    pub mod RNGEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRYPEN::RW;
    }

    /// Touch Sensing clock enable bit
    pub mod TOUCHEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRYPEN::RW;
    }

    /// CRC clock enable bit
    pub mod CRCEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRYPEN::RW;
    }

    /// NVM interface clock enable bit
    pub mod MIFEN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRYPEN::RW;
    }

    /// DMA clock enable bit
    pub mod DMAEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRYPEN::RW;
    }
}

/// APB2 peripheral clock enable register
pub mod APB2ENR {

    /// DBG clock enable bit
    pub mod DBGEN {
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

            /// 0b0: Clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// USART1 clock enable bit
    pub mod USART1EN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBGEN::RW;
    }

    /// SPI1 clock enable bit
    pub mod SPI1EN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBGEN::RW;
    }

    /// ADC clock enable bit
    pub mod ADCEN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBGEN::RW;
    }

    /// MiFaRe Firewall clock enable bit
    pub mod MIFIEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBGEN::RW;
    }

    /// TIM22 timer clock enable bit
    pub mod TIM22EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBGEN::RW;
    }

    /// TIM21 timer clock enable bit
    pub mod TIM21EN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBGEN::RW;
    }

    /// System configuration controller clock enable bit
    pub mod SYSCFGEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBGEN::RW;
    }
}

/// APB1 peripheral clock enable register
pub mod APB1ENR {

    /// Low power timer clock enable bit
    pub mod LPTIM1EN {
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

            /// 0b0: Clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DAC interface clock enable bit
    pub mod DACEN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// Power interface clock enable bit
    pub mod PWREN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// Clock recovery system clock enable bit
    pub mod CRSEN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// USB clock enable bit
    pub mod USBEN {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// I2C2 clock enable bit
    pub mod I2C2EN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// I2C1 clock enable bit
    pub mod I2C1EN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// LPUART1 clock enable bit
    pub mod LPUART1EN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// UART2 clock enable bit
    pub mod USART2EN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// SPI2 clock enable bit
    pub mod SPI2EN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// Window watchdog clock enable bit
    pub mod WWDGEN {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// Timer 6 clock enable bit
    pub mod TIM6EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// Timer2 clock enable bit
    pub mod TIM2EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// Timer3 clock enable bit
    pub mod TIM3EN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// Timer 7 clock enable bit
    pub mod TIM7EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// USART4 clock enable bit
    pub mod USART4EN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// USART5 clock enable bit
    pub mod USART5EN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// I2C3 clock enable bit
    pub mod I2C3EN {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }
}

/// GPIO clock enable in sleep mode register
pub mod IOPSMEN {

    /// IOPHSMEN
    pub mod IOPHSMEN {
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

            /// 0b0: Port x clock is disabled in Sleep mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: Port x clock is enabled in Sleep mode (if enabled by IOPHEN)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// IOPDSMEN
    pub mod IOPDSMEN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPHSMEN::RW;
    }

    /// IOPCSMEN
    pub mod IOPCSMEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPHSMEN::RW;
    }

    /// IOPBSMEN
    pub mod IOPBSMEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPHSMEN::RW;
    }

    /// IOPASMEN
    pub mod IOPASMEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPHSMEN::RW;
    }

    /// Port E clock enable during Sleep mode bit
    pub mod IOPESMEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IOPHSMEN::RW;
    }
}

/// AHB peripheral clock enable in sleep mode register
pub mod AHBSMENR {

    /// Crypto clock enable during sleep mode bit
    pub mod CRYPSMEN {
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

            /// 0b0: Crypto clock disabled in Sleep mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: Crypto clock enabled in Sleep mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Random Number Generator clock enable during sleep mode bit
    pub mod RNGSMEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Touch Sensing clock enable during sleep mode bit
    pub mod TOUCHSMEN {
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

    /// CRC clock enable during sleep mode bit
    pub mod CRCSMEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Test integration module clock disabled in Sleep mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: Test integration module clock enabled in Sleep mode (if enabled by CRCEN)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SRAM interface clock enable during sleep mode bit
    pub mod SRAMSMEN {
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

            /// 0b0: NVM interface clock disabled in Sleep mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: NVM interface clock enabled in Sleep mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// NVM interface clock enable during sleep mode bit
    pub mod MIFSMEN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SRAMSMEN::RW;
    }

    /// DMA clock enable during sleep mode bit
    pub mod DMASMEN {
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

            /// 0b0: DMA clock disabled in Sleep mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA clock enabled in Sleep mode
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// APB2 peripheral clock enable in sleep mode register
pub mod APB2SMENR {

    /// DBG clock enable during sleep mode bit
    pub mod DBGSMEN {
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

            /// 0b0: Clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// USART1 clock enable during sleep mode bit
    pub mod USART1SMEN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBGSMEN::RW;
    }

    /// SPI1 clock enable during sleep mode bit
    pub mod SPI1SMEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBGSMEN::RW;
    }

    /// ADC clock enable during sleep mode bit
    pub mod ADCSMEN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBGSMEN::RW;
    }

    /// TIM22 timer clock enable during sleep mode bit
    pub mod TIM22SMEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBGSMEN::RW;
    }

    /// TIM21 timer clock enable during sleep mode bit
    pub mod TIM21SMEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBGSMEN::RW;
    }

    /// System configuration controller clock enable during sleep mode bit
    pub mod SYSCFGSMEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBGSMEN::RW;
    }
}

/// APB1 peripheral clock enable in sleep mode register
pub mod APB1SMENR {

    /// Low power timer clock enable during sleep mode bit
    pub mod LPTIM1SMEN {
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

            /// 0b0: Clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DAC interface clock enable during sleep mode bit
    pub mod DACSMEN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// Power interface clock enable during sleep mode bit
    pub mod PWRSMEN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// Clock recovery system clock enable during sleep mode bit
    pub mod CRSSMEN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// USB clock enable during sleep mode bit
    pub mod USBSMEN {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// I2C2 clock enable during sleep mode bit
    pub mod I2C2SMEN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// I2C1 clock enable during sleep mode bit
    pub mod I2C1SMEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// LPUART1 clock enable during sleep mode bit
    pub mod LPUART1SMEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// UART2 clock enable during sleep mode bit
    pub mod USART2SMEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// SPI2 clock enable during sleep mode bit
    pub mod SPI2SMEN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// Window watchdog clock enable during sleep mode bit
    pub mod WWDGSMEN {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// Timer 6 clock enable during sleep mode bit
    pub mod TIM6SMEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// Timer2 clock enable during sleep mode bit
    pub mod TIM2SMEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// Timer3 clock enable during Sleep mode bit
    pub mod TIM3SMEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// Timer 7 clock enable during Sleep mode bit
    pub mod TIM7SMEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// USART4 clock enable during Sleep mode bit
    pub mod USART4SMEN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// USART5 clock enable during Sleep mode bit
    pub mod USART5SMEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// 2C3 clock enable during Sleep mode bit
    pub mod I2C3SMEN {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }
}

/// Clock configuration register
pub mod CCIPR {

    /// 48 MHz HSI48 clock source selection bit
    pub mod HSI48MSEL {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C3 clock source selection bits
    pub mod I2C3SEL {
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

            /// 0b00: APB clock selected as peripheral clock
            pub const APB: u32 = 0b00;

            /// 0b01: System clock selected as peripheral clock
            pub const SYSTEM: u32 = 0b01;

            /// 0b10: HSI16 clock selected as peripheral clock
            pub const HSI16: u32 = 0b10;
        }
    }

    /// Low Power Timer clock source selection bits
    pub mod LPTIM1SEL {
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

            /// 0b00: APB clock selected as Timer clock
            pub const APB: u32 = 0b00;

            /// 0b01: LSI clock selected as Timer clock
            pub const LSI: u32 = 0b01;

            /// 0b10: HSI16 clock selected as Timer clock
            pub const HSI16: u32 = 0b10;

            /// 0b11: LSE clock selected as Timer clock
            pub const LSE: u32 = 0b11;
        }
    }

    /// I2C1 clock source selection bits
    pub mod I2C1SEL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C3SEL::RW;
    }

    /// LPUART1 clock source selection bits
    pub mod LPUART1SEL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: APB clock selected as peripheral clock
            pub const APB: u32 = 0b00;

            /// 0b01: System clock selected as peripheral clock
            pub const SYSTEM: u32 = 0b01;

            /// 0b10: HSI16 clock selected as peripheral clock
            pub const HSI16: u32 = 0b10;

            /// 0b11: LSE clock selected as peripheral clock
            pub const LSE: u32 = 0b11;
        }
    }

    /// USART2 clock source selection bits
    pub mod USART2SEL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART1SEL::RW;
    }

    /// USART1 clock source selection bits
    pub mod USART1SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART1SEL::RW;
    }
}

/// Control and status register
pub mod CSR {

    /// Low-power reset flag
    pub mod LPWRRSTF {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
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

    /// Window watchdog reset flag
    pub mod WWDGRSTF {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        pub use super::LPWRRSTF::R;
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
        pub use super::LPWRRSTF::R;
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
        pub use super::LPWRRSTF::R;
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
        pub use super::LPWRRSTF::R;
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
        pub use super::LPWRRSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OBLRSTF
    pub mod OBLRSTF {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        pub use super::LPWRRSTF::R;
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

    /// RTC software reset bit
    pub mod RTCRST {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Resets the RTC peripheral
            pub const Reset: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RTC clock enable bit
    pub mod RTCEN {
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

            /// 0b0: RTC clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: RTC clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RTC and LCD clock source selection bits
    pub mod RTCSEL {
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

            /// 0b00: No clock
            pub const NoClock: u32 = 0b00;

            /// 0b01: LSE oscillator clock used as RTC clock
            pub const LSE: u32 = 0b01;

            /// 0b10: LSI oscillator clock used as RTC clock
            pub const LSI: u32 = 0b10;

            /// 0b11: HSE oscillator clock divided by a programmable prescaler (selection through the RTCPRE\[1:0\] bits in the RCC clock control register (RCC_CR)) used as the RTC clock
            pub const HSE: u32 = 0b11;
        }
    }

    /// CSS on LSE failure detection flag
    pub mod CSSLSED {
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

            /// 0b0: No failure detected on LSE (32 kHz oscillator)
            pub const NoFailure: u32 = 0b0;

            /// 0b1: Failure detected on LSE (32 kHz oscillator)
            pub const Failure: u32 = 0b1;
        }
    }

    /// CSSLSEON
    pub mod CSSLSEON {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Oscillator OFF
            pub const Off: u32 = 0b0;

            /// 0b1: Oscillator ON
            pub const On: u32 = 0b1;
        }
    }

    /// LSEDRV
    pub mod LSEDRV {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Lowest drive
            pub const Low: u32 = 0b00;

            /// 0b01: Medium low drive
            pub const MediumLow: u32 = 0b01;

            /// 0b10: Medium high drive
            pub const MediumHigh: u32 = 0b10;

            /// 0b11: Highest drive
            pub const High: u32 = 0b11;
        }
    }

    /// External low-speed oscillator bypass bit
    pub mod LSEBYP {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LSE oscillator not bypassed
            pub const NotBypassed: u32 = 0b0;

            /// 0b1: LSE oscillator bypassed
            pub const Bypassed: u32 = 0b1;
        }
    }

    /// External low-speed oscillator ready bit
    pub mod LSERDY {
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

            /// 0b0: Oscillator not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: Oscillator ready
            pub const Ready: u32 = 0b1;
        }
    }

    /// External low-speed oscillator enable bit
    pub mod LSEON {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CSSLSEON::RW;
    }

    /// Internal low-speed oscillator ready bit
    pub mod LSIRDY {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSERDY::RW;
    }

    /// Internal low-speed oscillator enable
    pub mod LSION {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CSSLSEON::RW;
    }
}
pub struct RegisterBlock {
    /// Clock control register
    pub CR: RWRegister<u32>,

    /// Internal clock sources calibration register
    pub ICSCR: RWRegister<u32>,

    /// Clock recovery RC register
    pub CRRCR: RWRegister<u32>,

    /// Clock configuration register
    pub CFGR: RWRegister<u32>,

    /// Clock interrupt enable register
    pub CIER: RORegister<u32>,

    /// Clock interrupt flag register
    pub CIFR: RORegister<u32>,

    /// Clock interrupt clear register
    pub CICR: RORegister<u32>,

    /// GPIO reset register
    pub IOPRSTR: RWRegister<u32>,

    /// AHB peripheral reset register
    pub AHBRSTR: RWRegister<u32>,

    /// APB2 peripheral reset register
    pub APB2RSTR: RWRegister<u32>,

    /// APB1 peripheral reset register
    pub APB1RSTR: RWRegister<u32>,

    /// GPIO clock enable register
    pub IOPENR: RWRegister<u32>,

    /// AHB peripheral clock enable register
    pub AHBENR: RWRegister<u32>,

    /// APB2 peripheral clock enable register
    pub APB2ENR: RWRegister<u32>,

    /// APB1 peripheral clock enable register
    pub APB1ENR: RWRegister<u32>,

    /// GPIO clock enable in sleep mode register
    pub IOPSMEN: RWRegister<u32>,

    /// AHB peripheral clock enable in sleep mode register
    pub AHBSMENR: RWRegister<u32>,

    /// APB2 peripheral clock enable in sleep mode register
    pub APB2SMENR: RWRegister<u32>,

    /// APB1 peripheral clock enable in sleep mode register
    pub APB1SMENR: RWRegister<u32>,

    /// Clock configuration register
    pub CCIPR: RWRegister<u32>,

    /// Control and status register
    pub CSR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub ICSCR: u32,
    pub CRRCR: u32,
    pub CFGR: u32,
    pub CIER: u32,
    pub CIFR: u32,
    pub CICR: u32,
    pub IOPRSTR: u32,
    pub AHBRSTR: u32,
    pub APB2RSTR: u32,
    pub APB1RSTR: u32,
    pub IOPENR: u32,
    pub AHBENR: u32,
    pub APB2ENR: u32,
    pub APB1ENR: u32,
    pub IOPSMEN: u32,
    pub AHBSMENR: u32,
    pub APB2SMENR: u32,
    pub APB1SMENR: u32,
    pub CCIPR: u32,
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
