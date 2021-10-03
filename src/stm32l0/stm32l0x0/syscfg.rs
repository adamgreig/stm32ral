#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System configuration controller register

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// SYSCFG configuration register 1
pub mod CFGR1 {

    /// Boot mode selected by the boot pins status bits
    pub mod BOOT_MODE {
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

            /// 0b00: Main Flash memory boot mode
            pub const MainFlash: u32 = 0b00;

            /// 0b01: System Flash memory boot mode
            pub const SystemFlash: u32 = 0b01;

            /// 0b11: Embedded SRAM boot mode
            pub const SRAM: u32 = 0b11;
        }
    }

    /// Memory mapping selection bits
    pub mod MEM_MODE {
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

            /// 0b00: Main Flash memory mapped at 0x0000_0000
            pub const MainFlash: u32 = 0b00;

            /// 0b01: System Flash memory mapped at 0x0000_0000
            pub const SystemFlash: u32 = 0b01;

            /// 0b11: Embedded SRAM mapped at 0x0000_0000
            pub const SRAM: u32 = 0b11;
        }
    }
}

/// SYSCFG configuration register 2
pub mod CFGR2 {

    /// I2C2 Fm+ drive capability enable bit
    pub mod I2C2_FMP {
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

    /// I2C1 Fm+ drive capability enable bit
    pub mod I2C1_FMP {
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

    /// Fm+ drive capability on PB9 enable bit
    pub mod I2C_PB9_FMP {
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

    /// Fm+ drive capability on PB8 enable bit
    pub mod I2C_PB8_FMP {
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

    /// Fm+ drive capability on PB7 enable bit
    pub mod I2C_PB7_FMP {
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

    /// Fm+ drive capability on PB6 enable bit
    pub mod I2C_PB6_FMP {
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

    /// Configuration of internal VLCD rail connection to optional external capacitor
    pub mod CAPA {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Firewall disable bit
    pub mod FWDIS {
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

/// external interrupt configuration register 1
pub mod EXTICR1 {

    /// EXTI x configuration (x = 0 to 3)
    pub mod EXTI3 {
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

            /// 0b0000: Select PA3 as the source input for the EXTI3 external interrupt
            pub const PA3: u32 = 0b0000;

            /// 0b0001: Select PB3 as the source input for the EXTI3 external interrupt
            pub const PB3: u32 = 0b0001;

            /// 0b0010: Select PC3 as the source input for the EXTI3 external interrupt
            pub const PC3: u32 = 0b0010;

            /// 0b0011: Select PD3 as the source input for the EXTI3 external interrupt
            pub const PD3: u32 = 0b0011;

            /// 0b0100: Select PE3 as the source input for the EXTI3 external interrupt
            pub const PE3: u32 = 0b0100;
        }
    }

    /// EXTI x configuration (x = 0 to 3)
    pub mod EXTI2 {
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

            /// 0b0000: Select PA2 as the source input for the EXTI2 external interrupt
            pub const PA2: u32 = 0b0000;

            /// 0b0001: Select PB2 as the source input for the EXTI2 external interrupt
            pub const PB2: u32 = 0b0001;

            /// 0b0010: Select PC2 as the source input for the EXTI2 external interrupt
            pub const PC2: u32 = 0b0010;

            /// 0b0011: Select PD2 as the source input for the EXTI2 external interrupt
            pub const PD2: u32 = 0b0011;

            /// 0b0100: Select PE2 as the source input for the EXTI2 external interrupt
            pub const PE2: u32 = 0b0100;
        }
    }

    /// EXTI x configuration (x = 0 to 3)
    pub mod EXTI1 {
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

            /// 0b0000: Select PA1 as the source input for the EXTI1 external interrupt
            pub const PA1: u32 = 0b0000;

            /// 0b0001: Select PB1 as the source input for the EXTI1 external interrupt
            pub const PB1: u32 = 0b0001;

            /// 0b0010: Select PC1 as the source input for the EXTI1 external interrupt
            pub const PC1: u32 = 0b0010;

            /// 0b0011: Select PD1 as the source input for the EXTI1 external interrupt
            pub const PD1: u32 = 0b0011;

            /// 0b0100: Select PE1 as the source input for the EXTI1 external interrupt
            pub const PE1: u32 = 0b0100;

            /// 0b0101: Select PH1 as the source input for the EXTI1 external interrupt
            pub const PH1: u32 = 0b0101;
        }
    }

    /// EXTI x configuration (x = 0 to 3)
    pub mod EXTI0 {
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

            /// 0b0000: Select PA0 as the source input for the EXTI0 external interrupt
            pub const PA0: u32 = 0b0000;

            /// 0b0001: Select PB0 as the source input for the EXTI0 external interrupt
            pub const PB0: u32 = 0b0001;

