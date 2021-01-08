#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! RCC

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode.
pub mod RCC_TZCR {

    /// TZEN
    pub mod TZEN {
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

    /// MCKPROT
    pub mod MCKPROT {
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
}

/// This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_OCENSETR {

    /// HSION
    pub mod HSION {
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

    /// HSIKERON
    pub mod HSIKERON {
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

    /// CSION
    pub mod CSION {
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

    /// CSIKERON
    pub mod CSIKERON {
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

    /// DIGBYP
    pub mod DIGBYP {
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

    /// HSEON
    pub mod HSEON {
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

    /// HSEKERON
    pub mod HSEKERON {
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

    /// HSEBYP
    pub mod HSEBYP {
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

    /// HSECSSON
    pub mod HSECSSON {
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
}

/// This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_OCENCLRR {

    /// HSION
    pub mod HSION {
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

    /// HSIKERON
    pub mod HSIKERON {
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

    /// CSION
    pub mod CSION {
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

    /// CSIKERON
    pub mod CSIKERON {
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

    /// DIGBYP
    pub mod DIGBYP {
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

    /// HSEON
    pub mod HSEON {
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

    /// HSEKERON
    pub mod HSEKERON {
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

    /// HSEBYP
    pub mod HSEBYP {
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

/// This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_HSICFGR {

    /// HSIDIV
    pub mod HSIDIV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSITRIM
    pub mod HSITRIM {
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

    /// HSICAL
    pub mod HSICAL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details.
pub mod RCC_CSICFGR {

    /// CSITRIM
    pub mod CSITRIM {
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

    /// CSICAL
    pub mod CSICAL {
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
}

/// This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_MPCKSELR {

    /// MPUSRC
    pub mod MPUSRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MPUSRCRDY
    pub mod MPUSRCRDY {
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

/// This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_ASSCKSELR {

    /// AXISSRC
    pub mod AXISSRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXISSRCRDY
    pub mod AXISSRCRDY {
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

/// This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_RCK12SELR {

    /// PLL12SRC
    pub mod PLL12SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL12SRCRDY
    pub mod PLL12SRCRDY {
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

/// This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MPCKDIVR {

    /// MPUDIV
    pub mod MPUDIV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MPUDIVRDY
    pub mod MPUDIVRDY {
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

/// This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_AXIDIVR {

    /// AXIDIV
    pub mod AXIDIV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXIDIVRDY
    pub mod AXIDIVRDY {
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

/// This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_APB4DIVR {

    /// APB4DIV
    pub mod APB4DIV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// APB4DIVRDY
    pub mod APB4DIVRDY {
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

/// This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_APB5DIVR {

    /// APB5DIV
    pub mod APB5DIV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// APB5DIVRDY
    pub mod APB5DIVRDY {
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

/// This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_RTCDIVR {

    /// RTCDIV
    pub mod RTCDIV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_MSSCKSELR {

    /// MCUSSRC
    pub mod MCUSSRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MCUSSRCRDY
    pub mod MCUSSRCRDY {
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

/// This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_PLL1CR {

    /// PLLON
    pub mod PLLON {
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

    /// PLL1RDY
    pub mod PLL1RDY {
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

    /// SSCG_CTRL
    pub mod SSCG_CTRL {
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

    /// DIVPEN
    pub mod DIVPEN {
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

    /// DIVQEN
    pub mod DIVQEN {
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

    /// DIVREN
    pub mod DIVREN {
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
}

/// This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_PLL1CFGR1 {

    /// DIVN
    pub mod DIVN {
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

    /// DIVM1
    pub mod DIVM1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_PLL1CFGR2 {

    /// DIVP
    pub mod DIVP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DIVQ
    pub mod DIVQ {
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

    /// DIVR
    pub mod DIVR {
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
}

/// This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_PLL1FRACR {

    /// FRACV
    pub mod FRACV {
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

    /// FRACLE
    pub mod FRACLE {
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

/// This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_PLL1CSGR {

    /// MOD_PER
    pub mod MOD_PER {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (13 bits: 0x1fff << 0)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TPDFN_DIS
    pub mod TPDFN_DIS {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RPDFN_DIS
    pub mod RPDFN_DIS {
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

    /// SSCG_MODE
    pub mod SSCG_MODE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// INC_STEP
    pub mod INC_STEP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (15 bits: 0x7fff << 16)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_PLL2CR {

    /// PLLON
    pub mod PLLON {
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

    /// PLL2RDY
    pub mod PLL2RDY {
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

    /// SSCG_CTRL
    pub mod SSCG_CTRL {
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

    /// DIVPEN
    pub mod DIVPEN {
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

    /// DIVQEN
    pub mod DIVQEN {
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

    /// DIVREN
    pub mod DIVREN {
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
}

/// This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_PLL2CFGR1 {

    /// DIVN
    pub mod DIVN {
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

    /// DIVM2
    pub mod DIVM2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_PLL2CFGR2 {
    pub use super::RCC_PLL1CFGR2::DIVP;
    pub use super::RCC_PLL1CFGR2::DIVQ;
    pub use super::RCC_PLL1CFGR2::DIVR;
}

/// This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_PLL2FRACR {
    pub use super::RCC_PLL1FRACR::FRACLE;
    pub use super::RCC_PLL1FRACR::FRACV;
}

/// This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod RCC_PLL2CSGR {
    pub use super::RCC_PLL1CSGR::INC_STEP;
    pub use super::RCC_PLL1CSGR::MOD_PER;
    pub use super::RCC_PLL1CSGR::RPDFN_DIS;
    pub use super::RCC_PLL1CSGR::SSCG_MODE;
    pub use super::RCC_PLL1CSGR::TPDFN_DIS;
}

/// This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_I2C46CKSELR {

    /// I2C46SRC
    pub mod I2C46SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_SPI6CKSELR {

    /// SPI6SRC
    pub mod SPI6SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_UART1CKSELR {

    /// UART1SRC
    pub mod UART1SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_RNG1CKSELR {

    /// RNG1SRC
    pub mod RNG1SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays.
pub mod RCC_CPERCKSELR {

    /// CKPERSRC
    pub mod CKPERSRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_STGENCKSELR {

    /// STGENSRC
    pub mod STGENSRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_DDRITFCR {

    /// DDRC1EN
    pub mod DDRC1EN {
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

    /// DDRC1LPEN
    pub mod DDRC1LPEN {
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

    /// DDRC2EN
    pub mod DDRC2EN {
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

    /// DDRC2LPEN
    pub mod DDRC2LPEN {
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

    /// DDRPHYCEN
    pub mod DDRPHYCEN {
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

    /// DDRPHYCLPEN
    pub mod DDRPHYCLPEN {
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

    /// DDRCAPBEN
    pub mod DDRCAPBEN {
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

    /// DDRCAPBLPEN
    pub mod DDRCAPBLPEN {
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

    /// AXIDCGEN
    pub mod AXIDCGEN {
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

    /// DDRPHYCAPBEN
    pub mod DDRPHYCAPBEN {
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

    /// DDRPHYCAPBLPEN
    pub mod DDRPHYCAPBLPEN {
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

    /// KERDCG_DLY
    pub mod KERDCG_DLY {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (3 bits: 0b111 << 11)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DDRCAPBRST
    pub mod DDRCAPBRST {
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

    /// DDRCAXIRST
    pub mod DDRCAXIRST {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DDRCORERST
    pub mod DDRCORERST {
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

    /// DPHYAPBRST
    pub mod DPHYAPBRST {
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

    /// DPHYRST
    pub mod DPHYRST {
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

    /// DPHYCTLRST
    pub mod DPHYCTLRST {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DDRCKMOD
    pub mod DDRCKMOD {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GSKPMOD
    pub mod GSKPMOD {
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

    /// GSKPCTRL
    pub mod GSKPCTRL {
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

    /// DFILP_WIDTH
    pub mod DFILP_WIDTH {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (3 bits: 0b111 << 25)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GSKP_DUR
    pub mod GSKP_DUR {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU.
pub mod RCC_MP_BOOTCR {

    /// MCU_BEN
    pub mod MCU_BEN {
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

    /// MPU_BEN
    pub mod MPU_BEN {
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
}

/// Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_SREQSETR {

    /// STPREQ_P0
    pub mod STPREQ_P0 {
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

    /// STPREQ_P1
    pub mod STPREQ_P1 {
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
}

/// Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_SREQCLRR {
    pub use super::RCC_MP_SREQSETR::STPREQ_P0;
    pub use super::RCC_MP_SREQSETR::STPREQ_P1;
}

/// The register contains global control bits. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_GCR {

    /// BOOT_MCU
    pub mod BOOT_MCU {
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

/// This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_APRSTCR {

    /// RDCTLEN
    pub mod RDCTLEN {
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

    /// RSTTO
    pub mod RSTTO {
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
}

/// This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_APRSTSR {

    /// RSTTOV
    pub mod RSTTOV {
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
}

/// This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_BDCR {

    /// LSEON
    pub mod LSEON {
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

    /// LSEBYP
    pub mod LSEBYP {
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

    /// LSERDY
    pub mod LSERDY {
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

    /// DIGBYP
    pub mod DIGBYP {
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

    /// LSEDRV
    pub mod LSEDRV {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSECSSON
    pub mod LSECSSON {
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

    /// LSECSSD
    pub mod LSECSSD {
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

    /// RTCSRC
    pub mod RTCSRC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RTCCKEN
    pub mod RTCCKEN {
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

    /// VSWRST
    pub mod VSWRST {
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

/// This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_RDLSICR {

    /// LSION
    pub mod LSION {
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

    /// LSIRDY
    pub mod LSIRDY {
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

    /// MRD
    pub mod MRD {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EADLY
    pub mod EADLY {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SPARE
    pub mod SPARE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (5 bits: 0b11111 << 27)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.
pub mod RCC_APB4RSTSETR {

    /// LTDCRST
    pub mod LTDCRST {
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

    /// DSIRST
    pub mod DSIRST {
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

    /// DDRPERFMRST
    pub mod DDRPERFMRST {
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

    /// USBPHYRST
    pub mod USBPHYRST {
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

/// This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.
pub mod RCC_APB4RSTCLRR {
    pub use super::RCC_APB4RSTSETR::DDRPERFMRST;
    pub use super::RCC_APB4RSTSETR::DSIRST;
    pub use super::RCC_APB4RSTSETR::LTDCRST;
    pub use super::RCC_APB4RSTSETR::USBPHYRST;
}

/// This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_APB5RSTSETR {

    /// SPI6RST
    pub mod SPI6RST {
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

    /// I2C4RST
    pub mod I2C4RST {
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

    /// I2C6RST
    pub mod I2C6RST {
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

    /// USART1RST
    pub mod USART1RST {
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

    /// STGENRST
    pub mod STGENRST {
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
}

/// This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_APB5RSTCLRR {
    pub use super::RCC_APB5RSTSETR::I2C4RST;
    pub use super::RCC_APB5RSTSETR::I2C6RST;
    pub use super::RCC_APB5RSTSETR::SPI6RST;
    pub use super::RCC_APB5RSTSETR::STGENRST;
    pub use super::RCC_APB5RSTSETR::USART1RST;
}

/// This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_AHB5RSTSETR {

    /// GPIOZRST
    pub mod GPIOZRST {
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

    /// CRYP1RST
    pub mod CRYP1RST {
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

    /// HASH1RST
    pub mod HASH1RST {
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

    /// RNG1RST
    pub mod RNG1RST {
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

    /// AXIMCRST
    pub mod AXIMCRST {
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

/// This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_AHB5RSTCLRR {
    pub use super::RCC_AHB5RSTSETR::AXIMCRST;
    pub use super::RCC_AHB5RSTSETR::CRYP1RST;
    pub use super::RCC_AHB5RSTSETR::GPIOZRST;
    pub use super::RCC_AHB5RSTSETR::HASH1RST;
    pub use super::RCC_AHB5RSTSETR::RNG1RST;
}

/// This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.
pub mod RCC_AHB6RSTSETR {

    /// GPURST
    pub mod GPURST {
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

    /// ETHMACRST
    pub mod ETHMACRST {
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

    /// FMCRST
    pub mod FMCRST {
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

    /// QSPIRST
    pub mod QSPIRST {
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

    /// SDMMC1RST
    pub mod SDMMC1RST {
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

    /// SDMMC2RST
    pub mod SDMMC2RST {
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

    /// CRC1RST
    pub mod CRC1RST {
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

    /// USBHRST
    pub mod USBHRST {
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
}

/// This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.
pub mod RCC_AHB6RSTCLRR {

    /// ETHMACRST
    pub mod ETHMACRST {
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

    /// FMCRST
    pub mod FMCRST {
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

    /// QSPIRST
    pub mod QSPIRST {
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

    /// SDMMC1RST
    pub mod SDMMC1RST {
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

    /// SDMMC2RST
    pub mod SDMMC2RST {
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

    /// CRC1RST
    pub mod CRC1RST {
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

    /// USBHRST
    pub mod USBHRST {
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
}

/// This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_TZAHB6RSTSETR {

    /// MDMARST
    pub mod MDMARST {
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

/// This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_TZAHB6RSTCLRR {
    pub use super::RCC_TZAHB6RSTSETR::MDMARST;
}

/// This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod RCC_MP_APB4ENSETR {

    /// LTDCEN
    pub mod LTDCEN {
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

    /// DSIEN
    pub mod DSIEN {
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

    /// DDRPERFMEN
    pub mod DDRPERFMEN {
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

    /// IWDG2APBEN
    pub mod IWDG2APBEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USBPHYEN
    pub mod USBPHYEN {
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

    /// STGENROEN
    pub mod STGENROEN {
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
}

/// This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod RCC_MP_APB4ENCLRR {
    pub use super::RCC_MP_APB4ENSETR::DDRPERFMEN;
    pub use super::RCC_MP_APB4ENSETR::DSIEN;
    pub use super::RCC_MP_APB4ENSETR::IWDG2APBEN;
    pub use super::RCC_MP_APB4ENSETR::LTDCEN;
    pub use super::RCC_MP_APB4ENSETR::STGENROEN;
    pub use super::RCC_MP_APB4ENSETR::USBPHYEN;
}

/// This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod RCC_MP_APB5ENSETR {

    /// SPI6EN
    pub mod SPI6EN {
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

    /// I2C4EN
    pub mod I2C4EN {
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

    /// I2C6EN
    pub mod I2C6EN {
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

    /// USART1EN
    pub mod USART1EN {
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

    /// RTCAPBEN
    pub mod RTCAPBEN {
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

    /// TZC1EN
    pub mod TZC1EN {
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

    /// TZC2EN
    pub mod TZC2EN {
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

    /// TZPCEN
    pub mod TZPCEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IWDG1APBEN
    pub mod IWDG1APBEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BSECEN
    pub mod BSECEN {
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

    /// STGENEN
    pub mod STGENEN {
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
}

/// This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod RCC_MP_APB5ENCLRR {
    pub use super::RCC_MP_APB5ENSETR::BSECEN;
    pub use super::RCC_MP_APB5ENSETR::I2C4EN;
    pub use super::RCC_MP_APB5ENSETR::I2C6EN;
    pub use super::RCC_MP_APB5ENSETR::IWDG1APBEN;
    pub use super::RCC_MP_APB5ENSETR::RTCAPBEN;
    pub use super::RCC_MP_APB5ENSETR::SPI6EN;
    pub use super::RCC_MP_APB5ENSETR::STGENEN;
    pub use super::RCC_MP_APB5ENSETR::TZC1EN;
    pub use super::RCC_MP_APB5ENSETR::TZC2EN;
    pub use super::RCC_MP_APB5ENSETR::TZPCEN;
    pub use super::RCC_MP_APB5ENSETR::USART1EN;
}

/// This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_AHB5ENSETR {

    /// GPIOZEN
    pub mod GPIOZEN {
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

    /// CRYP1EN
    pub mod CRYP1EN {
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

    /// HASH1EN
    pub mod HASH1EN {
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

    /// RNG1EN
    pub mod RNG1EN {
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

    /// BKPSRAMEN
    pub mod BKPSRAMEN {
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

    /// AXIMCEN
    pub mod AXIMCEN {
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

/// This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_AHB5ENCLRR {
    pub use super::RCC_MP_AHB5ENSETR::AXIMCEN;
    pub use super::RCC_MP_AHB5ENSETR::BKPSRAMEN;
    pub use super::RCC_MP_AHB5ENSETR::CRYP1EN;
    pub use super::RCC_MP_AHB5ENSETR::GPIOZEN;
    pub use super::RCC_MP_AHB5ENSETR::HASH1EN;
    pub use super::RCC_MP_AHB5ENSETR::RNG1EN;
}

/// This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod RCC_MP_AHB6ENSETR {

    /// MDMAEN
    pub mod MDMAEN {
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

    /// GPUEN
    pub mod GPUEN {
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

    /// ETHCKEN
    pub mod ETHCKEN {
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

    /// ETHTXEN
    pub mod ETHTXEN {
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

    /// ETHRXEN
    pub mod ETHRXEN {
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

    /// ETHMACEN
    pub mod ETHMACEN {
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

    /// FMCEN
    pub mod FMCEN {
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

    /// QSPIEN
    pub mod QSPIEN {
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

    /// SDMMC1EN
    pub mod SDMMC1EN {
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

    /// SDMMC2EN
    pub mod SDMMC2EN {
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

    /// CRC1EN
    pub mod CRC1EN {
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

    /// USBHEN
    pub mod USBHEN {
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
}

/// This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod RCC_MP_AHB6ENCLRR {
    pub use super::RCC_MP_AHB6ENSETR::CRC1EN;
    pub use super::RCC_MP_AHB6ENSETR::ETHCKEN;
    pub use super::RCC_MP_AHB6ENSETR::ETHMACEN;
    pub use super::RCC_MP_AHB6ENSETR::ETHRXEN;
    pub use super::RCC_MP_AHB6ENSETR::ETHTXEN;
    pub use super::RCC_MP_AHB6ENSETR::FMCEN;
    pub use super::RCC_MP_AHB6ENSETR::GPUEN;
    pub use super::RCC_MP_AHB6ENSETR::MDMAEN;
    pub use super::RCC_MP_AHB6ENSETR::QSPIEN;
    pub use super::RCC_MP_AHB6ENSETR::SDMMC1EN;
    pub use super::RCC_MP_AHB6ENSETR::SDMMC2EN;
    pub use super::RCC_MP_AHB6ENSETR::USBHEN;
}

/// This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_TZAHB6ENSETR {

    /// MDMAEN
    pub mod MDMAEN {
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

/// This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_TZAHB6ENCLRR {
    pub use super::RCC_MP_TZAHB6ENSETR::MDMAEN;
}

/// This register is used to set the peripheral clock enable bit
pub mod RCC_MC_APB4ENSETR {

    /// LTDCEN
    pub mod LTDCEN {
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

    /// DSIEN
    pub mod DSIEN {
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

    /// DDRPERFMEN
    pub mod DDRPERFMEN {
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

    /// USBPHYEN
    pub mod USBPHYEN {
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

    /// STGENROEN
    pub mod STGENROEN {
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
}

/// This register is used to clear the peripheral clock enable bit
pub mod RCC_MC_APB4ENCLRR {
    pub use super::RCC_MC_APB4ENSETR::DDRPERFMEN;
    pub use super::RCC_MC_APB4ENSETR::DSIEN;
    pub use super::RCC_MC_APB4ENSETR::LTDCEN;
    pub use super::RCC_MC_APB4ENSETR::STGENROEN;
    pub use super::RCC_MC_APB4ENSETR::USBPHYEN;
}

/// This register is used to set the peripheral clock enable bit
pub mod RCC_MC_APB5ENSETR {

