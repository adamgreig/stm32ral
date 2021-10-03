#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Reset and clock control
//!
//! Used by: stm32wl5x_cm0p, stm32wl5x_cm4

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Clock control register
pub mod CR {

    /// Main PLL clock ready flag
    pub mod PLLRDY {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLL unlocked
            pub const Unlocked: u32 = 0b0;

            /// 0b1: PLL Locked
            pub const Locked: u32 = 0b1;
        }
    }

    /// Main PLL enable
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

            /// 0b0: Main PLL Off
            pub const Off: u32 = 0b0;

            /// 0b1: Main PLL On
            pub const On: u32 = 0b1;
        }
    }

    /// Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO.
    pub mod HSEBYPPWR {
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

            /// 0b0: PB0 selected
            pub const PB0: u32 = 0b0;

            /// 0b1: VDDTCXO selected
            pub const VDDTCXO: u32 = 0b1;
        }
    }

    /// HSE32 sysclk prescaler
    pub mod HSEPRE {
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

            /// 0b0: SYSCLK not divided (HSE32)
            pub const Div1: u32 = 0b0;

            /// 0b1: SYSCLK divided by two (HSE32/2)
            pub const Div2: u32 = 0b1;
        }
    }

    /// HSE32 Clock security system enable
    pub mod CSSON {
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

            /// 0b0: HSE32 CSS off
            pub const Disabled: u32 = 0b0;

            /// 0b1: HSE32 CSS on if the HSE32 oscillator is stable and off if not
            pub const Enabled: u32 = 0b1;
        }
    }

    /// HSE32 clock ready flag
    pub mod HSERDY {
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

            /// 0b0: HSE32 oscillator not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: HSE32 oscillator ready
            pub const Ready: u32 = 0b1;
        }
    }

    /// HSE32 clock enable
    pub mod HSEON {
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

            /// 0b0: HSE32 oscillator for CPU disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: HSE32 oscillator for CPU enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// HSI16 kernel clock ready flag for peripherals requests.
    pub mod HSIKERDY {
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

            /// 0b0: HSI16 oscillator not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: HSI16 oscillator ready
            pub const Ready: u32 = 0b1;
        }
    }

    /// HSI16 automatic start from Stop
    pub mod HSIASFS {
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

            /// 0b0: HSI16 not enabled by hardware when exiting Stop modes with MSI as wakeup clock
            pub const Disabled: u32 = 0b0;

            /// 0b1: HSI16 enabled by hardware when exiting Stop mode with MSI as wakeup clock
            pub const Enabled: u32 = 0b1;
        }
    }

    /// HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)
    pub mod HSIRDY {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSIKERDY::RW;
    }

    /// HSI16 always enable for peripheral kernel clocks.
    pub mod HSIKERON {
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

            /// 0b0: No effect on HSI16 oscillator
            pub const NotForced: u32 = 0b0;

            /// 0b1: HSI16 oscillator forced on even in Stop modes
            pub const Forced: u32 = 0b1;
        }
    }

    /// HSI16 clock enable
    pub mod HSION {
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

            /// 0b0: HSI16 oscillator off
            pub const Disabled: u32 = 0b0;

            /// 0b1: HSI16 oscillator on
            pub const Enabled: u32 = 0b1;
        }
    }

    /// MSI clock ranges
    pub mod MSIRANGE {
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

            /// 0b0000: range 0 around 100 kHz
            pub const Range100K: u32 = 0b0000;

            /// 0b0001: range 1 around 200 kHz
            pub const Range200K: u32 = 0b0001;

            /// 0b0010: range 2 around 400 kHz
            pub const Range400K: u32 = 0b0010;

            /// 0b0011: range 3 around 800 kHz
            pub const Range800K: u32 = 0b0011;

            /// 0b0100: range 4 around 1 MHz
            pub const Range1M: u32 = 0b0100;

            /// 0b0101: range 5 around 2 MHz
            pub const Range2M: u32 = 0b0101;

            /// 0b0110: range 6 around 4 MHz (reset value)
            pub const Range4M: u32 = 0b0110;

            /// 0b0111: range 7 around 8 MHz
            pub const Range8M: u32 = 0b0111;

            /// 0b1000: range 8 around 16 MHz
            pub const Range16M: u32 = 0b1000;

            /// 0b1001: range 9 around 24 MHz
            pub const Range24M: u32 = 0b1001;

            /// 0b1010: range 10 around 32 MHz
            pub const Range32M: u32 = 0b1010;

            /// 0b1011: range 11 around 48 MHz
            pub const Range48M: u32 = 0b1011;
        }
    }

    /// MSI range control selection
    pub mod MSIRGSEL {
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

            /// 0b0: MSI frequency range defined by MSISRANGE\[3:0\] in the RCC_CSR register
            pub const CSR: u32 = 0b0;

            /// 0b1: MSI frequency range defined by MSIRANGE\[3:0\] in the RCC_CR register
            pub const CR: u32 = 0b1;
        }
    }

    /// MSI clock PLL enable
    pub mod MSIPLLEN {
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

            /// 0b0: MSI PLL Off
            pub const Off: u32 = 0b0;

            /// 0b1: MSI PLL On
            pub const On: u32 = 0b1;
        }
    }

    /// MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)
    pub mod MSIRDY {
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

            /// 0b0: MSI oscillator not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: MSI oscillator ready
            pub const Ready: u32 = 0b1;
        }
    }

    /// MSI clock enable
    pub mod MSION {
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

            /// 0b0: MSI oscillator off
            pub const Disabled: u32 = 0b0;

            /// 0b1: MSI oscillator on
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Internal clock sources calibration register
pub mod ICSCR {

    /// HSI16 clock trimming
    pub mod HSITRIM {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (7 bits: 0x7f << 24)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSI16 clock calibration
    pub mod HSICAL {
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

    /// MSI clock trimming
    pub mod MSITRIM {
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

    /// MSI clock calibration
    pub mod MSICAL {
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

    /// Microcontroller clock output
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

            /// 0b0010: MSI oscillator clock selected
            pub const MSI: u32 = 0b0010;

            /// 0b0011: HSI16 oscillator clock selected
            pub const HSI16: u32 = 0b0011;

            /// 0b0100: HSE32 oscillator clock selected
            pub const HSE32: u32 = 0b0100;

            /// 0b0101: Main PLLRCLK clock selected
            pub const PLLR: u32 = 0b0101;

            /// 0b0110: LSI oscillator clock selected
            pub const LSI: u32 = 0b0110;

            /// 0b1000: LSE oscillator clock selected
            pub const LSE: u32 = 0b1000;

            /// 0b1101: Main PLLPCLK clock selected
            pub const PLLP: u32 = 0b1101;

            /// 0b1110: Main PLLQCLK clock selected
            pub const PLLQ: u32 = 0b1110;
        }
    }

    /// PCLK2 prescaler flag (APB2)
    pub mod PPRE2F {
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

            /// 0b0: PCLK2 prescaler value not yet applied
            pub const NotApplied: u32 = 0b0;

            /// 0b1: PCLK2 prescaler value applied
            pub const Applied: u32 = 0b1;
        }
    }

    /// PCLK1 prescaler flag (APB1)
    pub mod PPRE1F {
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

            /// 0b0: PCLK1 prescaler value not yet applied
            pub const NotApplied: u32 = 0b0;

            /// 0b1: PCLK1 prescaler value applied
            pub const Applied: u32 = 0b1;
        }
    }

    /// HCLK1 prescaler flag (CPU1, AHB1, AHB2, and SRAM1)
    pub mod HPREF {
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

            /// 0b0: HCLK1 prescaler value not yet applied
            pub const NotApplied: u32 = 0b0;

            /// 0b1: HCLK1 prescaler value applied
            pub const Applied: u32 = 0b1;
        }
    }

    /// Wakeup from Stop and CSS backup clock selection
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

            /// 0b0: MSI oscillator selected as wakeup from stop clock and CSS backup clock
            pub const MSI: u32 = 0b0;

            /// 0b1: HSI16 oscillator selected as wakeup from stop clock and CSS backup clock
            pub const HSI16: u32 = 0b1;
        }
    }

    /// PCLK2 high-speed prescaler (APB2)
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

    /// PCLK1 low-speed prescaler (APB1)
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

    /// HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)
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

            /// 0b0001: SYSCLK divided by 3
            pub const Div3: u32 = 0b0001;

            /// 0b0010: SYSCLK divided by 5
            pub const Div5: u32 = 0b0010;

            /// 0b0101: SYSCLK divided by 6
            pub const Div6: u32 = 0b0101;

            /// 0b0110: SYSCLK divided by 10
            pub const Div10: u32 = 0b0110;

            /// 0b0111: SYSCLK divided by 32
            pub const Div32: u32 = 0b0111;

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

            /// 0b1110: SYSCLK divided by 128
            pub const Div256: u32 = 0b1110;

            /// 0b1111: SYSCLK divided by 512
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

            /// 0b01: HSI16 oscillator used as system clock
            pub const HSI16: u32 = 0b01;

            /// 0b10: HSE32 oscillator used as system clock
            pub const HSE32: u32 = 0b10;

            /// 0b11: PLLRCLK used as system clock
            pub const PLLR: u32 = 0b11;
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

