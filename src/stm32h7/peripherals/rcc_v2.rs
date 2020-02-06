#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Reset and clock control
//!
//! Used by: stm32h743v, stm32h753v

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// clock control register
pub mod CR {

    /// Internal high-speed clock enable
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

    /// High Speed Internal clock enable in Stop mode
    pub mod HSIKERON {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSION::RW;
    }

    /// HSI clock ready flag
    pub mod HSIRDY {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
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

    /// HSI clock divider
    pub mod HSIDIV {
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

            /// 0b00: No division
            pub const Div1: u32 = 0b00;

            /// 0b01: Division by 2
            pub const Div2: u32 = 0b01;

            /// 0b10: Division by 4
            pub const Div4: u32 = 0b10;

            /// 0b11: Division by 8
            pub const Div8: u32 = 0b11;
        }
    }

    /// HSI divider flag
    pub mod HSIDIVF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: New HSIDIV ratio has not yet propagated to hsi_ck
            pub const NotPropagated: u32 = 0b0;

            /// 0b1: HSIDIV ratio has propagated to hsi_ck
            pub const Propagated: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSI clock enable
    pub mod CSION {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSION::RW;
    }

    /// CSI clock ready flag
    pub mod CSIRDY {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        pub use super::HSIRDY::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSI clock enable in Stop mode
    pub mod CSIKERON {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSION::RW;
    }

    /// RC48 clock enable
    pub mod HSI48ON {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSION::RW;
    }

    /// RC48 clock ready flag
    pub mod HSI48RDY {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        pub use super::HSIRDY::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// D1 domain clocks ready flag
    pub mod D1CKRDY {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        pub use super::HSIRDY::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// D2 domain clocks ready flag
    pub mod D2CKRDY {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        pub use super::HSIRDY::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSE clock enable
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

    /// HSE clock ready flag
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

    /// HSE clock bypass
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

    /// HSE Clock Security System enable
    pub mod HSECSSON {
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

    /// PLL1 enable
    pub mod PLL1ON {
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

    /// PLL1 clock ready flag
    pub mod PLL1RDY {
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

    /// PLL2 enable
    pub mod PLL2ON {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSION::RW;
    }

    /// PLL2 clock ready flag
    pub mod PLL2RDY {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        pub use super::HSIRDY::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL3 enable
    pub mod PLL3ON {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HSION::RW;
    }

    /// PLL3 clock ready flag
    pub mod PLL3RDY {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        pub use super::HSIRDY::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ICSCR and HSICFGR
/// ICSCR: RCC Internal Clock Source Calibration Register
/// HSICFGR: RCC HSI configuration register
pub mod ICSCR {

    /// HSI clock calibration
    pub mod HSICAL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSI clock trimming
    pub mod HSITRIM {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (6 bits: 0x3f << 12)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSI clock calibration
    pub mod CSICAL {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (8 bits: 0xff << 18)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSI clock trimming
    pub mod CSITRIM {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (5 bits: 0b11111 << 26)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RCC Clock Recovery RC Register
pub mod CRRCR {

    /// Internal RC 48 MHz clock calibration
    pub mod HSI48CAL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RCC Clock Configuration Register
pub mod CFGR {

    /// System clock switch
    pub mod SW {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: HSI selected as system clock
            pub const HSI: u32 = 0b000;

            /// 0b001: CSI selected as system clock
            pub const CSI: u32 = 0b001;

            /// 0b010: HSE selected as system clock
            pub const HSE: u32 = 0b010;

            /// 0b011: PLL1 selected as system clock
            pub const PLL1: u32 = 0b011;
        }
    }

    /// System clock switch status
    pub mod SWS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values
        pub mod R {

            /// 0b000: HSI oscillator used as system clock
            pub const HSI: u32 = 0b000;

            /// 0b001: CSI oscillator used as system clock
            pub const CSI: u32 = 0b001;

            /// 0b010: HSE oscillator used as system clock
            pub const HSE: u32 = 0b010;

            /// 0b011: PLL1 used as system clock
            pub const PLL1: u32 = 0b011;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// System clock selection after a wake up from system Stop
    pub mod STOPWUCK {
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

            /// 0b0: HSI selected as wake up clock from system Stop
            pub const HSI: u32 = 0b0;

            /// 0b1: CSI selected as wake up clock from system Stop
            pub const CSI: u32 = 0b1;
        }
    }

    /// Kernel clock selection after a wake up from system Stop
    pub mod STOPKERWUCK {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::STOPWUCK::RW;
    }

    /// HSE division factor for RTC clock
    pub mod RTCPRE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// High Resolution Timer clock prescaler selection
    pub mod HRTIMSEL {
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

            /// 0b0: The HRTIM prescaler clock source is the same as other timers (rcc_timy_ker_ck)
            pub const TIMY_KER: u32 = 0b0;

            /// 0b1: The HRTIM prescaler clock source is the CPU clock (c_ck)
            pub const C_CK: u32 = 0b1;
        }
    }

    /// Timers clocks prescaler selection
    pub mod TIMPRE {
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

            /// 0b0: Timer kernel clock equal to 2x pclk by default
            pub const DefaultX2: u32 = 0b0;

            /// 0b1: Timer kernel clock equal to 4x pclk by default
            pub const DefaultX4: u32 = 0b1;
        }
    }

    /// MCO1 prescaler
    pub mod MCO1PRE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (4 bits: 0b1111 << 18)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Micro-controller clock output 1
    pub mod MCO1 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (3 bits: 0b111 << 22)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: HSI selected for micro-controller clock output
            pub const HSI: u32 = 0b000;

            /// 0b001: LSE selected for micro-controller clock output
            pub const LSE: u32 = 0b001;

            /// 0b010: HSE selected for micro-controller clock output
            pub const HSE: u32 = 0b010;

            /// 0b011: pll1_q selected for micro-controller clock output
            pub const PLL1_Q: u32 = 0b011;

            /// 0b100: HSI48 selected for micro-controller clock output
            pub const HSI48: u32 = 0b100;
        }
    }

    /// MCO2 prescaler
    pub mod MCO2PRE {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (4 bits: 0b1111 << 25)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Micro-controller clock output 2
    pub mod MCO2 {
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

            /// 0b000: System clock selected for micro-controller clock output
            pub const SYSCLK: u32 = 0b000;

            /// 0b001: pll2_p selected for micro-controller clock output
            pub const PLL2_P: u32 = 0b001;

            /// 0b010: HSE selected for micro-controller clock output
            pub const HSE: u32 = 0b010;

            /// 0b011: pll1_p selected for micro-controller clock output
            pub const PLL1_P: u32 = 0b011;

            /// 0b100: CSI selected for micro-controller clock output
            pub const CSI: u32 = 0b100;

            /// 0b101: LSI selected for micro-controller clock output
            pub const LSI: u32 = 0b101;
        }
    }
}

/// RCC Domain 1 Clock Configuration Register
pub mod D1CFGR {

    /// D1 domain AHB prescaler
    pub mod HPRE {
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

            /// 0b0000: sys_ck not divided
            pub const Div1: u32 = 0b0000;

            /// 0b1000: sys_ck divided by 2
            pub const Div2: u32 = 0b1000;

            /// 0b1001: sys_ck divided by 4
            pub const Div4: u32 = 0b1001;

            /// 0b1010: sys_ck divided by 8
            pub const Div8: u32 = 0b1010;

            /// 0b1011: sys_ck divided by 16
            pub const Div16: u32 = 0b1011;

            /// 0b1100: sys_ck divided by 64
            pub const Div64: u32 = 0b1100;

            /// 0b1101: sys_ck divided by 128
            pub const Div128: u32 = 0b1101;

            /// 0b1110: sys_ck divided by 256
            pub const Div256: u32 = 0b1110;

            /// 0b1111: sys_ck divided by 512
            pub const Div512: u32 = 0b1111;
        }
    }

    /// D1 domain APB3 prescaler
    pub mod D1PPRE {
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

            /// 0b000: rcc_hclk not divided
            pub const Div1: u32 = 0b000;

            /// 0b100: rcc_hclk divided by 2
            pub const Div2: u32 = 0b100;

            /// 0b101: rcc_hclk divided by 4
            pub const Div4: u32 = 0b101;

            /// 0b110: rcc_hclk divided by 8
            pub const Div8: u32 = 0b110;

            /// 0b111: rcc_hclk divided by 16
            pub const Div16: u32 = 0b111;
        }
    }

    /// D1 domain Core prescaler
    pub mod D1CPRE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HPRE::RW;
    }
}

/// RCC Domain 2 Clock Configuration Register
pub mod D2CFGR {

    /// D2 domain APB1 prescaler
    pub mod D2PPRE1 {
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

            /// 0b000: rcc_hclk not divided
            pub const Div1: u32 = 0b000;

            /// 0b100: rcc_hclk divided by 2
            pub const Div2: u32 = 0b100;

            /// 0b101: rcc_hclk divided by 4
            pub const Div4: u32 = 0b101;

            /// 0b110: rcc_hclk divided by 8
            pub const Div8: u32 = 0b110;

            /// 0b111: rcc_hclk divided by 16
            pub const Div16: u32 = 0b111;
        }
    }

    /// D2 domain APB2 prescaler
    pub mod D2PPRE2 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::D2PPRE1::RW;
    }
}

/// RCC Domain 3 Clock Configuration Register
pub mod D3CFGR {

    /// D3 domain APB4 prescaler
    pub mod D3PPRE {
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

            /// 0b000: rcc_hclk not divided
            pub const Div1: u32 = 0b000;

            /// 0b100: rcc_hclk divided by 2
            pub const Div2: u32 = 0b100;

            /// 0b101: rcc_hclk divided by 4
            pub const Div4: u32 = 0b101;

            /// 0b110: rcc_hclk divided by 8
            pub const Div8: u32 = 0b110;

            /// 0b111: rcc_hclk divided by 16
            pub const Div16: u32 = 0b111;
        }
    }
}

/// RCC PLLs Clock Source Selection Register
pub mod PLLCKSELR {

    /// DIVMx and PLLs clock source selection
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

            /// 0b00: HSI selected as PLL clock
            pub const HSI: u32 = 0b00;

            /// 0b01: CSI selected as PLL clock
            pub const CSI: u32 = 0b01;

            /// 0b10: HSE selected as PLL clock
            pub const HSE: u32 = 0b10;

            /// 0b11: No clock sent to DIVMx dividers and PLLs
            pub const None: u32 = 0b11;
        }
    }

    /// Prescaler for PLL1
    pub mod DIVM1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (6 bits: 0x3f << 4)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Prescaler for PLL2
    pub mod DIVM2 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (6 bits: 0x3f << 12)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Prescaler for PLL3
    pub mod DIVM3 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (6 bits: 0x3f << 20)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RCC PLLs Configuration Register
pub mod PLLCFGR {

    /// PLL1 fractional latch enable
    pub mod PLL1FRACEN {
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

            /// 0b0: Reset latch to tranfer FRACN to the Sigma-Delta modulator
            pub const Reset: u32 = 0b0;

            /// 0b1: Set latch to tranfer FRACN to the Sigma-Delta modulator
            pub const Set: u32 = 0b1;
        }
    }

    /// PLL1 VCO selection
    pub mod PLL1VCOSEL {
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

            /// 0b0: VCO frequency range 192 to 836 MHz
            pub const WideVCO: u32 = 0b0;

            /// 0b1: VCO frequency range 150 to 420 MHz
            pub const MediumVCO: u32 = 0b1;
        }
    }

    /// PLL1 input frequency range
    pub mod PLL1RGE {
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

            /// 0b00: Frequency is between 1 and 2 MHz
            pub const Range1: u32 = 0b00;

            /// 0b01: Frequency is between 2 and 4 MHz
            pub const Range2: u32 = 0b01;

            /// 0b10: Frequency is between 4 and 8 MHz
            pub const Range4: u32 = 0b10;

            /// 0b11: Frequency is between 8 and 16 MHz
            pub const Range8: u32 = 0b11;
        }
    }

