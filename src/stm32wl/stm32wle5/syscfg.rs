#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System configuration controller

use crate::{RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// memory remap register
pub mod MEMRMP {

    /// Memory mapping selection
    pub mod MEM_MODE {
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

            /// 0b000: Main Flash memory mapped at 0x0000_0000
            pub const MainFlash: u32 = 0b000;

            /// 0b001: System Flash memory mapped at 0x0000_0000
            pub const SystemFlash: u32 = 0b001;

            /// 0b011: Embedded SRAM mapped at 0x0000_0000
            pub const SRAM: u32 = 0b011;
        }
    }
}

/// configuration register 1
pub mod CFGR1 {

    /// I2C3 Fast-mode Plus driving capability activation
    pub mod I2C3_FMP {
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

            /// 0b0: FM+ mode is controlled by I2C_Pxx_FMP bits only
            pub const Standard: u32 = 0b0;

            /// 0b1: FM+ mode is enabled on all I2C3 pins selected through selection bits in GPIOx_AFR registers
            pub const FMP: u32 = 0b1;
        }
    }

    /// I2C2 Fast-mode Plus driving capability activation
    pub mod I2C2_FMP {
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

            /// 0b0: FM+ mode is controlled by I2C_Pxx_FMP bits only
            pub const Standard: u32 = 0b0;

            /// 0b1: FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers
            pub const FMP: u32 = 0b1;
        }
    }

    /// I2C1 Fast-mode Plus driving capability activation
    pub mod I2C1_FMP {
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

            /// 0b0: FM+ mode is controlled by I2C_Pxx_FMP bits only
            pub const Standard: u32 = 0b0;

            /// 0b1: FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers
            pub const FMP: u32 = 0b1;
        }
    }

    /// Fast-mode Plus (Fm+) driving capability activation on PB9
    pub mod I2C_PB9_FMP {
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

            /// 0b0: PB9 pin operate in standard mode
            pub const Standard: u32 = 0b0;

            /// 0b1: I2C FM+ mode enabled on PB9 and the Speed control is bypassed
            pub const FMP: u32 = 0b1;
        }
    }

    /// Fast-mode Plus (Fm+) driving capability activation on PB8
    pub mod I2C_PB8_FMP {
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

            /// 0b0: PB8 pin operate in standard mode
            pub const Standard: u32 = 0b0;

            /// 0b1: I2C FM+ mode enabled on PB8 and the Speed control is bypassed
            pub const FMP: u32 = 0b1;
        }
    }

    /// Fast-mode Plus (Fm+) driving capability activation on PB7
    pub mod I2C_PB7_FMP {
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

            /// 0b0: PB7 pin operate in standard mode
            pub const Standard: u32 = 0b0;

            /// 0b1: I2C FM+ mode enabled on PB7 and the Speed control is bypassed
            pub const FMP: u32 = 0b1;
        }
    }

    /// Fast-mode Plus (Fm+) driving capability activation on PB6
    pub mod I2C_PB6_FMP {
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

            /// 0b0: PB6 pin operate in standard mode
            pub const Standard: u32 = 0b0;

            /// 0b1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed
            pub const FMP: u32 = 0b1;
        }
    }

    /// I/O analog switch voltage booster enable
    pub mod BOOSTEN {
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

            /// 0b0: I/O analog switches are supplied by VDDA voltage. This is the recommended configuration when using the ADC in high VDDA voltage operation
            pub const Disabled: u32 = 0b0;

            /// 0b1: I/O analog switches are supplied by a dedicated voltage booster (supplied by VDD). This is the recommended configuration when using the ADC in low VDDA voltage operation
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// external interrupt configuration register 1
pub mod EXTICR1 {

    /// EXTI 3 configuration bits
    pub mod EXTI3 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select PA3 as the source input for the EXTI3 external interrupt
            pub const PA3: u32 = 0b000;

            /// 0b001: Select PB3 as the source input for the EXTI3 external interrupt
            pub const PB3: u32 = 0b001;

            /// 0b010: Select PC3 as the source input for the EXTI3 external interrupt
            pub const PC3: u32 = 0b010;

            /// 0b111: Select PH3 as the source input for the EXTI3 external interrupt
            pub const PH3: u32 = 0b111;
        }
    }

    /// EXTI 2 configuration bits
    pub mod EXTI2 {
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

            /// 0b000: Select PA2 as the source input for the EXTI2 external interrupt
            pub const PA2: u32 = 0b000;

            /// 0b001: Select PB2 as the source input for the EXTI2 external interrupt
            pub const PB2: u32 = 0b001;

            /// 0b010: Select PC2 as the source input for the EXTI2 external interrupt
            pub const PC2: u32 = 0b010;
        }
    }

    /// EXTI 1 configuration bits
    pub mod EXTI1 {
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

            /// 0b000: Select PA1 as the source input for the EXTI1 external interrupt
            pub const PA1: u32 = 0b000;

            /// 0b001: Select PB1 as the source input for the EXTI1 external interrupt
            pub const PB1: u32 = 0b001;

            /// 0b010: Select PC1 as the source input for the EXTI1 external interrupt
            pub const PC1: u32 = 0b010;
        }
    }

    /// EXTI 0 configuration bits
    pub mod EXTI0 {
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

            /// 0b000: Select PA0 as the source input for the EXTI0 external interrupt
            pub const PA0: u32 = 0b000;

            /// 0b001: Select PB0 as the source input for the EXTI0 external interrupt
            pub const PB0: u32 = 0b001;

            /// 0b010: Select PC0 as the source input for the EXTI0 external interrupt
            pub const PC0: u32 = 0b010;
        }
    }
}