/// PLL configuration register
pub mod PLLCFGR {

    /// Main PLL division factor for PLLRCLK
    pub mod PLLR {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (3 bits: 0b111 << 29)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b001: PLL = VCO/(N+1)
            pub const Div2: u32 = 0b001;

            /// 0b010: PLL = VCO/(N+1)
            pub const Div3: u32 = 0b010;

            /// 0b011: PLL = VCO/(N+1)
            pub const Div4: u32 = 0b011;

            /// 0b100: PLL = VCO/(N+1)
            pub const Div5: u32 = 0b100;

            /// 0b101: PLL = VCO/(N+1)
            pub const Div6: u32 = 0b101;

            /// 0b110: PLL = VCO/(N+1)
            pub const Div7: u32 = 0b110;

            /// 0b111: PLL = VCO/(N+1)
            pub const Div8: u32 = 0b111;
        }
    }

    /// Main PLL PLLRCLK output enable
    pub mod PLLREN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLLCLK output disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: PLLCLK output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Main PLL division factor for PLLQCLK
    pub mod PLLQ {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (3 bits: 0b111 << 25)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLLR::RW;
    }

    /// Main PLL PLLQCLK output enable
    pub mod PLLQEN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLLREN::RW;
    }

    /// Main PLL division factor for PLLPCLK.
    pub mod PLLP {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (5 bits: 0b11111 << 17)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00001: PLL = VCO/(N+1)
            pub const Div2: u32 = 0b00001;

            /// 0b00010: PLL = VCO/(N+1)
            pub const Div3: u32 = 0b00010;

            /// 0b00011: PLL = VCO/(N+1)
            pub const Div4: u32 = 0b00011;

            /// 0b00100: PLL = VCO/(N+1)
            pub const Div5: u32 = 0b00100;

            /// 0b00101: PLL = VCO/(N+1)
            pub const Div6: u32 = 0b00101;

            /// 0b00110: PLL = VCO/(N+1)
            pub const Div7: u32 = 0b00110;

            /// 0b00111: PLL = VCO/(N+1)
            pub const Div8: u32 = 0b00111;

            /// 0b01000: PLL = VCO/(N+1)
            pub const Div9: u32 = 0b01000;

            /// 0b01001: PLL = VCO/(N+1)
            pub const Div10: u32 = 0b01001;

            /// 0b01010: PLL = VCO/(N+1)
            pub const Div11: u32 = 0b01010;

            /// 0b01011: PLL = VCO/(N+1)
            pub const Div12: u32 = 0b01011;

            /// 0b01100: PLL = VCO/(N+1)
            pub const Div13: u32 = 0b01100;

            /// 0b01101: PLL = VCO/(N+1)
            pub const Div14: u32 = 0b01101;

            /// 0b01110: PLL = VCO/(N+1)
            pub const Div15: u32 = 0b01110;

            /// 0b01111: PLL = VCO/(N+1)
            pub const Div16: u32 = 0b01111;

            /// 0b10000: PLL = VCO/(N+1)
            pub const Div17: u32 = 0b10000;

            /// 0b10001: PLL = VCO/(N+1)
            pub const Div18: u32 = 0b10001;

            /// 0b10010: PLL = VCO/(N+1)
            pub const Div19: u32 = 0b10010;

            /// 0b10011: PLL = VCO/(N+1)
            pub const Div20: u32 = 0b10011;

            /// 0b10100: PLL = VCO/(N+1)
            pub const Div21: u32 = 0b10100;

            /// 0b10101: PLL = VCO/(N+1)
            pub const Div22: u32 = 0b10101;

            /// 0b10110: PLL = VCO/(N+1)
            pub const Div23: u32 = 0b10110;

            /// 0b10111: PLL = VCO/(N+1)
            pub const Div24: u32 = 0b10111;

            /// 0b11000: PLL = VCO/(N+1)
            pub const Div25: u32 = 0b11000;

            /// 0b11001: PLL = VCO/(N+1)
            pub const Div26: u32 = 0b11001;

            /// 0b11010: PLL = VCO/(N+1)
            pub const Div27: u32 = 0b11010;

            /// 0b11011: PLL = VCO/(N+1)
            pub const Div28: u32 = 0b11011;

            /// 0b11100: PLL = VCO/(N+1)
            pub const Div29: u32 = 0b11100;

            /// 0b11101: PLL = VCO/(N+1)
            pub const Div30: u32 = 0b11101;

            /// 0b11110: PLL = VCO/(N+1)
            pub const Div31: u32 = 0b11110;

            /// 0b11111: PLL = VCO/(N+1)
            pub const Div32: u32 = 0b11111;
        }
    }

    /// Main PLL PLLPCLK output enable
    pub mod PLLPEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLLREN::RW;
    }

    /// Main PLL multiplication factor for VCO
    pub mod PLLN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Division factor for the main PLL input clock
    pub mod PLLM {
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

            /// 0b000: VCO input = PLL input / PLLM
            pub const Div1: u32 = 0b000;

            /// 0b001: VCO input = PLL input / PLLM
            pub const Div2: u32 = 0b001;

            /// 0b010: VCO input = PLL input / PLLM
            pub const Div3: u32 = 0b010;

            /// 0b011: VCO input = PLL input / PLLM
            pub const Div4: u32 = 0b011;

            /// 0b100: VCO input = PLL input / PLLM
            pub const Div5: u32 = 0b100;

            /// 0b101: VCO input = PLL input / PLLM
            pub const Div6: u32 = 0b101;

            /// 0b110: VCO input = PLL input / PLLM
            pub const Div7: u32 = 0b110;

            /// 0b111: VCO input = PLL input / PLLM
            pub const Div8: u32 = 0b111;
        }
    }

    /// Main PLL entry clock source
    pub mod PLLSRC {
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

            /// 0b00: No clock sent to PLL
            pub const NoClock: u32 = 0b00;

            /// 0b01: MSI clock selected as PLL clock entry
            pub const MSI: u32 = 0b01;

            /// 0b10: HSI16 clock selected as PLL clock entry
            pub const HSI16: u32 = 0b10;

            /// 0b11: HSE32 clock selected as PLL clock entry
            pub const HSE32: u32 = 0b11;
        }
    }
}

