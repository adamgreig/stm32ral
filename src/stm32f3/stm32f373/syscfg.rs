#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System configuration controller

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// configuration register 1
pub mod CFGR1 {

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

            /// 0b10: Main Flash memory mapped at 0x0000_0000
            pub const MainFlash2: u32 = 0b10;

            /// 0b11: Embedded SRAM mapped at 0x0000_0000
            pub const SRAM: u32 = 0b11;
        }
    }

    /// TIM16 DMA request remapping bit
    pub mod TIM16_DMA_RMP {
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

            /// 0b0: TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 3
            pub const NotRemapped: u32 = 0b0;

            /// 0b1: TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 4
            pub const Remapped: u32 = 0b1;
        }
    }

    /// TIM17 DMA request remapping bit
    pub mod TIM17_DMA_RMP {
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

            /// 0b0: TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 1
            pub const NotRemapped: u32 = 0b0;

            /// 0b1: TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 2
            pub const Remapped: u32 = 0b1;
        }
    }

    /// TIM6 and DAC1 DMA request remapping bit
    pub mod TIM6_DAC1_OUT1_DMA_RMP {
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

            /// 0b0: TIM7 and DAC1_OUT1 DMA requests mapped on DMA2 channel 3
            pub const NotRemapped: u32 = 0b0;

            /// 0b1: TIM7 and DAC1_OUT1 DMA requests mapped on DMA1 channel 3
            pub const Remapped: u32 = 0b1;
        }
    }

    /// TIM7 and DAC2 DMA request remapping bit
    pub mod TIM7_DAC1_OUT2_DMA_RMP {
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

            /// 0b0: TIM7 and DAC1_OUT2 DMA requests mapped on DMA2 channel 4
            pub const NotRemapped: u32 = 0b0;

            /// 0b1: TIM7 and DAC1_OUT2 DMA requests mapped on DMA1 channel 4
            pub const Remapped: u32 = 0b1;
        }
    }

    /// Fast Mode Plus (FM+) driving capability activation bits.
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

    /// Fast Mode Plus (FM+) driving capability activation bits.
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

    /// Fast Mode Plus (FM+) driving capability activation bits.
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

    /// Fast Mode Plus (FM+) driving capability activation bits.
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

    /// I2C1 Fast Mode Plus
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

            /// 0b1: FM+ mode is enabled on all I2C1 pins selected through selection through IOPORT control registers AF selection bits
            pub const FMP: u32 = 0b1;
        }
    }

    /// I2C2 Fast Mode Plus
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

            /// 0b1: FM+ mode is enabled on all I2C2 pins selected through selection through IOPORT control registers AF selection bits
            pub const FMP: u32 = 0b1;
        }
    }

    /// VBAT monitoring enable
    pub mod VBAT_MON {
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

            /// 0b0: Disable the power switch to not deliver VBAT voltage on ADC channel 18 input
            pub const Disable: u32 = 0b0;

            /// 0b1: Enable the power switch to deliver VBAT voltage on ADC channel 18 input
            pub const Enable: u32 = 0b1;
        }
    }

    /// TIM18 and DAC2_OUT1 DMA request remapping bit
    pub mod TIM18_DAC2_OUT1_DMA_RMP {
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

            /// 0b0: TIM18 and DAC2_OUT1 DMA requests mapped on DMA2 channel 5
            pub const NotRemapped: u32 = 0b0;

            /// 0b1: TIM18 and DAC2_OUT1 DMA requests mapped on DMA1 channel 5
            pub const Remapped: u32 = 0b1;
        }
    }

    /// Inexact interrupt enable
    pub mod FPU_IE5 {
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

            /// 0b0: Inexact interrupt disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: Inexact interrupt enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Input denormal interrupt enable
    pub mod FPU_IE4 {
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

            /// 0b0: Input denormal interrupt disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: Input denormal interrupt enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Overflow interrupt enable
    pub mod FPU_IE3 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Overflow interrupt disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: Overflow interrupt enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Underflow interrupt enable
    pub mod FPU_IE2 {
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

            /// 0b0: Underflow interrupt disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: Underflow interrupt enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Devide-by-zero interrupt enable
    pub mod FPU_IE1 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Devide-by-zero interrupt disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: Devide-by-zero interrupt enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Invalid operation interrupt enable
    pub mod FPU_IE0 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Invalid operation interrupt disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: Invalid operation interrupt enable
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

    /// EXTI 2 configuration bits
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

            /// 0b0101: Select PF2 as the source input for the EXTI2 external interrupt
            pub const PF2: u32 = 0b0101;
        }
    }

    /// EXTI 1 configuration bits
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

            /// 0b0101: Select PF1 as the source input for the EXTI1 external interrupt
            pub const PF1: u32 = 0b0101;
        }
    }

    /// EXTI 0 configuration bits
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

            /// 0b0101: Select PF0 as the source input for the EXTI0 external interrupt
            pub const PF0: u32 = 0b0101;
        }
    }
}

/// external interrupt configuration register 2
pub mod EXTICR2 {