            /// 0b0010: Select PC0 as the source input for the EXTI0 external interrupt
            pub const PC0: u32 = 0b0010;

            /// 0b0011: Select PD0 as the source input for the EXTI0 external interrupt
            pub const PD0: u32 = 0b0011;

            /// 0b0100: Select PE0 as the source input for the EXTI0 external interrupt
            pub const PE0: u32 = 0b0100;

            /// 0b0101: Select PH0 as the source input for the EXTI0 external interrupt
            pub const PH0: u32 = 0b0101;
        }
    }
}

/// external interrupt configuration register 2
pub mod EXTICR2 {

    /// EXTI x configuration (x = 4 to 7)
    pub mod EXTI7 {
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

            /// 0b0000: Select PA7 as the source input for the EXTI7 external interrupt
            pub const PA7: u32 = 0b0000;

            /// 0b0001: Select PB7 as the source input for the EXTI7 external interrupt
            pub const PB7: u32 = 0b0001;

            /// 0b0010: Select PC7 as the source input for the EXTI7 external interrupt
            pub const PC7: u32 = 0b0010;

            /// 0b0011: Select PD7 as the source input for the EXTI7 external interrupt
            pub const PD7: u32 = 0b0011;

            /// 0b0100: Select PE7 as the source input for the EXTI7 external interrupt
            pub const PE7: u32 = 0b0100;
        }
    }

    /// EXTI x configuration (x = 4 to 7)
    pub mod EXTI6 {
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

            /// 0b0000: Select PA6 as the source input for the EXTI6 external interrupt
            pub const PA6: u32 = 0b0000;

            /// 0b0001: Select PB6 as the source input for the EXTI6 external interrupt
            pub const PB6: u32 = 0b0001;

            /// 0b0010: Select PC6 as the source input for the EXTI6 external interrupt
            pub const PC6: u32 = 0b0010;

            /// 0b0011: Select PD6 as the source input for the EXTI6 external interrupt
            pub const PD6: u32 = 0b0011;

            /// 0b0100: Select PE6 as the source input for the EXTI6 external interrupt
            pub const PE6: u32 = 0b0100;
        }
    }

    /// EXTI x configuration (x = 4 to 7)
    pub mod EXTI5 {
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

            /// 0b0000: Select PA5 as the source input for the EXTI5 external interrupt
            pub const PA5: u32 = 0b0000;

            /// 0b0001: Select PB5 as the source input for the EXTI5 external interrupt
            pub const PB5: u32 = 0b0001;

            /// 0b0010: Select PC5 as the source input for the EXTI5 external interrupt
            pub const PC5: u32 = 0b0010;

            /// 0b0011: Select PD5 as the source input for the EXTI5 external interrupt
            pub const PD5: u32 = 0b0011;

            /// 0b0100: Select PE5 as the source input for the EXTI5 external interrupt
            pub const PE5: u32 = 0b0100;
        }
    }

    /// EXTI x configuration (x = 4 to 7)
    pub mod EXTI4 {
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

            /// 0b0000: Select PA4 as the source input for the EXTI4 external interrupt
            pub const PA4: u32 = 0b0000;

            /// 0b0001: Select PB4 as the source input for the EXTI4 external interrupt
            pub const PB4: u32 = 0b0001;

            /// 0b0010: Select PC4 as the source input for the EXTI4 external interrupt
            pub const PC4: u32 = 0b0010;

            /// 0b0011: Select PD4 as the source input for the EXTI4 external interrupt
            pub const PD4: u32 = 0b0011;

            /// 0b0100: Select PE4 as the source input for the EXTI4 external interrupt
            pub const PE4: u32 = 0b0100;
        }
    }
}

/// external interrupt configuration register 3
pub mod EXTICR3 {

    /// EXTI x configuration (x = 8 to 11)
    pub mod EXTI11 {
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

            /// 0b0000: Select PA11 as the source input for the EXTI11 external interrupt
            pub const PA11: u32 = 0b0000;

            /// 0b0001: Select PB11 as the source input for the EXTI11 external interrupt
            pub const PB11: u32 = 0b0001;

            /// 0b0010: Select PC11 as the source input for the EXTI11 external interrupt
            pub const PC11: u32 = 0b0010;

            /// 0b0011: Select PD11 as the source input for the EXTI11 external interrupt
            pub const PD11: u32 = 0b0011;

            /// 0b0100: Select PE11 as the source input for the EXTI11 external interrupt
            pub const PE11: u32 = 0b0100;

            /// 0b0101: Select PH11 as the source input for the EXTI11 external interrupt
            pub const PH11: u32 = 0b0101;
        }
    }

    /// EXTI10
    pub mod EXTI10 {
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