/// Clock interrupt enable register
pub mod CIER {

    /// LSE clock security system interrupt enable
    pub mod LSECSSIE {
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

            /// 0b0: Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PLL ready interrupt enable
    pub mod PLLRDYIE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSIE::RW;
    }

    /// HSE32 ready interrupt enable
    pub mod HSERDYIE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSIE::RW;
    }

    /// HSI16 ready interrupt enable
    pub mod HSIRDYIE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSIE::RW;
    }

    /// MSI ready interrupt enable
    pub mod MSIRDYIE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSIE::RW;
    }

    /// LSE ready interrupt enable
    pub mod LSERDYIE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSIE::RW;
    }

    /// LSI ready interrupt enable
    pub mod LSIRDYIE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSIE::RW;
    }
}

/// Clock interrupt flag register
pub mod CIFR {

    /// LSE Clock security system interrupt flag
    pub mod LSECSSF {
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

            /// 0b0: Not interrupted
            pub const NotInterrupted: u32 = 0b0;

            /// 0b1: Interrupted
            pub const Interrupted: u32 = 0b1;
        }
    }

    /// HSE32 Clock security system interrupt flag
    pub mod CSSF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSF::RW;
    }

    /// PLL ready interrupt flag
    pub mod PLLRDYF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSF::RW;
    }

    /// HSE32 ready interrupt flag
    pub mod HSERDYF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSF::RW;
    }

    /// HSI16 ready interrupt flag
    pub mod HSIRDYF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSF::RW;
    }

    /// MSI ready interrupt flag
    pub mod MSIRDYF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSF::RW;
    }

    /// LSE ready interrupt flag
    pub mod LSERDYF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSF::RW;
    }

    /// LSI ready interrupt flag
    pub mod LSIRDYF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSF::RW;
    }
}

/// Clock interrupt clear register
pub mod CICR {

    /// LSE Clock security system interrupt clear
    pub mod LSECSSC {
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

            /// 0b1: Clear interrupt flag
            pub const Clear: u32 = 0b1;
        }
    }

    /// HSE32 Clock security system interrupt clear
    pub mod CSSC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSC::RW;
    }

    /// PLL ready interrupt clear
    pub mod PLLRDYC {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSC::RW;
    }

    /// HSE32 ready interrupt clear
    pub mod HSERDYC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSC::RW;
    }

    /// HSI16 ready interrupt clear
    pub mod HSIRDYC {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSC::RW;
    }

    /// MSI ready interrupt clear
    pub mod MSIRDYC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSC::RW;
    }

    /// LSE ready interrupt clear
    pub mod LSERDYC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSC::RW;
    }

    /// LSI ready interrupt clear
    pub mod LSIRDYC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSECSSC::RW;
    }
}

/// AHB1 peripheral reset register
pub mod AHB1RSTR {

    /// CRC reset
    pub mod CRCRST {
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

            /// 0b0: No effect
            pub const NoReset: u32 = 0b0;

            /// 0b1: Reset peripheral
            pub const Reset: u32 = 0b1;
        }
    }

    /// DMAMUX1 reset
    pub mod DMAMUX1RST {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRCRST::RW;
    }

    /// DMA2 reset
    pub mod DMA2RST {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRCRST::RW;
    }

    /// DMA1 reset
    pub mod DMA1RST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRCRST::RW;
    }
}

/// AHB2 peripheral reset register
pub mod AHB2RSTR {

    /// IO port H reset
    pub mod GPIOHRST {
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

            /// 0b0: No effect
            pub const NoReset: u32 = 0b0;

            /// 0b1: Reset peripheral
            pub const Reset: u32 = 0b1;
        }
    }

    /// IO port C reset
    pub mod GPIOCRST {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOHRST::RW;
    }

    /// IO port B reset
    pub mod GPIOBRST {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOHRST::RW;
    }

    /// IO port A reset
    pub mod GPIOARST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOHRST::RW;
    }
}

/// AHB3 peripheral reset register
pub mod AHB3RSTR {

    /// Flash interface reset
    pub mod FLASHRST {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoReset: u32 = 0b0;

            /// 0b1: Reset peripheral
            pub const Reset: u32 = 0b1;
        }
    }

    /// IPCCRST
    pub mod IPCCRST {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLASHRST::RW;
    }

    /// HSEMRST
    pub mod HSEMRST {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLASHRST::RW;
    }

    /// RNGRST
    pub mod RNGRST {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLASHRST::RW;
    }

    /// AESRST
    pub mod AESRST {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLASHRST::RW;
    }

    /// PKARST
    pub mod PKARST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLASHRST::RW;
    }
}