    /// EXTI 7 configuration bits
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

            /// 0b0101: Select PF7 as the source input for the EXTI7 external interrupt
            pub const PF7: u32 = 0b0101;
        }
    }

    /// EXTI 6 configuration bits
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

            /// 0b0101: Select PF6 as the source input for the EXTI6 external interrupt
            pub const PF6: u32 = 0b0101;
        }
    }

    /// EXTI 5 configuration bits
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

    /// EXTI 4 configuration bits
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

            /// 0b0101: Select PF4 as the source input for the EXTI4 external interrupt
            pub const PF4: u32 = 0b0101;
        }
    }
}

/// external interrupt configuration register 3
pub mod EXTICR3 {

    /// EXTI 11 configuration bits
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

            /// 0b0010: Select PC11 as the source input for the EXTI11 external interrupt
            pub const PC11: u32 = 0b0010;

            /// 0b0011: Select PD11 as the source input for the EXTI11 external interrupt
            pub const PD11: u32 = 0b0011;

            /// 0b0100: Select PE11 as the source input for the EXTI11 external interrupt
            pub const PE11: u32 = 0b0100;
        }
    }

    /// EXTI 10 configuration bits
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

            /// 0b0101: Select PF10 as the source input for the EXTI10 external interrupt
            pub const PF10: u32 = 0b0101;
        }
    }

    /// EXTI 9 configuration bits
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

            /// 0b0101: Select PF9 as the source input for the EXTI9 external interrupt
            pub const PF9: u32 = 0b0101;
        }
    }

    /// EXTI 8 configuration bits
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

    /// EXTI 15 configuration bits
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

    /// EXTI 14 configuration bits
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

    /// EXTI 13 configuration bits
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

            /// 0b0010: Select PC13 as the source input for the EXTI13 external interrupt
            pub const PC13: u32 = 0b0010;

            /// 0b0011: Select PD13 as the source input for the EXTI13 external interrupt
            pub const PD13: u32 = 0b0011;

            /// 0b0100: Select PE13 as the source input for the EXTI13 external interrupt
            pub const PE13: u32 = 0b0100;
        }
    }

    /// EXTI 12 configuration bits
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

            /// 0b0010: Select PC12 as the source input for the EXTI12 external interrupt
            pub const PC12: u32 = 0b0010;

            /// 0b0011: Select PD12 as the source input for the EXTI12 external interrupt
            pub const PD12: u32 = 0b0011;

            /// 0b0100: Select PE12 as the source input for the EXTI12 external interrupt
            pub const PE12: u32 = 0b0100;
        }
    }
}

/// configuration register 2
pub mod CFGR2 {

    /// Cortex-M0 LOCKUP bit enable bit
    pub mod LOCKUP_LOCK {
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

            /// 0b0: Cortex-M4F LOCKUP output disconnected from TIM1/15/16/17 Break input
            pub const Disconnected: u32 = 0b0;

            /// 0b1: Cortex-M4F LOCKUP output connected to TIM1/15/16/17 Break input
            pub const Connected: u32 = 0b1;
        }
    }

    /// SRAM parity lock bit
    pub mod SRAM_PARITY_LOCK {
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

            /// 0b0: SRAM parity error disconnected from TIM1/15/16/17 Break input
            pub const Disconnected: u32 = 0b0;

            /// 0b1: SRAM parity error connected to TIM1/15/16/17 Break input
            pub const Connected: u32 = 0b1;
        }
    }

    /// PVD lock enable bit
    pub mod PVD_LOCK {
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

            /// 0b0: PVD interrupt disconnected from TIM15/16/17 Break input
            pub const Disconnected: u32 = 0b0;

            /// 0b1: PVD interrupt connected to TIM15/16/17 Break input
            pub const Connected: u32 = 0b1;
        }
    }

    /// SRAM parity flag
    pub mod SRAM_PEF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No SRAM parity error detected
            pub const NoParityError: u32 = 0b0;

            /// 0b1: SRAM parity error detected
            pub const ParityErrorDetected: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear SRAM parity error flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// configuration register 1
    pub CFGR1: RWRegister<u32>,

    _reserved1: [u8; 4],

    /// external interrupt configuration register 1
    pub EXTICR1: RWRegister<u32>,

    /// external interrupt configuration register 2
    pub EXTICR2: RWRegister<u32>,

    /// external interrupt configuration register 3
    pub EXTICR3: RWRegister<u32>,

    /// external interrupt configuration register 4
    pub EXTICR4: RWRegister<u32>,

    /// configuration register 2
    pub CFGR2: RWRegister<u32>,
}
pub struct ResetValues {
    pub CFGR1: u32,
    pub EXTICR1: u32,
    pub EXTICR2: u32,
    pub EXTICR3: u32,
    pub EXTICR4: u32,
    pub CFGR2: u32,
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
        EXTICR1: 0x00000000,
        EXTICR2: 0x00000000,
        EXTICR3: 0x00000000,
        EXTICR4: 0x00000000,
        CFGR2: 0x00000000,
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