    /// SPI6EN
    pub mod SPI6EN {
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

    /// I2C4EN
    pub mod I2C4EN {
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

    /// I2C6EN
    pub mod I2C6EN {
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

    /// USART1EN
    pub mod USART1EN {
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

    /// RTCAPBEN
    pub mod RTCAPBEN {
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

    /// TZC1EN
    pub mod TZC1EN {
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

    /// TZC2EN
    pub mod TZC2EN {
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

    /// TZPCEN
    pub mod TZPCEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BSECEN
    pub mod BSECEN {
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

    /// STGENEN
    pub mod STGENEN {
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
}

/// This register is used to clear the peripheral clock enable bit
pub mod RCC_MC_APB5ENCLRR {
    pub use super::RCC_MC_APB5ENSETR::BSECEN;
    pub use super::RCC_MC_APB5ENSETR::I2C4EN;
    pub use super::RCC_MC_APB5ENSETR::I2C6EN;
    pub use super::RCC_MC_APB5ENSETR::RTCAPBEN;
    pub use super::RCC_MC_APB5ENSETR::SPI6EN;
    pub use super::RCC_MC_APB5ENSETR::STGENEN;
    pub use super::RCC_MC_APB5ENSETR::TZC1EN;
    pub use super::RCC_MC_APB5ENSETR::TZC2EN;
    pub use super::RCC_MC_APB5ENSETR::TZPCEN;
    pub use super::RCC_MC_APB5ENSETR::USART1EN;
}

/// This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MC_AHB5ENSETR {

    /// GPIOZEN
    pub mod GPIOZEN {
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

    /// CRYP1EN
    pub mod CRYP1EN {
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

    /// HASH1EN
    pub mod HASH1EN {
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

    /// RNG1EN
    pub mod RNG1EN {
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

    /// BKPSRAMEN
    pub mod BKPSRAMEN {
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
}

/// This register is used to clear the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MC_AHB5ENCLRR {
    pub use super::RCC_MC_AHB5ENSETR::BKPSRAMEN;
    pub use super::RCC_MC_AHB5ENSETR::CRYP1EN;
    pub use super::RCC_MC_AHB5ENSETR::GPIOZEN;
    pub use super::RCC_MC_AHB5ENSETR::HASH1EN;
    pub use super::RCC_MC_AHB5ENSETR::RNG1EN;
}

/// This register is used to set the peripheral clock enable bit
pub mod RCC_MC_AHB6ENSETR {
    pub use super::RCC_MP_AHB6ENSETR::CRC1EN;
    pub use super::RCC_MP_AHB6ENSETR::ETHCKEN;
    pub use super::RCC_MP_AHB6ENSETR::ETHMACEN;
    pub use super::RCC_MP_AHB6ENSETR::ETHRXEN;
    pub use super::RCC_MP_AHB6ENSETR::ETHTXEN;
    pub use super::RCC_MP_AHB6ENSETR::FMCEN;
    pub use super::RCC_MP_AHB6ENSETR::GPUEN;
    pub use super::RCC_MP_AHB6ENSETR::MDMAEN;
    pub use super::RCC_MP_AHB6ENSETR::QSPIEN;
    pub use super::RCC_MP_AHB6ENSETR::SDMMC1EN;
    pub use super::RCC_MP_AHB6ENSETR::SDMMC2EN;
    pub use super::RCC_MP_AHB6ENSETR::USBHEN;
}

/// This register is used to clear the peripheral clock enable bit
pub mod RCC_MC_AHB6ENCLRR {
    pub use super::RCC_MP_AHB6ENSETR::CRC1EN;
    pub use super::RCC_MP_AHB6ENSETR::ETHCKEN;
    pub use super::RCC_MP_AHB6ENSETR::ETHMACEN;
    pub use super::RCC_MP_AHB6ENSETR::ETHRXEN;
    pub use super::RCC_MP_AHB6ENSETR::ETHTXEN;
    pub use super::RCC_MP_AHB6ENSETR::FMCEN;
    pub use super::RCC_MP_AHB6ENSETR::GPUEN;
    pub use super::RCC_MP_AHB6ENSETR::MDMAEN;
    pub use super::RCC_MP_AHB6ENSETR::QSPIEN;
    pub use super::RCC_MP_AHB6ENSETR::SDMMC1EN;
    pub use super::RCC_MP_AHB6ENSETR::SDMMC2EN;
    pub use super::RCC_MP_AHB6ENSETR::USBHEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bits
pub mod RCC_MP_APB4LPENSETR {

    /// LTDCLPEN
    pub mod LTDCLPEN {
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

    /// DSILPEN
    pub mod DSILPEN {
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

    /// DDRPERFMLPEN
    pub mod DDRPERFMLPEN {
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

    /// IWDG2APBLPEN
    pub mod IWDG2APBLPEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USBPHYLPEN
    pub mod USBPHYLPEN {
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

    /// STGENROLPEN
    pub mod STGENROLPEN {
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