/// APB1 peripheral reset register 1
pub mod APB1RSTR1 {

    /// Low Power Timer 1 reset
    pub mod LPTIM1RST {
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

            /// 0b0: No effect
            pub const NoReset: u32 = 0b0;

            /// 0b1: Reset peripheral
            pub const Reset: u32 = 0b1;
        }
    }

    /// DAC1 reset
    pub mod DACRST {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1RST::RW;
    }

    /// I2C3 reset
    pub mod I2C3RST {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1RST::RW;
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
        pub use super::LPTIM1RST::RW;
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
        pub use super::LPTIM1RST::RW;
    }

    /// USART2 reset
    pub mod USART2RST {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1RST::RW;
    }

    /// SPI2S2 reset
    pub mod SPI2S2RST {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1RST::RW;
    }

    /// TIM2 timer reset
    pub mod TIM2RST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1RST::RW;
    }
}

/// APB1 peripheral reset register 2
pub mod APB1RSTR2 {

    /// Low-power timer 3 reset
    pub mod LPTIM3RST {
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

            /// 0b0: No effect
            pub const NoReset: u32 = 0b0;

            /// 0b1: Reset peripheral
            pub const Reset: u32 = 0b1;
        }
    }

    /// Low-power timer 2 reset
    pub mod LPTIM2RST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM3RST::RW;
    }

    /// Low-power UART 1 reset
    pub mod LPUART1RST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM3RST::RW;
    }
}

/// APB2 peripheral reset register
pub mod APB2RSTR {

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
        /// Read-write values (empty)
        pub mod RW {}
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
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SPI1 reset
    pub mod SPI1RST {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
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
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADC reset
    pub mod ADCRST {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// APB3 peripheral reset register
pub mod APB3RSTR {

    /// Sub-GHz radio SPI reset
    pub mod SUBGHZSPIRST {
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
}

/// AHB1 peripheral clock enable register
pub mod AHB1ENR {

    /// CPU1 CRC clock enable
    pub mod CRCEN {
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

            /// 0b0: Clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CPU1 DMAMUX1 clock enable
    pub mod DMAMUX1EN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRCEN::RW;
    }

    /// CPU1 DMA2 clock enable
    pub mod DMA2EN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRCEN::RW;
    }

    /// CPU1 DMA1 clock enable
    pub mod DMA1EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRCEN::RW;
    }
}

/// AHB2 peripheral clock enable register
pub mod AHB2ENR {

    /// CPU1 IO port H clock enable
    pub mod GPIOHEN {
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

            /// 0b0: Clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CPU1 IO port C clock enable
    pub mod GPIOCEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOHEN::RW;
    }

    /// CPU1 IO port B clock enable
    pub mod GPIOBEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOHEN::RW;
    }

    /// CPU1 IO port A clock enable
    pub mod GPIOAEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOHEN::RW;
    }
}

/// AHB3 peripheral clock enable register
pub mod AHB3ENR {

    /// CPU1 Flash interface clock enable
    pub mod FLASHEN {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
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

    /// IPCCEN
    pub mod IPCCEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLASHEN::RW;
    }

    /// HSEMEN
    pub mod HSEMEN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLASHEN::RW;
    }

    /// RNGEN
    pub mod RNGEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLASHEN::RW;
    }

    /// AESEN
    pub mod AESEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLASHEN::RW;
    }

    /// PKAEN
    pub mod PKAEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLASHEN::RW;
    }
}

/// APB1 peripheral clock enable register 1
pub mod APB1ENR1 {

    /// CPU1 Low power timer 1 clocks enable
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

    /// CPU1 DAC1 clock enable
    pub mod DAC1EN {
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

    /// CPU1 I2C3 clocks enable
    pub mod I2C3EN {
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

    /// CPU1 I2C2 clocks enable
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

    /// CPU1 I2C1 clocks enable
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

    /// CPU1 USART2 clock enable
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

    /// CPU1 SPI2S2 clock enable
    pub mod SPI2S2EN {
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

    /// CPU1 Window watchdog clock enable
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

    /// CPU1 RTC APB clock enable
    pub mod RTCAPBEN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// CPU1 TIM2 timer clock enable
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
}

/// APB1 peripheral clock enable register 2
pub mod APB1ENR2 {

    /// CPU1 Low power timer 3 clocks enable
    pub mod LPTIM3EN {
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

            /// 0b0: Clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CPU1 Low power timer 2 clocks enable
    pub mod LPTIM2EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM3EN::RW;
    }

    /// CPU1 Low power UART 1 clocks enable
    pub mod LPUART1EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM3EN::RW;
    }
}

/// APB2 peripheral clock enable register
pub mod APB2ENR {

    /// CPU1 TIM17 timer clock enable
    pub mod TIM17EN {
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

            /// 0b0: Clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CPU1 TIM16 timer clock enable
    pub mod TIM16EN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM17EN::RW;
    }

    /// CPU1 USART1clocks enable
    pub mod USART1EN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM17EN::RW;
    }

    /// CPU1 SPI1 clock enable
    pub mod SPI1EN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM17EN::RW;
    }

    /// CPU1 TIM1 timer clock enable
    pub mod TIM1EN {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM17EN::RW;
    }

    /// CPU1 ADC clocks enable
    pub mod ADCEN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM17EN::RW;
    }
}

/// APB3 peripheral clock enable register
pub mod APB3ENR {

    /// sub-GHz radio SPI clock enable
    pub mod SUBGHZSPIEN {
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

            /// 0b0: Clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// AHB1 peripheral clocks enable in Sleep modes register
pub mod AHB1SMENR {

    /// CRC clock enable during CPU1 CSleep mode.
    pub mod CRCSMEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMAMUX1 clock enable during CPU1 CSleep mode.
    pub mod DMAMUX1SMEN {
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

    /// DMA2 clock enable during CPU1 CSleep mode
    pub mod DMA2SMEN {
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

    /// DMA1 clock enable during CPU1 CSleep mode.
    pub mod DMA1SMEN {
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
}

/// AHB2 peripheral clocks enable in Sleep modes register
pub mod AHB2SMENR {

    /// IO port H clock enable during CPU1 CSleep mode.
    pub mod GPIOHSMEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IO port C clock enable during CPU1 CSleep mode.
    pub mod GPIOCSMEN {
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

    /// IO port B clock enable during CPU1 CSleep mode.
    pub mod GPIOBSMEN {
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