/// external interrupt configuration register 2
pub mod EXTICR2 {

    /// EXTI 7 configuration bits
    pub mod EXTI7 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select PA7 as the source input for the EXTI7 external interrupt
            pub const PA7: u32 = 0b000;

            /// 0b001: Select PB7 as the source input for the EXTI7 external interrupt
            pub const PB7: u32 = 0b001;
        }
    }

    /// EXTI 6 configuration bits
    pub mod EXTI6 {
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

            /// 0b000: Select PA6 as the source input for the EXTI6 external interrupt
            pub const PA6: u32 = 0b000;

            /// 0b001: Select PB6 as the source input for the EXTI6 external interrupt
            pub const PB6: u32 = 0b001;

            /// 0b010: Select PC6 as the source input for the EXTI6 external interrupt
            pub const PC6: u32 = 0b010;
        }
    }

    /// EXTI 5 configuration bits
    pub mod EXTI5 {
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

            /// 0b000: Select PA5 as the source input for the EXTI5 external interrupt
            pub const PA5: u32 = 0b000;

            /// 0b001: Select PB5 as the source input for the EXTI5 external interrupt
            pub const PB5: u32 = 0b001;

            /// 0b010: Select PC5 as the source input for the EXTI5 external interrupt
            pub const PC5: u32 = 0b010;
        }
    }

    /// EXTI 4 configuration bits
    pub mod EXTI4 {
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

            /// 0b000: Select PA4 as the source input for the EXTI4 external interrupt
            pub const PA4: u32 = 0b000;

            /// 0b001: Select PB4 as the source input for the EXTI4 external interrupt
            pub const PB4: u32 = 0b001;

            /// 0b010: Select PC4 as the source input for the EXTI4 external interrupt
            pub const PC4: u32 = 0b010;
        }
    }
}

/// external interrupt configuration register 3
pub mod EXTICR3 {

    /// EXTI 11 configuration bits
    pub mod EXTI11 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select PA11 as the source input for the EXTI11 external interrupt
            pub const PA11: u32 = 0b000;

            /// 0b001: Select PB11 as the source input for the EXTI11 external interrupt
            pub const PB11: u32 = 0b001;
        }
    }

    /// EXTI 10 configuration bits
    pub mod EXTI10 {
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

            /// 0b000: Select PA10 as the source input for the EXTI10 external interrupt
            pub const PA10: u32 = 0b000;

            /// 0b001: Select PB10 as the source input for the EXTI10 external interrupt
            pub const PB10: u32 = 0b001;
        }
    }

    /// EXTI 9 configuration bits
    pub mod EXTI9 {
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

            /// 0b000: Select PA9 as the source input for the EXTI9 external interrupt
            pub const PA9: u32 = 0b000;

            /// 0b001: Select PB9 as the source input for the EXTI9 external interrupt
            pub const PB9: u32 = 0b001;
        }
    }

    /// EXTI 8 configuration bits
    pub mod EXTI8 {
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

            /// 0b000: Select PA8 as the source input for the EXTI8 external interrupt
            pub const PA8: u32 = 0b000;

            /// 0b001: Select PB8 as the source input for the EXTI8 external interrupt
            pub const PB8: u32 = 0b001;
        }
    }
}