    /// PLL2 fractional latch enable
    pub mod PLL2FRACEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLL1FRACEN::RW;
    }

    /// PLL2 VCO selection
    pub mod PLL2VCOSEL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLL1VCOSEL::RW;
    }

    /// PLL2 input frequency range
    pub mod PLL2RGE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLL1RGE::RW;
    }

    /// PLL3 fractional latch enable
    pub mod PLL3FRACEN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLL1FRACEN::RW;
    }

    /// PLL3 VCO selection
    pub mod PLL3VCOSEL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLL1VCOSEL::RW;
    }

    /// PLL3 input frequency range
    pub mod PLL3RGE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PLL1RGE::RW;
    }

    /// PLL1 DIVP divider output enable
    pub mod DIVP1EN {
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

            /// 0b0: Clock ouput is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock output is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PLL1 DIVQ divider output enable
    pub mod DIVQ1EN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIVP1EN::RW;
    }

    /// PLL1 DIVR divider output enable
    pub mod DIVR1EN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIVP1EN::RW;
    }

    /// PLL2 DIVP divider output enable
    pub mod DIVP2EN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIVP1EN::RW;
    }

    /// PLL2 DIVQ divider output enable
    pub mod DIVQ2EN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIVP1EN::RW;
    }

    /// PLL2 DIVR divider output enable
    pub mod DIVR2EN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIVP1EN::RW;
    }

    /// PLL3 DIVP divider output enable
    pub mod DIVP3EN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIVP1EN::RW;
    }

    /// PLL3 DIVQ divider output enable
    pub mod DIVQ3EN {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIVP1EN::RW;
    }

    /// PLL3 DIVR divider output enable
    pub mod DIVR3EN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIVP1EN::RW;
    }
}

/// RCC PLL1 Dividers Configuration Register
pub mod PLL1DIVR {

    /// Multiplication factor for PLL1 VCO
    pub mod DIVN1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL1 DIVP division factor
    pub mod DIVP1 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (7 bits: 0x7f << 9)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000: pll_p_ck = vco_ck
            pub const Div1: u32 = 0b0000000;

            /// 0b0000001: pll_p_ck = vco_ck / 2
            pub const Div2: u32 = 0b0000001;

            /// 0b0000011: pll_p_ck = vco_ck / 4
            pub const Div4: u32 = 0b0000011;

            /// 0b0000101: pll_p_ck = vco_ck / 6
            pub const Div6: u32 = 0b0000101;

            /// 0b0000111: pll_p_ck = vco_ck / 8
            pub const Div8: u32 = 0b0000111;

            /// 0b0001001: pll_p_ck = vco_ck / 10
            pub const Div10: u32 = 0b0001001;

            /// 0b0001011: pll_p_ck = vco_ck / 12
            pub const Div12: u32 = 0b0001011;

            /// 0b0001101: pll_p_ck = vco_ck / 14
            pub const Div14: u32 = 0b0001101;

            /// 0b0001111: pll_p_ck = vco_ck / 16
            pub const Div16: u32 = 0b0001111;

            /// 0b0010001: pll_p_ck = vco_ck / 18
            pub const Div18: u32 = 0b0010001;

            /// 0b0010011: pll_p_ck = vco_ck / 20
            pub const Div20: u32 = 0b0010011;

            /// 0b0010101: pll_p_ck = vco_ck / 22
            pub const Div22: u32 = 0b0010101;

            /// 0b0010111: pll_p_ck = vco_ck / 24
            pub const Div24: u32 = 0b0010111;

            /// 0b0011001: pll_p_ck = vco_ck / 26
            pub const Div26: u32 = 0b0011001;

            /// 0b0011011: pll_p_ck = vco_ck / 28
            pub const Div28: u32 = 0b0011011;

            /// 0b0011101: pll_p_ck = vco_ck / 30
            pub const Div30: u32 = 0b0011101;

            /// 0b0011111: pll_p_ck = vco_ck / 32
            pub const Div32: u32 = 0b0011111;

            /// 0b0100001: pll_p_ck = vco_ck / 34
            pub const Div34: u32 = 0b0100001;

            /// 0b0100011: pll_p_ck = vco_ck / 36
            pub const Div36: u32 = 0b0100011;

            /// 0b0100101: pll_p_ck = vco_ck / 38
            pub const Div38: u32 = 0b0100101;

            /// 0b0100111: pll_p_ck = vco_ck / 40
            pub const Div40: u32 = 0b0100111;

            /// 0b0101001: pll_p_ck = vco_ck / 42
            pub const Div42: u32 = 0b0101001;

            /// 0b0101011: pll_p_ck = vco_ck / 44
            pub const Div44: u32 = 0b0101011;

            /// 0b0101101: pll_p_ck = vco_ck / 46
            pub const Div46: u32 = 0b0101101;

            /// 0b0101111: pll_p_ck = vco_ck / 48
            pub const Div48: u32 = 0b0101111;

            /// 0b0110001: pll_p_ck = vco_ck / 50
            pub const Div50: u32 = 0b0110001;

            /// 0b0110011: pll_p_ck = vco_ck / 52
            pub const Div52: u32 = 0b0110011;

            /// 0b0110101: pll_p_ck = vco_ck / 54
            pub const Div54: u32 = 0b0110101;

            /// 0b0110111: pll_p_ck = vco_ck / 56
            pub const Div56: u32 = 0b0110111;

            /// 0b0111001: pll_p_ck = vco_ck / 58
            pub const Div58: u32 = 0b0111001;

            /// 0b0111011: pll_p_ck = vco_ck / 60
            pub const Div60: u32 = 0b0111011;

            /// 0b0111101: pll_p_ck = vco_ck / 62
            pub const Div62: u32 = 0b0111101;

            /// 0b0111111: pll_p_ck = vco_ck / 64
            pub const Div64: u32 = 0b0111111;

            /// 0b1000001: pll_p_ck = vco_ck / 66
            pub const Div66: u32 = 0b1000001;

            /// 0b1000011: pll_p_ck = vco_ck / 68
            pub const Div68: u32 = 0b1000011;

            /// 0b1000101: pll_p_ck = vco_ck / 70
            pub const Div70: u32 = 0b1000101;

            /// 0b1000111: pll_p_ck = vco_ck / 72
            pub const Div72: u32 = 0b1000111;

            /// 0b1001001: pll_p_ck = vco_ck / 74
            pub const Div74: u32 = 0b1001001;

            /// 0b1001011: pll_p_ck = vco_ck / 76
            pub const Div76: u32 = 0b1001011;

            /// 0b1001101: pll_p_ck = vco_ck / 78
            pub const Div78: u32 = 0b1001101;

            /// 0b1001111: pll_p_ck = vco_ck / 80
            pub const Div80: u32 = 0b1001111;

            /// 0b1010001: pll_p_ck = vco_ck / 82
            pub const Div82: u32 = 0b1010001;

            /// 0b1010011: pll_p_ck = vco_ck / 84
            pub const Div84: u32 = 0b1010011;

            /// 0b1010101: pll_p_ck = vco_ck / 86
            pub const Div86: u32 = 0b1010101;

            /// 0b1010111: pll_p_ck = vco_ck / 88
            pub const Div88: u32 = 0b1010111;

            /// 0b1011001: pll_p_ck = vco_ck / 90
            pub const Div90: u32 = 0b1011001;

            /// 0b1011011: pll_p_ck = vco_ck / 92
            pub const Div92: u32 = 0b1011011;

            /// 0b1011101: pll_p_ck = vco_ck / 94
            pub const Div94: u32 = 0b1011101;

            /// 0b1011111: pll_p_ck = vco_ck / 96
            pub const Div96: u32 = 0b1011111;

            /// 0b1100001: pll_p_ck = vco_ck / 98
            pub const Div98: u32 = 0b1100001;

            /// 0b1100011: pll_p_ck = vco_ck / 100
            pub const Div100: u32 = 0b1100011;

            /// 0b1100101: pll_p_ck = vco_ck / 102
            pub const Div102: u32 = 0b1100101;

            /// 0b1100111: pll_p_ck = vco_ck / 104
            pub const Div104: u32 = 0b1100111;

            /// 0b1101001: pll_p_ck = vco_ck / 106
            pub const Div106: u32 = 0b1101001;

            /// 0b1101011: pll_p_ck = vco_ck / 108
            pub const Div108: u32 = 0b1101011;

            /// 0b1101101: pll_p_ck = vco_ck / 110
            pub const Div110: u32 = 0b1101101;

            /// 0b1101111: pll_p_ck = vco_ck / 112
            pub const Div112: u32 = 0b1101111;

            /// 0b1110001: pll_p_ck = vco_ck / 114
            pub const Div114: u32 = 0b1110001;

            /// 0b1110011: pll_p_ck = vco_ck / 116
            pub const Div116: u32 = 0b1110011;

            /// 0b1110101: pll_p_ck = vco_ck / 118
            pub const Div118: u32 = 0b1110101;

            /// 0b1110111: pll_p_ck = vco_ck / 120
            pub const Div120: u32 = 0b1110111;

            /// 0b1111001: pll_p_ck = vco_ck / 122
            pub const Div122: u32 = 0b1111001;

            /// 0b1111011: pll_p_ck = vco_ck / 124
            pub const Div124: u32 = 0b1111011;

            /// 0b1111101: pll_p_ck = vco_ck / 126
            pub const Div126: u32 = 0b1111101;

            /// 0b1111111: pll_p_ck = vco_ck / 128
            pub const Div128: u32 = 0b1111111;
        }
    }

    /// PLL1 DIVQ division factor
    pub mod DIVQ1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (7 bits: 0x7f << 16)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL1 DIVR division factor
    pub mod DIVR1 {
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
}

/// RCC PLL1 Fractional Divider Register
pub mod PLL1FRACR {

    /// Fractional part of the multiplication factor for PLL1 VCO
    pub mod FRACN1 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (13 bits: 0x1fff << 3)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RCC PLL2 Dividers Configuration Register
pub mod PLL2DIVR {

    /// Multiplication factor for PLL1 VCO
    pub mod DIVN2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL1 DIVP division factor
    pub mod DIVP2 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (7 bits: 0x7f << 9)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL1 DIVQ division factor
    pub mod DIVQ2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (7 bits: 0x7f << 16)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL1 DIVR division factor
    pub mod DIVR2 {
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
}

/// RCC PLL2 Fractional Divider Register
pub mod PLL2FRACR {

    /// Fractional part of the multiplication factor for PLL VCO
    pub mod FRACN2 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (13 bits: 0x1fff << 3)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RCC PLL3 Dividers Configuration Register
pub mod PLL3DIVR {

    /// Multiplication factor for PLL1 VCO
    pub mod DIVN3 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL DIVP division factor
    pub mod DIVP3 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (7 bits: 0x7f << 9)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL DIVQ division factor
    pub mod DIVQ3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (7 bits: 0x7f << 16)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL DIVR division factor
    pub mod DIVR3 {
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
}

/// RCC PLL3 Fractional Divider Register
pub mod PLL3FRACR {

    /// Fractional part of the multiplication factor for PLL3 VCO
    pub mod FRACN3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (13 bits: 0x1fff << 3)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RCC Domain 1 Kernel Clock Configuration Register
pub mod D1CCIPR {

    /// FMC kernel clock source selection
    pub mod FMCSEL {
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

            /// 0b00: rcc_hclk3 selected as peripheral clock
            pub const RCC_HCLK3: u32 = 0b00;

            /// 0b01: pll1_q selected as peripheral clock
            pub const PLL1_Q: u32 = 0b01;

            /// 0b10: pll2_r selected as peripheral clock
            pub const PLL2_R: u32 = 0b10;

            /// 0b11: PER selected as peripheral clock
            pub const PER: u32 = 0b11;
        }
    }

    /// QUADSPI kernel clock source selection
    pub mod QSPISEL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FMCSEL::RW;
    }

    /// SDMMC kernel clock source selection
    pub mod SDMMCSEL {
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

            /// 0b0: pll1_q selected as peripheral clock
            pub const PLL1_Q: u32 = 0b0;

            /// 0b1: pll2_r selected as peripheral clock
            pub const PLL2_R: u32 = 0b1;
        }
    }

    /// per_ck clock source selection
    pub mod CKPERSEL {
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

            /// 0b00: HSI selected as peripheral clock
            pub const HSI: u32 = 0b00;

            /// 0b01: CSI selected as peripheral clock
            pub const CSI: u32 = 0b01;

            /// 0b10: HSE selected as peripheral clock
            pub const HSE: u32 = 0b10;
        }
    }
}

/// RCC Domain 2 Kernel Clock Configuration Register
pub mod D2CCIP1R {

    /// SAI1 and DFSDM1 kernel Aclk clock source selection
    pub mod SAI1SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: pll1_q selected as peripheral clock
            pub const PLL1_Q: u32 = 0b000;

            /// 0b001: pll2_p selected as peripheral clock
            pub const PLL2_P: u32 = 0b001;

            /// 0b010: pll3_p selected as peripheral clock
            pub const PLL3_P: u32 = 0b010;