    /// IO port A clock enable during CPU1 CSleep mode.
    pub mod GPIOASMEN {
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
}

/// AHB3 peripheral clocks enable in Sleep and Stop modes register
pub mod AHB3SMENR {

    /// Flash interface clock enable during CPU1 CSleep mode.
    pub mod FLASHSMEN {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SRAM2 memory interface clock enable during CPU1 CSleep mode
    pub mod SRAM2SMEN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SRAM1 interface clock enable during CPU1 CSleep mode.
    pub mod SRAM1SMEN {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// True RNG clocks enable during CPU1 Csleep and CStop modes
    pub mod RNGSMEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AES accelerator clock enable during CPU1 CSleep mode.
    pub mod AESSMEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PKA accelerator clock enable during CPU1 CSleep mode.
    pub mod PKASMEN {
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
}

/// APB1 peripheral clocks enable in Sleep mode register 1
pub mod APB1SMENR1 {

    /// Low power timer 1 clock enable during CPU1 Csleep and CStop mode
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

    /// DAC1 clock enable during CPU1 CSleep mode.
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

    /// I2C3 clock enable during CPU1 Csleep and CStop modes
    pub mod I2C3SMEN {
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

    /// I2C2 clock enable during CPU1 Csleep and CStop modes
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

    /// I2C1 clock enable during CPU1 Csleep and CStop modes
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

    /// USART2 clock enable during CPU1 CSleep mode.
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

    /// SPI2S2 clock enable during CPU1 CSleep mode.
    pub mod SPI2S2SMEN {
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

    /// Window watchdog clocks enable during CPU1 CSleep mode.
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

    /// RTC bus clock enable during CPU1 CSleep mode.
    pub mod RTCAPBSMEN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// TIM2 timer clock enable during CPU1 CSleep mode.
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
}

/// APB1 peripheral clocks enable in Sleep mode register 2
pub mod APB1SMENR2 {

    /// Low power timer 3 clock enable during CPU1 Csleep and CStop modes
    pub mod LPTIM3SMEN {
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

            /// 0b0: Clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Low power timer 2 clock enable during CPU1 Csleep and CStop modes
    pub mod LPTIM2SMEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM3SMEN::RW;
    }

    /// Low power UART 1 clock enable during CPU1 Csleep and CStop modes.
    pub mod LPUART1SMEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM3SMEN::RW;
    }
}

/// APB2 peripheral clocks enable in Sleep mode register
pub mod APB2SMENR {

    /// TIM17 timer clock enable during CPU1 CSleep mode.
    pub mod TIM17SMEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TIM16 timer clock enable during CPU1 CSleep mode.
    pub mod TIM16SMEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USART1 clock enable during CPU1 Csleep and CStop modes.
    pub mod USART1SMEN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SPI1 clock enable during CPU1 CSleep mode.
    pub mod SPI1SMEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TIM1 timer clock enable during CPU1 CSleep mode.
    pub mod TIM1SMEN {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADC clocks enable during CPU1 Csleep and CStop modes
    pub mod ADCSMEN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// APB3 peripheral clock enable in Sleep mode register
pub mod APB3SMENR {

    /// Sub-GHz radio SPI clock enable during Sleep and Stop modes
    pub mod SUBGHZSPISMEN {
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
}

/// Peripherals independent clock configuration register
pub mod CCIPR {

    /// RNG clock source selection
    pub mod RNGSEL {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: PLLQ clock selected
            pub const PLLQ: u32 = 0b00;

            /// 0b01: LSI clock selected
            pub const LSI: u32 = 0b01;

            /// 0b10: LSE clock selected
            pub const LSE: u32 = 0b10;

            /// 0b11: MSI clock selected
            pub const MSI: u32 = 0b11;
        }
    }

    /// ADC clock source selection
    pub mod ADCSEL {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No clock selected
            pub const NoClock: u32 = 0b00;

            /// 0b01: HSI16 clock selected
            pub const HSI16: u32 = 0b01;

            /// 0b10: PLLP clock selected
            pub const PLLP: u32 = 0b10;

            /// 0b11: SYSCLK clock selected
            pub const SYSCLK: u32 = 0b11;
        }
    }

    /// Low power timer 3 clock source selection
    pub mod LPTIM3SEL {
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

            /// 0b00: PCLK clock selected
            pub const PCLK: u32 = 0b00;

            /// 0b01: LSI clock selected
            pub const LSI: u32 = 0b01;

            /// 0b10: HSI16 clock selected
            pub const HSI16: u32 = 0b10;

            /// 0b11: LSE clock selected
            pub const LSE: u32 = 0b11;
        }
    }

    /// Low power timer 2 clock source selection
    pub mod LPTIM2SEL {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM3SEL::RW;
    }

    /// Low power timer 1 clock source selection
    pub mod LPTIM1SEL {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM3SEL::RW;
    }

    /// I2C3 clock source selection
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

            /// 0b00: PCLK clock selected
            pub const PCLK: u32 = 0b00;

            /// 0b01: SYSCLK clock selected
            pub const SYSCLK: u32 = 0b01;

            /// 0b10: HSI16 clock selected
            pub const HSI16: u32 = 0b10;
        }
    }

    /// I2C2 clock source selection
    pub mod I2C2SEL {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::I2C3SEL::RW;
    }

    /// I2C1 clock source selection
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

    /// LPUART1 clock source selection
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

            /// 0b00: PCLK clock selected
            pub const PCLK: u32 = 0b00;

            /// 0b01: SYSCLK clock selected
            pub const SYSCLK: u32 = 0b01;

            /// 0b10: HSI16 clock selected
            pub const HSI16: u32 = 0b10;

            /// 0b11: LSE clock selected
            pub const LSE: u32 = 0b11;
        }
    }

    /// SPI2S2 I2S clock source selection
    pub mod SPI2S2SEL {
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

            /// 0b01: PLLQ clock selected
            pub const PLLQ: u32 = 0b01;

            /// 0b10: HSI16 clock selected
            pub const HSI16: u32 = 0b10;

            /// 0b11: External input I2S_CKIN selected
            pub const I2S: u32 = 0b11;
        }
    }

    /// USART2 clock source selection
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

    /// USART1 clock source selection
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

/// Backup domain control register
pub mod BDCR {

    /// Low speed clock output selection
    pub mod LSCOSEL {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LSI clock selected
            pub const LSI: u32 = 0b0;

            /// 0b1: LSE clock selected
            pub const LSE: u32 = 0b1;
        }
    }

    /// Low speed clock output enable
    pub mod LSCOEN {
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

            /// 0b0: LSCO disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: LSCO enabled
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
            pub const NotActive: u32 = 0b0;

            /// 0b1: Entire Backup domain reset
            pub const Reset: u32 = 0b1;
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