    /// STGENROSTPEN
    pub mod STGENROSTPEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used by the MCU
pub mod RCC_MP_APB4LPENCLRR {
    pub use super::RCC_MP_APB4LPENSETR::DDRPERFMLPEN;
    pub use super::RCC_MP_APB4LPENSETR::DSILPEN;
    pub use super::RCC_MP_APB4LPENSETR::IWDG2APBLPEN;
    pub use super::RCC_MP_APB4LPENSETR::LTDCLPEN;
    pub use super::RCC_MP_APB4LPENSETR::STGENROLPEN;
    pub use super::RCC_MP_APB4LPENSETR::STGENROSTPEN;
    pub use super::RCC_MP_APB4LPENSETR::USBPHYLPEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_APB5LPENSETR {

    /// SPI6LPEN
    pub mod SPI6LPEN {
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

    /// I2C4LPEN
    pub mod I2C4LPEN {
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

    /// I2C6LPEN
    pub mod I2C6LPEN {
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

    /// USART1LPEN
    pub mod USART1LPEN {
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

    /// RTCAPBLPEN
    pub mod RTCAPBLPEN {
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

    /// TZC1LPEN
    pub mod TZC1LPEN {
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

    /// TZC2LPEN
    pub mod TZC2LPEN {
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

    /// TZPCLPEN
    pub mod TZPCLPEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IWDG1APBLPEN
    pub mod IWDG1APBLPEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BSECLPEN
    pub mod BSECLPEN {
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

    /// STGENLPEN
    pub mod STGENLPEN {
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

    /// STGENSTPEN
    pub mod STGENSTPEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used by the Mpu.
pub mod RCC_MP_APB5LPENCLRR {
    pub use super::RCC_MP_APB5LPENSETR::BSECLPEN;
    pub use super::RCC_MP_APB5LPENSETR::I2C4LPEN;
    pub use super::RCC_MP_APB5LPENSETR::I2C6LPEN;
    pub use super::RCC_MP_APB5LPENSETR::IWDG1APBLPEN;
    pub use super::RCC_MP_APB5LPENSETR::RTCAPBLPEN;
    pub use super::RCC_MP_APB5LPENSETR::SPI6LPEN;
    pub use super::RCC_MP_APB5LPENSETR::STGENLPEN;
    pub use super::RCC_MP_APB5LPENSETR::STGENSTPEN;
    pub use super::RCC_MP_APB5LPENSETR::TZC1LPEN;
    pub use super::RCC_MP_APB5LPENSETR::TZC2LPEN;
    pub use super::RCC_MP_APB5LPENSETR::TZPCLPEN;
    pub use super::RCC_MP_APB5LPENSETR::USART1LPEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_AHB5LPENSETR {

    /// GPIOZLPEN
    pub mod GPIOZLPEN {
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

    /// CRYP1LPEN
    pub mod CRYP1LPEN {
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

    /// HASH1LPEN
    pub mod HASH1LPEN {
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

    /// RNG1LPEN
    pub mod RNG1LPEN {
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

    /// BKPSRAMLPEN
    pub mod BKPSRAMLPEN {
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
}

/// This register is used by the MCU
pub mod RCC_MP_AHB5LPENCLRR {
    pub use super::RCC_MP_AHB5LPENSETR::BKPSRAMLPEN;
    pub use super::RCC_MP_AHB5LPENSETR::CRYP1LPEN;
    pub use super::RCC_MP_AHB5LPENSETR::GPIOZLPEN;
    pub use super::RCC_MP_AHB5LPENSETR::HASH1LPEN;
    pub use super::RCC_MP_AHB5LPENSETR::RNG1LPEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bits
pub mod RCC_MP_AHB6LPENSETR {

    /// MDMALPEN
    pub mod MDMALPEN {
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

    /// GPULPEN
    pub mod GPULPEN {
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

    /// ETHCKLPEN
    pub mod ETHCKLPEN {
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

    /// ETHTXLPEN
    pub mod ETHTXLPEN {
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

    /// ETHRXLPEN
    pub mod ETHRXLPEN {
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

    /// ETHMACLPEN
    pub mod ETHMACLPEN {
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

    /// ETHSTPEN
    pub mod ETHSTPEN {
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

    /// FMCLPEN
    pub mod FMCLPEN {
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

    /// QSPILPEN
    pub mod QSPILPEN {
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

    /// SDMMC1LPEN
    pub mod SDMMC1LPEN {
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

    /// SDMMC2LPEN
    pub mod SDMMC2LPEN {
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

    /// CRC1LPEN
    pub mod CRC1LPEN {
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

    /// USBHLPEN
    pub mod USBHLPEN {
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
}

/// This register is used by the MCU in order to clear the PERxLPEN bits
pub mod RCC_MP_AHB6LPENCLRR {
    pub use super::RCC_MP_AHB6LPENSETR::CRC1LPEN;
    pub use super::RCC_MP_AHB6LPENSETR::ETHCKLPEN;
    pub use super::RCC_MP_AHB6LPENSETR::ETHMACLPEN;
    pub use super::RCC_MP_AHB6LPENSETR::ETHRXLPEN;
    pub use super::RCC_MP_AHB6LPENSETR::ETHSTPEN;
    pub use super::RCC_MP_AHB6LPENSETR::ETHTXLPEN;
    pub use super::RCC_MP_AHB6LPENSETR::FMCLPEN;
    pub use super::RCC_MP_AHB6LPENSETR::GPULPEN;
    pub use super::RCC_MP_AHB6LPENSETR::MDMALPEN;
    pub use super::RCC_MP_AHB6LPENSETR::QSPILPEN;
    pub use super::RCC_MP_AHB6LPENSETR::SDMMC1LPEN;
    pub use super::RCC_MP_AHB6LPENSETR::SDMMC2LPEN;
    pub use super::RCC_MP_AHB6LPENSETR::USBHLPEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_TZAHB6LPENSETR {

    /// MDMALPEN
    pub mod MDMALPEN {
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

/// This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_TZAHB6LPENCLRR {
    pub use super::RCC_MP_TZAHB6LPENSETR::MDMALPEN;
}

/// This register is used by the MCU in order to set the PERxLPEN bit.
pub mod RCC_MC_APB4LPENSETR {

    /// LTDCLPEN
    pub mod LTDCLPEN {
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

    /// DSILPEN
    pub mod DSILPEN {
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

    /// DDRPERFMLPEN
    pub mod DDRPERFMLPEN {
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

    /// USBPHYLPEN
    pub mod USBPHYLPEN {
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

    /// STGENROLPEN
    pub mod STGENROLPEN {
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

    /// STGENROSTPEN
    pub mod STGENROSTPEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used by the MCU in order to clear the PERxLPEN bit
pub mod RCC_MC_APB4LPENCLRR {
    pub use super::RCC_MC_APB4LPENSETR::DDRPERFMLPEN;
    pub use super::RCC_MC_APB4LPENSETR::DSILPEN;
    pub use super::RCC_MC_APB4LPENSETR::LTDCLPEN;
    pub use super::RCC_MC_APB4LPENSETR::STGENROLPEN;
    pub use super::RCC_MC_APB4LPENSETR::STGENROSTPEN;
    pub use super::RCC_MC_APB4LPENSETR::USBPHYLPEN;
}

/// This register is used by the MCU in order to set the PERxLPEN bit.
pub mod RCC_MC_APB5LPENSETR {

    /// SPI6LPEN
    pub mod SPI6LPEN {
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

    /// I2C4LPEN
    pub mod I2C4LPEN {
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

    /// I2C6LPEN
    pub mod I2C6LPEN {
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

    /// USART1LPEN
    pub mod USART1LPEN {
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

    /// RTCAPBLPEN
    pub mod RTCAPBLPEN {
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

    /// TZC1LPEN
    pub mod TZC1LPEN {
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

    /// TZC2LPEN
    pub mod TZC2LPEN {
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

    /// TZPCLPEN
    pub mod TZPCLPEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BSECLPEN
    pub mod BSECLPEN {
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

    /// STGENLPEN
    pub mod STGENLPEN {
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

    /// STGENSTPEN
    pub mod STGENSTPEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used by the MCU in order to clear the PERxLPEN bit
pub mod RCC_MC_APB5LPENCLRR {
    pub use super::RCC_MC_APB5LPENSETR::BSECLPEN;
    pub use super::RCC_MC_APB5LPENSETR::I2C4LPEN;
    pub use super::RCC_MC_APB5LPENSETR::I2C6LPEN;
    pub use super::RCC_MC_APB5LPENSETR::RTCAPBLPEN;
    pub use super::RCC_MC_APB5LPENSETR::SPI6LPEN;
    pub use super::RCC_MC_APB5LPENSETR::STGENLPEN;
    pub use super::RCC_MC_APB5LPENSETR::STGENSTPEN;
    pub use super::RCC_MC_APB5LPENSETR::TZC1LPEN;
    pub use super::RCC_MC_APB5LPENSETR::TZC2LPEN;
    pub use super::RCC_MC_APB5LPENSETR::TZPCLPEN;
    pub use super::RCC_MC_APB5LPENSETR::USART1LPEN;
}

/// This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MC_AHB5LPENSETR {
    pub use super::RCC_MP_AHB5LPENSETR::BKPSRAMLPEN;
    pub use super::RCC_MP_AHB5LPENSETR::CRYP1LPEN;
    pub use super::RCC_MP_AHB5LPENSETR::GPIOZLPEN;
    pub use super::RCC_MP_AHB5LPENSETR::HASH1LPEN;
    pub use super::RCC_MP_AHB5LPENSETR::RNG1LPEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MC_AHB5LPENCLRR {
    pub use super::RCC_MP_AHB5LPENSETR::BKPSRAMLPEN;
    pub use super::RCC_MP_AHB5LPENSETR::CRYP1LPEN;
    pub use super::RCC_MP_AHB5LPENSETR::GPIOZLPEN;
    pub use super::RCC_MP_AHB5LPENSETR::HASH1LPEN;
    pub use super::RCC_MP_AHB5LPENSETR::RNG1LPEN;
}

/// This register is used by the MCU in order to set the PERxLPEN bit.
pub mod RCC_MC_AHB6LPENSETR {
    pub use super::RCC_MP_AHB6LPENSETR::CRC1LPEN;
    pub use super::RCC_MP_AHB6LPENSETR::ETHCKLPEN;
    pub use super::RCC_MP_AHB6LPENSETR::ETHMACLPEN;
    pub use super::RCC_MP_AHB6LPENSETR::ETHRXLPEN;
    pub use super::RCC_MP_AHB6LPENSETR::ETHSTPEN;
    pub use super::RCC_MP_AHB6LPENSETR::ETHTXLPEN;
    pub use super::RCC_MP_AHB6LPENSETR::FMCLPEN;
    pub use super::RCC_MP_AHB6LPENSETR::GPULPEN;
    pub use super::RCC_MP_AHB6LPENSETR::MDMALPEN;
    pub use super::RCC_MP_AHB6LPENSETR::QSPILPEN;
    pub use super::RCC_MP_AHB6LPENSETR::SDMMC1LPEN;
    pub use super::RCC_MP_AHB6LPENSETR::SDMMC2LPEN;
    pub use super::RCC_MP_AHB6LPENSETR::USBHLPEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bit
pub mod RCC_MC_AHB6LPENCLRR {
    pub use super::RCC_MP_AHB6LPENSETR::CRC1LPEN;
    pub use super::RCC_MP_AHB6LPENSETR::ETHCKLPEN;
    pub use super::RCC_MP_AHB6LPENSETR::ETHMACLPEN;
    pub use super::RCC_MP_AHB6LPENSETR::ETHRXLPEN;
    pub use super::RCC_MP_AHB6LPENSETR::ETHSTPEN;
    pub use super::RCC_MP_AHB6LPENSETR::ETHTXLPEN;
    pub use super::RCC_MP_AHB6LPENSETR::FMCLPEN;
    pub use super::RCC_MP_AHB6LPENSETR::GPULPEN;
    pub use super::RCC_MP_AHB6LPENSETR::MDMALPEN;
    pub use super::RCC_MP_AHB6LPENSETR::QSPILPEN;
    pub use super::RCC_MP_AHB6LPENSETR::SDMMC1LPEN;
    pub use super::RCC_MP_AHB6LPENSETR::SDMMC2LPEN;
    pub use super::RCC_MP_AHB6LPENSETR::USBHLPEN;
}

/// This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\[1:0\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\[1:0\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_BR_RSTSCLRR {

    /// PORRSTF
    pub mod PORRSTF {
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

    /// BORRSTF
    pub mod BORRSTF {
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

    /// PADRSTF
    pub mod PADRSTF {
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

    /// HCSSRSTF
    pub mod HCSSRSTF {
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

    /// VCORERSTF
    pub mod VCORERSTF {
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

    /// MPSYSRSTF
    pub mod MPSYSRSTF {
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

    /// MCSYSRSTF
    pub mod MCSYSRSTF {
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

    /// IWDG1RSTF
    pub mod IWDG1RSTF {
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

    /// IWDG2RSTF
    pub mod IWDG2RSTF {
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

    /// MPUP0RSTF
    pub mod MPUP0RSTF {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MPUP1RSTF
    pub mod MPUP1RSTF {
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
}

/// This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset.
pub mod RCC_MP_GRSTCSETR {

    /// MPSYSRST
    pub mod MPSYSRST {
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

    /// MCURST
    pub mod MCURST {
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

    /// MPUP0RST
    pub mod MPUP0RST {
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

    /// MPUP1RST
    pub mod MPUP1RST {
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
}

/// This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_RSTSCLRR {

    /// PORRSTF
    pub mod PORRSTF {
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

    /// BORRSTF
    pub mod BORRSTF {
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

    /// PADRSTF
    pub mod PADRSTF {
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

    /// HCSSRSTF
    pub mod HCSSRSTF {
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

    /// VCORERSTF
    pub mod VCORERSTF {
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

    /// MPSYSRSTF
    pub mod MPSYSRSTF {
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

    /// MCSYSRSTF
    pub mod MCSYSRSTF {
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

    /// IWDG1RSTF
    pub mod IWDG1RSTF {
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

    /// IWDG2RSTF
    pub mod IWDG2RSTF {
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

    /// STDBYRSTF
    pub mod STDBYRSTF {
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

    /// CSTDBYRSTF
    pub mod CSTDBYRSTF {
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

    /// MPUP0RSTF
    pub mod MPUP0RSTF {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MPUP1RSTF
    pub mod MPUP1RSTF {
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

    /// SPARE
    pub mod SPARE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_IWDGFZSETR {

    /// FZ_IWDG1
    pub mod FZ_IWDG1 {
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

    /// FZ_IWDG2
    pub mod FZ_IWDG2 {
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
}

/// This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_IWDGFZCLRR {
    pub use super::RCC_MP_IWDGFZSETR::FZ_IWDG1;
    pub use super::RCC_MP_IWDGFZSETR::FZ_IWDG2;
}

/// This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_CIER {

    /// LSIRDYIE
    pub mod LSIRDYIE {
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

    /// LSERDYIE
    pub mod LSERDYIE {
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

    /// HSIRDYIE
    pub mod HSIRDYIE {
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

    /// HSERDYIE
    pub mod HSERDYIE {
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

    /// CSIRDYIE
    pub mod CSIRDYIE {
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

    /// PLL1DYIE
    pub mod PLL1DYIE {
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

    /// PLL2DYIE
    pub mod PLL2DYIE {
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

    /// PLL3DYIE
    pub mod PLL3DYIE {
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

    /// PLL4DYIE
    pub mod PLL4DYIE {
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

    /// LSECSSIE
    pub mod LSECSSIE {
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

    /// WKUPIE
    pub mod WKUPIE {
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
}

/// This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_CIFR {

    /// LSIRDYF
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

    /// LSERDYF
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

    /// HSIRDYF
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

    /// HSERDYF
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

    /// CSIRDYF
    pub mod CSIRDYF {
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

    /// PLL1DYF
    pub mod PLL1DYF {
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

    /// PLL2DYF
    pub mod PLL2DYF {
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

    /// PLL3DYF
    pub mod PLL3DYF {
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

    /// PLL4DYF
    pub mod PLL4DYF {
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

    /// LSECSSF
    pub mod LSECSSF {
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

    /// WKUPF
    pub mod WKUPF {
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
}

/// This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_PWRLPDLYCR {

    /// PWRLP_DLY
    pub mod PWRLP_DLY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (22 bits: 0x3fffff << 0)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MCTMPSKP
    pub mod MCTMPSKP {
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
}

/// This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MP_RSTSSETR {
    pub use super::RCC_MP_RSTSCLRR::BORRSTF;
    pub use super::RCC_MP_RSTSCLRR::CSTDBYRSTF;
    pub use super::RCC_MP_RSTSCLRR::HCSSRSTF;
    pub use super::RCC_MP_RSTSCLRR::IWDG1RSTF;
    pub use super::RCC_MP_RSTSCLRR::IWDG2RSTF;
    pub use super::RCC_MP_RSTSCLRR::MCSYSRSTF;
    pub use super::RCC_MP_RSTSCLRR::MPSYSRSTF;
    pub use super::RCC_MP_RSTSCLRR::MPUP0RSTF;
    pub use super::RCC_MP_RSTSCLRR::MPUP1RSTF;
    pub use super::RCC_MP_RSTSCLRR::PADRSTF;
    pub use super::RCC_MP_RSTSCLRR::PORRSTF;
    pub use super::RCC_MP_RSTSCLRR::SPARE;
    pub use super::RCC_MP_RSTSCLRR::STDBYRSTF;
    pub use super::RCC_MP_RSTSCLRR::VCORERSTF;
}

/// This register is used to select the clock generated on MCO1 output.
pub mod RCC_MCO1CFGR {

    /// MCO1SEL
    pub mod MCO1SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MCO1DIV
    pub mod MCO1DIV {
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

    /// MCO1ON
    pub mod MCO1ON {
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
}

/// This register is used to select the clock generated on MCO2 output.
pub mod RCC_MCO2CFGR {

    /// MCO2SEL
    pub mod MCO2SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MCO2DIV
    pub mod MCO2DIV {
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

    /// MCO2ON
    pub mod MCO2ON {
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
}

/// This is a read-only access register, It contains the status flags of oscillators. Writing has no effect.
pub mod RCC_OCRDYR {

    /// HSIRDY
    pub mod HSIRDY {
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

    /// HSIDIVRDY
    pub mod HSIDIVRDY {
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

    /// CSIRDY
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

    /// HSERDY
    pub mod HSERDY {
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

    /// MPUCKRDY
    pub mod MPUCKRDY {
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

    /// AXICKRDY
    pub mod AXICKRDY {
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

    /// CKREST
    pub mod CKREST {
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
}

/// This is register contains the enable control of the debug and trace function, and the clock divider for the trace function.
pub mod RCC_DBGCFGR {

    /// TRACEDIV
    pub mod TRACEDIV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DBGCKEN
    pub mod DBGCKEN {
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

    /// TRACECKEN
    pub mod TRACECKEN {
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

    /// DBGRST
    pub mod DBGRST {
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
}

/// This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod RCC_RCK3SELR {

    /// PLL3SRC
    pub mod PLL3SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL3SRCRDY
    pub mod PLL3SRCRDY {
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

/// This register is used to select the reference clock for PLL4.
pub mod RCC_RCK4SELR {

    /// PLL4SRC
    pub mod PLL4SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL4SRCRDY
    pub mod PLL4SRCRDY {
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

/// This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information.
pub mod RCC_TIMG1PRER {

    /// TIMG1PRE
    pub mod TIMG1PRE {
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

    /// TIMG1PRERDY
    pub mod TIMG1PRERDY {
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

/// This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information.
pub mod RCC_TIMG2PRER {

    /// TIMG2PRE
    pub mod TIMG2PRE {
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

    /// TIMG2PRERDY
    pub mod TIMG2PRERDY {
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

/// This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
pub mod RCC_MCUDIVR {

    /// MCUDIV
    pub mod MCUDIV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MCUDIVRDY
    pub mod MCUDIVRDY {
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

/// This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information.
pub mod RCC_APB1DIVR {

    /// APB1DIV
    pub mod APB1DIV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// APB1DIVRDY
    pub mod APB1DIVRDY {
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

/// This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information.
pub mod RCC_APB2DIVR {

    /// APB2DIV
    pub mod APB2DIV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// APB2DIVRDY
    pub mod APB2DIVRDY {
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

/// This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information.
pub mod RCC_APB3DIVR {

    /// APB3DIV
    pub mod APB3DIV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// APB3DIVRDY
    pub mod APB3DIVRDY {
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

/// This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod RCC_PLL3CR {

    /// PLLON
    pub mod PLLON {
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

    /// PLL3RDY
    pub mod PLL3RDY {
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

    /// SSCG_CTRL
    pub mod SSCG_CTRL {
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

    /// DIVPEN
    pub mod DIVPEN {
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

    /// DIVQEN
    pub mod DIVQEN {
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

    /// DIVREN
    pub mod DIVREN {
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
}

/// This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod RCC_PLL3CFGR1 {

    /// DIVN
    pub mod DIVN {
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

    /// DIVM3
    pub mod DIVM3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IFRGE
    pub mod IFRGE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod RCC_PLL3CFGR2 {
    pub use super::RCC_PLL1CFGR2::DIVP;
    pub use super::RCC_PLL1CFGR2::DIVQ;
    pub use super::RCC_PLL1CFGR2::DIVR;
}

/// This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod RCC_PLL3FRACR {
    pub use super::RCC_PLL1FRACR::FRACLE;
    pub use super::RCC_PLL1FRACR::FRACV;
}

/// This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod RCC_PLL3CSGR {
    pub use super::RCC_PLL1CSGR::INC_STEP;
    pub use super::RCC_PLL1CSGR::MOD_PER;
    pub use super::RCC_PLL1CSGR::RPDFN_DIS;
    pub use super::RCC_PLL1CSGR::SSCG_MODE;
    pub use super::RCC_PLL1CSGR::TPDFN_DIS;
}

/// This register is used to control the PLL4.
pub mod RCC_PLL4CR {

    /// PLLON
    pub mod PLLON {
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

    /// PLL4RDY
    pub mod PLL4RDY {
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

    /// SSCG_CTRL
    pub mod SSCG_CTRL {
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

    /// DIVPEN
    pub mod DIVPEN {
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

    /// DIVQEN
    pub mod DIVQEN {
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

    /// DIVREN
    pub mod DIVREN {
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
}

/// This register is used to configure the PLL4.
pub mod RCC_PLL4CFGR1 {

    /// DIVN
    pub mod DIVN {
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

    /// DIVM4
    pub mod DIVM4 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IFRGE
    pub mod IFRGE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to configure the PLL4.
pub mod RCC_PLL4CFGR2 {
    pub use super::RCC_PLL1CFGR2::DIVP;
    pub use super::RCC_PLL1CFGR2::DIVQ;
    pub use super::RCC_PLL1CFGR2::DIVR;
}

/// This register is used to fine-tune the frequency of the PLL4 VCO.
pub mod RCC_PLL4FRACR {
    pub use super::RCC_PLL1FRACR::FRACLE;
    pub use super::RCC_PLL1FRACR::FRACV;
}

/// This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod RCC_PLL4CSGR {
    pub use super::RCC_PLL1CSGR::INC_STEP;
    pub use super::RCC_PLL1CSGR::MOD_PER;
    pub use super::RCC_PLL1CSGR::RPDFN_DIS;
    pub use super::RCC_PLL1CSGR::SSCG_MODE;
    pub use super::RCC_PLL1CSGR::TPDFN_DIS;
}

/// This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_I2C12CKSELR {

    /// I2C12SRC
    pub mod I2C12SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the I2C3 and I2C5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_I2C35CKSELR {

    /// I2C35SRC
    pub mod I2C35SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_SAI1CKSELR {

    /// SAI1SRC
    pub mod SAI1SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_SAI2CKSELR {

    /// SAI2SRC
    pub mod SAI2SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_SAI3CKSELR {

    /// SAI3SRC
    pub mod SAI3SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_SAI4CKSELR {

    /// SAI4SRC
    pub mod SAI4SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_SPI2S1CKSELR {

    /// SPI1SRC
    pub mod SPI1SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_SPI2S23CKSELR {

    /// SPI23SRC
    pub mod SPI23SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_SPI45CKSELR {

    /// SPI45SRC
    pub mod SPI45SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_UART6CKSELR {

    /// UART6SRC
    pub mod UART6SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_UART24CKSELR {

    /// UART24SRC
    pub mod UART24SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_UART35CKSELR {

    /// UART35SRC
    pub mod UART35SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_UART78CKSELR {

    /// UART78SRC
    pub mod UART78SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_SDMMC12CKSELR {

    /// SDMMC12SRC
    pub mod SDMMC12SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_SDMMC3CKSELR {

    /// SDMMC3SRC
    pub mod SDMMC3SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_ETHCKSELR {

    /// ETHSRC
    pub mod ETHSRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ETHPTPDIV
    pub mod ETHPTPDIV {
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
}

/// This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_QSPICKSELR {

    /// QSPISRC
    pub mod QSPISRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_FMCCKSELR {

    /// FMCSRC
    pub mod FMCSRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_FDCANCKSELR {

    /// FDCANSRC
    pub mod FDCANSRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod RCC_SPDIFCKSELR {

    /// SPDIFSRC
    pub mod SPDIFSRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the CEC-HDMI.
pub mod RCC_CECCKSELR {

    /// CECSRC
    pub mod CECSRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG
pub mod RCC_USBCKSELR {

    /// USBPHYSRC
    pub mod USBPHYSRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USBOSRC
    pub mod USBOSRC {
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
}

/// This register is used to control the selection of the kernel clock for the RNG2.
pub mod RCC_RNG2CKSELR {

    /// RNG2SRC
    pub mod RNG2SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the DSI block.
pub mod RCC_DSICKSELR {

    /// DSISRC
    pub mod DSISRC {
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

/// This register is used to control the selection of the kernel clock for the ADC block.
pub mod RCC_ADCCKSELR {

    /// ADCSRC
    pub mod ADCSRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks.
pub mod RCC_LPTIM45CKSELR {

    /// LPTIM45SRC
    pub mod LPTIM45SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks.
pub mod RCC_LPTIM23CKSELR {

    /// LPTIM23SRC
    pub mod LPTIM23SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the selection of the kernel clock for the LPTIM1 block.
pub mod RCC_LPTIM1CKSELR {

    /// LPTIM1SRC
    pub mod LPTIM1SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to activate the reset of the corresponding peripheral.
pub mod RCC_APB1RSTSETR {

    /// TIM2RST
    pub mod TIM2RST {
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

    /// TIM3RST
    pub mod TIM3RST {
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

    /// TIM4RST
    pub mod TIM4RST {
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

    /// TIM5RST
    pub mod TIM5RST {
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

    /// TIM6RST
    pub mod TIM6RST {
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

    /// TIM7RST
    pub mod TIM7RST {
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

    /// TIM12RST
    pub mod TIM12RST {
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

    /// TIM13RST
    pub mod TIM13RST {
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

    /// TIM14RST
    pub mod TIM14RST {
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

    /// LPTIM1RST
    pub mod LPTIM1RST {
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

    /// SPI2RST
    pub mod SPI2RST {
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

    /// SPI3RST
    pub mod SPI3RST {
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

    /// USART2RST
    pub mod USART2RST {
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

    /// USART3RST
    pub mod USART3RST {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// UART4RST
    pub mod UART4RST {
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

    /// UART5RST
    pub mod UART5RST {
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

    /// UART7RST
    pub mod UART7RST {
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

    /// UART8RST
    pub mod UART8RST {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C1RST
    pub mod I2C1RST {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C2RST
    pub mod I2C2RST {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C3RST
    pub mod I2C3RST {
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

    /// I2C5RST
    pub mod I2C5RST {
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

    /// SPDIFRST
    pub mod SPDIFRST {
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

    /// CECRST
    pub mod CECRST {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC12RST
    pub mod DAC12RST {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MDIOSRST
    pub mod MDIOSRST {
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

/// This register is used to release the reset of the corresponding peripheral.
pub mod RCC_APB1RSTCLRR {
    pub use super::RCC_APB1RSTSETR::CECRST;
    pub use super::RCC_APB1RSTSETR::DAC12RST;
    pub use super::RCC_APB1RSTSETR::I2C1RST;
    pub use super::RCC_APB1RSTSETR::I2C2RST;
    pub use super::RCC_APB1RSTSETR::I2C3RST;
    pub use super::RCC_APB1RSTSETR::I2C5RST;
    pub use super::RCC_APB1RSTSETR::LPTIM1RST;
    pub use super::RCC_APB1RSTSETR::MDIOSRST;
    pub use super::RCC_APB1RSTSETR::SPDIFRST;
    pub use super::RCC_APB1RSTSETR::SPI2RST;
    pub use super::RCC_APB1RSTSETR::SPI3RST;
    pub use super::RCC_APB1RSTSETR::TIM12RST;
    pub use super::RCC_APB1RSTSETR::TIM13RST;
    pub use super::RCC_APB1RSTSETR::TIM14RST;
    pub use super::RCC_APB1RSTSETR::TIM2RST;
    pub use super::RCC_APB1RSTSETR::TIM3RST;
    pub use super::RCC_APB1RSTSETR::TIM4RST;
    pub use super::RCC_APB1RSTSETR::TIM5RST;
    pub use super::RCC_APB1RSTSETR::TIM6RST;
    pub use super::RCC_APB1RSTSETR::TIM7RST;
    pub use super::RCC_APB1RSTSETR::UART4RST;
    pub use super::RCC_APB1RSTSETR::UART5RST;
    pub use super::RCC_APB1RSTSETR::UART7RST;
    pub use super::RCC_APB1RSTSETR::UART8RST;
    pub use super::RCC_APB1RSTSETR::USART2RST;
    pub use super::RCC_APB1RSTSETR::USART3RST;
}

/// This register is used to activate the reset of the corresponding peripheral.
pub mod RCC_APB2RSTSETR {

    /// TIM1RST
    pub mod TIM1RST {
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

    /// TIM8RST
    pub mod TIM8RST {
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

    /// TIM15RST
    pub mod TIM15RST {
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

    /// TIM16RST
    pub mod TIM16RST {
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

    /// TIM17RST
    pub mod TIM17RST {
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

    /// SPI1RST
    pub mod SPI1RST {
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

    /// SPI4RST
    pub mod SPI4RST {
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

    /// SPI5RST
    pub mod SPI5RST {
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

    /// USART6RST
    pub mod USART6RST {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI1RST
    pub mod SAI1RST {
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

    /// SAI2RST
    pub mod SAI2RST {
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

    /// SAI3RST
    pub mod SAI3RST {
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

    /// DFSDMRST
    pub mod DFSDMRST {
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

    /// FDCANRST
    pub mod FDCANRST {
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
}

/// This register is used to release the reset of the corresponding peripheral.
pub mod RCC_APB2RSTCLRR {
    pub use super::RCC_APB2RSTSETR::DFSDMRST;
    pub use super::RCC_APB2RSTSETR::FDCANRST;
    pub use super::RCC_APB2RSTSETR::SAI1RST;
    pub use super::RCC_APB2RSTSETR::SAI2RST;
    pub use super::RCC_APB2RSTSETR::SAI3RST;
    pub use super::RCC_APB2RSTSETR::SPI1RST;
    pub use super::RCC_APB2RSTSETR::SPI4RST;
    pub use super::RCC_APB2RSTSETR::SPI5RST;
    pub use super::RCC_APB2RSTSETR::TIM15RST;
    pub use super::RCC_APB2RSTSETR::TIM16RST;
    pub use super::RCC_APB2RSTSETR::TIM17RST;
    pub use super::RCC_APB2RSTSETR::TIM1RST;
    pub use super::RCC_APB2RSTSETR::TIM8RST;
    pub use super::RCC_APB2RSTSETR::USART6RST;
}

/// This register is used to activate the reset of the corresponding peripheral.
pub mod RCC_APB3RSTSETR {

    /// LPTIM2RST
    pub mod LPTIM2RST {
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

    /// LPTIM3RST
    pub mod LPTIM3RST {
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

    /// LPTIM4RST
    pub mod LPTIM4RST {
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

    /// LPTIM5RST
    pub mod LPTIM5RST {
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

    /// SAI4RST
    pub mod SAI4RST {
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

    /// SYSCFGRST
    pub mod SYSCFGRST {
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

    /// VREFRST
    pub mod VREFRST {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DTSRST
    pub mod DTSRST {
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

/// This register is used to release the reset of the corresponding peripheral.
pub mod RCC_APB3RSTCLRR {
    pub use super::RCC_APB3RSTSETR::DTSRST;
    pub use super::RCC_APB3RSTSETR::LPTIM2RST;
    pub use super::RCC_APB3RSTSETR::LPTIM3RST;
    pub use super::RCC_APB3RSTSETR::LPTIM4RST;
    pub use super::RCC_APB3RSTSETR::LPTIM5RST;
    pub use super::RCC_APB3RSTSETR::SAI4RST;
    pub use super::RCC_APB3RSTSETR::SYSCFGRST;
    pub use super::RCC_APB3RSTSETR::VREFRST;
}

/// This register is used to activate the reset of the corresponding peripheral.
pub mod RCC_AHB2RSTSETR {

    /// DMA1RST
    pub mod DMA1RST {
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

    /// DMA2RST
    pub mod DMA2RST {
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

    /// DMAMUXRST
    pub mod DMAMUXRST {
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

    /// ADC12RST
    pub mod ADC12RST {
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

    /// USBORST
    pub mod USBORST {
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

    /// SDMMC3RST
    pub mod SDMMC3RST {
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

/// This register is used to release the reset of the corresponding peripheral.
pub mod RCC_AHB2RSTCLRR {
    pub use super::RCC_AHB2RSTSETR::ADC12RST;
    pub use super::RCC_AHB2RSTSETR::DMA1RST;
    pub use super::RCC_AHB2RSTSETR::DMA2RST;
    pub use super::RCC_AHB2RSTSETR::DMAMUXRST;
    pub use super::RCC_AHB2RSTSETR::SDMMC3RST;
    pub use super::RCC_AHB2RSTSETR::USBORST;
}

/// This register is used to activate the reset of the corresponding peripheral.
pub mod RCC_AHB3RSTSETR {

    /// DCMIRST
    pub mod DCMIRST {
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

    /// CRYP2RST
    pub mod CRYP2RST {
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

    /// HASH2RST
    pub mod HASH2RST {
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

    /// RNG2RST
    pub mod RNG2RST {
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

    /// CRC2RST
    pub mod CRC2RST {
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

    /// HSEMRST
    pub mod HSEMRST {
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

    /// IPCCRST
    pub mod IPCCRST {
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
}

/// This register is used to release the reset of the corresponding peripheral.
pub mod RCC_AHB3RSTCLRR {
    pub use super::RCC_AHB3RSTSETR::CRC2RST;
    pub use super::RCC_AHB3RSTSETR::CRYP2RST;
    pub use super::RCC_AHB3RSTSETR::DCMIRST;
    pub use super::RCC_AHB3RSTSETR::HASH2RST;
    pub use super::RCC_AHB3RSTSETR::HSEMRST;
    pub use super::RCC_AHB3RSTSETR::IPCCRST;
    pub use super::RCC_AHB3RSTSETR::RNG2RST;
}

/// This register is used to activate the reset of the corresponding peripheral
pub mod RCC_AHB4RSTSETR {

    /// GPIOARST
    pub mod GPIOARST {
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

    /// GPIOBRST
    pub mod GPIOBRST {
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

    /// GPIOCRST
    pub mod GPIOCRST {
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

    /// GPIODRST
    pub mod GPIODRST {
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

    /// GPIOERST
    pub mod GPIOERST {
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

    /// GPIOFRST
    pub mod GPIOFRST {
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

    /// GPIOGRST
    pub mod GPIOGRST {
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

    /// GPIOHRST
    pub mod GPIOHRST {
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

    /// GPIOIRST
    pub mod GPIOIRST {
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

    /// GPIOJRST
    pub mod GPIOJRST {
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

    /// GPIOKRST
    pub mod GPIOKRST {
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

/// This register is used to release the reset of the corresponding peripheral.
pub mod RCC_AHB4RSTCLRR {
    pub use super::RCC_AHB4RSTSETR::GPIOARST;
    pub use super::RCC_AHB4RSTSETR::GPIOBRST;
    pub use super::RCC_AHB4RSTSETR::GPIOCRST;
    pub use super::RCC_AHB4RSTSETR::GPIODRST;
    pub use super::RCC_AHB4RSTSETR::GPIOERST;
    pub use super::RCC_AHB4RSTSETR::GPIOFRST;
    pub use super::RCC_AHB4RSTSETR::GPIOGRST;
    pub use super::RCC_AHB4RSTSETR::GPIOHRST;
    pub use super::RCC_AHB4RSTSETR::GPIOIRST;
    pub use super::RCC_AHB4RSTSETR::GPIOJRST;
    pub use super::RCC_AHB4RSTSETR::GPIOKRST;
}

/// This register is used to set the peripheral clock enable bit
pub mod RCC_MP_APB1ENSETR {

    /// TIM2EN
    pub mod TIM2EN {
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

    /// TIM3EN
    pub mod TIM3EN {
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

    /// TIM4EN
    pub mod TIM4EN {
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

    /// TIM5EN
    pub mod TIM5EN {
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

    /// TIM6EN
    pub mod TIM6EN {
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

    /// TIM7EN
    pub mod TIM7EN {
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

    /// TIM12EN
    pub mod TIM12EN {
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

    /// TIM13EN
    pub mod TIM13EN {
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

    /// TIM14EN
    pub mod TIM14EN {
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

    /// LPTIM1EN
    pub mod LPTIM1EN {
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

    /// SPI2EN
    pub mod SPI2EN {
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

    /// SPI3EN
    pub mod SPI3EN {
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

    /// USART2EN
    pub mod USART2EN {
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

    /// USART3EN
    pub mod USART3EN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// UART4EN
    pub mod UART4EN {
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

    /// UART5EN
    pub mod UART5EN {
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

    /// UART7EN
    pub mod UART7EN {
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

    /// UART8EN
    pub mod UART8EN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C1EN
    pub mod I2C1EN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C2EN
    pub mod I2C2EN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C3EN
    pub mod I2C3EN {
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

    /// I2C5EN
    pub mod I2C5EN {
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

    /// SPDIFEN
    pub mod SPDIFEN {
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

    /// CECEN
    pub mod CECEN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC12EN
    pub mod DAC12EN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MDIOSEN
    pub mod MDIOSEN {
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

/// This register is used to clear the peripheral clock enable bit
pub mod RCC_MP_APB1ENCLRR {
    pub use super::RCC_MP_APB1ENSETR::CECEN;
    pub use super::RCC_MP_APB1ENSETR::DAC12EN;
    pub use super::RCC_MP_APB1ENSETR::I2C1EN;
    pub use super::RCC_MP_APB1ENSETR::I2C2EN;
    pub use super::RCC_MP_APB1ENSETR::I2C3EN;
    pub use super::RCC_MP_APB1ENSETR::I2C5EN;
    pub use super::RCC_MP_APB1ENSETR::LPTIM1EN;
    pub use super::RCC_MP_APB1ENSETR::MDIOSEN;
    pub use super::RCC_MP_APB1ENSETR::SPDIFEN;
    pub use super::RCC_MP_APB1ENSETR::SPI2EN;
    pub use super::RCC_MP_APB1ENSETR::SPI3EN;
    pub use super::RCC_MP_APB1ENSETR::TIM12EN;
    pub use super::RCC_MP_APB1ENSETR::TIM13EN;
    pub use super::RCC_MP_APB1ENSETR::TIM14EN;
    pub use super::RCC_MP_APB1ENSETR::TIM2EN;
    pub use super::RCC_MP_APB1ENSETR::TIM3EN;
    pub use super::RCC_MP_APB1ENSETR::TIM4EN;
    pub use super::RCC_MP_APB1ENSETR::TIM5EN;
    pub use super::RCC_MP_APB1ENSETR::TIM6EN;
    pub use super::RCC_MP_APB1ENSETR::TIM7EN;
    pub use super::RCC_MP_APB1ENSETR::UART4EN;
    pub use super::RCC_MP_APB1ENSETR::UART5EN;
    pub use super::RCC_MP_APB1ENSETR::UART7EN;
    pub use super::RCC_MP_APB1ENSETR::UART8EN;
    pub use super::RCC_MP_APB1ENSETR::USART2EN;
    pub use super::RCC_MP_APB1ENSETR::USART3EN;
}

/// This register is used to set the peripheral clock enable bit
pub mod RCC_MP_APB2ENSETR {

    /// TIM1EN
    pub mod TIM1EN {
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

    /// TIM8EN
    pub mod TIM8EN {
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

    /// TIM15EN
    pub mod TIM15EN {
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

    /// TIM16EN
    pub mod TIM16EN {
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

    /// TIM17EN
    pub mod TIM17EN {
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

    /// SPI1EN
    pub mod SPI1EN {
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

    /// SPI4EN
    pub mod SPI4EN {
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

    /// SPI5EN
    pub mod SPI5EN {
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

    /// USART6EN
    pub mod USART6EN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI1EN
    pub mod SAI1EN {
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

    /// SAI2EN
    pub mod SAI2EN {
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

    /// SAI3EN
    pub mod SAI3EN {
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

    /// DFSDMEN
    pub mod DFSDMEN {
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

    /// ADFSDMEN
    pub mod ADFSDMEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FDCANEN
    pub mod FDCANEN {
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
}

/// This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
pub mod RCC_MP_APB2ENCLRR {
    pub use super::RCC_MP_APB2ENSETR::ADFSDMEN;
    pub use super::RCC_MP_APB2ENSETR::DFSDMEN;
    pub use super::RCC_MP_APB2ENSETR::FDCANEN;
    pub use super::RCC_MP_APB2ENSETR::SAI1EN;
    pub use super::RCC_MP_APB2ENSETR::SAI2EN;
    pub use super::RCC_MP_APB2ENSETR::SAI3EN;
    pub use super::RCC_MP_APB2ENSETR::SPI1EN;
    pub use super::RCC_MP_APB2ENSETR::SPI4EN;
    pub use super::RCC_MP_APB2ENSETR::SPI5EN;
    pub use super::RCC_MP_APB2ENSETR::TIM15EN;
    pub use super::RCC_MP_APB2ENSETR::TIM16EN;
    pub use super::RCC_MP_APB2ENSETR::TIM17EN;
    pub use super::RCC_MP_APB2ENSETR::TIM1EN;
    pub use super::RCC_MP_APB2ENSETR::TIM8EN;
    pub use super::RCC_MP_APB2ENSETR::USART6EN;
}

/// This register is used to set the peripheral clock enable bit
pub mod RCC_MP_APB3ENSETR {

    /// LPTIM2EN
    pub mod LPTIM2EN {
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

    /// LPTIM3EN
    pub mod LPTIM3EN {
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

    /// LPTIM4EN
    pub mod LPTIM4EN {
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

    /// LPTIM5EN
    pub mod LPTIM5EN {
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

    /// SAI4EN
    pub mod SAI4EN {
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

    /// SYSCFGEN
    pub mod SYSCFGEN {
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

    /// VREFEN
    pub mod VREFEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DTSEN
    pub mod DTSEN {
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

    /// HDPEN
    pub mod HDPEN {
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
}

/// This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
pub mod RCC_MP_APB3ENCLRR {
    pub use super::RCC_MP_APB3ENSETR::DTSEN;
    pub use super::RCC_MP_APB3ENSETR::HDPEN;
    pub use super::RCC_MP_APB3ENSETR::LPTIM2EN;
    pub use super::RCC_MP_APB3ENSETR::LPTIM3EN;
    pub use super::RCC_MP_APB3ENSETR::LPTIM4EN;
    pub use super::RCC_MP_APB3ENSETR::LPTIM5EN;
    pub use super::RCC_MP_APB3ENSETR::SAI4EN;
    pub use super::RCC_MP_APB3ENSETR::SYSCFGEN;
    pub use super::RCC_MP_APB3ENSETR::VREFEN;
}

/// This register is used to set the peripheral clock enable bit of the corresponding peripheral
pub mod RCC_MP_AHB2ENSETR {

    /// DMA1EN
    pub mod DMA1EN {
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

    /// DMA2EN
    pub mod DMA2EN {
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

    /// DMAMUXEN
    pub mod DMAMUXEN {
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

    /// ADC12EN
    pub mod ADC12EN {
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

    /// USBOEN
    pub mod USBOEN {
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

    /// SDMMC3EN
    pub mod SDMMC3EN {
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

/// This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
pub mod RCC_MP_AHB2ENCLRR {
    pub use super::RCC_MP_AHB2ENSETR::ADC12EN;
    pub use super::RCC_MP_AHB2ENSETR::DMA1EN;
    pub use super::RCC_MP_AHB2ENSETR::DMA2EN;
    pub use super::RCC_MP_AHB2ENSETR::DMAMUXEN;
    pub use super::RCC_MP_AHB2ENSETR::SDMMC3EN;
    pub use super::RCC_MP_AHB2ENSETR::USBOEN;
}

/// This register is used to set the peripheral clock enable bit of the corresponding peripheral
pub mod RCC_MP_AHB3ENSETR {

    /// DCMIEN
    pub mod DCMIEN {
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

    /// CRYP2EN
    pub mod CRYP2EN {
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

    /// HASH2EN
    pub mod HASH2EN {
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

    /// RNG2EN
    pub mod RNG2EN {
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

    /// CRC2EN
    pub mod CRC2EN {
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

    /// HSEMEN
    pub mod HSEMEN {
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

    /// IPCCEN
    pub mod IPCCEN {
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
}

/// This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
pub mod RCC_MP_AHB3ENCLRR {
    pub use super::RCC_MP_AHB3ENSETR::CRC2EN;
    pub use super::RCC_MP_AHB3ENSETR::CRYP2EN;
    pub use super::RCC_MP_AHB3ENSETR::DCMIEN;
    pub use super::RCC_MP_AHB3ENSETR::HASH2EN;
    pub use super::RCC_MP_AHB3ENSETR::HSEMEN;
    pub use super::RCC_MP_AHB3ENSETR::IPCCEN;
    pub use super::RCC_MP_AHB3ENSETR::RNG2EN;
}

/// This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU.
pub mod RCC_MP_AHB4ENSETR {

    /// GPIOAEN
    pub mod GPIOAEN {
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

    /// GPIOBEN
    pub mod GPIOBEN {
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

    /// GPIOCEN
    pub mod GPIOCEN {
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

    /// GPIODEN
    pub mod GPIODEN {
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

    /// GPIOEEN
    pub mod GPIOEEN {
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

    /// GPIOFEN
    pub mod GPIOFEN {
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

    /// GPIOGEN
    pub mod GPIOGEN {
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

    /// GPIOHEN
    pub mod GPIOHEN {
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

    /// GPIOIEN
    pub mod GPIOIEN {
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

    /// GPIOJEN
    pub mod GPIOJEN {
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

    /// GPIOKEN
    pub mod GPIOKEN {
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

/// This register is used to clear the peripheral clock enable bit
pub mod RCC_MP_AHB4ENCLRR {
    pub use super::RCC_MP_AHB4ENSETR::GPIOAEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOBEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOCEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIODEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOEEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOFEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOGEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOHEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOIEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOJEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOKEN;
}

/// This register is used to set the peripheral clock enable bit
pub mod RCC_MP_MLAHBENSETR {

    /// RETRAMEN
    pub mod RETRAMEN {
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
}

/// This register is used to clear the peripheral clock enable bit.
pub mod RCC_MP_MLAHBENCLRR {
    pub use super::RCC_MP_MLAHBENSETR::RETRAMEN;
}

/// This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MCU. Writing has no effect, reading will return . Writing a sets the corresponding bit to .
pub mod RCC_MC_APB1ENSETR {

    /// TIM2EN
    pub mod TIM2EN {
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

    /// TIM3EN
    pub mod TIM3EN {
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

    /// TIM4EN
    pub mod TIM4EN {
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

    /// TIM5EN
    pub mod TIM5EN {
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

    /// TIM6EN
    pub mod TIM6EN {
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

    /// TIM7EN
    pub mod TIM7EN {
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

    /// TIM12EN
    pub mod TIM12EN {
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

    /// TIM13EN
    pub mod TIM13EN {
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

    /// TIM14EN
    pub mod TIM14EN {
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

    /// LPTIM1EN
    pub mod LPTIM1EN {
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

    /// SPI2EN
    pub mod SPI2EN {
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

    /// SPI3EN
    pub mod SPI3EN {
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

    /// USART2EN
    pub mod USART2EN {
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

    /// USART3EN
    pub mod USART3EN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// UART4EN
    pub mod UART4EN {
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

    /// UART5EN
    pub mod UART5EN {
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

    /// UART7EN
    pub mod UART7EN {
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

    /// UART8EN
    pub mod UART8EN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C1EN
    pub mod I2C1EN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C2EN
    pub mod I2C2EN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C3EN
    pub mod I2C3EN {
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

    /// I2C5EN
    pub mod I2C5EN {
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

    /// SPDIFEN
    pub mod SPDIFEN {
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

    /// CECEN
    pub mod CECEN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WWDG1EN
    pub mod WWDG1EN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC12EN
    pub mod DAC12EN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MDIOSEN
    pub mod MDIOSEN {
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

/// This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
pub mod RCC_MC_APB1ENCLRR {
    pub use super::RCC_MP_APB1ENSETR::CECEN;
    pub use super::RCC_MP_APB1ENSETR::DAC12EN;
    pub use super::RCC_MP_APB1ENSETR::I2C1EN;
    pub use super::RCC_MP_APB1ENSETR::I2C2EN;
    pub use super::RCC_MP_APB1ENSETR::I2C3EN;
    pub use super::RCC_MP_APB1ENSETR::I2C5EN;
    pub use super::RCC_MP_APB1ENSETR::LPTIM1EN;
    pub use super::RCC_MP_APB1ENSETR::MDIOSEN;
    pub use super::RCC_MP_APB1ENSETR::SPDIFEN;
    pub use super::RCC_MP_APB1ENSETR::SPI2EN;
    pub use super::RCC_MP_APB1ENSETR::SPI3EN;
    pub use super::RCC_MP_APB1ENSETR::TIM12EN;
    pub use super::RCC_MP_APB1ENSETR::TIM13EN;
    pub use super::RCC_MP_APB1ENSETR::TIM14EN;
    pub use super::RCC_MP_APB1ENSETR::TIM2EN;
    pub use super::RCC_MP_APB1ENSETR::TIM3EN;
    pub use super::RCC_MP_APB1ENSETR::TIM4EN;
    pub use super::RCC_MP_APB1ENSETR::TIM5EN;
    pub use super::RCC_MP_APB1ENSETR::TIM6EN;
    pub use super::RCC_MP_APB1ENSETR::TIM7EN;
    pub use super::RCC_MP_APB1ENSETR::UART4EN;
    pub use super::RCC_MP_APB1ENSETR::UART5EN;
    pub use super::RCC_MP_APB1ENSETR::UART7EN;
    pub use super::RCC_MP_APB1ENSETR::UART8EN;
    pub use super::RCC_MP_APB1ENSETR::USART2EN;
    pub use super::RCC_MP_APB1ENSETR::USART3EN;
}

/// This register is used to set the peripheral clock enable bit
pub mod RCC_MC_APB2ENSETR {
    pub use super::RCC_MP_APB2ENSETR::ADFSDMEN;
    pub use super::RCC_MP_APB2ENSETR::DFSDMEN;
    pub use super::RCC_MP_APB2ENSETR::FDCANEN;
    pub use super::RCC_MP_APB2ENSETR::SAI1EN;
    pub use super::RCC_MP_APB2ENSETR::SAI2EN;
    pub use super::RCC_MP_APB2ENSETR::SAI3EN;
    pub use super::RCC_MP_APB2ENSETR::SPI1EN;
    pub use super::RCC_MP_APB2ENSETR::SPI4EN;
    pub use super::RCC_MP_APB2ENSETR::SPI5EN;
    pub use super::RCC_MP_APB2ENSETR::TIM15EN;
    pub use super::RCC_MP_APB2ENSETR::TIM16EN;
    pub use super::RCC_MP_APB2ENSETR::TIM17EN;
    pub use super::RCC_MP_APB2ENSETR::TIM1EN;
    pub use super::RCC_MP_APB2ENSETR::TIM8EN;
    pub use super::RCC_MP_APB2ENSETR::USART6EN;
}

/// This register is used to clear the peripheral clock enable bit
pub mod RCC_MC_APB2ENCLRR {
    pub use super::RCC_MP_APB2ENSETR::ADFSDMEN;
    pub use super::RCC_MP_APB2ENSETR::DFSDMEN;
    pub use super::RCC_MP_APB2ENSETR::FDCANEN;
    pub use super::RCC_MP_APB2ENSETR::SAI1EN;
    pub use super::RCC_MP_APB2ENSETR::SAI2EN;
    pub use super::RCC_MP_APB2ENSETR::SAI3EN;
    pub use super::RCC_MP_APB2ENSETR::SPI1EN;
    pub use super::RCC_MP_APB2ENSETR::SPI4EN;
    pub use super::RCC_MP_APB2ENSETR::SPI5EN;
    pub use super::RCC_MP_APB2ENSETR::TIM15EN;
    pub use super::RCC_MP_APB2ENSETR::TIM16EN;
    pub use super::RCC_MP_APB2ENSETR::TIM17EN;
    pub use super::RCC_MP_APB2ENSETR::TIM1EN;
    pub use super::RCC_MP_APB2ENSETR::TIM8EN;
    pub use super::RCC_MP_APB2ENSETR::USART6EN;
}

/// This register is used to set the peripheral clock enable bit
pub mod RCC_MC_APB3ENSETR {
    pub use super::RCC_MP_APB3ENSETR::DTSEN;
    pub use super::RCC_MP_APB3ENSETR::HDPEN;
    pub use super::RCC_MP_APB3ENSETR::LPTIM2EN;
    pub use super::RCC_MP_APB3ENSETR::LPTIM3EN;
    pub use super::RCC_MP_APB3ENSETR::LPTIM4EN;
    pub use super::RCC_MP_APB3ENSETR::LPTIM5EN;
    pub use super::RCC_MP_APB3ENSETR::SAI4EN;
    pub use super::RCC_MP_APB3ENSETR::SYSCFGEN;
    pub use super::RCC_MP_APB3ENSETR::VREFEN;
}

/// This register is used to clear the peripheral clock enable bit
pub mod RCC_MC_APB3ENCLRR {
    pub use super::RCC_MP_APB3ENSETR::DTSEN;
    pub use super::RCC_MP_APB3ENSETR::HDPEN;
    pub use super::RCC_MP_APB3ENSETR::LPTIM2EN;
    pub use super::RCC_MP_APB3ENSETR::LPTIM3EN;
    pub use super::RCC_MP_APB3ENSETR::LPTIM4EN;
    pub use super::RCC_MP_APB3ENSETR::LPTIM5EN;
    pub use super::RCC_MP_APB3ENSETR::SAI4EN;
    pub use super::RCC_MP_APB3ENSETR::SYSCFGEN;
    pub use super::RCC_MP_APB3ENSETR::VREFEN;
}

/// This register is used to set the peripheral clock enable bit
pub mod RCC_MC_AHB2ENSETR {
    pub use super::RCC_MP_AHB2ENSETR::ADC12EN;
    pub use super::RCC_MP_AHB2ENSETR::DMA1EN;
    pub use super::RCC_MP_AHB2ENSETR::DMA2EN;
    pub use super::RCC_MP_AHB2ENSETR::DMAMUXEN;
    pub use super::RCC_MP_AHB2ENSETR::SDMMC3EN;
    pub use super::RCC_MP_AHB2ENSETR::USBOEN;
}

/// This register is used to clear the peripheral clock enable bit
pub mod RCC_MC_AHB2ENCLRR {
    pub use super::RCC_MP_AHB2ENSETR::ADC12EN;
    pub use super::RCC_MP_AHB2ENSETR::DMA1EN;
    pub use super::RCC_MP_AHB2ENSETR::DMA2EN;
    pub use super::RCC_MP_AHB2ENSETR::DMAMUXEN;
    pub use super::RCC_MP_AHB2ENSETR::SDMMC3EN;
    pub use super::RCC_MP_AHB2ENSETR::USBOEN;
}

/// This register is used to set the peripheral clock enable bit
pub mod RCC_MC_AHB3ENSETR {
    pub use super::RCC_MP_AHB3ENSETR::CRC2EN;
    pub use super::RCC_MP_AHB3ENSETR::CRYP2EN;
    pub use super::RCC_MP_AHB3ENSETR::DCMIEN;
    pub use super::RCC_MP_AHB3ENSETR::HASH2EN;
    pub use super::RCC_MP_AHB3ENSETR::HSEMEN;
    pub use super::RCC_MP_AHB3ENSETR::IPCCEN;
    pub use super::RCC_MP_AHB3ENSETR::RNG2EN;
}

/// This register is used to clear the peripheral clock enable bit
pub mod RCC_MC_AHB3ENCLRR {
    pub use super::RCC_MP_AHB3ENSETR::CRC2EN;
    pub use super::RCC_MP_AHB3ENSETR::CRYP2EN;
    pub use super::RCC_MP_AHB3ENSETR::DCMIEN;
    pub use super::RCC_MP_AHB3ENSETR::HASH2EN;
    pub use super::RCC_MP_AHB3ENSETR::HSEMEN;
    pub use super::RCC_MP_AHB3ENSETR::IPCCEN;
    pub use super::RCC_MP_AHB3ENSETR::RNG2EN;
}

/// This register is used to set the peripheral clock enable bit
pub mod RCC_MC_AHB4ENSETR {
    pub use super::RCC_MP_AHB4ENSETR::GPIOAEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOBEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOCEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIODEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOEEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOFEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOGEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOHEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOIEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOJEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOKEN;
}

/// This register is used to clear the peripheral clock enable bit
pub mod RCC_MC_AHB4ENCLRR {
    pub use super::RCC_MP_AHB4ENSETR::GPIOAEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOBEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOCEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIODEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOEEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOFEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOGEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOHEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOIEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOJEN;
    pub use super::RCC_MP_AHB4ENSETR::GPIOKEN;
}

/// This register is used to set the peripheral clock enable bit
pub mod RCC_MC_AXIMENSETR {

    /// SYSRAMEN
    pub mod SYSRAMEN {
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

/// This register is used to clear the peripheral clock enable bit
pub mod RCC_MC_AXIMENCLRR {
    pub use super::RCC_MC_AXIMENSETR::SYSRAMEN;
}

/// This register is used to set the peripheral clock enable bit
pub mod RCC_MC_MLAHBENSETR {
    pub use super::RCC_MP_MLAHBENSETR::RETRAMEN;
}

/// This register is used to clear the peripheral clock enable bit
pub mod RCC_MC_MLAHBENCLRR {
    pub use super::RCC_MP_MLAHBENSETR::RETRAMEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bits
pub mod RCC_MP_APB1LPENSETR {

    /// TIM2LPEN
    pub mod TIM2LPEN {
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

    /// TIM3LPEN
    pub mod TIM3LPEN {
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

    /// TIM4LPEN
    pub mod TIM4LPEN {
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

    /// TIM5LPEN
    pub mod TIM5LPEN {
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

    /// TIM6LPEN
    pub mod TIM6LPEN {
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

    /// TIM7LPEN
    pub mod TIM7LPEN {
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

    /// TIM12LPEN
    pub mod TIM12LPEN {
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

    /// TIM13LPEN
    pub mod TIM13LPEN {
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

    /// TIM14LPEN
    pub mod TIM14LPEN {
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

    /// LPTIM1LPEN
    pub mod LPTIM1LPEN {
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

    /// SPI2LPEN
    pub mod SPI2LPEN {
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

    /// SPI3LPEN
    pub mod SPI3LPEN {
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

    /// USART2LPEN
    pub mod USART2LPEN {
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

    /// USART3LPEN
    pub mod USART3LPEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// UART4LPEN
    pub mod UART4LPEN {
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

    /// UART5LPEN
    pub mod UART5LPEN {
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

    /// UART7LPEN
    pub mod UART7LPEN {
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

    /// UART8LPEN
    pub mod UART8LPEN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C1LPEN
    pub mod I2C1LPEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C2LPEN
    pub mod I2C2LPEN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C3LPEN
    pub mod I2C3LPEN {
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

    /// I2C5LPEN
    pub mod I2C5LPEN {
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

    /// SPDIFLPEN
    pub mod SPDIFLPEN {
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

    /// CECLPEN
    pub mod CECLPEN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC12LPEN
    pub mod DAC12LPEN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MDIOSLPEN
    pub mod MDIOSLPEN {
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

/// This register is used by the MPU in order to clear the PERxLPEN bits .
pub mod RCC_MP_APB1LPENCLRR {
    pub use super::RCC_MP_APB1LPENSETR::CECLPEN;
    pub use super::RCC_MP_APB1LPENSETR::DAC12LPEN;
    pub use super::RCC_MP_APB1LPENSETR::I2C1LPEN;
    pub use super::RCC_MP_APB1LPENSETR::I2C2LPEN;
    pub use super::RCC_MP_APB1LPENSETR::I2C3LPEN;
    pub use super::RCC_MP_APB1LPENSETR::I2C5LPEN;
    pub use super::RCC_MP_APB1LPENSETR::LPTIM1LPEN;
    pub use super::RCC_MP_APB1LPENSETR::MDIOSLPEN;
    pub use super::RCC_MP_APB1LPENSETR::SPDIFLPEN;
    pub use super::RCC_MP_APB1LPENSETR::SPI2LPEN;
    pub use super::RCC_MP_APB1LPENSETR::SPI3LPEN;
    pub use super::RCC_MP_APB1LPENSETR::TIM12LPEN;
    pub use super::RCC_MP_APB1LPENSETR::TIM13LPEN;
    pub use super::RCC_MP_APB1LPENSETR::TIM14LPEN;
    pub use super::RCC_MP_APB1LPENSETR::TIM2LPEN;
    pub use super::RCC_MP_APB1LPENSETR::TIM3LPEN;
    pub use super::RCC_MP_APB1LPENSETR::TIM4LPEN;
    pub use super::RCC_MP_APB1LPENSETR::TIM5LPEN;
    pub use super::RCC_MP_APB1LPENSETR::TIM6LPEN;
    pub use super::RCC_MP_APB1LPENSETR::TIM7LPEN;
    pub use super::RCC_MP_APB1LPENSETR::UART4LPEN;
    pub use super::RCC_MP_APB1LPENSETR::UART5LPEN;
    pub use super::RCC_MP_APB1LPENSETR::UART7LPEN;
    pub use super::RCC_MP_APB1LPENSETR::UART8LPEN;
    pub use super::RCC_MP_APB1LPENSETR::USART2LPEN;
    pub use super::RCC_MP_APB1LPENSETR::USART3LPEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bits
pub mod RCC_MP_APB2LPENSETR {

    /// TIM1LPEN
    pub mod TIM1LPEN {
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

    /// TIM8LPEN
    pub mod TIM8LPEN {
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

    /// TIM15LPEN
    pub mod TIM15LPEN {
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

    /// TIM16LPEN
    pub mod TIM16LPEN {
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

    /// TIM17LPEN
    pub mod TIM17LPEN {
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

    /// SPI1LPEN
    pub mod SPI1LPEN {
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

    /// SPI4LPEN
    pub mod SPI4LPEN {
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

    /// SPI5LPEN
    pub mod SPI5LPEN {
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

    /// USART6LPEN
    pub mod USART6LPEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI1LPEN
    pub mod SAI1LPEN {
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

    /// SAI2LPEN
    pub mod SAI2LPEN {
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

    /// SAI3LPEN
    pub mod SAI3LPEN {
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

    /// DFSDMLPEN
    pub mod DFSDMLPEN {
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

    /// ADFSDMLPEN
    pub mod ADFSDMLPEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FDCANLPEN
    pub mod FDCANLPEN {
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
}

/// This register is used by the MCU in order to clear the PERxLPEN bits
pub mod RCC_MP_APB2LPENCLRR {
    pub use super::RCC_MP_APB2LPENSETR::ADFSDMLPEN;
    pub use super::RCC_MP_APB2LPENSETR::DFSDMLPEN;
    pub use super::RCC_MP_APB2LPENSETR::FDCANLPEN;
    pub use super::RCC_MP_APB2LPENSETR::SAI1LPEN;
    pub use super::RCC_MP_APB2LPENSETR::SAI2LPEN;
    pub use super::RCC_MP_APB2LPENSETR::SAI3LPEN;
    pub use super::RCC_MP_APB2LPENSETR::SPI1LPEN;
    pub use super::RCC_MP_APB2LPENSETR::SPI4LPEN;
    pub use super::RCC_MP_APB2LPENSETR::SPI5LPEN;
    pub use super::RCC_MP_APB2LPENSETR::TIM15LPEN;
    pub use super::RCC_MP_APB2LPENSETR::TIM16LPEN;
    pub use super::RCC_MP_APB2LPENSETR::TIM17LPEN;
    pub use super::RCC_MP_APB2LPENSETR::TIM1LPEN;
    pub use super::RCC_MP_APB2LPENSETR::TIM8LPEN;
    pub use super::RCC_MP_APB2LPENSETR::USART6LPEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bits
pub mod RCC_MP_APB3LPENSETR {

    /// LPTIM2LPEN
    pub mod LPTIM2LPEN {
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

    /// LPTIM3LPEN
    pub mod LPTIM3LPEN {
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

    /// LPTIM4LPEN
    pub mod LPTIM4LPEN {
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

    /// LPTIM5LPEN
    pub mod LPTIM5LPEN {
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

    /// SAI4LPEN
    pub mod SAI4LPEN {
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

    /// SYSCFGLPEN
    pub mod SYSCFGLPEN {
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

    /// VREFLPEN
    pub mod VREFLPEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DTSLPEN
    pub mod DTSLPEN {
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

/// This register is used by the MCU in order to clear the PERxLPEN bits
pub mod RCC_MP_APB3LPENCLRR {
    pub use super::RCC_MP_APB3LPENSETR::DTSLPEN;
    pub use super::RCC_MP_APB3LPENSETR::LPTIM2LPEN;
    pub use super::RCC_MP_APB3LPENSETR::LPTIM3LPEN;
    pub use super::RCC_MP_APB3LPENSETR::LPTIM4LPEN;
    pub use super::RCC_MP_APB3LPENSETR::LPTIM5LPEN;
    pub use super::RCC_MP_APB3LPENSETR::SAI4LPEN;
    pub use super::RCC_MP_APB3LPENSETR::SYSCFGLPEN;
    pub use super::RCC_MP_APB3LPENSETR::VREFLPEN;
}

/// This register is used by the MPU in order to set the PERxLPEN bit.
pub mod RCC_MP_AHB2LPENSETR {

    /// DMA1LPEN
    pub mod DMA1LPEN {
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

    /// DMA2LPEN
    pub mod DMA2LPEN {
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

    /// DMAMUXLPEN
    pub mod DMAMUXLPEN {
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

    /// ADC12LPEN
    pub mod ADC12LPEN {
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

    /// USBOLPEN
    pub mod USBOLPEN {
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

    /// SDMMC3LPEN
    pub mod SDMMC3LPEN {
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

/// This register is used by the MCU in order to clear the PERxLPEN bits
pub mod RCC_MP_AHB2LPENCLRR {
    pub use super::RCC_MP_AHB2LPENSETR::ADC12LPEN;
    pub use super::RCC_MP_AHB2LPENSETR::DMA1LPEN;
    pub use super::RCC_MP_AHB2LPENSETR::DMA2LPEN;
    pub use super::RCC_MP_AHB2LPENSETR::DMAMUXLPEN;
    pub use super::RCC_MP_AHB2LPENSETR::SDMMC3LPEN;
    pub use super::RCC_MP_AHB2LPENSETR::USBOLPEN;
}

/// This register is used by the MPU
pub mod RCC_MP_AHB3LPENSETR {

    /// DCMILPEN
    pub mod DCMILPEN {
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

    /// CRYP2LPEN
    pub mod CRYP2LPEN {
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

    /// HASH2LPEN
    pub mod HASH2LPEN {
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

    /// RNG2LPEN
    pub mod RNG2LPEN {
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

    /// CRC2LPEN
    pub mod CRC2LPEN {
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

    /// HSEMLPEN
    pub mod HSEMLPEN {
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

    /// IPCCLPEN
    pub mod IPCCLPEN {
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
}

/// This register is used by the MPU in order to clear the PERxLPEN bit
pub mod RCC_MP_AHB3LPENCLRR {
    pub use super::RCC_MP_AHB3LPENSETR::CRC2LPEN;
    pub use super::RCC_MP_AHB3LPENSETR::CRYP2LPEN;
    pub use super::RCC_MP_AHB3LPENSETR::DCMILPEN;
    pub use super::RCC_MP_AHB3LPENSETR::HASH2LPEN;
    pub use super::RCC_MP_AHB3LPENSETR::HSEMLPEN;
    pub use super::RCC_MP_AHB3LPENSETR::IPCCLPEN;
    pub use super::RCC_MP_AHB3LPENSETR::RNG2LPEN;
}

/// This register is used by the MPU
pub mod RCC_MP_AHB4LPENSETR {

    /// GPIOALPEN
    pub mod GPIOALPEN {
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

    /// GPIOBLPEN
    pub mod GPIOBLPEN {
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

    /// GPIOCLPEN
    pub mod GPIOCLPEN {
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

    /// GPIODLPEN
    pub mod GPIODLPEN {
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

    /// GPIOELPEN
    pub mod GPIOELPEN {
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

    /// GPIOFLPEN
    pub mod GPIOFLPEN {
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

    /// GPIOGLPEN
    pub mod GPIOGLPEN {
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

    /// GPIOHLPEN
    pub mod GPIOHLPEN {
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

    /// GPIOILPEN
    pub mod GPIOILPEN {
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

    /// GPIOJLPEN
    pub mod GPIOJLPEN {
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

    /// GPIOKLPEN
    pub mod GPIOKLPEN {
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

/// This register is used by the MPU
pub mod RCC_MP_AHB4LPENCLRR {
    pub use super::RCC_MP_AHB4LPENSETR::GPIOALPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOBLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOCLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIODLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOELPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOFLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOGLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOHLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOILPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOJLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOKLPEN;
}

/// This register is used by the MPU
pub mod RCC_MP_AXIMLPENSETR {

    /// SYSRAMLPEN
    pub mod SYSRAMLPEN {
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

/// This register is used by the MPU
pub mod RCC_MP_AXIMLPENCLRR {
    pub use super::RCC_MP_AXIMLPENSETR::SYSRAMLPEN;
}

/// This register is used by the MPU in order to set the PERxLPEN bit
pub mod RCC_MP_MLAHBLPENSETR {

    /// SRAM1LPEN
    pub mod SRAM1LPEN {
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

    /// SRAM2LPEN
    pub mod SRAM2LPEN {
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

    /// SRAM34LPEN
    pub mod SRAM34LPEN {
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

    /// RETRAMLPEN
    pub mod RETRAMLPEN {
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
}

/// This register is used by the MPU in order to clear the PERxLPEN bit
pub mod RCC_MP_MLAHBLPENCLRR {
    pub use super::RCC_MP_MLAHBLPENSETR::RETRAMLPEN;
    pub use super::RCC_MP_MLAHBLPENSETR::SRAM1LPEN;
    pub use super::RCC_MP_MLAHBLPENSETR::SRAM2LPEN;
    pub use super::RCC_MP_MLAHBLPENSETR::SRAM34LPEN;
}

/// This register is used by the MCU in order to set the PERxLPEN bit.
pub mod RCC_MC_APB1LPENSETR {

    /// TIM2LPEN
    pub mod TIM2LPEN {
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

    /// TIM3LPEN
    pub mod TIM3LPEN {
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

    /// TIM4LPEN
    pub mod TIM4LPEN {
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

    /// TIM5LPEN
    pub mod TIM5LPEN {
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

    /// TIM6LPEN
    pub mod TIM6LPEN {
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

    /// TIM7LPEN
    pub mod TIM7LPEN {
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

    /// TIM12LPEN
    pub mod TIM12LPEN {
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

    /// TIM13LPEN
    pub mod TIM13LPEN {
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

    /// TIM14LPEN
    pub mod TIM14LPEN {
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

    /// LPTIM1LPEN
    pub mod LPTIM1LPEN {
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

    /// SPI2LPEN
    pub mod SPI2LPEN {
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

    /// SPI3LPEN
    pub mod SPI3LPEN {
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

    /// USART2LPEN
    pub mod USART2LPEN {
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

    /// USART3LPEN
    pub mod USART3LPEN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// UART4LPEN
    pub mod UART4LPEN {
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

    /// UART5LPEN
    pub mod UART5LPEN {
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

    /// UART7LPEN
    pub mod UART7LPEN {
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

    /// UART8LPEN
    pub mod UART8LPEN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C1LPEN
    pub mod I2C1LPEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C2LPEN
    pub mod I2C2LPEN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I2C3LPEN
    pub mod I2C3LPEN {
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

    /// I2C5LPEN
    pub mod I2C5LPEN {
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

    /// SPDIFLPEN
    pub mod SPDIFLPEN {
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

    /// CECLPEN
    pub mod CECLPEN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WWDG1LPEN
    pub mod WWDG1LPEN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC12LPEN
    pub mod DAC12LPEN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MDIOSLPEN
    pub mod MDIOSLPEN {
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

/// This register is used by the MCU in order to clear the PERxLPEN bits
pub mod RCC_MC_APB1LPENCLRR {
    pub use super::RCC_MC_APB1LPENSETR::CECLPEN;
    pub use super::RCC_MC_APB1LPENSETR::DAC12LPEN;
    pub use super::RCC_MC_APB1LPENSETR::I2C1LPEN;
    pub use super::RCC_MC_APB1LPENSETR::I2C2LPEN;
    pub use super::RCC_MC_APB1LPENSETR::I2C3LPEN;
    pub use super::RCC_MC_APB1LPENSETR::I2C5LPEN;
    pub use super::RCC_MC_APB1LPENSETR::LPTIM1LPEN;
    pub use super::RCC_MC_APB1LPENSETR::MDIOSLPEN;
    pub use super::RCC_MC_APB1LPENSETR::SPDIFLPEN;
    pub use super::RCC_MC_APB1LPENSETR::SPI2LPEN;
    pub use super::RCC_MC_APB1LPENSETR::SPI3LPEN;
    pub use super::RCC_MC_APB1LPENSETR::TIM12LPEN;
    pub use super::RCC_MC_APB1LPENSETR::TIM13LPEN;
    pub use super::RCC_MC_APB1LPENSETR::TIM14LPEN;
    pub use super::RCC_MC_APB1LPENSETR::TIM2LPEN;
    pub use super::RCC_MC_APB1LPENSETR::TIM3LPEN;
    pub use super::RCC_MC_APB1LPENSETR::TIM4LPEN;
    pub use super::RCC_MC_APB1LPENSETR::TIM5LPEN;
    pub use super::RCC_MC_APB1LPENSETR::TIM6LPEN;
    pub use super::RCC_MC_APB1LPENSETR::TIM7LPEN;
    pub use super::RCC_MC_APB1LPENSETR::UART4LPEN;
    pub use super::RCC_MC_APB1LPENSETR::UART5LPEN;
    pub use super::RCC_MC_APB1LPENSETR::UART7LPEN;
    pub use super::RCC_MC_APB1LPENSETR::UART8LPEN;
    pub use super::RCC_MC_APB1LPENSETR::USART2LPEN;
    pub use super::RCC_MC_APB1LPENSETR::USART3LPEN;
    pub use super::RCC_MC_APB1LPENSETR::WWDG1LPEN;
}

/// This register is used by the MCU in order to set the PERxLPEN bit.
pub mod RCC_MC_APB2LPENSETR {
    pub use super::RCC_MP_APB2LPENSETR::ADFSDMLPEN;
    pub use super::RCC_MP_APB2LPENSETR::DFSDMLPEN;
    pub use super::RCC_MP_APB2LPENSETR::FDCANLPEN;
    pub use super::RCC_MP_APB2LPENSETR::SAI1LPEN;
    pub use super::RCC_MP_APB2LPENSETR::SAI2LPEN;
    pub use super::RCC_MP_APB2LPENSETR::SAI3LPEN;
    pub use super::RCC_MP_APB2LPENSETR::SPI1LPEN;
    pub use super::RCC_MP_APB2LPENSETR::SPI4LPEN;
    pub use super::RCC_MP_APB2LPENSETR::SPI5LPEN;
    pub use super::RCC_MP_APB2LPENSETR::TIM15LPEN;
    pub use super::RCC_MP_APB2LPENSETR::TIM16LPEN;
    pub use super::RCC_MP_APB2LPENSETR::TIM17LPEN;
    pub use super::RCC_MP_APB2LPENSETR::TIM1LPEN;
    pub use super::RCC_MP_APB2LPENSETR::TIM8LPEN;
    pub use super::RCC_MP_APB2LPENSETR::USART6LPEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bit
pub mod RCC_MC_APB2LPENCLRR {
    pub use super::RCC_MP_APB2LPENSETR::ADFSDMLPEN;
    pub use super::RCC_MP_APB2LPENSETR::DFSDMLPEN;
    pub use super::RCC_MP_APB2LPENSETR::FDCANLPEN;
    pub use super::RCC_MP_APB2LPENSETR::SAI1LPEN;
    pub use super::RCC_MP_APB2LPENSETR::SAI2LPEN;
    pub use super::RCC_MP_APB2LPENSETR::SAI3LPEN;
    pub use super::RCC_MP_APB2LPENSETR::SPI1LPEN;
    pub use super::RCC_MP_APB2LPENSETR::SPI4LPEN;
    pub use super::RCC_MP_APB2LPENSETR::SPI5LPEN;
    pub use super::RCC_MP_APB2LPENSETR::TIM15LPEN;
    pub use super::RCC_MP_APB2LPENSETR::TIM16LPEN;
    pub use super::RCC_MP_APB2LPENSETR::TIM17LPEN;
    pub use super::RCC_MP_APB2LPENSETR::TIM1LPEN;
    pub use super::RCC_MP_APB2LPENSETR::TIM8LPEN;
    pub use super::RCC_MP_APB2LPENSETR::USART6LPEN;
}

/// This register is used by the MCU in order to set the PERxLPEN bit.
pub mod RCC_MC_APB3LPENSETR {
    pub use super::RCC_MP_APB3LPENSETR::DTSLPEN;
    pub use super::RCC_MP_APB3LPENSETR::LPTIM2LPEN;
    pub use super::RCC_MP_APB3LPENSETR::LPTIM3LPEN;
    pub use super::RCC_MP_APB3LPENSETR::LPTIM4LPEN;
    pub use super::RCC_MP_APB3LPENSETR::LPTIM5LPEN;
    pub use super::RCC_MP_APB3LPENSETR::SAI4LPEN;
    pub use super::RCC_MP_APB3LPENSETR::SYSCFGLPEN;
    pub use super::RCC_MP_APB3LPENSETR::VREFLPEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bit
pub mod RCC_MC_APB3LPENCLRR {
    pub use super::RCC_MP_APB3LPENSETR::DTSLPEN;
    pub use super::RCC_MP_APB3LPENSETR::LPTIM2LPEN;
    pub use super::RCC_MP_APB3LPENSETR::LPTIM3LPEN;
    pub use super::RCC_MP_APB3LPENSETR::LPTIM4LPEN;
    pub use super::RCC_MP_APB3LPENSETR::LPTIM5LPEN;
    pub use super::RCC_MP_APB3LPENSETR::SAI4LPEN;
    pub use super::RCC_MP_APB3LPENSETR::SYSCFGLPEN;
    pub use super::RCC_MP_APB3LPENSETR::VREFLPEN;
}

/// This register is used by the MCU in order to set the PERxLPEN bit.
pub mod RCC_MC_AHB2LPENSETR {
    pub use super::RCC_MP_AHB2LPENSETR::ADC12LPEN;
    pub use super::RCC_MP_AHB2LPENSETR::DMA1LPEN;
    pub use super::RCC_MP_AHB2LPENSETR::DMA2LPEN;
    pub use super::RCC_MP_AHB2LPENSETR::DMAMUXLPEN;
    pub use super::RCC_MP_AHB2LPENSETR::SDMMC3LPEN;
    pub use super::RCC_MP_AHB2LPENSETR::USBOLPEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bit
pub mod RCC_MC_AHB2LPENCLRR {
    pub use super::RCC_MP_AHB2LPENSETR::ADC12LPEN;
    pub use super::RCC_MP_AHB2LPENSETR::DMA1LPEN;
    pub use super::RCC_MP_AHB2LPENSETR::DMA2LPEN;
    pub use super::RCC_MP_AHB2LPENSETR::DMAMUXLPEN;
    pub use super::RCC_MP_AHB2LPENSETR::SDMMC3LPEN;
    pub use super::RCC_MP_AHB2LPENSETR::USBOLPEN;
}

/// This register is used by the MCU in order to set the PERxLPEN bit.
pub mod RCC_MC_AHB3LPENSETR {
    pub use super::RCC_MP_AHB3LPENSETR::CRC2LPEN;
    pub use super::RCC_MP_AHB3LPENSETR::CRYP2LPEN;
    pub use super::RCC_MP_AHB3LPENSETR::DCMILPEN;
    pub use super::RCC_MP_AHB3LPENSETR::HASH2LPEN;
    pub use super::RCC_MP_AHB3LPENSETR::HSEMLPEN;
    pub use super::RCC_MP_AHB3LPENSETR::IPCCLPEN;
    pub use super::RCC_MP_AHB3LPENSETR::RNG2LPEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bit
pub mod RCC_MC_AHB3LPENCLRR {
    pub use super::RCC_MP_AHB3LPENSETR::CRC2LPEN;
    pub use super::RCC_MP_AHB3LPENSETR::CRYP2LPEN;
    pub use super::RCC_MP_AHB3LPENSETR::DCMILPEN;
    pub use super::RCC_MP_AHB3LPENSETR::HASH2LPEN;
    pub use super::RCC_MP_AHB3LPENSETR::HSEMLPEN;
    pub use super::RCC_MP_AHB3LPENSETR::IPCCLPEN;
    pub use super::RCC_MP_AHB3LPENSETR::RNG2LPEN;
}

/// This register is used by the MCU in order to set the PERxLPEN bit.
pub mod RCC_MC_AHB4LPENSETR {
    pub use super::RCC_MP_AHB4LPENSETR::GPIOALPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOBLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOCLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIODLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOELPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOFLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOGLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOHLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOILPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOJLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOKLPEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
pub mod RCC_MC_AHB4LPENCLRR {
    pub use super::RCC_MP_AHB4LPENSETR::GPIOALPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOBLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOCLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIODLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOELPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOFLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOGLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOHLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOILPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOJLPEN;
    pub use super::RCC_MP_AHB4LPENSETR::GPIOKLPEN;
}

/// This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.
pub mod RCC_MC_AXIMLPENSETR {
    pub use super::RCC_MP_AXIMLPENSETR::SYSRAMLPEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
pub mod RCC_MC_AXIMLPENCLRR {
    pub use super::RCC_MP_AXIMLPENSETR::SYSRAMLPEN;
}

/// This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.
pub mod RCC_MC_MLAHBLPENSETR {
    pub use super::RCC_MP_MLAHBLPENSETR::RETRAMLPEN;
    pub use super::RCC_MP_MLAHBLPENSETR::SRAM1LPEN;
    pub use super::RCC_MP_MLAHBLPENSETR::SRAM2LPEN;
    pub use super::RCC_MP_MLAHBLPENSETR::SRAM34LPEN;
}

/// This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
pub mod RCC_MC_MLAHBLPENCLRR {
    pub use super::RCC_MP_MLAHBLPENSETR::RETRAMLPEN;
    pub use super::RCC_MP_MLAHBLPENSETR::SRAM1LPEN;
    pub use super::RCC_MP_MLAHBLPENSETR::SRAM2LPEN;
    pub use super::RCC_MP_MLAHBLPENSETR::SRAM34LPEN;
}

/// This register is used by the MCU to check the reset source.
pub mod RCC_MC_RSTSCLRR {

    /// PORRSTF
    pub mod PORRSTF {
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

    /// BORRSTF
    pub mod BORRSTF {
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

    /// PADRSTF
    pub mod PADRSTF {
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

    /// HCSSRSTF
    pub mod HCSSRSTF {
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

    /// VCORERSTF
    pub mod VCORERSTF {
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

    /// MCURSTF
    pub mod MCURSTF {
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

    /// MPSYSRSTF
    pub mod MPSYSRSTF {
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

    /// MCSYSRSTF
    pub mod MCSYSRSTF {
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

    /// IWDG1RSTF
    pub mod IWDG1RSTF {
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

    /// IWDG2RSTF
    pub mod IWDG2RSTF {
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

    /// WWDG1RSTF
    pub mod WWDG1RSTF {
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

/// This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details.
pub mod RCC_MC_CIER {
    pub use super::RCC_MP_CIER::CSIRDYIE;
    pub use super::RCC_MP_CIER::HSERDYIE;
    pub use super::RCC_MP_CIER::HSIRDYIE;
    pub use super::RCC_MP_CIER::LSECSSIE;
    pub use super::RCC_MP_CIER::LSERDYIE;
    pub use super::RCC_MP_CIER::LSIRDYIE;
    pub use super::RCC_MP_CIER::PLL1DYIE;
    pub use super::RCC_MP_CIER::PLL2DYIE;
    pub use super::RCC_MP_CIER::PLL3DYIE;
    pub use super::RCC_MP_CIER::PLL4DYIE;
    pub use super::RCC_MP_CIER::WKUPIE;
}

/// This register shall be used by the MCU in order to read and clear the interrupt flags.
pub mod RCC_MC_CIFR {
    pub use super::RCC_MP_CIFR::CSIRDYF;
    pub use super::RCC_MP_CIFR::HSERDYF;
    pub use super::RCC_MP_CIFR::HSIRDYF;
    pub use super::RCC_MP_CIFR::LSECSSF;
    pub use super::RCC_MP_CIFR::LSERDYF;
    pub use super::RCC_MP_CIFR::LSIRDYF;
    pub use super::RCC_MP_CIFR::PLL1DYF;
    pub use super::RCC_MP_CIFR::PLL2DYF;
    pub use super::RCC_MP_CIFR::PLL3DYF;
    pub use super::RCC_MP_CIFR::PLL4DYF;
    pub use super::RCC_MP_CIFR::WKUPF;
}

/// This register gives the IP version
pub mod RCC_VERR {

    /// MINREV
    pub mod MINREV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MAJREV
    pub mod MAJREV {
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
}

/// This register gives the unique identifier of the RCC
pub mod RCC_IDR {

    /// ID
    pub mod ID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register gives the decoding space, which is for the RCC of 4 kB.
pub mod RCC_SIDR {

    /// SID
    pub mod SID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
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
    /// This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode.
    pub RCC_TZCR: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_OCENSETR: RWRegister<u32>,

    /// This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_OCENCLRR: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_HSICFGR: RWRegister<u32>,

    /// This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details.
    pub RCC_CSICFGR: RWRegister<u32>,

    /// This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_MPCKSELR: RWRegister<u32>,

    /// This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_ASSCKSELR: RWRegister<u32>,

    /// This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_RCK12SELR: RWRegister<u32>,

    /// This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
    pub RCC_MPCKDIVR: RWRegister<u32>,

    /// This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
    pub RCC_AXIDIVR: RWRegister<u32>,

    _reserved3: [u32; 2],

    /// This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
    pub RCC_APB4DIVR: RWRegister<u32>,

    /// This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
    pub RCC_APB5DIVR: RWRegister<u32>,

    /// This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode.
    pub RCC_RTCDIVR: RWRegister<u32>,

    /// This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_MSSCKSELR: RWRegister<u32>,

    _reserved4: [u32; 13],

    /// This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_PLL1CR: RWRegister<u32>,

    /// This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_PLL1CFGR1: RWRegister<u32>,

    /// This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_PLL1CFGR2: RWRegister<u32>,

    /// This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_PLL1FRACR: RWRegister<u32>,

    /// This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_PLL1CSGR: RWRegister<u32>,

    /// This register is used to control the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_PLL2CR: RWRegister<u32>,

    /// This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_PLL2CFGR1: RWRegister<u32>,

    /// This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_PLL2CFGR2: RWRegister<u32>,

    /// This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_PLL2FRACR: RWRegister<u32>,

    /// This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub RCC_PLL2CSGR: RWRegister<u32>,

    _reserved5: [u32; 6],

    /// This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
    pub RCC_I2C46CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
    pub RCC_SPI6CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
    pub RCC_UART1CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
    pub RCC_RNG1CKSELR: RWRegister<u32>,

    /// This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays.
    pub RCC_CPERCKSELR: RWRegister<u32>,

    /// This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
    pub RCC_STGENCKSELR: RWRegister<u32>,

    /// This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode.
    pub RCC_DDRITFCR: RWRegister<u32>,

    _reserved6: [u32; 9],

    /// This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU.
    pub RCC_MP_BOOTCR: RWRegister<u32>,

    /// Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_SREQSETR: RWRegister<u32>,

    /// Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_SREQCLRR: RWRegister<u32>,

    /// The register contains global control bits. If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_GCR: RWRegister<u32>,

    /// This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_APRSTCR: RWRegister<u32>,

    /// This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_APRSTSR: RORegister<u32>,

    _reserved7: [u32; 10],

    /// This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode.
    pub RCC_BDCR: RWRegister<u32>,

    /// This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode.
    pub RCC_RDLSICR: RWRegister<u32>,

    _reserved8: [u32; 14],

    /// This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.
    pub RCC_APB4RSTSETR: RWRegister<u32>,

    /// This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.
    pub RCC_APB4RSTCLRR: RWRegister<u32>,

    /// This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    pub RCC_APB5RSTSETR: RWRegister<u32>,

    /// This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    pub RCC_APB5RSTCLRR: RWRegister<u32>,

    /// This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    pub RCC_AHB5RSTSETR: RWRegister<u32>,

    /// This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    pub RCC_AHB5RSTCLRR: RWRegister<u32>,

    /// This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.
    pub RCC_AHB6RSTSETR: RWRegister<u32>,

    /// This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.
    pub RCC_AHB6RSTCLRR: RWRegister<u32>,

    /// This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    pub RCC_TZAHB6RSTSETR: RWRegister<u32>,

    /// This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    pub RCC_TZAHB6RSTCLRR: RWRegister<u32>,

    _reserved9: [u32; 22],

    /// This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    pub RCC_MP_APB4ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    pub RCC_MP_APB4ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    pub RCC_MP_APB5ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    pub RCC_MP_APB5ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_AHB5ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_AHB5ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    pub RCC_MP_AHB6ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    pub RCC_MP_AHB6ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_TZAHB6ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_TZAHB6ENCLRR: RWRegister<u32>,

    _reserved10: [u32; 22],

    /// This register is used to set the peripheral clock enable bit
    pub RCC_MC_APB4ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit
    pub RCC_MC_APB4ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit
    pub RCC_MC_APB5ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit
    pub RCC_MC_APB5ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.
    pub RCC_MC_AHB5ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.
    pub RCC_MC_AHB5ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit
    pub RCC_MC_AHB6ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit
    pub RCC_MC_AHB6ENCLRR: RWRegister<u32>,

    _reserved11: [u32; 24],

    /// This register is used by the MCU in order to clear the PERxLPEN bits
    pub RCC_MP_APB4LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU
    pub RCC_MP_APB4LPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_APB5LPENSETR: RWRegister<u32>,

    /// This register is used by the Mpu.
    pub RCC_MP_APB5LPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_AHB5LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU
    pub RCC_MP_AHB5LPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bits
    pub RCC_MP_AHB6LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bits
    pub RCC_MP_AHB6LPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_TZAHB6LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_TZAHB6LPENCLRR: RWRegister<u32>,

    _reserved12: [u32; 22],

    /// This register is used by the MCU in order to set the PERxLPEN bit.
    pub RCC_MC_APB4LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bit
    pub RCC_MC_APB4LPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to set the PERxLPEN bit.
    pub RCC_MC_APB5LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bit
    pub RCC_MC_APB5LPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode.
    pub RCC_MC_AHB5LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode.
    pub RCC_MC_AHB5LPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to set the PERxLPEN bit.
    pub RCC_MC_AHB6LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bit
    pub RCC_MC_AHB6LPENCLRR: RWRegister<u32>,

    _reserved13: [u32; 24],

    /// This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\[1:0\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\[1:0\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode.
    pub RCC_BR_RSTSCLRR: RWRegister<u32>,

    /// This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset.
    pub RCC_MP_GRSTCSETR: RWRegister<u32>,

    /// This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_RSTSCLRR: RWRegister<u32>,

    /// This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_IWDGFZSETR: RWRegister<u32>,

    /// This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_IWDGFZCLRR: RWRegister<u32>,

    /// This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_CIER: RWRegister<u32>,

    /// This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_CIFR: RWRegister<u32>,

    /// This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode.
    pub RCC_PWRLPDLYCR: RWRegister<u32>,

    /// This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.
    pub RCC_MP_RSTSSETR: RWRegister<u32>,

    _reserved14: [u32; 247],

    /// This register is used to select the clock generated on MCO1 output.
    pub RCC_MCO1CFGR: RWRegister<u32>,

    /// This register is used to select the clock generated on MCO2 output.
    pub RCC_MCO2CFGR: RWRegister<u32>,

    /// This is a read-only access register, It contains the status flags of oscillators. Writing has no effect.
    pub RCC_OCRDYR: RORegister<u32>,

    /// This is register contains the enable control of the debug and trace function, and the clock divider for the trace function.
    pub RCC_DBGCFGR: RWRegister<u32>,

    _reserved15: [u32; 4],

    /// This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    pub RCC_RCK3SELR: RWRegister<u32>,

    /// This register is used to select the reference clock for PLL4.
    pub RCC_RCK4SELR: RWRegister<u32>,

    /// This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information.
    pub RCC_TIMG1PRER: RWRegister<u32>,

    /// This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information.
    pub RCC_TIMG2PRER: RWRegister<u32>,

    /// This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
    pub RCC_MCUDIVR: RWRegister<u32>,

    /// This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information.
    pub RCC_APB1DIVR: RWRegister<u32>,

    /// This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information.
    pub RCC_APB2DIVR: RWRegister<u32>,

    /// This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information.
    pub RCC_APB3DIVR: RWRegister<u32>,

    _reserved16: [u32; 16],

    /// This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    pub RCC_PLL3CR: RWRegister<u32>,

    /// This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    pub RCC_PLL3CFGR1: RWRegister<u32>,

    /// This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    pub RCC_PLL3CFGR2: RWRegister<u32>,

    /// This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    pub RCC_PLL3FRACR: RWRegister<u32>,

    /// This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    pub RCC_PLL3CSGR: RWRegister<u32>,

    /// This register is used to control the PLL4.
    pub RCC_PLL4CR: RWRegister<u32>,

    /// This register is used to configure the PLL4.
    pub RCC_PLL4CFGR1: RWRegister<u32>,

    /// This register is used to configure the PLL4.
    pub RCC_PLL4CFGR2: RWRegister<u32>,

    /// This register is used to fine-tune the frequency of the PLL4 VCO.
    pub RCC_PLL4FRACR: RWRegister<u32>,

    /// This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    pub RCC_PLL4CSGR: RWRegister<u32>,

    _reserved17: [u32; 6],

    /// This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_I2C12CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the I2C3 and I2C5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_I2C35CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_SAI1CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_SAI2CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_SAI3CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_SAI4CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_SPI2S1CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_SPI2S23CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_SPI45CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_UART6CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_UART24CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_UART35CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_UART78CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_SDMMC12CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_SDMMC3CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_ETHCKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_QSPICKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_FMCCKSELR: RWRegister<u32>,

    _reserved18: [u32; 1],

    /// This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_FDCANCKSELR: RWRegister<u32>,

    _reserved19: [u32; 1],

    /// This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub RCC_SPDIFCKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the CEC-HDMI.
    pub RCC_CECCKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG
    pub RCC_USBCKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the RNG2.
    pub RCC_RNG2CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the DSI block.
    pub RCC_DSICKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the ADC block.
    pub RCC_ADCCKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks.
    pub RCC_LPTIM45CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks.
    pub RCC_LPTIM23CKSELR: RWRegister<u32>,

    /// This register is used to control the selection of the kernel clock for the LPTIM1 block.
    pub RCC_LPTIM1CKSELR: RWRegister<u32>,

    _reserved20: [u32; 18],

    /// This register is used to activate the reset of the corresponding peripheral.
    pub RCC_APB1RSTSETR: RWRegister<u32>,

    /// This register is used to release the reset of the corresponding peripheral.
    pub RCC_APB1RSTCLRR: RWRegister<u32>,

    /// This register is used to activate the reset of the corresponding peripheral.
    pub RCC_APB2RSTSETR: RWRegister<u32>,

    /// This register is used to release the reset of the corresponding peripheral.
    pub RCC_APB2RSTCLRR: RWRegister<u32>,

    /// This register is used to activate the reset of the corresponding peripheral.
    pub RCC_APB3RSTSETR: RWRegister<u32>,

    /// This register is used to release the reset of the corresponding peripheral.
    pub RCC_APB3RSTCLRR: RWRegister<u32>,

    /// This register is used to activate the reset of the corresponding peripheral.
    pub RCC_AHB2RSTSETR: RWRegister<u32>,

    /// This register is used to release the reset of the corresponding peripheral.
    pub RCC_AHB2RSTCLRR: RWRegister<u32>,

    /// This register is used to activate the reset of the corresponding peripheral.
    pub RCC_AHB3RSTSETR: RWRegister<u32>,

    /// This register is used to release the reset of the corresponding peripheral.
    pub RCC_AHB3RSTCLRR: RWRegister<u32>,

    /// This register is used to activate the reset of the corresponding peripheral
    pub RCC_AHB4RSTSETR: RWRegister<u32>,

    /// This register is used to release the reset of the corresponding peripheral.
    pub RCC_AHB4RSTCLRR: RWRegister<u32>,

    _reserved21: [u32; 20],

    /// This register is used to set the peripheral clock enable bit
    pub RCC_MP_APB1ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit
    pub RCC_MP_APB1ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit
    pub RCC_MP_APB2ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
    pub RCC_MP_APB2ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit
    pub RCC_MP_APB3ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
    pub RCC_MP_APB3ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit of the corresponding peripheral
    pub RCC_MP_AHB2ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
    pub RCC_MP_AHB2ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit of the corresponding peripheral
    pub RCC_MP_AHB3ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
    pub RCC_MP_AHB3ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU.
    pub RCC_MP_AHB4ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit
    pub RCC_MP_AHB4ENCLRR: RWRegister<u32>,

    _reserved22: [u32; 2],

    /// This register is used to set the peripheral clock enable bit
    pub RCC_MP_MLAHBENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit.
    pub RCC_MP_MLAHBENCLRR: RWRegister<u32>,

    _reserved23: [u32; 16],

    /// This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MCU. Writing has no effect, reading will return . Writing a sets the corresponding bit to .
    pub RCC_MC_APB1ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
    pub RCC_MC_APB1ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit
    pub RCC_MC_APB2ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit
    pub RCC_MC_APB2ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit
    pub RCC_MC_APB3ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit
    pub RCC_MC_APB3ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit
    pub RCC_MC_AHB2ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit
    pub RCC_MC_AHB2ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit
    pub RCC_MC_AHB3ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit
    pub RCC_MC_AHB3ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit
    pub RCC_MC_AHB4ENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit
    pub RCC_MC_AHB4ENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit
    pub RCC_MC_AXIMENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit
    pub RCC_MC_AXIMENCLRR: RWRegister<u32>,

    /// This register is used to set the peripheral clock enable bit
    pub RCC_MC_MLAHBENSETR: RWRegister<u32>,

    /// This register is used to clear the peripheral clock enable bit
    pub RCC_MC_MLAHBENCLRR: RWRegister<u32>,

    _reserved24: [u32; 16],

    /// This register is used by the MCU in order to clear the PERxLPEN bits
    pub RCC_MP_APB1LPENSETR: RWRegister<u32>,

    /// This register is used by the MPU in order to clear the PERxLPEN bits .
    pub RCC_MP_APB1LPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bits
    pub RCC_MP_APB2LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bits
    pub RCC_MP_APB2LPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bits
    pub RCC_MP_APB3LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bits
    pub RCC_MP_APB3LPENCLRR: RWRegister<u32>,

    /// This register is used by the MPU in order to set the PERxLPEN bit.
    pub RCC_MP_AHB2LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bits
    pub RCC_MP_AHB2LPENCLRR: RWRegister<u32>,

    /// This register is used by the MPU
    pub RCC_MP_AHB3LPENSETR: RWRegister<u32>,

    /// This register is used by the MPU in order to clear the PERxLPEN bit
    pub RCC_MP_AHB3LPENCLRR: RWRegister<u32>,

    /// This register is used by the MPU
    pub RCC_MP_AHB4LPENSETR: RWRegister<u32>,

    /// This register is used by the MPU
    pub RCC_MP_AHB4LPENCLRR: RWRegister<u32>,

    /// This register is used by the MPU
    pub RCC_MP_AXIMLPENSETR: RWRegister<u32>,

    /// This register is used by the MPU
    pub RCC_MP_AXIMLPENCLRR: RWRegister<u32>,

    /// This register is used by the MPU in order to set the PERxLPEN bit
    pub RCC_MP_MLAHBLPENSETR: RWRegister<u32>,

    /// This register is used by the MPU in order to clear the PERxLPEN bit
    pub RCC_MP_MLAHBLPENCLRR: RWRegister<u32>,

    _reserved25: [u32; 16],

    /// This register is used by the MCU in order to set the PERxLPEN bit.
    pub RCC_MC_APB1LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bits
    pub RCC_MC_APB1LPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to set the PERxLPEN bit.
    pub RCC_MC_APB2LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bit
    pub RCC_MC_APB2LPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to set the PERxLPEN bit.
    pub RCC_MC_APB3LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bit
    pub RCC_MC_APB3LPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to set the PERxLPEN bit.
    pub RCC_MC_AHB2LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bit
    pub RCC_MC_AHB2LPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to set the PERxLPEN bit.
    pub RCC_MC_AHB3LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bit
    pub RCC_MC_AHB3LPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to set the PERxLPEN bit.
    pub RCC_MC_AHB4LPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
    pub RCC_MC_AHB4LPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.
    pub RCC_MC_AXIMLPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
    pub RCC_MC_AXIMLPENCLRR: RWRegister<u32>,

    /// This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.
    pub RCC_MC_MLAHBLPENSETR: RWRegister<u32>,

    /// This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
    pub RCC_MC_MLAHBLPENCLRR: RWRegister<u32>,

    _reserved26: [u32; 16],

    /// This register is used by the MCU to check the reset source.
    pub RCC_MC_RSTSCLRR: RWRegister<u32>,

    _reserved27: [u32; 4],

    /// This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details.
    pub RCC_MC_CIER: RWRegister<u32>,

    /// This register shall be used by the MCU in order to read and clear the interrupt flags.
    pub RCC_MC_CIFR: RWRegister<u32>,

    _reserved28: [u32; 246],

    /// This register gives the IP version
    pub RCC_VERR: RORegister<u32>,

    /// This register gives the unique identifier of the RCC
    pub RCC_IDR: RORegister<u32>,

    /// This register gives the decoding space, which is for the RCC of 4 kB.
    pub RCC_SIDR: RORegister<u32>,
}
pub struct ResetValues {
    pub RCC_TZCR: u32,
    pub RCC_OCENSETR: u32,
    pub RCC_OCENCLRR: u32,
    pub RCC_HSICFGR: u32,
    pub RCC_CSICFGR: u32,
    pub RCC_MPCKSELR: u32,
    pub RCC_ASSCKSELR: u32,
    pub RCC_RCK12SELR: u32,
    pub RCC_MPCKDIVR: u32,
    pub RCC_AXIDIVR: u32,
    pub RCC_APB4DIVR: u32,
    pub RCC_APB5DIVR: u32,
    pub RCC_RTCDIVR: u32,
    pub RCC_MSSCKSELR: u32,
    pub RCC_PLL1CR: u32,
    pub RCC_PLL1CFGR1: u32,
    pub RCC_PLL1CFGR2: u32,
    pub RCC_PLL1FRACR: u32,
    pub RCC_PLL1CSGR: u32,
    pub RCC_PLL2CR: u32,
    pub RCC_PLL2CFGR1: u32,
    pub RCC_PLL2CFGR2: u32,
    pub RCC_PLL2FRACR: u32,
    pub RCC_PLL2CSGR: u32,
    pub RCC_I2C46CKSELR: u32,
    pub RCC_SPI6CKSELR: u32,
    pub RCC_UART1CKSELR: u32,
    pub RCC_RNG1CKSELR: u32,
    pub RCC_CPERCKSELR: u32,
    pub RCC_STGENCKSELR: u32,
    pub RCC_DDRITFCR: u32,
    pub RCC_MP_BOOTCR: u32,
    pub RCC_MP_SREQSETR: u32,
    pub RCC_MP_SREQCLRR: u32,
    pub RCC_MP_GCR: u32,
    pub RCC_MP_APRSTCR: u32,
    pub RCC_MP_APRSTSR: u32,
    pub RCC_BDCR: u32,
    pub RCC_RDLSICR: u32,
    pub RCC_APB4RSTSETR: u32,
    pub RCC_APB4RSTCLRR: u32,
    pub RCC_APB5RSTSETR: u32,
    pub RCC_APB5RSTCLRR: u32,
    pub RCC_AHB5RSTSETR: u32,
    pub RCC_AHB5RSTCLRR: u32,
    pub RCC_AHB6RSTSETR: u32,
    pub RCC_AHB6RSTCLRR: u32,
    pub RCC_TZAHB6RSTSETR: u32,
    pub RCC_TZAHB6RSTCLRR: u32,
    pub RCC_MP_APB4ENSETR: u32,
    pub RCC_MP_APB4ENCLRR: u32,
    pub RCC_MP_APB5ENSETR: u32,
    pub RCC_MP_APB5ENCLRR: u32,
    pub RCC_MP_AHB5ENSETR: u32,
    pub RCC_MP_AHB5ENCLRR: u32,
    pub RCC_MP_AHB6ENSETR: u32,
    pub RCC_MP_AHB6ENCLRR: u32,
    pub RCC_MP_TZAHB6ENSETR: u32,
    pub RCC_MP_TZAHB6ENCLRR: u32,
    pub RCC_MC_APB4ENSETR: u32,
    pub RCC_MC_APB4ENCLRR: u32,
    pub RCC_MC_APB5ENSETR: u32,
    pub RCC_MC_APB5ENCLRR: u32,
    pub RCC_MC_AHB5ENSETR: u32,
    pub RCC_MC_AHB5ENCLRR: u32,
    pub RCC_MC_AHB6ENSETR: u32,
    pub RCC_MC_AHB6ENCLRR: u32,
    pub RCC_MP_APB4LPENSETR: u32,
    pub RCC_MP_APB4LPENCLRR: u32,
    pub RCC_MP_APB5LPENSETR: u32,
    pub RCC_MP_APB5LPENCLRR: u32,
    pub RCC_MP_AHB5LPENSETR: u32,
    pub RCC_MP_AHB5LPENCLRR: u32,
    pub RCC_MP_AHB6LPENSETR: u32,
    pub RCC_MP_AHB6LPENCLRR: u32,
    pub RCC_MP_TZAHB6LPENSETR: u32,
    pub RCC_MP_TZAHB6LPENCLRR: u32,
    pub RCC_MC_APB4LPENSETR: u32,
    pub RCC_MC_APB4LPENCLRR: u32,
    pub RCC_MC_APB5LPENSETR: u32,
    pub RCC_MC_APB5LPENCLRR: u32,
    pub RCC_MC_AHB5LPENSETR: u32,
    pub RCC_MC_AHB5LPENCLRR: u32,
    pub RCC_MC_AHB6LPENSETR: u32,
    pub RCC_MC_AHB6LPENCLRR: u32,
    pub RCC_BR_RSTSCLRR: u32,
    pub RCC_MP_GRSTCSETR: u32,
    pub RCC_MP_RSTSCLRR: u32,
    pub RCC_MP_IWDGFZSETR: u32,
    pub RCC_MP_IWDGFZCLRR: u32,
    pub RCC_MP_CIER: u32,
    pub RCC_MP_CIFR: u32,
    pub RCC_PWRLPDLYCR: u32,
    pub RCC_MP_RSTSSETR: u32,
    pub RCC_MCO1CFGR: u32,
    pub RCC_MCO2CFGR: u32,
    pub RCC_OCRDYR: u32,
    pub RCC_DBGCFGR: u32,
    pub RCC_RCK3SELR: u32,
    pub RCC_RCK4SELR: u32,
    pub RCC_TIMG1PRER: u32,
    pub RCC_TIMG2PRER: u32,
    pub RCC_MCUDIVR: u32,
    pub RCC_APB1DIVR: u32,
    pub RCC_APB2DIVR: u32,
    pub RCC_APB3DIVR: u32,
    pub RCC_PLL3CR: u32,
    pub RCC_PLL3CFGR1: u32,
    pub RCC_PLL3CFGR2: u32,
    pub RCC_PLL3FRACR: u32,
    pub RCC_PLL3CSGR: u32,
    pub RCC_PLL4CR: u32,
    pub RCC_PLL4CFGR1: u32,
    pub RCC_PLL4CFGR2: u32,
    pub RCC_PLL4FRACR: u32,
    pub RCC_PLL4CSGR: u32,
    pub RCC_I2C12CKSELR: u32,
    pub RCC_I2C35CKSELR: u32,
    pub RCC_SAI1CKSELR: u32,
    pub RCC_SAI2CKSELR: u32,
    pub RCC_SAI3CKSELR: u32,
    pub RCC_SAI4CKSELR: u32,
    pub RCC_SPI2S1CKSELR: u32,
    pub RCC_SPI2S23CKSELR: u32,
    pub RCC_SPI45CKSELR: u32,
    pub RCC_UART6CKSELR: u32,
    pub RCC_UART24CKSELR: u32,
    pub RCC_UART35CKSELR: u32,
    pub RCC_UART78CKSELR: u32,
    pub RCC_SDMMC12CKSELR: u32,
    pub RCC_SDMMC3CKSELR: u32,
    pub RCC_ETHCKSELR: u32,
    pub RCC_QSPICKSELR: u32,
    pub RCC_FMCCKSELR: u32,
    pub RCC_FDCANCKSELR: u32,
    pub RCC_SPDIFCKSELR: u32,
    pub RCC_CECCKSELR: u32,
    pub RCC_USBCKSELR: u32,
    pub RCC_RNG2CKSELR: u32,
    pub RCC_DSICKSELR: u32,
    pub RCC_ADCCKSELR: u32,
    pub RCC_LPTIM45CKSELR: u32,
    pub RCC_LPTIM23CKSELR: u32,
    pub RCC_LPTIM1CKSELR: u32,
    pub RCC_APB1RSTSETR: u32,
    pub RCC_APB1RSTCLRR: u32,
    pub RCC_APB2RSTSETR: u32,
    pub RCC_APB2RSTCLRR: u32,
    pub RCC_APB3RSTSETR: u32,
    pub RCC_APB3RSTCLRR: u32,
    pub RCC_AHB2RSTSETR: u32,
    pub RCC_AHB2RSTCLRR: u32,
    pub RCC_AHB3RSTSETR: u32,
    pub RCC_AHB3RSTCLRR: u32,
    pub RCC_AHB4RSTSETR: u32,
    pub RCC_AHB4RSTCLRR: u32,
    pub RCC_MP_APB1ENSETR: u32,
    pub RCC_MP_APB1ENCLRR: u32,
    pub RCC_MP_APB2ENSETR: u32,
    pub RCC_MP_APB2ENCLRR: u32,
    pub RCC_MP_APB3ENSETR: u32,
    pub RCC_MP_APB3ENCLRR: u32,
    pub RCC_MP_AHB2ENSETR: u32,
    pub RCC_MP_AHB2ENCLRR: u32,
    pub RCC_MP_AHB3ENSETR: u32,
    pub RCC_MP_AHB3ENCLRR: u32,
    pub RCC_MP_AHB4ENSETR: u32,
    pub RCC_MP_AHB4ENCLRR: u32,
    pub RCC_MP_MLAHBENSETR: u32,
    pub RCC_MP_MLAHBENCLRR: u32,
    pub RCC_MC_APB1ENSETR: u32,
    pub RCC_MC_APB1ENCLRR: u32,
    pub RCC_MC_APB2ENSETR: u32,
    pub RCC_MC_APB2ENCLRR: u32,
    pub RCC_MC_APB3ENSETR: u32,
    pub RCC_MC_APB3ENCLRR: u32,
    pub RCC_MC_AHB2ENSETR: u32,
    pub RCC_MC_AHB2ENCLRR: u32,
    pub RCC_MC_AHB3ENSETR: u32,
    pub RCC_MC_AHB3ENCLRR: u32,
    pub RCC_MC_AHB4ENSETR: u32,
    pub RCC_MC_AHB4ENCLRR: u32,
    pub RCC_MC_AXIMENSETR: u32,
    pub RCC_MC_AXIMENCLRR: u32,
    pub RCC_MC_MLAHBENSETR: u32,
    pub RCC_MC_MLAHBENCLRR: u32,
    pub RCC_MP_APB1LPENSETR: u32,
    pub RCC_MP_APB1LPENCLRR: u32,
    pub RCC_MP_APB2LPENSETR: u32,
    pub RCC_MP_APB2LPENCLRR: u32,
    pub RCC_MP_APB3LPENSETR: u32,
    pub RCC_MP_APB3LPENCLRR: u32,
    pub RCC_MP_AHB2LPENSETR: u32,
    pub RCC_MP_AHB2LPENCLRR: u32,
    pub RCC_MP_AHB3LPENSETR: u32,
    pub RCC_MP_AHB3LPENCLRR: u32,
    pub RCC_MP_AHB4LPENSETR: u32,
    pub RCC_MP_AHB4LPENCLRR: u32,
    pub RCC_MP_AXIMLPENSETR: u32,
    pub RCC_MP_AXIMLPENCLRR: u32,
    pub RCC_MP_MLAHBLPENSETR: u32,
    pub RCC_MP_MLAHBLPENCLRR: u32,
    pub RCC_MC_APB1LPENSETR: u32,
    pub RCC_MC_APB1LPENCLRR: u32,
    pub RCC_MC_APB2LPENSETR: u32,
    pub RCC_MC_APB2LPENCLRR: u32,
    pub RCC_MC_APB3LPENSETR: u32,
    pub RCC_MC_APB3LPENCLRR: u32,
    pub RCC_MC_AHB2LPENSETR: u32,
    pub RCC_MC_AHB2LPENCLRR: u32,
    pub RCC_MC_AHB3LPENSETR: u32,
    pub RCC_MC_AHB3LPENCLRR: u32,
    pub RCC_MC_AHB4LPENSETR: u32,
    pub RCC_MC_AHB4LPENCLRR: u32,
    pub RCC_MC_AXIMLPENSETR: u32,
    pub RCC_MC_AXIMLPENCLRR: u32,
    pub RCC_MC_MLAHBLPENSETR: u32,
    pub RCC_MC_MLAHBLPENCLRR: u32,
    pub RCC_MC_RSTSCLRR: u32,
    pub RCC_MC_CIER: u32,
    pub RCC_MC_CIFR: u32,
    pub RCC_VERR: u32,
    pub RCC_IDR: u32,
    pub RCC_SIDR: u32,
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

/// Access functions for the RCC peripheral instance
pub mod RCC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RCC
    pub const reset: ResetValues = ResetValues {
        RCC_TZCR: 0x00000003,
        RCC_OCENSETR: 0x00000001,
        RCC_OCENCLRR: 0x00000001,
        RCC_HSICFGR: 0x00000000,
        RCC_CSICFGR: 0x00001000,
        RCC_MPCKSELR: 0x80000000,
        RCC_ASSCKSELR: 0x80000000,
        RCC_RCK12SELR: 0x80000000,
        RCC_MPCKDIVR: 0x80000001,
        RCC_AXIDIVR: 0x80000000,
        RCC_APB4DIVR: 0x80000000,
        RCC_APB5DIVR: 0x80000000,
        RCC_RTCDIVR: 0x00000000,
        RCC_MSSCKSELR: 0x80000000,
        RCC_PLL1CR: 0x00000000,
        RCC_PLL1CFGR1: 0x00010031,
        RCC_PLL1CFGR2: 0x00010100,
        RCC_PLL1FRACR: 0x00000000,
        RCC_PLL1CSGR: 0x00000000,
        RCC_PLL2CR: 0x00000000,
        RCC_PLL2CFGR1: 0x00010063,
        RCC_PLL2CFGR2: 0x00010101,
        RCC_PLL2FRACR: 0x00000000,
        RCC_PLL2CSGR: 0x00000000,
        RCC_I2C46CKSELR: 0x00000000,
        RCC_SPI6CKSELR: 0x00000000,
        RCC_UART1CKSELR: 0x00000000,
        RCC_RNG1CKSELR: 0x00000000,
        RCC_CPERCKSELR: 0x00000000,
        RCC_STGENCKSELR: 0x00000000,
        RCC_DDRITFCR: 0x000FD02A,
        RCC_MP_BOOTCR: 0x00000000,
        RCC_MP_SREQSETR: 0x00000000,
        RCC_MP_SREQCLRR: 0x00000000,
        RCC_MP_GCR: 0x00000000,
        RCC_MP_APRSTCR: 0x00007F00,
        RCC_MP_APRSTSR: 0x00000000,
        RCC_BDCR: 0x00000020,
        RCC_RDLSICR: 0x00000000,
        RCC_APB4RSTSETR: 0x00000000,
        RCC_APB4RSTCLRR: 0x00000000,
        RCC_APB5RSTSETR: 0x00000000,
        RCC_APB5RSTCLRR: 0x00000000,
        RCC_AHB5RSTSETR: 0x00000000,
        RCC_AHB5RSTCLRR: 0x00000000,
        RCC_AHB6RSTSETR: 0x00000000,
        RCC_AHB6RSTCLRR: 0x00000000,
        RCC_TZAHB6RSTSETR: 0x00000000,
        RCC_TZAHB6RSTCLRR: 0x00000000,
        RCC_MP_APB4ENSETR: 0x00000000,
        RCC_MP_APB4ENCLRR: 0x00000000,
        RCC_MP_APB5ENSETR: 0x00000000,
        RCC_MP_APB5ENCLRR: 0x00000000,
        RCC_MP_AHB5ENSETR: 0x00010000,
        RCC_MP_AHB5ENCLRR: 0x00010000,
        RCC_MP_AHB6ENSETR: 0x00000000,
        RCC_MP_AHB6ENCLRR: 0x00000000,
        RCC_MP_TZAHB6ENSETR: 0x00000000,
        RCC_MP_TZAHB6ENCLRR: 0x00000000,
        RCC_MC_APB4ENSETR: 0x00000000,
        RCC_MC_APB4ENCLRR: 0x00000000,
        RCC_MC_APB5ENSETR: 0x00000000,
        RCC_MC_APB5ENCLRR: 0x00000000,
        RCC_MC_AHB5ENSETR: 0x00000000,
        RCC_MC_AHB5ENCLRR: 0x00000000,
        RCC_MC_AHB6ENSETR: 0x00000000,
        RCC_MC_AHB6ENCLRR: 0x00000000,
        RCC_MP_APB4LPENSETR: 0x00118111,
        RCC_MP_APB4LPENCLRR: 0x00118111,
        RCC_MP_APB5LPENSETR: 0x0011391D,
        RCC_MP_APB5LPENCLRR: 0x0011391D,
        RCC_MP_AHB5LPENSETR: 0x00000171,
        RCC_MP_AHB5LPENCLRR: 0x00000171,
        RCC_MP_AHB6LPENSETR: 0x011357A1,
        RCC_MP_AHB6LPENCLRR: 0x011357A1,
        RCC_MP_TZAHB6LPENSETR: 0x00000001,
        RCC_MP_TZAHB6LPENCLRR: 0x00000001,
        RCC_MC_APB4LPENSETR: 0x00110111,
        RCC_MC_APB4LPENCLRR: 0x00110111,
        RCC_MC_APB5LPENSETR: 0x0011391D,
        RCC_MC_APB5LPENCLRR: 0x0011391D,
        RCC_MC_AHB5LPENSETR: 0x00000171,
        RCC_MC_AHB5LPENCLRR: 0x00000171,
        RCC_MC_AHB6LPENSETR: 0x011357A1,
        RCC_MC_AHB6LPENCLRR: 0x011357A1,
        RCC_BR_RSTSCLRR: 0x00000015,
        RCC_MP_GRSTCSETR: 0x00000000,
        RCC_MP_RSTSCLRR: 0x00000000,
        RCC_MP_IWDGFZSETR: 0x00000000,
        RCC_MP_IWDGFZCLRR: 0x00000000,
        RCC_MP_CIER: 0x00000000,
        RCC_MP_CIFR: 0x00000000,
        RCC_PWRLPDLYCR: 0x00000000,
        RCC_MP_RSTSSETR: 0x00000000,
        RCC_MCO1CFGR: 0x00000000,
        RCC_MCO2CFGR: 0x00000000,
        RCC_OCRDYR: 0x00000000,
        RCC_DBGCFGR: 0x00000001,
        RCC_RCK3SELR: 0x80000000,
        RCC_RCK4SELR: 0x80000000,
        RCC_TIMG1PRER: 0x80000000,
        RCC_TIMG2PRER: 0x80000000,
        RCC_MCUDIVR: 0x80000000,
        RCC_APB1DIVR: 0x80000000,
        RCC_APB2DIVR: 0x80000000,
        RCC_APB3DIVR: 0x80000000,
        RCC_PLL3CR: 0x00000000,
        RCC_PLL3CFGR1: 0x00010031,
        RCC_PLL3CFGR2: 0x00010101,
        RCC_PLL3FRACR: 0x00000000,
        RCC_PLL3CSGR: 0x00000000,
        RCC_PLL4CR: 0x00000000,
        RCC_PLL4CFGR1: 0x00010031,
        RCC_PLL4CFGR2: 0x00000000,
        RCC_PLL4FRACR: 0x00000000,
        RCC_PLL4CSGR: 0x00000000,
        RCC_I2C12CKSELR: 0x00000000,
        RCC_I2C35CKSELR: 0x00000000,
        RCC_SAI1CKSELR: 0x00000000,
        RCC_SAI2CKSELR: 0x00000000,
        RCC_SAI3CKSELR: 0x00000000,
        RCC_SAI4CKSELR: 0x00000000,
        RCC_SPI2S1CKSELR: 0x00000000,
        RCC_SPI2S23CKSELR: 0x00000000,
        RCC_SPI45CKSELR: 0x00000000,
        RCC_UART6CKSELR: 0x00000000,
        RCC_UART24CKSELR: 0x00000000,
        RCC_UART35CKSELR: 0x00000000,
        RCC_UART78CKSELR: 0x00000000,
        RCC_SDMMC12CKSELR: 0x00000003,
        RCC_SDMMC3CKSELR: 0x00000000,
        RCC_ETHCKSELR: 0x00000000,
        RCC_QSPICKSELR: 0x00000000,
        RCC_FMCCKSELR: 0x00000000,
        RCC_FDCANCKSELR: 0x00000000,
        RCC_SPDIFCKSELR: 0x00000000,
        RCC_CECCKSELR: 0x00000000,
        RCC_USBCKSELR: 0x00000000,
        RCC_RNG2CKSELR: 0x00000000,
        RCC_DSICKSELR: 0x00000000,
        RCC_ADCCKSELR: 0x00000000,
        RCC_LPTIM45CKSELR: 0x00000000,
        RCC_LPTIM23CKSELR: 0x00000000,
        RCC_LPTIM1CKSELR: 0x00000000,
        RCC_APB1RSTSETR: 0x00000000,
        RCC_APB1RSTCLRR: 0x00000000,
        RCC_APB2RSTSETR: 0x00000000,
        RCC_APB2RSTCLRR: 0x00000000,
        RCC_APB3RSTSETR: 0x00000000,
        RCC_APB3RSTCLRR: 0x00000000,
        RCC_AHB2RSTSETR: 0x00000000,
        RCC_AHB2RSTCLRR: 0x00000000,
        RCC_AHB3RSTSETR: 0x00000000,
        RCC_AHB3RSTCLRR: 0x00000000,
        RCC_AHB4RSTSETR: 0x00000000,
        RCC_AHB4RSTCLRR: 0x00000000,
        RCC_MP_APB1ENSETR: 0x00000000,
        RCC_MP_APB1ENCLRR: 0x00000000,
        RCC_MP_APB2ENSETR: 0x00000000,
        RCC_MP_APB2ENCLRR: 0x00000000,
        RCC_MP_APB3ENSETR: 0x00000000,
        RCC_MP_APB3ENCLRR: 0x00000000,
        RCC_MP_AHB2ENSETR: 0x00000000,
        RCC_MP_AHB2ENCLRR: 0x00000000,
        RCC_MP_AHB3ENSETR: 0x00000000,
        RCC_MP_AHB3ENCLRR: 0x00000000,
        RCC_MP_AHB4ENSETR: 0x00000000,
        RCC_MP_AHB4ENCLRR: 0x00000000,
        RCC_MP_MLAHBENSETR: 0x00000010,
        RCC_MP_MLAHBENCLRR: 0x00000010,
        RCC_MC_APB1ENSETR: 0x00000000,
        RCC_MC_APB1ENCLRR: 0x00000000,
        RCC_MC_APB2ENSETR: 0x00000000,
        RCC_MC_APB2ENCLRR: 0x00000000,
        RCC_MC_APB3ENSETR: 0x00000000,
        RCC_MC_APB3ENCLRR: 0x00000000,
        RCC_MC_AHB2ENSETR: 0x00000000,
        RCC_MC_AHB2ENCLRR: 0x00000000,
        RCC_MC_AHB3ENSETR: 0x00000000,
        RCC_MC_AHB3ENCLRR: 0x00000000,
        RCC_MC_AHB4ENSETR: 0x00000000,
        RCC_MC_AHB4ENCLRR: 0x00000000,
        RCC_MC_AXIMENSETR: 0x00000000,
        RCC_MC_AXIMENCLRR: 0x00000000,
        RCC_MC_MLAHBENSETR: 0x00000010,
        RCC_MC_MLAHBENCLRR: 0x00000010,
        RCC_MP_APB1LPENSETR: 0xADEFDBFF,
        RCC_MP_APB1LPENCLRR: 0xADEFDBFF,
        RCC_MP_APB2LPENSETR: 0x0137271F,
        RCC_MP_APB2LPENCLRR: 0x0137271F,
        RCC_MP_APB3LPENSETR: 0x0003290F,
        RCC_MP_APB3LPENCLRR: 0x0003290F,
        RCC_MP_AHB2LPENSETR: 0x00010127,
        RCC_MP_AHB2LPENCLRR: 0x00010127,
        RCC_MP_AHB3LPENSETR: 0x000018F1,
        RCC_MP_AHB3LPENCLRR: 0x000018F1,
        RCC_MP_AHB4LPENSETR: 0x000007FF,
        RCC_MP_AHB4LPENCLRR: 0x000007FF,
        RCC_MP_AXIMLPENSETR: 0x00000001,
        RCC_MP_AXIMLPENCLRR: 0x00000001,
        RCC_MP_MLAHBLPENSETR: 0x00000017,
        RCC_MP_MLAHBLPENCLRR: 0x00000017,
        RCC_MC_APB1LPENSETR: 0xBDEFDBFF,
        RCC_MC_APB1LPENCLRR: 0xBDEFDBFF,
        RCC_MC_APB2LPENSETR: 0x0137271F,
        RCC_MC_APB2LPENCLRR: 0x0137271F,
        RCC_MC_APB3LPENSETR: 0x0003290F,
        RCC_MC_APB3LPENCLRR: 0x0003290F,
        RCC_MC_AHB2LPENSETR: 0x00010127,
        RCC_MC_AHB2LPENCLRR: 0x00010127,
        RCC_MC_AHB3LPENSETR: 0x000018F1,
        RCC_MC_AHB3LPENCLRR: 0x000018F1,
        RCC_MC_AHB4LPENSETR: 0x000007FF,
        RCC_MC_AHB4LPENCLRR: 0x000007FF,
        RCC_MC_AXIMLPENSETR: 0x00000001,
        RCC_MC_AXIMLPENCLRR: 0x00000001,
        RCC_MC_MLAHBLPENSETR: 0x00000017,
        RCC_MC_MLAHBLPENCLRR: 0x00000017,
        RCC_MC_RSTSCLRR: 0x00000015,
        RCC_MC_CIER: 0x00000000,
        RCC_MC_CIFR: 0x00000000,
        RCC_VERR: 0x00000011,
        RCC_IDR: 0x00000001,
        RCC_SIDR: 0xA3C5DD04,
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
pub const RCC: *const RegisterBlock = 0x50000000 as *const _;