            /// 0b011: I2S_CKIN selected as peripheral clock
            pub const I2S_CKIN: u32 = 0b011;

            /// 0b100: PER selected as peripheral clock
            pub const PER: u32 = 0b100;
        }
    }

    /// SAI2 and SAI3 kernel clock source selection
    pub mod SAI23SEL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SAI1SEL::RW;
    }

    /// SPI/I2S1,2 and 3 kernel clock source selection
    pub mod SPI123SEL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SAI1SEL::RW;
    }

    /// SPI4 and 5 kernel clock source selection
    pub mod SPI45SEL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: APB clock selected as peripheral clock
            pub const APB: u32 = 0b000;

            /// 0b001: pll2_q selected as peripheral clock
            pub const PLL2_Q: u32 = 0b001;

            /// 0b010: pll3_q selected as peripheral clock
            pub const PLL3_Q: u32 = 0b010;

            /// 0b011: hsi_ker selected as peripheral clock
            pub const HSI_KER: u32 = 0b011;

            /// 0b100: csi_ker selected as peripheral clock
            pub const CSI_KER: u32 = 0b100;

            /// 0b101: HSE selected as peripheral clock
            pub const HSE: u32 = 0b101;
        }
    }

    /// SPDIFRX kernel clock source selection
    pub mod SPDIFSEL {
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

            /// 0b00: pll1_q selected as peripheral clock
            pub const PLL1_Q: u32 = 0b00;

            /// 0b01: pll2_r selected as peripheral clock
            pub const PLL2_R: u32 = 0b01;

            /// 0b10: pll3_r selected as peripheral clock
            pub const PLL3_R: u32 = 0b10;

            /// 0b11: hsi_ker selected as peripheral clock
            pub const HSI_KER: u32 = 0b11;
        }
    }

    /// DFSDM1 kernel Clk clock source selection
    pub mod DFSDM1SEL {
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

            /// 0b0: rcc_pclk2 selected as peripheral clock
            pub const RCC_PCLK2: u32 = 0b0;

            /// 0b1: System clock selected as peripheral clock
            pub const SYS: u32 = 0b1;
        }
    }

    /// FDCAN kernel clock source selection
    pub mod FDCANSEL {
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

            /// 0b00: HSE selected as peripheral clock
            pub const HSE: u32 = 0b00;

            /// 0b01: pll1_q selected as peripheral clock
            pub const PLL1_Q: u32 = 0b01;

            /// 0b10: pll2_q selected as peripheral clock
            pub const PLL2_Q: u32 = 0b10;
        }
    }

    /// SWPMI kernel clock source selection
    pub mod SWPSEL {
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

            /// 0b0: pclk selected as peripheral clock
            pub const PCLK: u32 = 0b0;

            /// 0b1: hsi_ker selected as peripheral clock
            pub const HSI_KER: u32 = 0b1;
        }
    }
}

/// RCC Domain 2 Kernel Clock Configuration Register
pub mod D2CCIP2R {

    /// USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection
    pub mod USART234578SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: rcc_pclk1 selected as peripheral clock
            pub const RCC_PCLK1: u32 = 0b000;

            /// 0b001: pll2_q selected as peripheral clock
            pub const PLL2_Q: u32 = 0b001;

            /// 0b010: pll3_q selected as peripheral clock
            pub const PLL3_Q: u32 = 0b010;

            /// 0b011: hsi_ker selected as peripheral clock
            pub const HSI_KER: u32 = 0b011;

            /// 0b100: csi_ker selected as peripheral clock
            pub const CSI_KER: u32 = 0b100;

            /// 0b101: LSE selected as peripheral clock
            pub const LSE: u32 = 0b101;
        }
    }

    /// USART1 and 6 kernel clock source selection
    pub mod USART16SEL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: rcc_pclk2 selected as peripheral clock
            pub const RCC_PCLK2: u32 = 0b000;

            /// 0b001: pll2_q selected as peripheral clock
            pub const PLL2_Q: u32 = 0b001;

            /// 0b010: pll3_q selected as peripheral clock
            pub const PLL3_Q: u32 = 0b010;

            /// 0b011: hsi_ker selected as peripheral clock
            pub const HSI_KER: u32 = 0b011;

            /// 0b100: csi_ker selected as peripheral clock
            pub const CSI_KER: u32 = 0b100;

            /// 0b101: LSE selected as peripheral clock
            pub const LSE: u32 = 0b101;
        }
    }

    /// RNG kernel clock source selection
    pub mod RNGSEL {
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

            /// 0b00: HSI48 selected as peripheral clock
            pub const HSI48: u32 = 0b00;

            /// 0b01: pll1_q selected as peripheral clock
            pub const PLL1_Q: u32 = 0b01;

            /// 0b10: LSE selected as peripheral clock
            pub const LSE: u32 = 0b10;

            /// 0b11: LSI selected as peripheral clock
            pub const LSI: u32 = 0b11;
        }
    }

    /// I2C1,2,3 kernel clock source selection
    pub mod I2C123SEL {
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

            /// 0b00: rcc_pclk1 selected as peripheral clock
            pub const RCC_PCLK1: u32 = 0b00;

            /// 0b01: pll3_r selected as peripheral clock
            pub const PLL3_R: u32 = 0b01;

            /// 0b10: hsi_ker selected as peripheral clock
            pub const HSI_KER: u32 = 0b10;

            /// 0b11: csi_ker selected as peripheral clock
            pub const CSI_KER: u32 = 0b11;
        }
    }

    /// USBOTG 1 and 2 kernel clock source selection
    pub mod USBSEL {
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

            /// 0b00: Disable the kernel clock
            pub const DISABLE: u32 = 0b00;

            /// 0b01: pll1_q selected as peripheral clock
            pub const PLL1_Q: u32 = 0b01;

            /// 0b10: pll3_q selected as peripheral clock
            pub const PLL3_Q: u32 = 0b10;

            /// 0b11: HSI48 selected as peripheral clock
            pub const HSI48: u32 = 0b11;
        }
    }

    /// HDMI-CEC kernel clock source selection
    pub mod CECSEL {
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

            /// 0b00: LSE selected as peripheral clock
            pub const LSE: u32 = 0b00;

            /// 0b01: LSI selected as peripheral clock
            pub const LSI: u32 = 0b01;

            /// 0b10: csi_ker selected as peripheral clock
            pub const CSI_KER: u32 = 0b10;
        }
    }

    /// LPTIM1 kernel clock source selection
    pub mod LPTIM1SEL {
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

            /// 0b000: rcc_pclk1 selected as peripheral clock
            pub const RCC_PCLK1: u32 = 0b000;

            /// 0b001: pll2_p selected as peripheral clock
            pub const PLL2_P: u32 = 0b001;

            /// 0b010: pll3_r selected as peripheral clock
            pub const PLL3_R: u32 = 0b010;

            /// 0b011: LSE selected as peripheral clock
            pub const LSE: u32 = 0b011;

            /// 0b100: LSI selected as peripheral clock
            pub const LSI: u32 = 0b100;

            /// 0b101: PER selected as peripheral clock
            pub const PER: u32 = 0b101;
        }
    }
}

/// RCC Domain 3 Kernel Clock Configuration Register
pub mod D3CCIPR {

    /// LPUART1 kernel clock source selection
    pub mod LPUART1SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: rcc_pclk_d3 selected as peripheral clock
            pub const RCC_PCLK_D3: u32 = 0b000;

            /// 0b001: pll2_q selected as peripheral clock
            pub const PLL2_Q: u32 = 0b001;

            /// 0b010: pll3_q selected as peripheral clock
            pub const PLL3_Q: u32 = 0b010;

            /// 0b011: hsi_ker selected as peripheral clock
            pub const HSI_KER: u32 = 0b011;

            /// 0b100: csi_ker selected as peripheral clock
            pub const CSI_KER: u32 = 0b100;

            /// 0b101: LSE selected as peripheral clock
            pub const LSE: u32 = 0b101;
        }
    }

    /// I2C4 kernel clock source selection
    pub mod I2C4SEL {
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

            /// 0b00: rcc_pclk4 selected as peripheral clock
            pub const RCC_PCLK4: u32 = 0b00;

            /// 0b01: pll3_r selected as peripheral clock
            pub const PLL3_R: u32 = 0b01;

            /// 0b10: hsi_ker selected as peripheral clock
            pub const HSI_KER: u32 = 0b10;

            /// 0b11: csi_ker selected as peripheral clock
            pub const CSI_KER: u32 = 0b11;
        }
    }

    /// LPTIM2 kernel clock source selection
    pub mod LPTIM2SEL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (3 bits: 0b111 << 10)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: rcc_pclk4 selected as peripheral clock
            pub const RCC_PCLK4: u32 = 0b000;

            /// 0b001: pll2_p selected as peripheral clock
            pub const PLL2_P: u32 = 0b001;

            /// 0b010: pll3_r selected as peripheral clock
            pub const PLL3_R: u32 = 0b010;

            /// 0b011: LSE selected as peripheral clock
            pub const LSE: u32 = 0b011;

            /// 0b100: LSI selected as peripheral clock
            pub const LSI: u32 = 0b100;

            /// 0b101: PER selected as peripheral clock
            pub const PER: u32 = 0b101;
        }
    }

    /// LPTIM3,4,5 kernel clock source selection
    pub mod LPTIM345SEL {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPTIM2SEL::RW;
    }

    /// SAR ADC kernel clock source selection
    pub mod ADCSEL {
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

            /// 0b00: pll2_p selected as peripheral clock
            pub const PLL2_P: u32 = 0b00;

            /// 0b01: pll3_r selected as peripheral clock
            pub const PLL3_R: u32 = 0b01;

            /// 0b10: PER selected as peripheral clock
            pub const PER: u32 = 0b10;
        }
    }

    /// Sub-Block A of SAI4 kernel clock source selection
    pub mod SAI4ASEL {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (3 bits: 0b111 << 21)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: pll1_q selected as peripheral clock
            pub const PLL1_Q: u32 = 0b000;

            /// 0b001: pll2_p selected as peripheral clock
            pub const PLL2_P: u32 = 0b001;

            /// 0b010: pll3_p selected as peripheral clock
            pub const PLL3_P: u32 = 0b010;

            /// 0b011: i2s_ckin selected as peripheral clock
            pub const I2S_CKIN: u32 = 0b011;

            /// 0b100: PER selected as peripheral clock
            pub const PER: u32 = 0b100;
        }
    }

    /// Sub-Block B of SAI4 kernel clock source selection
    pub mod SAI4BSEL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SAI4ASEL::RW;
    }

    /// SPI6 kernel clock source selection
    pub mod SPI6SEL {
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

            /// 0b000: rcc_pclk4 selected as peripheral clock
            pub const RCC_PCLK4: u32 = 0b000;

            /// 0b001: pll2_q selected as peripheral clock
            pub const PLL2_Q: u32 = 0b001;

            /// 0b010: pll3_q selected as peripheral clock
            pub const PLL3_Q: u32 = 0b010;

            /// 0b011: hsi_ker selected as peripheral clock
            pub const HSI_KER: u32 = 0b011;

            /// 0b100: csi_ker selected as peripheral clock
            pub const CSI_KER: u32 = 0b100;

            /// 0b101: HSE selected as peripheral clock
            pub const HSE: u32 = 0b101;
        }
    }
}

/// RCC Clock Source Interrupt Enable Register
pub mod CIER {

    /// LSI ready Interrupt Enable
    pub mod LSIRDYIE {
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

            /// 0b0: Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LSE ready Interrupt Enable
    pub mod LSERDYIE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYIE::RW;
    }

    /// HSI ready Interrupt Enable
    pub mod HSIRDYIE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYIE::RW;
    }

    /// HSE ready Interrupt Enable
    pub mod HSERDYIE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYIE::RW;
    }

    /// CSI ready Interrupt Enable
    pub mod CSIRDYIE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYIE::RW;
    }

    /// RC48 ready Interrupt Enable
    pub mod HSI48RDYIE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYIE::RW;
    }

    /// PLL1 ready Interrupt Enable
    pub mod PLL1RDYIE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYIE::RW;
    }

    /// PLL2 ready Interrupt Enable
    pub mod PLL2RDYIE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYIE::RW;
    }

    /// PLL3 ready Interrupt Enable
    pub mod PLL3RDYIE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYIE::RW;
    }

    /// LSE clock security system Interrupt Enable
    pub mod LSECSSIE {
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
}

/// RCC Clock Source Interrupt Flag Register
pub mod CIFR {