            /// 0b0: RTC kernel clock disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: RTC kernel clock enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LSE system clock ready
    pub mod LSESYSRDY {
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

            /// 0b0: LSE system clock not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: LSE system clock ready
            pub const Ready: u32 = 0b1;
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

            /// 0b01: LSE oscillator clock selected
            pub const LSE: u32 = 0b01;

            /// 0b10: LSI oscillator clock selected
            pub const LSI: u32 = 0b10;

            /// 0b11: HSE32 oscillator clock divided by 32 selected
            pub const HSE32: u32 = 0b11;
        }
    }

    /// LSE system clock enable
    pub mod LSESYSEN {
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

            /// 0b0: LSE system clock disabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: LSE system clock enabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CSS on LSE failure Detection
    pub mod LSECSSD {
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

            /// 0b0: No failure detected on LSE
            pub const NoFailure: u32 = 0b0;

            /// 0b1: Failure detected on LSE
            pub const Failure: u32 = 0b1;
        }
    }

    /// CSS on LSE enable
    pub mod LSECSSON {
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

            /// 0b0: CSS on LSE disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CSS on LSE enabled
            pub const Enabled: u32 = 0b1;
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

            /// 0b00: Xtal mode lower driving capability
            pub const Low: u32 = 0b00;

            /// 0b01: Xtal mode medium-low driving capability
            pub const MedLow: u32 = 0b01;

            /// 0b10: Xtal mode medium-high driving capability
            pub const MedHigh: u32 = 0b10;

            /// 0b11: Xtal mode higher driving capability
            pub const High: u32 = 0b11;
        }
    }

    /// LSE oscillator bypass
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

            /// 0b0: LSE oscillator not bypassed
            pub const Disabled: u32 = 0b0;

            /// 0b1: LSE oscillator bypassed
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LSE oscillator ready
    pub mod LSERDY {
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

            /// 0b0: LSE oscillator not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: LSE oscillator ready
            pub const Ready: u32 = 0b1;
        }
    }

    /// LSE oscillator enable
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

            /// 0b0: LSE oscillator off
            pub const Off: u32 = 0b0;

            /// 0b1: LSE oscillator on
            pub const On: u32 = 0b1;
        }
    }
}

/// Control/status register
pub mod CSR {

    /// Low-power reset flag
    pub mod LPWRRSTF {
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

            /// 0b0: No reset occurred
            pub const NoReset: u32 = 0b0;

            /// 0b1: Reset occurred
            pub const Reset: u32 = 0b1;
        }
    }

    /// Window watchdog reset flag
    pub mod WWDGRSTF {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPWRRSTF::RW;
    }

    /// Independent window watchdog reset flag
    pub mod IWDGRSTF {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPWRRSTF::RW;
    }

    /// Software reset flag
    pub mod SFTRSTF {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPWRRSTF::RW;
    }

    /// BOR flag
    pub mod BORRSTF {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPWRRSTF::RW;
    }

    /// Pin reset flag
    pub mod PINRSTF {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPWRRSTF::RW;
    }

    /// Option byte loader reset flag
    pub mod OBLRSTF {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPWRRSTF::RW;
    }

    /// Radio illegal access flag
    pub mod RFILARSTF {
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

            /// 0b0: No SUBGHZ radio illegal command occurred
            pub const NoIllegalCommand: u32 = 0b0;

            /// 0b1: SUBGHZ radio illegal command occurred
            pub const IllegalCommand: u32 = 0b1;
        }
    }

    /// Remove reset flag
    pub mod RMVF {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Reset flags reset
            pub const Clear: u32 = 0b1;
        }
    }

    /// Radio reset
    pub mod RFRST {
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

            /// 0b0: Sub-GHz radio software reset removed
            pub const Removed: u32 = 0b0;

            /// 0b1: Sub-GHz radio software reset active
            pub const Reset: u32 = 0b1;
        }
    }

    /// Radio in reset status flag
    pub mod RFRSTF {
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

            /// 0b0: Sub-GHz radio out of reset
            pub const NoReset: u32 = 0b0;

            /// 0b1: Sub-GHz radio in reset
            pub const Reset: u32 = 0b1;
        }
    }

    /// MSI clock ranges
    pub mod MSISRANGE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0100: Range 4 around 1 MHz
            pub const f_1MHz: u32 = 0b0100;

            /// 0b0101: Range 5 around 2 MHz
            pub const f_2MHz: u32 = 0b0101;

            /// 0b0110: Range 6 around 4 MHz (reset value)
            pub const f_4MHz: u32 = 0b0110;

            /// 0b0111: Range 7 around 8 MHz
            pub const f_8MHz: u32 = 0b0111;
        }
    }

    /// LSI frequency prescaler
    pub mod LSIPRE {
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

            /// 0b0: LSI clock not divided
            pub const Div1: u32 = 0b0;

            /// 0b1: LSI clock divided by 128
            pub const Div128: u32 = 0b1;
        }
    }

    /// LSI oscillator ready
    pub mod LSIRDY {
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

            /// 0b0: LSI oscillator not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: LSI oscillator ready
            pub const Ready: u32 = 0b1;
        }
    }

    /// LSI oscillator enable
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

            /// 0b0: LSI oscillator off
            pub const Off: u32 = 0b0;

            /// 0b1: LSI oscillator on
            pub const On: u32 = 0b1;
        }
    }
}

/// Extended clock recovery register
pub mod EXTCFGR {

    /// CLK2 prescaler flag (CPU2)
    pub mod C2HPREF {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HCLK3 shared prescaler flag (AHB3, Flash, and SRAM2)
    pub mod SHDHPREF {
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

            /// 0b0: HCLK3 prescaler value not yet applied
            pub const NotApplied: u32 = 0b0;

            /// 0b1: HCLK3 prescaler value applied
            pub const Applied: u32 = 0b1;
        }
    }

    /// \[dual core device only\] HCLK2 prescaler (CPU2)
    pub mod C2HPRE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HCLK3 shared prescaler (AHB3, Flash, and SRAM2)
    pub mod SHDHPRE {
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

            /// 0b0000: SYSCLK not divided
            pub const Div1: u32 = 0b0000;

            /// 0b0001: SYSCLK divided by 3
            pub const Div3: u32 = 0b0001;

            /// 0b0010: SYSCLK divided by 5
            pub const Div5: u32 = 0b0010;

            /// 0b0101: SYSCLK divided by 6
            pub const Div6: u32 = 0b0101;

            /// 0b0110: SYSCLK divided by 10
            pub const Div10: u32 = 0b0110;

            /// 0b0111: SYSCLK divided by 32
            pub const Div32: u32 = 0b0111;

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