/// external interrupt configuration register 4
pub mod EXTICR4 {

    /// EXTI15 configuration bits
    pub mod EXTI15 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select PA15 as the source input for the EXTI15 external interrupt
            pub const PA15: u32 = 0b000;

            /// 0b001: Select PB15 as the source input for the EXTI15 external interrupt
            pub const PB15: u32 = 0b001;

            /// 0b010: Select PC15 as the source input for the EXTI15 external interrupt
            pub const PC15: u32 = 0b010;
        }
    }

    /// EXTI14 configuration bits
    pub mod EXTI14 {
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

            /// 0b000: Select PA14 as the source input for the EXTI14 external interrupt
            pub const PA14: u32 = 0b000;

            /// 0b001: Select PB14 as the source input for the EXTI14 external interrupt
            pub const PB14: u32 = 0b001;

            /// 0b010: Select PC14 as the source input for the EXTI14 external interrupt
            pub const PC14: u32 = 0b010;
        }
    }

    /// EXTI13 configuration bits
    pub mod EXTI13 {
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

            /// 0b000: Select PA13 as the source input for the EXTI13 external interrupt
            pub const PA13: u32 = 0b000;

            /// 0b001: Select PB13 as the source input for the EXTI13 external interrupt
            pub const PB13: u32 = 0b001;

            /// 0b010: Select PC13 as the source input for the EXTI13 external interrupt
            pub const PC13: u32 = 0b010;
        }
    }

    /// EXTI12 configuration bits
    pub mod EXTI12 {
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

            /// 0b000: Select PA12 as the source input for the EXTI12 external interrupt
            pub const PA12: u32 = 0b000;

            /// 0b001: Select PB12 as the source input for the EXTI12 external interrupt
            pub const PB12: u32 = 0b001;
        }
    }
}

/// SCSR
pub mod SCSR {

    /// PKA SRAM busy by erase operation
    pub mod PKASRAMBSY {
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

            /// 0b0: No PKA SRAM erase operation is ongoing
            pub const Idle: u32 = 0b0;