    /// LSI ready Interrupt Flag
    pub mod LSIRDYF {
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

    /// LSE ready Interrupt Flag
    pub mod LSERDYF {
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

    /// HSI ready Interrupt Flag
    pub mod HSIRDYF {
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

    /// HSE ready Interrupt Flag
    pub mod HSERDYF {
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

    /// CSI ready Interrupt Flag
    pub mod CSIRDY {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RC48 ready Interrupt Flag
    pub mod HSI48RDYF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL1 ready Interrupt Flag
    pub mod PLL1RDYF {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL2 ready Interrupt Flag
    pub mod PLL2RDYF {
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

    /// PLL3 ready Interrupt Flag
    pub mod PLL3RDYF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSE clock security system Interrupt Flag
    pub mod LSECSSF {
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

    /// HSE clock security system Interrupt Flag
    pub mod HSECSSF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RCC Clock Source Interrupt Clear Register
pub mod CICR {

    /// LSI ready Interrupt Clear
    pub mod LSIRDYC {
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

            /// 0b1: Clear interrupt flag
            pub const Clear: u32 = 0b1;
        }
    }

    /// LSE ready Interrupt Clear
    pub mod LSERDYC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYC::RW;
    }

    /// HSI ready Interrupt Clear
    pub mod HSIRDYC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYC::RW;
    }

    /// HSE ready Interrupt Clear
    pub mod HSERDYC {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYC::RW;
    }

    /// CSI ready Interrupt Clear
    pub mod HSE_ready_Interrupt_Clear {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RC48 ready Interrupt Clear
    pub mod HSI48RDYC {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYC::RW;
    }

    /// PLL1 ready Interrupt Clear
    pub mod PLL1RDYC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYC::RW;
    }

    /// PLL2 ready Interrupt Clear
    pub mod PLL2RDYC {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYC::RW;
    }

    /// PLL3 ready Interrupt Clear
    pub mod PLL3RDYC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYC::RW;
    }

    /// LSE clock security system Interrupt Clear
    pub mod LSECSSC {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYC::RW;
    }

    /// HSE clock security system Interrupt Clear
    pub mod HSECSSC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LSIRDYC::RW;
    }
}

/// RCC Backup Domain Control Register
pub mod BDCR {

    /// LSE oscillator enabled
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

    /// LSE oscillator ready
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

            /// 0b0: LSE crystal oscillator not bypassed
            pub const NotBypassed: u32 = 0b0;

            /// 0b1: LSE crystal oscillator bypassed with external clock
            pub const Bypassed: u32 = 0b1;
        }
    }

    /// LSE oscillator driving capability
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

            /// 0b00: Lowest LSE oscillator driving capability
            pub const Lowest: u32 = 0b00;

            /// 0b01: Medium low LSE oscillator driving capability
            pub const MediumLow: u32 = 0b01;

            /// 0b10: Medium high LSE oscillator driving capability
            pub const MediumHigh: u32 = 0b10;

            /// 0b11: Highest LSE oscillator driving capability
            pub const Highest: u32 = 0b11;
        }
    }

    /// LSE clock security system enable
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

            /// 0b0: Clock security system on 32 kHz oscillator off
            pub const SecurityOff: u32 = 0b0;

            /// 0b1: Clock security system on 32 kHz oscillator on
            pub const SecurityOn: u32 = 0b1;
        }
    }

    /// LSE clock security system failure detection
    pub mod LSECSSD {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No failure detected on 32 kHz oscillator
            pub const NoFailure: u32 = 0b0;

            /// 0b1: Failure detected on 32 kHz oscillator
            pub const Failure: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
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

    /// VSwitch domain software reset
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

            /// 0b1: Resets the entire VSW domain
            pub const Reset: u32 = 0b1;
        }
    }
}

/// RCC Clock Control and Status Register
pub mod CSR {

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

            /// 0b0: LSI oscillator Off
            pub const Off: u32 = 0b0;

            /// 0b1: LSI oscillator On
            pub const On: u32 = 0b1;
        }
    }

    /// LSI oscillator ready
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
}

/// RCC AHB3 Reset Register
pub mod AHB3RSTR {

    /// MDMA block reset
    pub mod MDMARST {
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

    /// DMA2D block reset
    pub mod DMA2DRST {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMARST::RW;
    }

    /// JPGDEC block reset
    pub mod JPGDECRST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMARST::RW;
    }

    /// FMC block reset
    pub mod FMCRST {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMARST::RW;
    }

    /// QUADSPI and QUADSPI delay block reset
    pub mod QSPIRST {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMARST::RW;
    }

    /// SDMMC1 and SDMMC1 delay block reset
    pub mod SDMMC1RST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMARST::RW;
    }

    /// CPU reset
    pub mod CPURST {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMARST::RW;
    }
}

/// RCC AHB1 Peripheral Reset Register
pub mod AHB1RSTR {

    /// DMA1 block reset
    pub mod DMA1RST {
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

    /// DMA2 block reset
    pub mod DMA2RST {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1RST::RW;
    }

    /// ADC1&2 block reset
    pub mod ADC12RST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1RST::RW;
    }

    /// ETH1MAC block reset
    pub mod ETH1MACRST {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1RST::RW;
    }

    /// USB1OTG block reset
    pub mod USB1OTGRST {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1RST::RW;
    }

    /// USB2OTG block reset
    pub mod USB2OTGRST {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1RST::RW;
    }
}

/// RCC AHB2 Peripheral Reset Register
pub mod AHB2RSTR {

    /// CAMITF block reset
    pub mod CAMITFRST {
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

    /// Cryptography block reset
    pub mod CRYPTRST {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CAMITFRST::RW;
    }

    /// Hash block reset
    pub mod HASHRST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CAMITFRST::RW;
    }

    /// Random Number Generator block reset
    pub mod RNGRST {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CAMITFRST::RW;
    }

    /// SDMMC2 and SDMMC2 Delay block reset
    pub mod SDMMC2RST {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CAMITFRST::RW;
    }
}

/// RCC AHB4 Peripheral Reset Register
pub mod AHB4RSTR {

    /// GPIO block reset
    pub mod GPIOARST {
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

    /// GPIO block reset
    pub mod GPIOBRST {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// GPIO block reset
    pub mod GPIOCRST {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// GPIO block reset
    pub mod GPIODRST {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// GPIO block reset
    pub mod GPIOERST {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// GPIO block reset
    pub mod GPIOFRST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// GPIO block reset
    pub mod GPIOGRST {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// GPIO block reset
    pub mod GPIOHRST {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// GPIO block reset
    pub mod GPIOIRST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// GPIO block reset
    pub mod GPIOJRST {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// GPIO block reset
    pub mod GPIOKRST {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// CRC block reset
    pub mod CRCRST {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// BDMA block reset
    pub mod BDMARST {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// ADC3 block reset
    pub mod ADC3RST {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }

    /// HSEM block reset
    pub mod HSEMRST {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOARST::RW;
    }
}

/// RCC APB3 Peripheral Reset Register
pub mod APB3RSTR {

    /// LTDC block reset
    pub mod LTDCRST {
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

            /// 0b1: Reset the selected module
            pub const Reset: u32 = 0b1;
        }
    }
}

/// RCC APB1 Peripheral Reset Register
pub mod APB1LRSTR {

    /// TIM block reset
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

    /// TIM block reset
    pub mod TIM3RST {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// TIM block reset
    pub mod TIM4RST {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// TIM block reset
    pub mod TIM5RST {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// TIM block reset
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

    /// TIM block reset
    pub mod TIM7RST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// TIM block reset
    pub mod TIM12RST {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// TIM block reset
    pub mod TIM13RST {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// TIM block reset
    pub mod TIM14RST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// TIM block reset
    pub mod LPTIM1RST {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// SPI2 block reset
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

    /// SPI3 block reset
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

    /// SPDIFRX block reset
    pub mod SPDIFRXRST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// USART2 block reset
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

    /// USART3 block reset
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

    /// UART4 block reset
    pub mod UART4RST {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// UART5 block reset
    pub mod UART5RST {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// I2C1 block reset
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

    /// I2C2 block reset
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

    /// I2C3 block reset
    pub mod I2C3RST {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// HDMI-CEC block reset
    pub mod CECRST {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }

    /// DAC1 and 2 Blocks Reset
    pub mod DAC12RST {
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

    /// UART7 block reset
    pub mod UART7RST {
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

    /// UART8 block reset
    pub mod UART8RST {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2RST::RW;
    }
}

/// RCC APB1 Peripheral Reset Register
pub mod APB1HRSTR {

    /// Clock Recovery System reset
    pub mod CRSRST {
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

            /// 0b1: Reset the selected module
            pub const Reset: u32 = 0b1;
        }
    }

    /// SWPMI block reset
    pub mod SWPRST {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRSRST::RW;
    }

    /// OPAMP block reset
    pub mod OPAMPRST {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRSRST::RW;
    }

    /// MDIOS block reset
    pub mod MDIOSRST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRSRST::RW;
    }

    /// FDCAN block reset
    pub mod FDCANRST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRSRST::RW;
    }
}

/// RCC APB2 Peripheral Reset Register
pub mod APB2RSTR {

    /// TIM1 block reset
    pub mod TIM1RST {
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

    /// TIM8 block reset
    pub mod TIM8RST {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// USART1 block reset
    pub mod USART1RST {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// USART6 block reset
    pub mod USART6RST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// SPI1 block reset
    pub mod SPI1RST {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// SPI4 block reset
    pub mod SPI4RST {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// TIM15 block reset
    pub mod TIM15RST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// TIM16 block reset
    pub mod TIM16RST {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// TIM17 block reset
    pub mod TIM17RST {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// SPI5 block reset
    pub mod SPI5RST {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// SAI1 block reset
    pub mod SAI1RST {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// SAI2 block reset
    pub mod SAI2RST {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// SAI3 block reset
    pub mod SAI3RST {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// DFSDM1 block reset
    pub mod DFSDM1RST {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }

    /// HRTIM block reset
    pub mod HRTIMRST {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1RST::RW;
    }
}

/// RCC APB4 Peripheral Reset Register
pub mod APB4RSTR {

    /// SYSCFG block reset
    pub mod SYSCFGRST {
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

            /// 0b1: Reset the selected module
            pub const Reset: u32 = 0b1;
        }
    }

    /// LPUART1 block reset
    pub mod LPUART1RST {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGRST::RW;
    }

    /// SPI6 block reset
    pub mod SPI6RST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGRST::RW;
    }

    /// I2C4 block reset
    pub mod I2C4RST {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGRST::RW;
    }

    /// LPTIM2 block reset
    pub mod LPTIM2RST {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGRST::RW;
    }

    /// LPTIM3 block reset
    pub mod LPTIM3RST {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGRST::RW;
    }

    /// LPTIM4 block reset
    pub mod LPTIM4RST {
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

    /// LPTIM5 block reset
    pub mod LPTIM5RST {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGRST::RW;
    }

    /// COMP12 Blocks Reset
    pub mod COMP12RST {
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

    /// VREF block reset
    pub mod VREFRST {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGRST::RW;
    }

    /// SAI4 block reset
    pub mod SAI4RST {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGRST::RW;
    }
}

/// RCC Global Control Register
pub mod GCR {

    /// WWDG1 reset scope control
    pub mod WW1RSC {
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

            /// 0b0: Clear WWDG1 scope control
            pub const Clear: u32 = 0b0;

            /// 0b1: Set WWDG1 scope control
            pub const Set: u32 = 0b1;
        }
    }
}

/// RCC D3 Autonomous mode Register
pub mod D3AMR {

    /// BDMA and DMAMUX Autonomous mode enable
    pub mod BDMAAMEN {
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

            /// 0b0: Clock disabled in autonomous mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: Clock enabled in autonomous mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LPUART1 Autonomous mode enable
    pub mod LPUART1AMEN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BDMAAMEN::RW;
    }

    /// SPI6 Autonomous mode enable
    pub mod SPI6AMEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BDMAAMEN::RW;
    }

    /// I2C4 Autonomous mode enable
    pub mod I2C4AMEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BDMAAMEN::RW;
    }

    /// LPTIM2 Autonomous mode enable
    pub mod LPTIM2AMEN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BDMAAMEN::RW;
    }

    /// LPTIM3 Autonomous mode enable
    pub mod LPTIM3AMEN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BDMAAMEN::RW;
    }

    /// LPTIM4 Autonomous mode enable
    pub mod LPTIM4AMEN {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BDMAAMEN::RW;
    }

    /// LPTIM5 Autonomous mode enable
    pub mod LPTIM5AMEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BDMAAMEN::RW;
    }