            /// 0b0000: Select PA10 as the source input for the EXTI10 external interrupt
            pub const PA10: u32 = 0b0000;

            /// 0b0001: Select PB10 as the source input for the EXTI10 external interrupt
            pub const PB10: u32 = 0b0001;

            /// 0b0010: Select PC10 as the source input for the EXTI10 external interrupt
            pub const PC10: u32 = 0b0010;

            /// 0b0011: Select PD10 as the source input for the EXTI10 external interrupt
            pub const PD10: u32 = 0b0011;

            /// 0b0100: Select PE10 as the source input for the EXTI10 external interrupt
            pub const PE10: u32 = 0b0100;

            /// 0b0101: Select PH10 as the source input for the EXTI10 external interrupt
            pub const PH10: u32 = 0b0101;
        }
    }

    /// EXTI x configuration (x = 8 to 11)
    pub mod EXTI9 {
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

            /// 0b0000: Select PA9 as the source input for the EXTI9 external interrupt
            pub const PA9: u32 = 0b0000;

            /// 0b0001: Select PB9 as the source input for the EXTI9 external interrupt
            pub const PB9: u32 = 0b0001;

            /// 0b0010: Select PC9 as the source input for the EXTI9 external interrupt
            pub const PC9: u32 = 0b0010;

            /// 0b0011: Select PD9 as the source input for the EXTI9 external interrupt
            pub const PD9: u32 = 0b0011;

            /// 0b0100: Select PE9 as the source input for the EXTI9 external interrupt
            pub const PE9: u32 = 0b0100;

            /// 0b0101: Select PH9 as the source input for the EXTI9 external interrupt
            pub const PH9: u32 = 0b0101;
        }
    }

    /// EXTI x configuration (x = 8 to 11)
    pub mod EXTI8 {
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

            /// 0b0000: Select PA8 as the source input for the EXTI8 external interrupt
            pub const PA8: u32 = 0b0000;

            /// 0b0001: Select PB8 as the source input for the EXTI8 external interrupt
            pub const PB8: u32 = 0b0001;

            /// 0b0010: Select PC8 as the source input for the EXTI8 external interrupt
            pub const PC8: u32 = 0b0010;

            /// 0b0011: Select PD8 as the source input for the EXTI8 external interrupt
            pub const PD8: u32 = 0b0011;

            /// 0b0100: Select PE8 as the source input for the EXTI8 external interrupt
            pub const PE8: u32 = 0b0100;
        }
    }
}

/// external interrupt configuration register 4
pub mod EXTICR4 {

    /// EXTI x configuration (x = 12 to 15)
    pub mod EXTI15 {
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

            /// 0b0000: Select PA15 as the source input for the EXTI15 external interrupt
            pub const PA15: u32 = 0b0000;

            /// 0b0001: Select PB15 as the source input for the EXTI15 external interrupt
            pub const PB15: u32 = 0b0001;

            /// 0b0010: Select PC15 as the source input for the EXTI15 external interrupt
            pub const PC15: u32 = 0b0010;

            /// 0b0011: Select PD15 as the source input for the EXTI15 external interrupt
            pub const PD15: u32 = 0b0011;

            /// 0b0100: Select PE15 as the source input for the EXTI15 external interrupt
            pub const PE15: u32 = 0b0100;
        }
    }

    /// EXTI14
    pub mod EXTI14 {
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

            /// 0b0000: Select PA14 as the source input for the EXTI14 external interrupt
            pub const PA14: u32 = 0b0000;

            /// 0b0001: Select PB14 as the source input for the EXTI14 external interrupt
            pub const PB14: u32 = 0b0001;

            /// 0b0010: Select PC14 as the source input for the EXTI14 external interrupt
            pub const PC14: u32 = 0b0010;

            /// 0b0011: Select PD14 as the source input for the EXTI14 external interrupt
            pub const PD14: u32 = 0b0011;

            /// 0b0100: Select PE14 as the source input for the EXTI14 external interrupt
            pub const PE14: u32 = 0b0100;
        }
    }

    /// EXTI13
    pub mod EXTI13 {
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

            /// 0b0000: Select PA13 as the source input for the EXTI13 external interrupt
            pub const PA13: u32 = 0b0000;

            /// 0b0001: Select PB13 as the source input for the EXTI13 external interrupt
            pub const PB13: u32 = 0b0001;

            /// 0b0010: Select PC13 as the source input for the EXTI13 external interrupt
            pub const PC13: u32 = 0b0010;

            /// 0b0011: Select PD13 as the source input for the EXTI13 external interrupt
            pub const PD13: u32 = 0b0011;

            /// 0b0100: Select PE13 as the source input for the EXTI13 external interrupt
            pub const PE13: u32 = 0b0100;
        }
    }

    /// EXTI12
    pub mod EXTI12 {
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

            /// 0b0000: Select PA12 as the source input for the EXTI12 external interrupt
            pub const PA12: u32 = 0b0000;

            /// 0b0001: Select PB12 as the source input for the EXTI12 external interrupt
            pub const PB12: u32 = 0b0001;

            /// 0b0010: Select PC12 as the source input for the EXTI12 external interrupt
            pub const PC12: u32 = 0b0010;

            /// 0b0011: Select PD12 as the source input for the EXTI12 external interrupt
            pub const PD12: u32 = 0b0011;

            /// 0b0100: Select PE12 as the source input for the EXTI12 external interrupt
            pub const PE12: u32 = 0b0100;
        }
    }
}

