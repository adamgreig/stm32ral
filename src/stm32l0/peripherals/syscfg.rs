#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System configuration controller and Comparator
//!
//! Used by: stm32l0x2, stm32l0x3

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

    /// User bank swapping
    pub mod UFB {
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

            /// 0b0: Flash Program memory Bank 1 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 1 at 0x0808 0000 (aliased at 0x0008 0000 if MEM_MODE=00)
            pub const Bank1: u32 = 0b0;

            /// 0b1: Flash Program memory Bank 2 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 2 at 0x0808 0000 (and aliased at 0x0008 0000 if MEM_MODE=00)
            pub const Bank2: u32 = 0b1;
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
        /// Read-write values
        pub mod RW {

            /// 0b0: FM+ mode is controlled by I2C_Pxx_FMP bits only
            pub const Standard: u32 = 0b0;

            /// 0b1: FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers
            pub const FMP: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: FM+ mode is controlled by I2C_Pxx_FMP bits only
            pub const Standard: u32 = 0b0;

            /// 0b1: FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers
            pub const FMP: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: PB9 pin operate in standard mode
            pub const Standard: u32 = 0b0;

            /// 0b1: I2C FM+ mode enabled on PB9 and the Speed control is bypassed
            pub const FMP: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: PB8 pin operate in standard mode
            pub const Standard: u32 = 0b0;

            /// 0b1: I2C FM+ mode enabled on PB8 and the Speed control is bypassed
            pub const FMP: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: PB7 pin operate in standard mode
            pub const Standard: u32 = 0b0;

            /// 0b1: I2C FM+ mode enabled on PB7 and the Speed control is bypassed
            pub const FMP: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: PB6 pin operate in standard mode
            pub const Standard: u32 = 0b0;

            /// 0b1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed
            pub const FMP: u32 = 0b1;
        }
    }

    /// I2C3 Fm+ drive capability enable bit
    pub mod I2C3_FMP {
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

            /// 0b0: FM+ mode is controlled by I2C_Pxx_FMP bits only
            pub const Standard: u32 = 0b0;

            /// 0b1: FM+ mode is enabled on all I2C3 pins selected through selection bits in GPIOx_AFR registers
            pub const FMP: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Firewall access enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: Firewall access disabled
            pub const Disabled: u32 = 0b1;
        }
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

            /// 0b0101: Select PH3 as the source input for the EXTI3 external interrupt
            pub const PH3: u32 = 0b0101;
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

            /// 0b0101: Select PH2 as the source input for the EXTI2 external interrupt
            pub const PH2: u32 = 0b0101;
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

            /// 0b0101: Select PH8 as the source input for the EXTI8 external interrupt
            pub const PH8: u32 = 0b0101;
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

    /// VREFINT reference for HSI48 oscillator enable bit
    pub mod ENREF_HSI48 {
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

    /// SYSCFG_CFGR3 lock bit
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

    /// VREFINT reference for COMP2 scaler enable bit
    pub mod ENBUF_VREFINT_COMP2 {
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

            /// 0b0: Disables the buffer used to generate VREFINT references for COMP2
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enables the buffer used to generate VREFINT references for COMP2
            pub const Enabled: u32 = 0b1;
        }
    }

    /// VREFINT reference for ADC enable bit
    pub mod ENBUF_VREFINT_ADC {
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

            /// 0b0: Disables the buffer used to generate VREFINT reference for the ADC
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enables the buffer used to generate VREFINT reference for the ADC
            pub const Enabled: u32 = 0b1;
        }
    }

    /// VREFINT enable and scaler control for COMP2 enable bit
    pub mod EN_VREFINT {
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

            /// 0b0: VREFINT voltage disabled in low-power mode (if ULP=1) and scaler for COMP2 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: VREFINT voltage enabled in low-power mode and scaler for COMP2 enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Comparator 1 control and status register
pub mod COMP1_CSR {

    /// COMP1_CSR register lock bit
    pub mod COMP1LOCK {
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

            /// 0b0: COMP1_CSR\[31:0\] for comparator 1 are read/write
            pub const ReadWrite: u32 = 0b0;

            /// 0b1: COMP1_CSR\[31:0\] for comparator 1 are read-only
            pub const ReadOnly: u32 = 0b1;
        }
    }

    /// Comparator 1 output status bit
    pub mod COMP1VALUE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Comparator values are not equal
            pub const NotEqual: u32 = 0b0;

            /// 0b1: Comparator values are equal
            pub const Equal: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Comparator 1 polarity selection bit
    pub mod COMP1POLARITY {
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

            /// 0b0: Comparator 1 output value not inverted
            pub const NotInverted: u32 = 0b0;

            /// 0b1: Comparator 1 output value inverted
            pub const Inverted: u32 = 0b1;
        }
    }

    /// Comparator 1 LPTIM input propagation bit
    pub mod COMP1LPTIMIN1 {
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

            /// 0b0: Comparator 1 output gated
            pub const Gated: u32 = 0b0;

            /// 0b1: Comparator 1 output sent to LPTIM input 1
            pub const NotGated: u32 = 0b1;
        }
    }

    /// Comparator 1 window mode selection bit
    pub mod COMP1WM {
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

            /// 0b0: Plus input of comparator 1 connected to PA1
            pub const PA1: u32 = 0b0;

            /// 0b1: Plus input of comparator 1 shorted with Plus input of comparator 2 (see COMP1_CSR)
            pub const Comp2Plus: u32 = 0b1;
        }
    }

    /// Comparator 1 Input Minus connection configuration bit
    pub mod COMP1INNSEL {
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

            /// 0b00: VREFINT
            pub const VREFINT: u32 = 0b00;

            /// 0b01: PA0
            pub const PA0: u32 = 0b01;

            /// 0b10: PA4
            pub const PA4: u32 = 0b10;

            /// 0b11: PA5
            pub const PA5: u32 = 0b11;
        }
    }

    /// Comparator 1 enable bit
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

            /// 0b0: Comparator 1 switched OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: Comparator 1 switched ON
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Comparator 2 control and status register
pub mod COMP2_CSR {