    /// COMP12 Autonomous mode enable
    pub mod COMP12AMEN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BDMAAMEN::RW;
    }

    /// VREF Autonomous mode enable
    pub mod VREFAMEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BDMAAMEN::RW;
    }

    /// RTC Autonomous mode enable
    pub mod RTCAMEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BDMAAMEN::RW;
    }

    /// CRC Autonomous mode enable
    pub mod CRCAMEN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BDMAAMEN::RW;
    }

    /// SAI4 Autonomous mode enable
    pub mod SAI4AMEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BDMAAMEN::RW;
    }

    /// ADC3 Autonomous mode enable
    pub mod ADC3AMEN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BDMAAMEN::RW;
    }

    /// Backup RAM Autonomous mode enable
    pub mod BKPRAMAMEN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BDMAAMEN::RW;
    }

    /// SRAM4 Autonomous mode enable
    pub mod SRAM4AMEN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BDMAAMEN::RW;
    }
}

/// RCC Reset Status Register
pub mod RSR {

    /// Remove reset flag
    pub mod RMVF {
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

            /// 0b0: Not clearing the the reset flags
            pub const NotActive: u32 = 0b0;

            /// 0b1: Clear the reset flags
            pub const Clear: u32 = 0b1;
        }
    }

    /// CPU reset flag
    pub mod CPURSTF {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No reset occoured for block
            pub const NoResetOccoured: u32 = 0b0;

            /// 0b1: Reset occoured for block
            pub const ResetOccourred: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// D1 domain power switch reset flag
    pub mod D1RSTF {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        pub use super::CPURSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// D2 domain power switch reset flag
    pub mod D2RSTF {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        pub use super::CPURSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BOR reset flag
    pub mod BORRSTF {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        pub use super::CPURSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Pin reset flag (NRST)
    pub mod PINRSTF {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        pub use super::CPURSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// POR/PDR reset flag
    pub mod PORRSTF {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        pub use super::CPURSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// System reset from CPU reset flag
    pub mod SFTRSTF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        pub use super::CPURSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Independent Watchdog reset flag
    pub mod IWDG1RSTF {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        pub use super::CPURSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Window Watchdog reset flag
    pub mod WWDG1RSTF {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        pub use super::CPURSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reset due to illegal D1 DStandby or CPU CStop flag
    pub mod LPWRRSTF {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        pub use super::CPURSTF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RCC Reset Status Register
pub mod C1_RSR {
    pub use super::RSR::BORRSTF;
    pub use super::RSR::CPURSTF;
    pub use super::RSR::D1RSTF;
    pub use super::RSR::D2RSTF;
    pub use super::RSR::IWDG1RSTF;
    pub use super::RSR::LPWRRSTF;
    pub use super::RSR::PINRSTF;
    pub use super::RSR::PORRSTF;
    pub use super::RSR::RMVF;
    pub use super::RSR::SFTRSTF;
    pub use super::RSR::WWDG1RSTF;
}

/// RCC AHB3 Clock Register
pub mod C1_AHB3ENR {

    /// MDMA Peripheral Clock Enable
    pub mod MDMAEN {
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

    /// DMA2D Peripheral Clock Enable
    pub mod DMA2DEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMAEN::RW;
    }

    /// JPGDEC Peripheral Clock Enable
    pub mod JPGDECEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMAEN::RW;
    }

    /// FMC Peripheral Clocks Enable
    pub mod FMCEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMAEN::RW;
    }

    /// QUADSPI and QUADSPI Delay Clock Enable
    pub mod QSPIEN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMAEN::RW;
    }

    /// SDMMC1 and SDMMC1 Delay Clock Enable
    pub mod SDMMC1EN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMAEN::RW;
    }
}

/// RCC AHB3 Clock Register
pub mod AHB3ENR {
    pub use super::C1_AHB3ENR::DMA2DEN;
    pub use super::C1_AHB3ENR::FMCEN;
    pub use super::C1_AHB3ENR::JPGDECEN;
    pub use super::C1_AHB3ENR::MDMAEN;
    pub use super::C1_AHB3ENR::QSPIEN;
    pub use super::C1_AHB3ENR::SDMMC1EN;
}

/// RCC AHB1 Clock Register
pub mod AHB1ENR {

    /// DMA1 Clock Enable
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

    /// DMA2 Clock Enable
    pub mod DMA2EN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// ADC1/2 Peripheral Clocks Enable
    pub mod ADC12EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// Ethernet MAC bus interface Clock Enable
    pub mod ETH1MACEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// Ethernet Transmission Clock Enable
    pub mod ETH1TXEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// Ethernet Reception Clock Enable
    pub mod ETH1RXEN {
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

    /// Enable USB_PHY2 clocks
    pub mod USB2OTGHSULPIEN {
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

    /// USB1OTG Peripheral Clocks Enable
    pub mod USB1OTGHSEN {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// USB_PHY1 Clocks Enable
    pub mod USB1OTGHSULPIEN {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// USB2OTG Peripheral Clocks Enable
    pub mod USB2OTGHSEN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }
}

/// RCC AHB1 Clock Register
pub mod C1_AHB1ENR {

    /// DMA1 Clock Enable
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

    /// DMA2 Clock Enable
    pub mod DMA2EN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// ADC1/2 Peripheral Clocks Enable
    pub mod ADC12EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// Ethernet MAC bus interface Clock Enable
    pub mod ETH1MACEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// Ethernet Transmission Clock Enable
    pub mod ETH1TXEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// Ethernet Reception Clock Enable
    pub mod ETH1RXEN {
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

    /// USB1OTG Peripheral Clocks Enable
    pub mod USB1OTGHSEN {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// USB_PHY1 Clocks Enable
    pub mod USB1OTGHSULPIEN {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }

    /// USB2OTG Peripheral Clocks Enable
    pub mod USB2OTGHSEN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1EN::RW;
    }
}

/// RCC AHB2 Clock Register
pub mod C1_AHB2ENR {

    /// DCMI peripheral clock
    pub mod DCMIEN {
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

    /// CRYPT peripheral clock enable
    pub mod CRYPTEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCMIEN::RW;
    }

    /// HASH peripheral clock enable
    pub mod HASHEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCMIEN::RW;
    }

    /// RNG peripheral clocks enable
    pub mod RNGEN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCMIEN::RW;
    }

    /// SDMMC2 and SDMMC2 delay clock enable
    pub mod SDMMC2EN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCMIEN::RW;
    }

    /// SRAM1 block enable
    pub mod SRAM1EN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCMIEN::RW;
    }

    /// SRAM2 block enable
    pub mod SRAM2EN {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCMIEN::RW;
    }

    /// SRAM3 block enable
    pub mod SRAM3EN {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCMIEN::RW;
    }
}

/// RCC AHB2 Clock Register
pub mod AHB2ENR {
    pub use super::C1_AHB2ENR::CRYPTEN;
    pub use super::C1_AHB2ENR::DCMIEN;
    pub use super::C1_AHB2ENR::HASHEN;
    pub use super::C1_AHB2ENR::RNGEN;
    pub use super::C1_AHB2ENR::SDMMC2EN;
    pub use super::C1_AHB2ENR::SRAM1EN;
    pub use super::C1_AHB2ENR::SRAM2EN;
    pub use super::C1_AHB2ENR::SRAM3EN;
}

/// RCC AHB4 Clock Register
pub mod AHB4ENR {

    /// 0GPIO peripheral clock enable
    pub mod GPIOAEN {
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

    /// 0GPIO peripheral clock enable
    pub mod GPIOBEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// 0GPIO peripheral clock enable
    pub mod GPIOCEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// 0GPIO peripheral clock enable
    pub mod GPIODEN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// 0GPIO peripheral clock enable
    pub mod GPIOEEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// 0GPIO peripheral clock enable
    pub mod GPIOFEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// 0GPIO peripheral clock enable
    pub mod GPIOGEN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// 0GPIO peripheral clock enable
    pub mod GPIOHEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// 0GPIO peripheral clock enable
    pub mod GPIOIEN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// 0GPIO peripheral clock enable
    pub mod GPIOJEN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// 0GPIO peripheral clock enable
    pub mod GPIOKEN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// CRC peripheral clock enable
    pub mod CRCEN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// BDMA and DMAMUX2 Clock Enable
    pub mod BDMAEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// ADC3 Peripheral Clocks Enable
    pub mod ADC3EN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// HSEM peripheral clock enable
    pub mod HSEMEN {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }

    /// Backup RAM Clock Enable
    pub mod BKPRAMEN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOAEN::RW;
    }
}

/// RCC AHB4 Clock Register
pub mod C1_AHB4ENR {
    pub use super::AHB4ENR::ADC3EN;
    pub use super::AHB4ENR::BDMAEN;
    pub use super::AHB4ENR::BKPRAMEN;
    pub use super::AHB4ENR::CRCEN;
    pub use super::AHB4ENR::GPIOAEN;
    pub use super::AHB4ENR::GPIOBEN;
    pub use super::AHB4ENR::GPIOCEN;
    pub use super::AHB4ENR::GPIODEN;
    pub use super::AHB4ENR::GPIOEEN;
    pub use super::AHB4ENR::GPIOFEN;
    pub use super::AHB4ENR::GPIOGEN;
    pub use super::AHB4ENR::GPIOHEN;
    pub use super::AHB4ENR::GPIOIEN;
    pub use super::AHB4ENR::GPIOJEN;
    pub use super::AHB4ENR::GPIOKEN;
    pub use super::AHB4ENR::HSEMEN;
}

/// RCC APB3 Clock Register
pub mod C1_APB3ENR {

    /// LTDC peripheral clock enable
    pub mod LTDCEN {
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

            /// 0b0: The selected clock is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// WWDG1 Clock Enable
    pub mod WWDG1EN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LTDCEN::RW;
    }
}

/// RCC APB3 Clock Register
pub mod APB3ENR {
    pub use super::C1_APB3ENR::LTDCEN;
    pub use super::C1_APB3ENR::WWDG1EN;
}

/// RCC APB1 Clock Register
pub mod APB1LENR {

    /// TIM peripheral clock enable
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

    /// TIM peripheral clock enable
    pub mod TIM3EN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// TIM peripheral clock enable
    pub mod TIM4EN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// TIM peripheral clock enable
    pub mod TIM5EN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// TIM peripheral clock enable
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

    /// TIM peripheral clock enable
    pub mod TIM7EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// TIM peripheral clock enable
    pub mod TIM12EN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// TIM peripheral clock enable
    pub mod TIM13EN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// TIM peripheral clock enable
    pub mod TIM14EN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// LPTIM1 Peripheral Clocks Enable
    pub mod LPTIM1EN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// SPI2 Peripheral Clocks Enable
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

    /// SPI3 Peripheral Clocks Enable
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

    /// SPDIFRX Peripheral Clocks Enable
    pub mod SPDIFRXEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// USART2 Peripheral Clocks Enable
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

    /// USART3 Peripheral Clocks Enable
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

    /// UART4 Peripheral Clocks Enable
    pub mod UART4EN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// UART5 Peripheral Clocks Enable
    pub mod UART5EN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// I2C1 Peripheral Clocks Enable
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

    /// I2C2 Peripheral Clocks Enable
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

    /// I2C3 Peripheral Clocks Enable
    pub mod I2C3EN {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// HDMI-CEC peripheral clock enable
    pub mod CECEN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }

    /// DAC1&2 peripheral clock enable
    pub mod DAC12EN {
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

    /// UART7 Peripheral Clocks Enable
    pub mod UART7EN {
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

    /// UART8 Peripheral Clocks Enable
    pub mod UART8EN {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2EN::RW;
    }
}

/// RCC APB1 Clock Register
pub mod C1_APB1LENR {
    pub use super::APB1LENR::CECEN;
    pub use super::APB1LENR::DAC12EN;
    pub use super::APB1LENR::I2C1EN;
    pub use super::APB1LENR::I2C2EN;
    pub use super::APB1LENR::I2C3EN;
    pub use super::APB1LENR::LPTIM1EN;
    pub use super::APB1LENR::SPDIFRXEN;
    pub use super::APB1LENR::SPI2EN;
    pub use super::APB1LENR::SPI3EN;
    pub use super::APB1LENR::TIM12EN;
    pub use super::APB1LENR::TIM13EN;
    pub use super::APB1LENR::TIM14EN;
    pub use super::APB1LENR::TIM2EN;
    pub use super::APB1LENR::TIM3EN;
    pub use super::APB1LENR::TIM4EN;
    pub use super::APB1LENR::TIM5EN;
    pub use super::APB1LENR::TIM6EN;
    pub use super::APB1LENR::TIM7EN;
    pub use super::APB1LENR::UART4EN;
    pub use super::APB1LENR::UART5EN;
    pub use super::APB1LENR::UART7EN;
    pub use super::APB1LENR::UART8EN;
    pub use super::APB1LENR::USART2EN;
    pub use super::APB1LENR::USART3EN;
}

/// RCC APB1 Clock Register
pub mod APB1HENR {