            /// 0b1110: SYSCLK divided by 128
            pub const Div256: u32 = 0b1110;

            /// 0b1111: SYSCLK divided by 512
            pub const Div512: u32 = 0b1111;
        }
    }
}

/// CPU2 AHB1 peripheral clock enable register
pub mod C2AHB1ENR {
    pub use super::AHB1ENR::CRCEN;
    pub use super::AHB1ENR::DMA1EN;
    pub use super::AHB1ENR::DMA2EN;
    pub use super::AHB1ENR::DMAMUX1EN;
}

/// CPU2 AHB2 peripheral clock enable register
pub mod C2AHB2ENR {
    pub use super::AHB2ENR::GPIOAEN;
    pub use super::AHB2ENR::GPIOBEN;
    pub use super::AHB2ENR::GPIOCEN;
    pub use super::AHB2ENR::GPIOHEN;
}

/// CPU2 AHB3 peripheral clock enable register \[dual core device only\]
pub mod C2AHB3ENR {
    pub use super::AHB3ENR::AESEN;
    pub use super::AHB3ENR::FLASHEN;
    pub use super::AHB3ENR::HSEMEN;
    pub use super::AHB3ENR::IPCCEN;
    pub use super::AHB3ENR::PKAEN;
    pub use super::AHB3ENR::RNGEN;
}

/// CPU2 APB1 peripheral clock enable register 1 \[dual core device only\]
pub mod C2APB1ENR1 {

    /// CPU2 Low power timer 1 clocks enable
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

    /// CPU2 DAC1 clock enable
    pub mod DAC1EN {
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

    /// CPU2 I2C3 clocks enable
    pub mod I2C3EN {
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

    /// CPU2 I2C2 clocks enable
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

    /// CPU2 I2C1 clocks enable
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

    /// CPU2 USART2 clock enable
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

    /// CPU2 SPI2S2 clock enable
    pub mod SPI2S2EN {
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

    /// CPU2 RTC APB clock enable
    pub mod RTCAPBEN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1EN::RW;
    }

    /// CPU2 TIM2 timer clock enable
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
}

/// CPU2 APB1 peripheral clock enable register 2 \[dual core device only\]
pub mod C2APB1ENR2 {
    pub use super::APB1ENR2::LPTIM2EN;
    pub use super::APB1ENR2::LPTIM3EN;
    pub use super::APB1ENR2::LPUART1EN;
}

/// CPU2 APB2 peripheral clock enable register \[dual core device only\]
pub mod C2APB2ENR {
    pub use super::APB2ENR::ADCEN;
    pub use super::APB2ENR::SPI1EN;
    pub use super::APB2ENR::TIM16EN;
    pub use super::APB2ENR::TIM17EN;
    pub use super::APB2ENR::TIM1EN;
    pub use super::APB2ENR::USART1EN;
}

/// CPU2 APB3 peripheral clock enable register \[dual core device only\]
pub mod C2APB3ENR {
    pub use super::APB3ENR::SUBGHZSPIEN;
}

/// CPU2 AHB1 peripheral clocks enable in Sleep modes register \[dual core device only\]
pub mod C2AHB1SMENR {
    pub use super::AHB1SMENR::CRCSMEN;
    pub use super::AHB1SMENR::DMA1SMEN;
    pub use super::AHB1SMENR::DMA2SMEN;
    pub use super::AHB1SMENR::DMAMUX1SMEN;
}

/// CPU2 AHB2 peripheral clocks enable in Sleep modes register \[dual core device only\]
pub mod C2AHB2SMENR {
    pub use super::AHB2SMENR::GPIOASMEN;
    pub use super::AHB2SMENR::GPIOBSMEN;
    pub use super::AHB2SMENR::GPIOCSMEN;
    pub use super::AHB2SMENR::GPIOHSMEN;
}

/// CPU2 AHB3 peripheral clocks enable in Sleep mode register \[dual core device only\]
pub mod C2AHB3SMENR {
    pub use super::AHB3SMENR::AESSMEN;
    pub use super::AHB3SMENR::FLASHSMEN;
    pub use super::AHB3SMENR::PKASMEN;
    pub use super::AHB3SMENR::RNGSMEN;
    pub use super::AHB3SMENR::SRAM1SMEN;
    pub use super::AHB3SMENR::SRAM2SMEN;
}

/// CPU2 APB1 peripheral clocks enable in Sleep mode register 1 \[dual core device only\]
pub mod C2APB1SMENR1 {

    /// Low power timer 1 clock enable during CPU2 CSleep and CStop mode
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

    /// DAC1 clock enable during CPU2 CSleep mode.
    pub mod DAC1SMEN {
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

    /// I2C3 clock enable during CPU2 CSleep and CStop modes
    pub mod I2C3SMEN {
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

    /// I2C2 clock enable during CPU2 CSleep and CStop modes
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

    /// I2C1 clock enable during CPU2 CSleep and CStop modes
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

    /// USART2 clock enable during CPU2 CSleep mode.
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

    /// SPI2S2 clock enable during CPU2 CSleep mode.
    pub mod SPI2S2SMEN {
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

    /// RTC bus clock enable during CPU2 CSleep mode.
    pub mod RTCAPBSMEN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM1SMEN::RW;
    }

    /// TIM2 timer clock enable during CPU2 CSleep mode.
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
}

/// CPU2 APB1 peripheral clocks enable in Sleep mode register 2 \[dual core device only\]
pub mod C2APB1SMENR2 {
    pub use super::APB1SMENR2::LPTIM2SMEN;
    pub use super::APB1SMENR2::LPTIM3SMEN;
    pub use super::APB1SMENR2::LPUART1SMEN;
}

/// CPU2 APB2 peripheral clocks enable in Sleep mode register \[dual core device only\]
pub mod C2APB2SMENR {
    pub use super::APB2SMENR::ADCSMEN;
    pub use super::APB2SMENR::SPI1SMEN;
    pub use super::APB2SMENR::TIM16SMEN;
    pub use super::APB2SMENR::TIM17SMEN;
    pub use super::APB2SMENR::TIM1SMEN;
    pub use super::APB2SMENR::USART1SMEN;
}

/// CPU2 APB3 peripheral clock enable in Sleep mode register \[dual core device only\]
pub mod C2APB3SMENR {
    pub use super::APB3SMENR::SUBGHZSPISMEN;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Clock control register
    pub CR: RWRegister<u32>,

    /// Internal clock sources calibration register
    pub ICSCR: RWRegister<u32>,

    /// Clock configuration register
    pub CFGR: RWRegister<u32>,

    /// PLL configuration register
    pub PLLCFGR: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// Clock interrupt enable register
    pub CIER: RWRegister<u32>,