/// SYSCFG configuration register 3
pub mod CFGR3 {

    /// REF_CTRL lock bit
    pub mod REF_LOCK {
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

            /// 0b0: SYSCFG_CFGR3\[31:0\] bits are read/write
            pub const ReadWrite: u32 = 0b0;

            /// 0b1: SYSCFG_CFGR3\[31:0\] bits are read-only
            pub const ReadOnly: u32 = 0b1;
        }
    }

    /// VREFINT ready flag
    pub mod VREFINT_RDYF {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VREFINT OFF
            pub const NotReady: u32 = 0b0;

            /// 0b1: VREFINT ready
            pub const Ready: u32 = 0b1;
        }
    }

    /// VREFINT for comparator ready flag
    pub mod VREFINT_COMP_RDYF {
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

    /// VREFINT for ADC ready flag
    pub mod VREFINT_ADC_RDYF {
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

    /// Sensor for ADC ready flag
    pub mod SENSOR_ADC_RDYF {
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

    /// VREFINT for 48 MHz RC oscillator ready flag
    pub mod REF_RC48MHz_RDYF {
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

    /// VREFINT reference for 48 MHz RC oscillator enable bit
    pub mod ENREF_RC48MHz {
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

    /// VREFINT reference for comparator 2 enable bit
    pub mod ENBUF_VREFINT_COMP {
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

    /// Sensor reference for ADC enable bit
    pub mod ENBUF_SENSOR_ADC {
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

            /// 0b0: Disables the buffer used to generate VREFINT reference for the temperature sensor
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enables the buffer used to generate VREFINT reference for the temperature sensor
            pub const Enabled: u32 = 0b1;
        }
    }

    /// VREFINT reference for ADC enable bit
    pub mod ENBUF_BGAP_ADC {
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

    /// BGAP_ADC connection bit
    pub mod SEL_VREF_OUT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: no pad connected
            pub const NoConnection: u32 = 0b00;

            /// 0b01: PB0 connected
            pub const PB0: u32 = 0b01;

            /// 0b10: PB1 connected
            pub const PB1: u32 = 0b10;

            /// 0b11: PB0 and PB1 connected
            pub const Both: u32 = 0b11;
        }
    }

    /// Vref Enable bit
    pub mod EN_BGAP {
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
#[repr(C)]
pub struct RegisterBlock {
    /// SYSCFG configuration register 1
    pub CFGR1: RWRegister<u32>,

    /// SYSCFG configuration register 2
    pub CFGR2: RWRegister<u32>,

    /// external interrupt configuration register 1
    pub EXTICR1: RWRegister<u32>,

    /// external interrupt configuration register 2
    pub EXTICR2: RWRegister<u32>,

    /// external interrupt configuration register 3
    pub EXTICR3: RWRegister<u32>,

    /// external interrupt configuration register 4
    pub EXTICR4: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// SYSCFG configuration register 3
    pub CFGR3: RWRegister<u32>,
}
pub struct ResetValues {
    pub CFGR1: u32,
    pub CFGR2: u32,
    pub EXTICR1: u32,
    pub EXTICR2: u32,
    pub EXTICR3: u32,
    pub EXTICR4: u32,
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
#[cfg(feature = "rtic")]
unsafe impl Send for Instance {}

/// Access functions for the SYSCFG peripheral instance
pub mod SYSCFG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40010000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SYSCFG
    pub const reset: ResetValues = ResetValues {
        CFGR1: 0x00000000,
        CFGR2: 0x00000000,
        EXTICR1: 0x00000000,
        EXTICR2: 0x00000000,
        EXTICR3: 0x00000000,
        EXTICR4: 0x00000000,
        CFGR3: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SYSCFG_TAKEN: bool = false;

    /// Safe access to SYSCFG
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
            if SYSCFG_TAKEN {
                None
            } else {
                SYSCFG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SYSCFG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SYSCFG_TAKEN && inst.addr == INSTANCE.addr {
                SYSCFG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SYSCFG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SYSCFG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SYSCFG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SYSCFG: *const RegisterBlock = 0x40010000 as *const _;