    /// Clock Recovery System peripheral clock enable
    pub mod CRSEN {
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

            /// 0b0: The selected clock is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SWPMI Peripheral Clocks Enable
    pub mod SWPEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRSEN::RW;
    }

    /// OPAMP peripheral clock enable
    pub mod OPAMPEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRSEN::RW;
    }

    /// MDIOS peripheral clock enable
    pub mod MDIOSEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRSEN::RW;
    }

    /// FDCAN Peripheral Clocks Enable
    pub mod FDCANEN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRSEN::RW;
    }
}

/// RCC APB1 Clock Register
pub mod C1_APB1HENR {
    pub use super::APB1HENR::CRSEN;
    pub use super::APB1HENR::FDCANEN;
    pub use super::APB1HENR::MDIOSEN;
    pub use super::APB1HENR::OPAMPEN;
    pub use super::APB1HENR::SWPEN;
}

/// RCC APB2 Clock Register
pub mod C1_APB2ENR {

    /// TIM1 peripheral clock enable
    pub mod TIM1EN {
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

    /// TIM8 peripheral clock enable
    pub mod TIM8EN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// USART1 Peripheral Clocks Enable
    pub mod USART1EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// USART6 Peripheral Clocks Enable
    pub mod USART6EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// SPI1 Peripheral Clocks Enable
    pub mod SPI1EN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// SPI4 Peripheral Clocks Enable
    pub mod SPI4EN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// TIM16 peripheral clock enable
    pub mod TIM16EN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// TIM15 peripheral clock enable
    pub mod TIM15EN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// TIM17 peripheral clock enable
    pub mod TIM17EN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// SPI5 Peripheral Clocks Enable
    pub mod SPI5EN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// SAI1 Peripheral Clocks Enable
    pub mod SAI1EN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// SAI2 Peripheral Clocks Enable
    pub mod SAI2EN {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// SAI3 Peripheral Clocks Enable
    pub mod SAI3EN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// DFSDM1 Peripheral Clocks Enable
    pub mod DFSDM1EN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }

    /// HRTIM peripheral clock enable
    pub mod HRTIMEN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1EN::RW;
    }
}

/// RCC APB2 Clock Register
pub mod APB2ENR {
    pub use super::C1_APB2ENR::DFSDM1EN;
    pub use super::C1_APB2ENR::HRTIMEN;
    pub use super::C1_APB2ENR::SAI1EN;
    pub use super::C1_APB2ENR::SAI2EN;
    pub use super::C1_APB2ENR::SAI3EN;
    pub use super::C1_APB2ENR::SPI1EN;
    pub use super::C1_APB2ENR::SPI4EN;
    pub use super::C1_APB2ENR::SPI5EN;
    pub use super::C1_APB2ENR::TIM15EN;
    pub use super::C1_APB2ENR::TIM16EN;
    pub use super::C1_APB2ENR::TIM17EN;
    pub use super::C1_APB2ENR::TIM1EN;
    pub use super::C1_APB2ENR::TIM8EN;
    pub use super::C1_APB2ENR::USART1EN;
    pub use super::C1_APB2ENR::USART6EN;
}

/// RCC APB4 Clock Register
pub mod APB4ENR {

    /// SYSCFG peripheral clock enable
    pub mod SYSCFGEN {
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

            /// 0b0: The selected clock is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LPUART1 Peripheral Clocks Enable
    pub mod LPUART1EN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGEN::RW;
    }

    /// SPI6 Peripheral Clocks Enable
    pub mod SPI6EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGEN::RW;
    }

    /// I2C4 Peripheral Clocks Enable
    pub mod I2C4EN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGEN::RW;
    }

    /// LPTIM2 Peripheral Clocks Enable
    pub mod LPTIM2EN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGEN::RW;
    }

    /// LPTIM3 Peripheral Clocks Enable
    pub mod LPTIM3EN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGEN::RW;
    }

    /// LPTIM4 Peripheral Clocks Enable
    pub mod LPTIM4EN {
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

    /// LPTIM5 Peripheral Clocks Enable
    pub mod LPTIM5EN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGEN::RW;
    }

    /// COMP1/2 peripheral clock enable
    pub mod COMP12EN {
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

    /// VREF peripheral clock enable
    pub mod VREFEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGEN::RW;
    }

    /// RTC APB Clock Enable
    pub mod RTCAPBEN {
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

    /// SAI4 Peripheral Clocks Enable
    pub mod SAI4EN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGEN::RW;
    }
}

/// RCC APB4 Clock Register
pub mod C1_APB4ENR {
    pub use super::APB4ENR::COMP12EN;
    pub use super::APB4ENR::I2C4EN;
    pub use super::APB4ENR::LPTIM2EN;
    pub use super::APB4ENR::LPTIM3EN;
    pub use super::APB4ENR::LPTIM4EN;
    pub use super::APB4ENR::LPTIM5EN;
    pub use super::APB4ENR::LPUART1EN;
    pub use super::APB4ENR::RTCAPBEN;
    pub use super::APB4ENR::SAI4EN;
    pub use super::APB4ENR::SPI6EN;
    pub use super::APB4ENR::SYSCFGEN;
    pub use super::APB4ENR::VREFEN;
}

/// RCC AHB3 Sleep Clock Register
pub mod C1_AHB3LPENR {

    /// MDMA Clock Enable During CSleep Mode
    pub mod MDMALPEN {
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

            /// 0b0: The selected clock is disabled during csleep mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled during csleep mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMA2D Clock Enable During CSleep Mode
    pub mod DMA2DLPEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// JPGDEC Clock Enable During CSleep Mode
    pub mod JPGDECLPEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// Flash interface clock enable during csleep mode
    pub mod FLASHPREN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FMC Peripheral Clocks Enable During CSleep Mode
    pub mod FMCLPEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode
    pub mod QSPILPEN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode
    pub mod SDMMC1LPEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// D1DTCM1 Block Clock Enable During CSleep mode
    pub mod D1DTCM1LPEN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// D1 DTCM2 Block Clock Enable During CSleep mode
    pub mod DTCM2LPEN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// D1ITCM Block Clock Enable During CSleep mode
    pub mod ITCMLPEN {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// AXISRAM Block Clock Enable During CSleep mode
    pub mod AXISRAMLPEN {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }
}

/// RCC AHB3 Sleep Clock Register
pub mod AHB3LPENR {

    /// MDMA Clock Enable During CSleep Mode
    pub mod MDMALPEN {
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

            /// 0b0: The selected clock is disabled during csleep mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled during csleep mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMA2D Clock Enable During CSleep Mode
    pub mod DMA2DLPEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// JPGDEC Clock Enable During CSleep Mode
    pub mod JPGDECLPEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// FLITF Clock Enable During CSleep Mode
    pub mod FLASHLPEN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// FMC Peripheral Clocks Enable During CSleep Mode
    pub mod FMCLPEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode
    pub mod QSPILPEN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode
    pub mod SDMMC1LPEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// D1DTCM1 Block Clock Enable During CSleep mode
    pub mod D1DTCM1LPEN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// D1 DTCM2 Block Clock Enable During CSleep mode
    pub mod DTCM2LPEN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// D1ITCM Block Clock Enable During CSleep mode
    pub mod ITCMLPEN {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }

    /// AXISRAM Block Clock Enable During CSleep mode
    pub mod AXISRAMLPEN {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MDMALPEN::RW;
    }
}

/// RCC AHB1 Sleep Clock Register
pub mod AHB1LPENR {

    /// DMA1 Clock Enable During CSleep Mode
    pub mod DMA1LPEN {
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

            /// 0b0: The selected clock is disabled during csleep mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled during csleep mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMA2 Clock Enable During CSleep Mode
    pub mod DMA2LPEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1LPEN::RW;
    }

    /// ADC1/2 Peripheral Clocks Enable During CSleep Mode
    pub mod ADC12LPEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1LPEN::RW;
    }

    /// Ethernet MAC bus interface Clock Enable During CSleep Mode
    pub mod ETH1MACLPEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1LPEN::RW;
    }

    /// Ethernet Transmission Clock Enable During CSleep Mode
    pub mod ETH1TXLPEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1LPEN::RW;
    }

    /// Ethernet Reception Clock Enable During CSleep Mode
    pub mod ETH1RXLPEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1LPEN::RW;
    }

    /// USB1OTG peripheral clock enable during CSleep mode
    pub mod USB1OTGHSLPEN {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1LPEN::RW;
    }

    /// USB_PHY1 clock enable during CSleep mode
    pub mod USB1OTGHSULPILPEN {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1LPEN::RW;
    }

    /// USB2OTG peripheral clock enable during CSleep mode
    pub mod USB2OTGHSLPEN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1LPEN::RW;
    }

    /// USB_PHY2 clocks enable during CSleep mode
    pub mod USB2OTGHSULPILPEN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMA1LPEN::RW;
    }
}

/// RCC AHB1 Sleep Clock Register
pub mod C1_AHB1LPENR {
    pub use super::AHB1LPENR::ADC12LPEN;
    pub use super::AHB1LPENR::DMA1LPEN;
    pub use super::AHB1LPENR::DMA2LPEN;
    pub use super::AHB1LPENR::ETH1MACLPEN;
    pub use super::AHB1LPENR::ETH1RXLPEN;
    pub use super::AHB1LPENR::ETH1TXLPEN;
    pub use super::AHB1LPENR::USB1OTGHSLPEN;
    pub use super::AHB1LPENR::USB1OTGHSULPILPEN;
    pub use super::AHB1LPENR::USB2OTGHSLPEN;
    pub use super::AHB1LPENR::USB2OTGHSULPILPEN;
}

/// RCC AHB2 Sleep Clock Register
pub mod C1_AHB2LPENR {

    /// DCMI peripheral clock enable during csleep mode
    pub mod DCMILPEN {
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

            /// 0b0: The selected clock is disabled during csleep mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled during csleep mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CRYPT peripheral clock enable during CSleep mode
    pub mod CRYPTLPEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCMILPEN::RW;
    }

    /// HASH peripheral clock enable during CSleep mode
    pub mod HASHLPEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCMILPEN::RW;
    }

    /// SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode
    pub mod SDMMC2LPEN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCMILPEN::RW;
    }

    /// RNG peripheral clock enable during CSleep mode
    pub mod RNGLPEN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCMILPEN::RW;
    }

    /// SRAM1 Clock Enable During CSleep Mode
    pub mod SRAM1LPEN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCMILPEN::RW;
    }

    /// SRAM2 Clock Enable During CSleep Mode
    pub mod SRAM2LPEN {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCMILPEN::RW;
    }

    /// SRAM3 Clock Enable During CSleep Mode
    pub mod SRAM3LPEN {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCMILPEN::RW;
    }
}

/// RCC AHB2 Sleep Clock Register
pub mod AHB2LPENR {
    pub use super::C1_AHB2LPENR::CRYPTLPEN;
    pub use super::C1_AHB2LPENR::DCMILPEN;
    pub use super::C1_AHB2LPENR::HASHLPEN;
    pub use super::C1_AHB2LPENR::RNGLPEN;
    pub use super::C1_AHB2LPENR::SDMMC2LPEN;
    pub use super::C1_AHB2LPENR::SRAM1LPEN;
    pub use super::C1_AHB2LPENR::SRAM2LPEN;
    pub use super::C1_AHB2LPENR::SRAM3LPEN;
}

/// RCC AHB4 Sleep Clock Register
pub mod AHB4LPENR {

    /// GPIO peripheral clock enable during CSleep mode
    pub mod GPIOALPEN {
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

            /// 0b0: The selected clock is disabled during csleep mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled during csleep mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// GPIO peripheral clock enable during CSleep mode
    pub mod GPIOBLPEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOALPEN::RW;
    }

    /// GPIO peripheral clock enable during CSleep mode
    pub mod GPIOCLPEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOALPEN::RW;
    }

    /// GPIO peripheral clock enable during CSleep mode
    pub mod GPIODLPEN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOALPEN::RW;
    }