            /// 0b1: PKA SRAM erase operation is ongoing
            pub const Busy: u32 = 0b1;
        }
    }

    /// SRAM1, SRAM2 and PKA SRAM busy by erase operation
    pub mod SRAMBSY {
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

            /// 0b0: No SRAM1 or SRAM2 erase operation is ongoing
            pub const Idle: u32 = 0b0;

            /// 0b1: SRAM1 or SRAM2 erase operation is ongoing
            pub const Busy: u32 = 0b1;
        }
    }

    /// SRAM2 erase
    pub mod SRAM2ER {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Start SRAM2 erase operation
            pub const Erase: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CFGR2
pub mod CFGR2 {

    /// SRAM2 parity error flag
    pub mod SPF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No SRAM2 parity error detected
            pub const Nominal: u32 = 0b0;

            /// 0b1: SRAM2 parity error detected
            pub const Error: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear SRAM2 parity error flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ECC Lock
    pub mod ECCL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: ECC error disconnected from TIM1/16/17 break input
            pub const Disconnected: u32 = 0b0;

            /// 0b1: ECC error connected to TIM1/16/17 break input
            pub const Connected: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Connect ECC error to TIM1/16/17 break input
            pub const Connect: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PVD lock enable bit
    pub mod PVDL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: PVD interrupt disconnected from TIM1/16/17 break input. PVDE and PLS\[2:0\] bits can be programmed by the application
            pub const Disconnected: u32 = 0b0;

            /// 0b1: PVD interrupt connected to TIM1/16/17 break input. PVDE and PLS\[2:0\] bits are read only
            pub const Connected: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Connect PVD interretup to TIM1/16/17 break input
            pub const Connect: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SRAM2 parity lock bit
    pub mod SPL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: SRAM2 parity error signal disconnected from TIM1/16/17 break input
            pub const Disconnected: u32 = 0b0;

            /// 0b1: SRAM2 parity error signal connected to TIM1/16/17 break input
            pub const Connected: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Connect SRAM2 parity error signal to TIM1/16/17 break input
            pub const Connect: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU1 LOCKUP (Hardfault) output enable bit
    pub mod CLL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: CPU LOCKUP output disconnected from TIM1/16/17 break input
            pub const Disconnected: u32 = 0b0;

            /// 0b1: CPU LOCKUP output connected to TIM1/16/17 break input
            pub const Connected: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Connect CPU LOCKUP output to TIM1/16/17 break input
            pub const Connect: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SWPR
pub mod SWPR {

    /// SRAM2 1Kbyte page 31 write protection
    pub mod P31WP {
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

            /// 0b0: SRAM2 1 KB page protection disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: SRAM2 1 KB page protection enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SRAM2 1Kbyte page 30 write protection
    pub mod P30WP {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 29 write protection
    pub mod P29WP {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 28 write protection
    pub mod P28WP {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 27 write protection
    pub mod P27WP {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 26 write protection
    pub mod P26WP {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 25 write protection
    pub mod P25WP {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 24 write protection
    pub mod P24WP {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 23 write protection
    pub mod P23WP {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 22 write protection
    pub mod P22WP {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 21 write protection
    pub mod P21WP {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 20 write protection
    pub mod P20WP {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 19 write protection
    pub mod P19WP {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 18 write protection
    pub mod P18WP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 17 write protection
    pub mod P17WP {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 16 write protection
    pub mod P16WP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 15 write protection
    pub mod P15WP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 14 write protection
    pub mod P14WP {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 13 write protection
    pub mod P13WP {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 12 write protection
    pub mod P12WP {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 11 write protection
    pub mod P11WP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 10 write protection
    pub mod P10WP {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 9 write protection
    pub mod P9WP {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 8 write protection
    pub mod P8WP {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 7 write protection
    pub mod P7WP {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 6 write protection
    pub mod P6WP {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 5 write protection
    pub mod P5WP {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 4 write protection
    pub mod P4WP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 3 write protection
    pub mod P3WP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 2 write protection
    pub mod P2WP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 1 write protection
    pub mod P1WP {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }

    /// SRAM2 1Kbyte page 0 write protection
    pub mod P0WP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::P31WP::RW;
    }
}

/// SKR
pub mod SKR {

    /// SRAM2 write protection key for software erase
    pub mod KEY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b11001010: Step 1 to remove SRAM2ER bits write protection
            pub const Step1: u32 = 0b11001010;

            /// 0b01010011: Step 2 to remove SRAM2ER bits write protection
            pub const Step2: u32 = 0b01010011;

            /// 0b00010001: Activate SRAM2ER bits write protection
            pub const WriteProtect: u32 = 0b00010001;
        }
    }
}

/// radio debug control register
pub mod RFDCR {

    /// radio debug test bus selection
    pub mod RFTBSEL {
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

            /// 0b0: Digital test bus selected on RF_ADTB\[3:0\]
            pub const Digital: u32 = 0b0;

            /// 0b1: Analog test bus selected on RF_ADTB\[3:0\]
            pub const Analog: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// memory remap register
    pub MEMRMP: RWRegister<u32>,

    /// configuration register 1
    pub CFGR1: RWRegister<u32>,

    /// external interrupt configuration register 1
    pub EXTICR1: RWRegister<u32>,

    /// external interrupt configuration register 2
    pub EXTICR2: RWRegister<u32>,

    /// external interrupt configuration register 3
    pub EXTICR3: RWRegister<u32>,

    /// external interrupt configuration register 4
    pub EXTICR4: RWRegister<u32>,

    /// SCSR
    pub SCSR: RWRegister<u32>,

    /// CFGR2
    pub CFGR2: RWRegister<u32>,

    /// SWPR
    pub SWPR: RWRegister<u32>,

    /// SKR
    pub SKR: WORegister<u32>,

    _reserved1: [u32; 120],

    /// radio debug control register
    pub RFDCR: RWRegister<u32>,
}
pub struct ResetValues {
    pub MEMRMP: u32,
    pub CFGR1: u32,
    pub EXTICR1: u32,
    pub EXTICR2: u32,
    pub EXTICR3: u32,
    pub EXTICR4: u32,
    pub SCSR: u32,
    pub CFGR2: u32,
    pub SWPR: u32,
    pub SKR: u32,
    pub RFDCR: u32,
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
        MEMRMP: 0x00000000,
        CFGR1: 0x7C000001,
        EXTICR1: 0x00000000,
        EXTICR2: 0x00000000,
        EXTICR3: 0x00000000,
        EXTICR4: 0x00000000,
        SCSR: 0x00000000,
        CFGR2: 0x00000000,
        SWPR: 0x00000000,
        SKR: 0x00000000,
        RFDCR: 0x00000000,
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