    /// Clock interrupt flag register
    pub CIFR: RORegister<u32>,

    /// Clock interrupt clear register
    pub CICR: WORegister<u32>,

    _reserved2: [u32; 1],

    /// AHB1 peripheral reset register
    pub AHB1RSTR: RWRegister<u32>,

    /// AHB2 peripheral reset register
    pub AHB2RSTR: RWRegister<u32>,

    /// AHB3 peripheral reset register
    pub AHB3RSTR: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// APB1 peripheral reset register 1
    pub APB1RSTR1: RWRegister<u32>,

    /// APB1 peripheral reset register 2
    pub APB1RSTR2: RWRegister<u32>,

    /// APB2 peripheral reset register
    pub APB2RSTR: RWRegister<u32>,

    /// APB3 peripheral reset register
    pub APB3RSTR: RWRegister<u32>,

    /// AHB1 peripheral clock enable register
    pub AHB1ENR: RWRegister<u32>,

    /// AHB2 peripheral clock enable register
    pub AHB2ENR: RWRegister<u32>,

    /// AHB3 peripheral clock enable register
    pub AHB3ENR: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// APB1 peripheral clock enable register 1
    pub APB1ENR1: RWRegister<u32>,

    /// APB1 peripheral clock enable register 2
    pub APB1ENR2: RWRegister<u32>,

    /// APB2 peripheral clock enable register
    pub APB2ENR: RWRegister<u32>,

    /// APB3 peripheral clock enable register
    pub APB3ENR: RWRegister<u32>,

    /// AHB1 peripheral clocks enable in Sleep modes register
    pub AHB1SMENR: RWRegister<u32>,

    /// AHB2 peripheral clocks enable in Sleep modes register
    pub AHB2SMENR: RWRegister<u32>,

    /// AHB3 peripheral clocks enable in Sleep and Stop modes register
    pub AHB3SMENR: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// APB1 peripheral clocks enable in Sleep mode register 1
    pub APB1SMENR1: RWRegister<u32>,

    /// APB1 peripheral clocks enable in Sleep mode register 2
    pub APB1SMENR2: RWRegister<u32>,

    /// APB2 peripheral clocks enable in Sleep mode register
    pub APB2SMENR: RWRegister<u32>,

    /// APB3 peripheral clock enable in Sleep mode register
    pub APB3SMENR: RWRegister<u32>,

    /// Peripherals independent clock configuration register
    pub CCIPR: RWRegister<u32>,

    _reserved6: [u32; 1],

    /// Backup domain control register
    pub BDCR: RWRegister<u32>,

    /// Control/status register
    pub CSR: RWRegister<u32>,

    _reserved7: [u32; 28],

    /// Extended clock recovery register
    pub EXTCFGR: RWRegister<u32>,

    _reserved8: [u32; 15],

    /// CPU2 AHB1 peripheral clock enable register
    pub C2AHB1ENR: RWRegister<u32>,

    /// CPU2 AHB2 peripheral clock enable register
    pub C2AHB2ENR: RWRegister<u32>,

    /// CPU2 AHB3 peripheral clock enable register \[dual core device only\]
    pub C2AHB3ENR: RWRegister<u32>,

    _reserved9: [u32; 1],

    /// CPU2 APB1 peripheral clock enable register 1 \[dual core device only\]
    pub C2APB1ENR1: RWRegister<u32>,

    /// CPU2 APB1 peripheral clock enable register 2 \[dual core device only\]
    pub C2APB1ENR2: RWRegister<u32>,

    /// CPU2 APB2 peripheral clock enable register \[dual core device only\]
    pub C2APB2ENR: RWRegister<u32>,

    /// CPU2 APB3 peripheral clock enable register \[dual core device only\]
    pub C2APB3ENR: RWRegister<u32>,

    /// CPU2 AHB1 peripheral clocks enable in Sleep modes register \[dual core device only\]
    pub C2AHB1SMENR: RWRegister<u32>,

    /// CPU2 AHB2 peripheral clocks enable in Sleep modes register \[dual core device only\]
    pub C2AHB2SMENR: RWRegister<u32>,

    /// CPU2 AHB3 peripheral clocks enable in Sleep mode register \[dual core device only\]
    pub C2AHB3SMENR: RWRegister<u32>,

    _reserved10: [u32; 1],

    /// CPU2 APB1 peripheral clocks enable in Sleep mode register 1 \[dual core device only\]
    pub C2APB1SMENR1: RWRegister<u32>,

    /// CPU2 APB1 peripheral clocks enable in Sleep mode register 2 \[dual core device only\]
    pub C2APB1SMENR2: RWRegister<u32>,

    /// CPU2 APB2 peripheral clocks enable in Sleep mode register \[dual core device only\]
    pub C2APB2SMENR: RWRegister<u32>,

    /// CPU2 APB3 peripheral clock enable in Sleep mode register \[dual core device only\]
    pub C2APB3SMENR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub ICSCR: u32,
    pub CFGR: u32,
    pub PLLCFGR: u32,
    pub CIER: u32,
    pub CIFR: u32,
    pub CICR: u32,
    pub AHB1RSTR: u32,
    pub AHB2RSTR: u32,
    pub AHB3RSTR: u32,
    pub APB1RSTR1: u32,
    pub APB1RSTR2: u32,
    pub APB2RSTR: u32,
    pub APB3RSTR: u32,
    pub AHB1ENR: u32,
    pub AHB2ENR: u32,
    pub AHB3ENR: u32,
    pub APB1ENR1: u32,
    pub APB1ENR2: u32,
    pub APB2ENR: u32,
    pub APB3ENR: u32,
    pub AHB1SMENR: u32,
    pub AHB2SMENR: u32,
    pub AHB3SMENR: u32,
    pub APB1SMENR1: u32,
    pub APB1SMENR2: u32,
    pub APB2SMENR: u32,
    pub APB3SMENR: u32,
    pub CCIPR: u32,
    pub BDCR: u32,
    pub CSR: u32,
    pub EXTCFGR: u32,
    pub C2AHB1ENR: u32,
    pub C2AHB2ENR: u32,
    pub C2AHB3ENR: u32,
    pub C2APB1ENR1: u32,
    pub C2APB1ENR2: u32,
    pub C2APB2ENR: u32,
    pub C2APB3ENR: u32,
    pub C2AHB1SMENR: u32,
    pub C2AHB2SMENR: u32,
    pub C2AHB3SMENR: u32,
    pub C2APB1SMENR1: u32,
    pub C2APB1SMENR2: u32,
    pub C2APB2SMENR: u32,
    pub C2APB3SMENR: u32,
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