    /// GPIO peripheral clock enable during CSleep mode
    pub mod GPIOELPEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOALPEN::RW;
    }

    /// GPIO peripheral clock enable during CSleep mode
    pub mod GPIOFLPEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOALPEN::RW;
    }

    /// GPIO peripheral clock enable during CSleep mode
    pub mod GPIOGLPEN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOALPEN::RW;
    }

    /// GPIO peripheral clock enable during CSleep mode
    pub mod GPIOHLPEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOALPEN::RW;
    }

    /// GPIO peripheral clock enable during CSleep mode
    pub mod GPIOILPEN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOALPEN::RW;
    }

    /// GPIO peripheral clock enable during CSleep mode
    pub mod GPIOJLPEN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOALPEN::RW;
    }

    /// GPIO peripheral clock enable during CSleep mode
    pub mod GPIOKLPEN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOALPEN::RW;
    }

    /// CRC peripheral clock enable during CSleep mode
    pub mod CRCLPEN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOALPEN::RW;
    }

    /// BDMA Clock Enable During CSleep Mode
    pub mod BDMALPEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOALPEN::RW;
    }

    /// ADC3 Peripheral Clocks Enable During CSleep Mode
    pub mod ADC3LPEN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOALPEN::RW;
    }

    /// Backup RAM Clock Enable During CSleep Mode
    pub mod BKPRAMLPEN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOALPEN::RW;
    }

    /// SRAM4 Clock Enable During CSleep Mode
    pub mod SRAM4LPEN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPIOALPEN::RW;
    }
}

/// RCC AHB4 Sleep Clock Register
pub mod C1_AHB4LPENR {
    pub use super::AHB4LPENR::ADC3LPEN;
    pub use super::AHB4LPENR::BDMALPEN;
    pub use super::AHB4LPENR::BKPRAMLPEN;
    pub use super::AHB4LPENR::CRCLPEN;
    pub use super::AHB4LPENR::GPIOALPEN;
    pub use super::AHB4LPENR::GPIOBLPEN;
    pub use super::AHB4LPENR::GPIOCLPEN;
    pub use super::AHB4LPENR::GPIODLPEN;
    pub use super::AHB4LPENR::GPIOELPEN;
    pub use super::AHB4LPENR::GPIOFLPEN;
    pub use super::AHB4LPENR::GPIOGLPEN;
    pub use super::AHB4LPENR::GPIOHLPEN;
    pub use super::AHB4LPENR::GPIOILPEN;
    pub use super::AHB4LPENR::GPIOJLPEN;
    pub use super::AHB4LPENR::GPIOKLPEN;
    pub use super::AHB4LPENR::SRAM4LPEN;
}

/// RCC APB3 Sleep Clock Register
pub mod C1_APB3LPENR {

    /// LTDC peripheral clock enable during CSleep mode
    pub mod LTDCLPEN {
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

            /// 0b0: The selected clock is disabled during csleep mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled during csleep mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// WWDG1 Clock Enable During CSleep Mode
    pub mod WWDG1LPEN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LTDCLPEN::RW;
    }
}

/// RCC APB3 Sleep Clock Register
pub mod APB3LPENR {
    pub use super::C1_APB3LPENR::LTDCLPEN;
    pub use super::C1_APB3LPENR::WWDG1LPEN;
}

/// RCC APB1 Low Sleep Clock Register
pub mod APB1LLPENR {

    /// TIM2 peripheral clock enable during CSleep mode
    pub mod TIM2LPEN {
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

            /// 0b0: The selected clock is disabled during csleep mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled during csleep mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TIM3 peripheral clock enable during CSleep mode
    pub mod TIM3LPEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// TIM4 peripheral clock enable during CSleep mode
    pub mod TIM4LPEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// TIM5 peripheral clock enable during CSleep mode
    pub mod TIM5LPEN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// TIM6 peripheral clock enable during CSleep mode
    pub mod TIM6LPEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// TIM7 peripheral clock enable during CSleep mode
    pub mod TIM7LPEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// TIM12 peripheral clock enable during CSleep mode
    pub mod TIM12LPEN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// TIM13 peripheral clock enable during CSleep mode
    pub mod TIM13LPEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// TIM14 peripheral clock enable during CSleep mode
    pub mod TIM14LPEN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// LPTIM1 Peripheral Clocks Enable During CSleep Mode
    pub mod LPTIM1LPEN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// SPI2 Peripheral Clocks Enable During CSleep Mode
    pub mod SPI2LPEN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// SPI3 Peripheral Clocks Enable During CSleep Mode
    pub mod SPI3LPEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// SPDIFRX Peripheral Clocks Enable During CSleep Mode
    pub mod SPDIFRXLPEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// USART2 Peripheral Clocks Enable During CSleep Mode
    pub mod USART2LPEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// USART3 Peripheral Clocks Enable During CSleep Mode
    pub mod USART3LPEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// UART4 Peripheral Clocks Enable During CSleep Mode
    pub mod UART4LPEN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// UART5 Peripheral Clocks Enable During CSleep Mode
    pub mod UART5LPEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// I2C1 Peripheral Clocks Enable During CSleep Mode
    pub mod I2C1LPEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// I2C2 Peripheral Clocks Enable During CSleep Mode
    pub mod I2C2LPEN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// I2C3 Peripheral Clocks Enable During CSleep Mode
    pub mod I2C3LPEN {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// HDMI-CEC Peripheral Clocks Enable During CSleep Mode
    pub mod CECLPEN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// DAC1/2 peripheral clock enable during CSleep mode
    pub mod DAC12LPEN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// USART7 Peripheral Clocks Enable During CSleep Mode
    pub mod USART7LPEN {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }

    /// USART8 Peripheral Clocks Enable During CSleep Mode
    pub mod USART8LPEN {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM2LPEN::RW;
    }
}

/// RCC APB1 Low Sleep Clock Register
pub mod C1_APB1LLPENR {
    pub use super::APB1LLPENR::CECLPEN;
    pub use super::APB1LLPENR::DAC12LPEN;
    pub use super::APB1LLPENR::I2C1LPEN;
    pub use super::APB1LLPENR::I2C2LPEN;
    pub use super::APB1LLPENR::I2C3LPEN;
    pub use super::APB1LLPENR::LPTIM1LPEN;
    pub use super::APB1LLPENR::SPDIFRXLPEN;
    pub use super::APB1LLPENR::SPI2LPEN;
    pub use super::APB1LLPENR::SPI3LPEN;
    pub use super::APB1LLPENR::TIM12LPEN;
    pub use super::APB1LLPENR::TIM13LPEN;
    pub use super::APB1LLPENR::TIM14LPEN;
    pub use super::APB1LLPENR::TIM2LPEN;
    pub use super::APB1LLPENR::TIM3LPEN;
    pub use super::APB1LLPENR::TIM4LPEN;
    pub use super::APB1LLPENR::TIM5LPEN;
    pub use super::APB1LLPENR::TIM6LPEN;
    pub use super::APB1LLPENR::TIM7LPEN;
    pub use super::APB1LLPENR::UART4LPEN;
    pub use super::APB1LLPENR::UART5LPEN;
    pub use super::APB1LLPENR::USART2LPEN;
    pub use super::APB1LLPENR::USART3LPEN;
    pub use super::APB1LLPENR::USART7LPEN;
    pub use super::APB1LLPENR::USART8LPEN;
}

/// RCC APB1 High Sleep Clock Register
pub mod C1_APB1HLPENR {

    /// Clock Recovery System peripheral clock enable during CSleep mode
    pub mod CRSLPEN {
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

            /// 0b0: The selected clock is disabled during csleep mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled during csleep mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SWPMI Peripheral Clocks Enable During CSleep Mode
    pub mod SWPLPEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRSLPEN::RW;
    }

    /// OPAMP peripheral clock enable during CSleep mode
    pub mod OPAMPLPEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRSLPEN::RW;
    }

    /// MDIOS peripheral clock enable during CSleep mode
    pub mod MDIOSLPEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRSLPEN::RW;
    }

    /// FDCAN Peripheral Clocks Enable During CSleep Mode
    pub mod FDCANLPEN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CRSLPEN::RW;
    }
}

/// RCC APB1 High Sleep Clock Register
pub mod APB1HLPENR {
    pub use super::C1_APB1HLPENR::CRSLPEN;
    pub use super::C1_APB1HLPENR::FDCANLPEN;
    pub use super::C1_APB1HLPENR::MDIOSLPEN;
    pub use super::C1_APB1HLPENR::OPAMPLPEN;
    pub use super::C1_APB1HLPENR::SWPLPEN;
}

/// RCC APB2 Sleep Clock Register
pub mod APB2LPENR {

    /// TIM1 peripheral clock enable during CSleep mode
    pub mod TIM1LPEN {
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

            /// 0b0: The selected clock is disabled during csleep mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled during csleep mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TIM8 peripheral clock enable during CSleep mode
    pub mod TIM8LPEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1LPEN::RW;
    }

    /// USART1 Peripheral Clocks Enable During CSleep Mode
    pub mod USART1LPEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1LPEN::RW;
    }

    /// USART6 Peripheral Clocks Enable During CSleep Mode
    pub mod USART6LPEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1LPEN::RW;
    }

    /// SPI1 Peripheral Clocks Enable During CSleep Mode
    pub mod SPI1LPEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1LPEN::RW;
    }

    /// SPI4 Peripheral Clocks Enable During CSleep Mode
    pub mod SPI4LPEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1LPEN::RW;
    }

    /// TIM15 peripheral clock enable during CSleep mode
    pub mod TIM15LPEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1LPEN::RW;
    }

    /// TIM16 peripheral clock enable during CSleep mode
    pub mod TIM16LPEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1LPEN::RW;
    }

    /// TIM17 peripheral clock enable during CSleep mode
    pub mod TIM17LPEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1LPEN::RW;
    }

    /// SPI5 Peripheral Clocks Enable During CSleep Mode
    pub mod SPI5LPEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1LPEN::RW;
    }

    /// SAI1 Peripheral Clocks Enable During CSleep Mode
    pub mod SAI1LPEN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1LPEN::RW;
    }

    /// SAI2 Peripheral Clocks Enable During CSleep Mode
    pub mod SAI2LPEN {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1LPEN::RW;
    }

    /// SAI3 Peripheral Clocks Enable During CSleep Mode
    pub mod SAI3LPEN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1LPEN::RW;
    }

    /// DFSDM1 Peripheral Clocks Enable During CSleep Mode
    pub mod DFSDM1LPEN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1LPEN::RW;
    }

    /// HRTIM peripheral clock enable during CSleep mode
    pub mod HRTIMLPEN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIM1LPEN::RW;
    }
}

/// RCC APB2 Sleep Clock Register
pub mod C1_APB2LPENR {
    pub use super::APB2LPENR::DFSDM1LPEN;
    pub use super::APB2LPENR::HRTIMLPEN;
    pub use super::APB2LPENR::SAI1LPEN;
    pub use super::APB2LPENR::SAI2LPEN;
    pub use super::APB2LPENR::SAI3LPEN;
    pub use super::APB2LPENR::SPI1LPEN;
    pub use super::APB2LPENR::SPI4LPEN;
    pub use super::APB2LPENR::SPI5LPEN;
    pub use super::APB2LPENR::TIM15LPEN;
    pub use super::APB2LPENR::TIM16LPEN;
    pub use super::APB2LPENR::TIM17LPEN;
    pub use super::APB2LPENR::TIM1LPEN;
    pub use super::APB2LPENR::TIM8LPEN;
    pub use super::APB2LPENR::USART1LPEN;
    pub use super::APB2LPENR::USART6LPEN;
}

/// RCC APB4 Sleep Clock Register
pub mod C1_APB4LPENR {

    /// SYSCFG peripheral clock enable during CSleep mode
    pub mod SYSCFGLPEN {
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

            /// 0b0: The selected clock is disabled during csleep mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: The selected clock is enabled during csleep mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LPUART1 Peripheral Clocks Enable During CSleep Mode
    pub mod LPUART1LPEN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGLPEN::RW;
    }