    /// COMP2_CSR register lock bit
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

            /// 0b0: COMP2_CSR\[31:0\] for comparator 2 are read/write
            pub const ReadWrite: u32 = 0b0;

            /// 0b1: COMP2_CSR\[31:0\] for comparator 2 are read-only
            pub const ReadOnly: u32 = 0b1;
        }
    }

    /// Comparator 2 output status bit
    pub mod COMP2VALUE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Comparator values are not equal
            pub const NotEqual: u32 = 0b0;

            /// 0b1: Comparator values are equal
            pub const Equal: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Comparator 2 polarity selection bit
    pub mod COMP2POLARITY {
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

            /// 0b0: Comparator 2 output value not inverted
            pub const NotInverted: u32 = 0b0;

            /// 0b1: Comparator 2 output value inverted
            pub const Inverted: u32 = 0b1;
        }
    }

    /// Comparator 2 LPTIM input 1 propagation bit
    pub mod COMP2LPTIMIN1 {
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

            /// 0b0: Comparator 2 output gated
            pub const Gated: u32 = 0b0;

            /// 0b1: Comparator 2 output sent to LPTIM input 1
            pub const NotGated: u32 = 0b1;
        }
    }

    /// Comparator 2 LPTIM input 2 propagation bit
    pub mod COMP2LPTIMIN2 {
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

            /// 0b0: Comparator 2 output gated
            pub const Gated: u32 = 0b0;

            /// 0b1: Comparator 2 output sent to LPTIM input 2
            pub const NotGated: u32 = 0b1;
        }
    }

    /// Comparator 2 Input Plus connection configuration bit
    pub mod COMP2INPSEL {
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

            /// 0b000: PA3
            pub const PA3: u32 = 0b000;

            /// 0b001: PB4
            pub const PB4: u32 = 0b001;

            /// 0b010: PB5
            pub const PB5: u32 = 0b010;

            /// 0b011: PB6
            pub const PB6: u32 = 0b011;

            /// 0b100: PB7
            pub const PB7: u32 = 0b100;

            /// 0b101: PA7
            pub const PA7: u32 = 0b101;
        }
    }

    /// Comparator 2 Input Minus connection configuration bit
    pub mod COMP2INNSEL {
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

            /// 0b000: VREFINT
            pub const VREFINT: u32 = 0b000;

            /// 0b001: PA2
            pub const PA2: u32 = 0b001;

            /// 0b010: PA4
            pub const PA4: u32 = 0b010;

            /// 0b011: PA5
            pub const PA5: u32 = 0b011;

            /// 0b100: 1/4 VREFINT
            pub const VREFINT_Div4: u32 = 0b100;

            /// 0b101: 1/2 VREFINT
            pub const VREFINT_Div2: u32 = 0b101;

            /// 0b110: 3/4 VREFINT
            pub const VREFINT_Div3_4: u32 = 0b110;

            /// 0b111: PB3
            pub const PB3: u32 = 0b111;
        }
    }

    /// Comparator 2 power mode selection bit
    pub mod COMP2SPEED {
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

            /// 0b0: Slow speed
            pub const Slow: u32 = 0b0;

            /// 0b1: Fast speed
            pub const Fast: u32 = 0b1;
        }
    }

    /// Comparator 2 enable bit
    pub mod COMP2EN {
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

            /// 0b0: Comparator 2 switched OFF
            pub const Disabled: u32 = 0b0;

            /// 0b1: Comparator 2 switched ON
            pub const Enabled: u32 = 0b1;
        }
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

    /// Comparator 1 control and status register
    pub COMP1_CSR: RWRegister<u32>,

    /// Comparator 2 control and status register
    pub COMP2_CSR: RWRegister<u32>,

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
    pub COMP1_CSR: u32,
    pub COMP2_CSR: u32,
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