    /// SPI6 Peripheral Clocks Enable During CSleep Mode
    pub mod SPI6LPEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGLPEN::RW;
    }

    /// I2C4 Peripheral Clocks Enable During CSleep Mode
    pub mod I2C4LPEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGLPEN::RW;
    }

    /// LPTIM2 Peripheral Clocks Enable During CSleep Mode
    pub mod LPTIM2LPEN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGLPEN::RW;
    }

    /// LPTIM3 Peripheral Clocks Enable During CSleep Mode
    pub mod LPTIM3LPEN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGLPEN::RW;
    }

    /// LPTIM4 Peripheral Clocks Enable During CSleep Mode
    pub mod LPTIM4LPEN {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGLPEN::RW;
    }

    /// LPTIM5 Peripheral Clocks Enable During CSleep Mode
    pub mod LPTIM5LPEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGLPEN::RW;
    }

    /// COMP1/2 peripheral clock enable during CSleep mode
    pub mod COMP12LPEN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGLPEN::RW;
    }

    /// VREF peripheral clock enable during CSleep mode
    pub mod VREFLPEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGLPEN::RW;
    }

    /// RTC APB Clock Enable During CSleep Mode
    pub mod RTCAPBLPEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGLPEN::RW;
    }

    /// SAI4 Peripheral Clocks Enable During CSleep Mode
    pub mod SAI4LPEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSCFGLPEN::RW;
    }
}

/// RCC APB4 Sleep Clock Register
pub mod APB4LPENR {
    pub use super::C1_APB4LPENR::COMP12LPEN;
    pub use super::C1_APB4LPENR::I2C4LPEN;
    pub use super::C1_APB4LPENR::LPTIM2LPEN;
    pub use super::C1_APB4LPENR::LPTIM3LPEN;
    pub use super::C1_APB4LPENR::LPTIM4LPEN;
    pub use super::C1_APB4LPENR::LPTIM5LPEN;
    pub use super::C1_APB4LPENR::LPUART1LPEN;
    pub use super::C1_APB4LPENR::RTCAPBLPEN;
    pub use super::C1_APB4LPENR::SAI4LPEN;
    pub use super::C1_APB4LPENR::SPI6LPEN;
    pub use super::C1_APB4LPENR::SYSCFGLPEN;
    pub use super::C1_APB4LPENR::VREFLPEN;
}

/// RCC CSI configuration register
pub mod CSICFGR {

    /// CSI clock trimming
    pub mod CSITRIM {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (6 bits: 0x3f << 24)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSI clock calibration
    pub mod CSICAL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
pub struct RegisterBlock {
    /// clock control register
    pub CR: RWRegister<u32>,

    /// ICSCR and HSICFGR
    /// ICSCR: RCC Internal Clock Source Calibration Register
    /// HSICFGR: RCC HSI configuration register
    pub ICSCR: RWRegister<u32>,

    /// RCC Clock Recovery RC Register
    pub CRRCR: RORegister<u32>,

    /// RCC CSI configuration register
    pub CSICFGR: RWRegister<u32>,

    /// RCC Clock Configuration Register
    pub CFGR: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// RCC Domain 1 Clock Configuration Register
    pub D1CFGR: RWRegister<u32>,

    /// RCC Domain 2 Clock Configuration Register
    pub D2CFGR: RWRegister<u32>,

    /// RCC Domain 3 Clock Configuration Register
    pub D3CFGR: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// RCC PLLs Clock Source Selection Register
    pub PLLCKSELR: RWRegister<u32>,

    /// RCC PLLs Configuration Register
    pub PLLCFGR: RWRegister<u32>,

    /// RCC PLL1 Dividers Configuration Register
    pub PLL1DIVR: RWRegister<u32>,

    /// RCC PLL1 Fractional Divider Register
    pub PLL1FRACR: RWRegister<u32>,

    /// RCC PLL2 Dividers Configuration Register
    pub PLL2DIVR: RWRegister<u32>,

    /// RCC PLL2 Fractional Divider Register
    pub PLL2FRACR: RWRegister<u32>,

    /// RCC PLL3 Dividers Configuration Register
    pub PLL3DIVR: RWRegister<u32>,

    /// RCC PLL3 Fractional Divider Register
    pub PLL3FRACR: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// RCC Domain 1 Kernel Clock Configuration Register
    pub D1CCIPR: RWRegister<u32>,

    /// RCC Domain 2 Kernel Clock Configuration Register
    pub D2CCIP1R: RWRegister<u32>,

    /// RCC Domain 2 Kernel Clock Configuration Register
    pub D2CCIP2R: RWRegister<u32>,

    /// RCC Domain 3 Kernel Clock Configuration Register
    pub D3CCIPR: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// RCC Clock Source Interrupt Enable Register
    pub CIER: RWRegister<u32>,

    /// RCC Clock Source Interrupt Flag Register
    pub CIFR: RORegister<u32>,

    /// RCC Clock Source Interrupt Clear Register
    pub CICR: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// RCC Backup Domain Control Register
    pub BDCR: RWRegister<u32>,

    /// RCC Clock Control and Status Register
    pub CSR: RWRegister<u32>,

    _reserved6: [u32; 1],

    /// RCC AHB3 Reset Register
    pub AHB3RSTR: RWRegister<u32>,

    /// RCC AHB1 Peripheral Reset Register
    pub AHB1RSTR: RWRegister<u32>,

    /// RCC AHB2 Peripheral Reset Register
    pub AHB2RSTR: RWRegister<u32>,

    /// RCC AHB4 Peripheral Reset Register
    pub AHB4RSTR: RWRegister<u32>,

    /// RCC APB3 Peripheral Reset Register
    pub APB3RSTR: RWRegister<u32>,

    /// RCC APB1 Peripheral Reset Register
    pub APB1LRSTR: RWRegister<u32>,

    /// RCC APB1 Peripheral Reset Register
    pub APB1HRSTR: RWRegister<u32>,

    /// RCC APB2 Peripheral Reset Register
    pub APB2RSTR: RWRegister<u32>,

    /// RCC APB4 Peripheral Reset Register
    pub APB4RSTR: RWRegister<u32>,

    /// RCC Global Control Register
    pub GCR: RWRegister<u32>,

    _reserved7: [u32; 1],

    /// RCC D3 Autonomous mode Register
    pub D3AMR: RWRegister<u32>,

    _reserved8: [u32; 9],

    /// RCC Reset Status Register
    pub RSR: RWRegister<u32>,

    /// RCC AHB3 Clock Register
    pub AHB3ENR: RWRegister<u32>,

    /// RCC AHB1 Clock Register
    pub AHB1ENR: RWRegister<u32>,

    /// RCC AHB2 Clock Register
    pub AHB2ENR: RWRegister<u32>,

    /// RCC AHB4 Clock Register
    pub AHB4ENR: RWRegister<u32>,

    /// RCC APB3 Clock Register
    pub APB3ENR: RWRegister<u32>,

    /// RCC APB1 Clock Register
    pub APB1LENR: RWRegister<u32>,

    /// RCC APB1 Clock Register
    pub APB1HENR: RWRegister<u32>,

    /// RCC APB2 Clock Register
    pub APB2ENR: RWRegister<u32>,

    /// RCC APB4 Clock Register
    pub APB4ENR: RWRegister<u32>,

    _reserved9: [u32; 1],

    /// RCC AHB3 Sleep Clock Register
    pub AHB3LPENR: RWRegister<u32>,

    /// RCC AHB1 Sleep Clock Register
    pub AHB1LPENR: RWRegister<u32>,

    /// RCC AHB2 Sleep Clock Register
    pub AHB2LPENR: RWRegister<u32>,

    /// RCC AHB4 Sleep Clock Register
    pub AHB4LPENR: RWRegister<u32>,

    /// RCC APB3 Sleep Clock Register
    pub APB3LPENR: RWRegister<u32>,

    /// RCC APB1 Low Sleep Clock Register
    pub APB1LLPENR: RWRegister<u32>,

    /// RCC APB1 High Sleep Clock Register
    pub APB1HLPENR: RWRegister<u32>,

    /// RCC APB2 Sleep Clock Register
    pub APB2LPENR: RWRegister<u32>,

    /// RCC APB4 Sleep Clock Register
    pub APB4LPENR: RWRegister<u32>,

    _reserved10: [u32; 4],

    /// RCC Reset Status Register
    pub C1_RSR: RWRegister<u32>,

    /// RCC AHB3 Clock Register
    pub C1_AHB3ENR: RWRegister<u32>,

    /// RCC AHB1 Clock Register
    pub C1_AHB1ENR: RWRegister<u32>,

    /// RCC AHB2 Clock Register
    pub C1_AHB2ENR: RWRegister<u32>,

    /// RCC AHB4 Clock Register
    pub C1_AHB4ENR: RWRegister<u32>,

    /// RCC APB3 Clock Register
    pub C1_APB3ENR: RWRegister<u32>,

    /// RCC APB1 Clock Register
    pub C1_APB1LENR: RWRegister<u32>,

    /// RCC APB1 Clock Register
    pub C1_APB1HENR: RWRegister<u32>,

    /// RCC APB2 Clock Register
    pub C1_APB2ENR: RWRegister<u32>,

    /// RCC APB4 Clock Register
    pub C1_APB4ENR: RWRegister<u32>,

    _reserved11: [u32; 1],

    /// RCC AHB3 Sleep Clock Register
    pub C1_AHB3LPENR: RWRegister<u32>,

    /// RCC AHB1 Sleep Clock Register
    pub C1_AHB1LPENR: RWRegister<u32>,

    /// RCC AHB2 Sleep Clock Register
    pub C1_AHB2LPENR: RWRegister<u32>,

    /// RCC AHB4 Sleep Clock Register
    pub C1_AHB4LPENR: RWRegister<u32>,

    /// RCC APB3 Sleep Clock Register
    pub C1_APB3LPENR: RWRegister<u32>,

    /// RCC APB1 Low Sleep Clock Register
    pub C1_APB1LLPENR: RWRegister<u32>,

    /// RCC APB1 High Sleep Clock Register
    pub C1_APB1HLPENR: RWRegister<u32>,

    /// RCC APB2 Sleep Clock Register
    pub C1_APB2LPENR: RWRegister<u32>,

    /// RCC APB4 Sleep Clock Register
    pub C1_APB4LPENR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub ICSCR: u32,
    pub CRRCR: u32,
    pub CSICFGR: u32,
    pub CFGR: u32,
    pub D1CFGR: u32,
    pub D2CFGR: u32,
    pub D3CFGR: u32,
    pub PLLCKSELR: u32,
    pub PLLCFGR: u32,
    pub PLL1DIVR: u32,
    pub PLL1FRACR: u32,
    pub PLL2DIVR: u32,
    pub PLL2FRACR: u32,
    pub PLL3DIVR: u32,
    pub PLL3FRACR: u32,
    pub D1CCIPR: u32,
    pub D2CCIP1R: u32,
    pub D2CCIP2R: u32,
    pub D3CCIPR: u32,
    pub CIER: u32,
    pub CIFR: u32,
    pub CICR: u32,
    pub BDCR: u32,
    pub CSR: u32,
    pub AHB3RSTR: u32,
    pub AHB1RSTR: u32,
    pub AHB2RSTR: u32,
    pub AHB4RSTR: u32,
    pub APB3RSTR: u32,
    pub APB1LRSTR: u32,
    pub APB1HRSTR: u32,
    pub APB2RSTR: u32,
    pub APB4RSTR: u32,
    pub GCR: u32,
    pub D3AMR: u32,
    pub RSR: u32,
    pub AHB3ENR: u32,
    pub AHB1ENR: u32,
    pub AHB2ENR: u32,
    pub AHB4ENR: u32,
    pub APB3ENR: u32,
    pub APB1LENR: u32,
    pub APB1HENR: u32,
    pub APB2ENR: u32,
    pub APB4ENR: u32,
    pub AHB3LPENR: u32,
    pub AHB1LPENR: u32,
    pub AHB2LPENR: u32,
    pub AHB4LPENR: u32,
    pub APB3LPENR: u32,
    pub APB1LLPENR: u32,
    pub APB1HLPENR: u32,
    pub APB2LPENR: u32,
    pub APB4LPENR: u32,
    pub C1_RSR: u32,
    pub C1_AHB3ENR: u32,
    pub C1_AHB1ENR: u32,
    pub C1_AHB2ENR: u32,
    pub C1_AHB4ENR: u32,
    pub C1_APB3ENR: u32,
    pub C1_APB1LENR: u32,
    pub C1_APB1HENR: u32,
    pub C1_APB2ENR: u32,
    pub C1_APB4ENR: u32,
    pub C1_AHB3LPENR: u32,
    pub C1_AHB1LPENR: u32,
    pub C1_AHB2LPENR: u32,
    pub C1_AHB4LPENR: u32,
    pub C1_APB3LPENR: u32,
    pub C1_APB1LLPENR: u32,
    pub C1_APB1HLPENR: u32,
    pub C1_APB2LPENR: u32,
    pub C1_APB4LPENR: u32,
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
