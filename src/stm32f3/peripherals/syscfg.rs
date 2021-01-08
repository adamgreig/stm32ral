#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System configuration controller
//!
//! Used by: stm32f303, stm32f3x8

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// configuration register 1
pub mod CFGR1 {

    /// Memory mapping selection bits
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

            /// 0b100: FMC Bank (Only the first two banks) (Available on STM32F303xD/E only)
            pub const FMC: u32 = 0b100;
        }
    }

    /// USB interrupt remap
    pub mod USB_IT_RMP {
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

            /// 0b0: USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 19, 20 and 42 respectively
            pub const NotRemapped: u32 = 0b0;

            /// 0b1: USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 74, 75 and 76 respectively
            pub const Remapped: u32 = 0b1;
        }
    }

    /// Timer 1 ITR3 selection
    pub mod TIM1_ITR3_RMP {
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

            /// 0b0: TIM1_ITR3 = TIM4_TRGO in STM32F303xB/C and STM32F358xC devices
            pub const NotRemapped: u32 = 0b0;

            /// 0b1: TIM1_ITR3 = TIM17_OC
            pub const Remapped: u32 = 0b1;
        }
    }

    /// DAC trigger remap (when TSEL = 001)
    pub mod DAC1_TRIG_RMP {
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

            /// 0b0: DAC trigger is TIM8_TRGO in STM32F303xB/C and STM32F358xC devices
            pub const NotRemapped: u32 = 0b0;

            /// 0b1: DAC trigger is TIM3_TRGO
            pub const Remapped: u32 = 0b1;
        }
    }

    /// ADC24 DMA remapping bit
    pub mod ADC2_DMA_RMP {
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

            /// 0b0: ADC24 DMA requests mapped on DMA2 channels 1 and 2
            pub const NotRemapped: u32 = 0b0;

            /// 0b1: ADC24 DMA requests mapped on DMA2 channels 3 and 4
            pub const Remapped: u32 = 0b1;
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
    pub mod TIM6_DAC1_CH1_DMA_RMP {
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

            /// 0b0: TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3 in STM32F303xB/C and STM32F358xC
            pub const NotRemapped: u32 = 0b0;

            /// 0b1: TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3
            pub const Remapped: u32 = 0b1;
        }
    }

    /// TIM7 and DAC2 DMA request remapping bit
    pub mod TIM7_DAC1_CH2_DMA_RMP {
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

            /// 0b0: TIM7_UP and DAC_CH2 DMA requests mapped on DMA2 channel 4 in STM32F303xB/C and STM32F358xC devices
            pub const NotRemapped: u32 = 0b0;

            /// 0b1: TIM7_UP and DAC_CH2 DMA requests mapped on DMA1 channel 4
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

    /// Encoder mode
    pub mod ENCODER_MODE {
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

            /// 0b00: No redirection
            pub const NoRedirection: u32 = 0b00;

            /// 0b01: TIM2 IC1 and TIM2 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively
            pub const MapTim2Tim15: u32 = 0b01;

            /// 0b10: TIM3 IC1 and TIM3 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively
            pub const MapTim3Tim15: u32 = 0b10;

            /// 0b11: TIM4 IC1 and TIM4 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively (STM32F303xB/C and STM32F358xC devices only)
            pub const MapTim4Tim15: u32 = 0b11;
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

    /// DAC2 channel1 DMA remap
    pub mod DAC2_CH1_DMA_RMP {
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

            /// 0b0: Not remapped
            pub const NotRemapped: u32 = 0b0;

            /// 0b1: DAC2_CH1 DMA requests mapped on DMA1 channel 5
            pub const Remapped: u32 = 0b1;
        }
    }

    /// I2C3 Fast Mode Plus
    pub mod I2C3_FMP {
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

            /// 0b0: FM+ mode is controlled by I2C_Pxx_FMP bits only
            pub const Standard: u32 = 0b0;

            /// 0b1: FM+ mode is enabled on all I2C3 pins selected through selection through IOPORT control registers AF selection bits
            pub const FMP: u32 = 0b1;
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

            /// 0b0101: Select PF3 as the source input for the EXTI3 external interrupt
            pub const PF3: u32 = 0b0101;

            /// 0b0110: Select PG3 as the source input for the EXTI3 external interrupt
            pub const PG3: u32 = 0b0110;
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

            /// 0b0110: Select PG2 as the source input for the EXTI2 external interrupt
            pub const PG2: u32 = 0b0110;

            /// 0b0111: Select PH2 as the source input for the EXTI2 external interrupt
            pub const PH2: u32 = 0b0111;
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

            /// 0b0110: Select PG1 as the source input for the EXTI1 external interrupt
            pub const PG1: u32 = 0b0110;

            /// 0b0111: Select PH1 as the source input for the EXTI1 external interrupt
            pub const PH1: u32 = 0b0111;
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

            /// 0b0110: Select PG0 as the source input for the EXTI0 external interrupt
            pub const PG0: u32 = 0b0110;

            /// 0b0111: Select PH0 as the source input for the EXTI0 external interrupt
            pub const PH0: u32 = 0b0111;
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

            /// 0b0110: Select PG7 as the source input for the EXTI7 external interrupt
            pub const PG7: u32 = 0b0110;
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

            /// 0b0110: Select PG6 as the source input for the EXTI6 external interrupt
            pub const PG6: u32 = 0b0110;
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

            /// 0b0101: Select PF5 as the source input for the EXTI5 external interrupt
            pub const PF5: u32 = 0b0101;

            /// 0b0110: Select PG5 as the source input for the EXTI5 external interrupt
            pub const PG5: u32 = 0b0110;
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

            /// 0b0110: Select PG4 as the source input for the EXTI4 external interrupt
            pub const PG4: u32 = 0b0110;
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

            /// 0b0001: Select PB11 as the source input for the EXTI11 external interrupt
            pub const PB11: u32 = 0b0001;

            /// 0b0010: Select PC11 as the source input for the EXTI11 external interrupt
            pub const PC11: u32 = 0b0010;

            /// 0b0011: Select PD11 as the source input for the EXTI11 external interrupt
            pub const PD11: u32 = 0b0011;

            /// 0b0100: Select PE11 as the source input for the EXTI11 external interrupt
            pub const PE11: u32 = 0b0100;

            /// 0b0101: Select PF11 as the source input for the EXTI11 external interrupt
            pub const PF11: u32 = 0b0101;

            /// 0b0110: Select PG11 as the source input for the EXTI11 external interrupt
            pub const PG11: u32 = 0b0110;
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

            /// 0b0110: Select PG10 as the source input for the EXTI10 external interrupt
            pub const PG10: u32 = 0b0110;
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

            /// 0b0110: Select PG9 as the source input for the EXTI9 external interrupt
            pub const PG9: u32 = 0b0110;
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

            /// 0b0101: Select PF8 as the source input for the EXTI8 external interrupt
            pub const PF8: u32 = 0b0101;

            /// 0b0110: Select PG8 as the source input for the EXTI8 external interrupt
            pub const PG8: u32 = 0b0110;
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

            /// 0b0101: Select PF15 as the source input for the EXTI15 external interrupt
            pub const PF15: u32 = 0b0101;

            /// 0b0110: Select PG15 as the source input for the EXTI15 external interrupt
            pub const PG15: u32 = 0b0110;
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

            /// 0b0101: Select PF14 as the source input for the EXTI14 external interrupt
            pub const PF14: u32 = 0b0101;

            /// 0b0110: Select PG14 as the source input for the EXTI14 external interrupt
            pub const PG14: u32 = 0b0110;
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

            /// 0b0001: Select PB13 as the source input for the EXTI13 external interrupt
            pub const PB13: u32 = 0b0001;

            /// 0b0010: Select PC13 as the source input for the EXTI13 external interrupt
            pub const PC13: u32 = 0b0010;

            /// 0b0011: Select PD13 as the source input for the EXTI13 external interrupt
            pub const PD13: u32 = 0b0011;

            /// 0b0100: Select PE13 as the source input for the EXTI13 external interrupt
            pub const PE13: u32 = 0b0100;

            /// 0b0101: Select PF13 as the source input for the EXTI13 external interrupt
            pub const PF13: u32 = 0b0101;

            /// 0b0110: Select PG13 as the source input for the EXTI13 external interrupt
            pub const PG13: u32 = 0b0110;
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

            /// 0b0001: Select PB12 as the source input for the EXTI12 external interrupt
            pub const PB12: u32 = 0b0001;

            /// 0b0010: Select PC12 as the source input for the EXTI12 external interrupt
            pub const PC12: u32 = 0b0010;

            /// 0b0011: Select PD12 as the source input for the EXTI12 external interrupt
            pub const PD12: u32 = 0b0011;

            /// 0b0100: Select PE12 as the source input for the EXTI12 external interrupt
            pub const PE12: u32 = 0b0100;

            /// 0b0101: Select PF12 as the source input for the EXTI12 external interrupt
            pub const PF12: u32 = 0b0101;

            /// 0b0110: Select PG12 as the source input for the EXTI12 external interrupt
            pub const PG12: u32 = 0b0110;
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

    /// Bypass address bit 29 in parity calculation
    pub mod BYP_ADDR_PAR {
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

            /// 0b0: The ramload operation is performed taking into consideration bit 29 of the address when the parity is calculated
            pub const NoBypass: u32 = 0b0;

            /// 0b1: The ramload operation is performed without taking into consideration bit 29 of the address when the parity is calculated
            pub const Bypass: u32 = 0b1;
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

/// CCM SRAM protection register
pub mod RCR {

    /// CCM SRAM page write protection bit
    pub mod PAGE0_WP {
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

            /// 0b0: Write protection of pagex is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Write protection of pagex is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CCM SRAM page write protection bit
    pub mod PAGE1_WP {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PAGE0_WP::RW;
    }

    /// CCM SRAM page write protection bit
    pub mod PAGE2_WP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PAGE0_WP::RW;
    }

    /// CCM SRAM page write protection bit
    pub mod PAGE3_WP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PAGE0_WP::RW;
    }

    /// CCM SRAM page write protection bit
    pub mod PAGE4_WP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PAGE0_WP::RW;
    }

    /// CCM SRAM page write protection bit
    pub mod PAGE5_WP {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PAGE0_WP::RW;
    }

    /// CCM SRAM page write protection bit
    pub mod PAGE6_WP {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PAGE0_WP::RW;
    }

    /// CCM SRAM page write protection bit
    pub mod PAGE7_WP {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PAGE0_WP::RW;
    }

    /// CCM SRAM page write protection bit
    pub mod PAGE8_WP {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PAGE0_WP::RW;
    }

    /// CCM SRAM page write protection bit
    pub mod PAGE9_WP {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PAGE0_WP::RW;
    }

    /// CCM SRAM page write protection bit
    pub mod PAGE10_WP {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PAGE0_WP::RW;
    }

    /// CCM SRAM page write protection bit
    pub mod PAGE11_WP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PAGE0_WP::RW;
    }

    /// CCM SRAM page write protection bit
    pub mod PAGE12_WP {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PAGE0_WP::RW;
    }

    /// CCM SRAM page write protection bit
    pub mod PAGE13_WP {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PAGE0_WP::RW;
    }

    /// CCM SRAM page write protection bit
    pub mod PAGE14_WP {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PAGE0_WP::RW;
    }

    /// CCM SRAM page write protection bit
    pub mod PAGE15_WP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PAGE0_WP::RW;
    }
}

/// configuration register 3
pub mod CFGR3 {

    /// SPI1_RX DMA remapping bit
    pub mod SPI1_RX_DMA_RMP {
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

            /// 0b00: SPI1_RX mapped on DMA1 CH2
            pub const MapDma1Ch3: u32 = 0b00;

            /// 0b01: SPI1_RX mapped on DMA1 CH4
            pub const MapDma1Ch5: u32 = 0b01;

            /// 0b10: SPI1_RX mapped on DMA1 CH6
            pub const MapDma1Ch7: u32 = 0b10;
        }
    }

    /// SPI1_TX DMA remapping bit
    pub mod SPI1_TX_DMA_RMP {
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

            /// 0b00: SPI1_TX mapped on DMA1 CH3
            pub const MapDma1Ch3: u32 = 0b00;

            /// 0b01: SPI1_TX mapped on DMA1 CH5
            pub const MapDma1Ch5: u32 = 0b01;

            /// 0b10: SPI1_TX mapped on DMA1 CH7
            pub const MapDma1Ch7: u32 = 0b10;
        }
    }

    /// I2C1_RX DMA remapping bit
    pub mod I2C1_RX_DMA_RMP {
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

            /// 0b00: I2C1_RX mapped on DMA1 CH7
            pub const MapDma1Ch7: u32 = 0b00;

            /// 0b01: I2C1_RX mapped on DMA1 CH3
            pub const MapDma1Ch3: u32 = 0b01;

            /// 0b10: I2C1_RX mapped on DMA1 CH5
            pub const MapDma1Ch5: u32 = 0b10;
        }
    }

    /// I2C1_TX DMA remapping bit
    pub mod I2C1_TX_DMA_RMP {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: I2C1_TX mapped on DMA1 CH6
            pub const MapDma1Ch6: u32 = 0b00;

            /// 0b01: I2C1_TX mapped on DMA1 CH2
            pub const MapDma1Ch2: u32 = 0b01;

            /// 0b10: I2C1_TX mapped on DMA1 CH4
            pub const MapDma1Ch4: u32 = 0b10;
        }
    }

    /// ADC2 DMA remapping bit
    pub mod ADC2_DMA_RMP {
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

            /// 0b00: ADC2 mapped on DMA2
            pub const MapDma2: u32 = 0b00;

            /// 0b11: ADC2 mapped on DMA1 channel 2
            pub const MapDma1Ch2: u32 = 0b11;

            /// 0b100: DC2 mapped on DMA1 channel 4
            pub const MapDma1Ch4: u32 = 0b100;
        }
    }
}

/// configuration register 4
pub mod CFGR4 {

    /// Controls the Input trigger of ADC12 regular channel EXT2
    pub mod ADC12_EXT2_RMP {
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

            /// 0b0: Trigger source is TIM3_CC3
            pub const Tim1: u32 = 0b0;

            /// 0b1: rigger source is TIM20_TRGO
            pub const Tim20: u32 = 0b1;
        }
    }

    /// Controls the Input trigger of ADC12 regular channel EXT3
    pub mod ADC12_EXT3_RMP {
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

            /// 0b0: Trigger source is TIM2_CC2
            pub const Tim2: u32 = 0b0;

            /// 0b1: rigger source is TIM20_TRGO2
            pub const Tim20: u32 = 0b1;
        }
    }

    /// Controls the Input trigger of ADC12 regular channel EXT5
    pub mod ADC12_EXT5_RMP {
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

            /// 0b0: Trigger source is TIM4_CC4
            pub const Tim4: u32 = 0b0;

            /// 0b1: Trigger source is TIM20_CC1
            pub const Tim20: u32 = 0b1;
        }
    }

    /// Controls the Input trigger of ADC12 regular channel EXT13
    pub mod ADC12_EXT13_RMP {
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

            /// 0b0: Trigger source is TIM6_TRGO
            pub const Tim6: u32 = 0b0;

            /// 0b1: Trigger source is TIM20_CC2
            pub const Tim20: u32 = 0b1;
        }
    }

    /// Controls the Input trigger of ADC12 regular channel EXT15
    pub mod ADC12_EXT15_RMP {
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

            /// 0b0: Trigger source is TIM3_CC4
            pub const Tim3: u32 = 0b0;

            /// 0b1: Trigger source is TIM20_CC3
            pub const Tim20: u32 = 0b1;
        }
    }

    /// Controls the Input trigger of ADC12 injected channel JEXT3
    pub mod ADC12_JEXT3_RMP {
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

            /// 0b0: Trigger source is TIM2_CC1
            pub const Tim2: u32 = 0b0;

            /// 0b1: Trigger source is TIM20_TRGO
            pub const Tim20: u32 = 0b1;
        }
    }

    /// Controls the Input trigger of ADC12 injected channel JEXT6
    pub mod ADC12_JEXT6_RMP {
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

            /// 0b0: Trigger source is EXTI line 15
            pub const Exti15: u32 = 0b0;

            /// 0b1: Trigger source is TIM20_TRGO2
            pub const Tim20: u32 = 0b1;
        }
    }

    /// Controls the Input trigger of ADC12 injected channel JEXT13
    pub mod ADC12_JEXT13_RMP {
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

            /// 0b0: Trigger source is TIM3_CC1
            pub const Tim3: u32 = 0b0;

            /// 0b1: Trigger source is TIM20_CC4
            pub const Tim20: u32 = 0b1;
        }
    }

    /// Controls the Input trigger of ADC34 regular channel EXT5
    pub mod ADC34_EXT5_RMP {
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

            /// 0b0: Trigger source is EXTI line 2 when reset at 0
            pub const Exti2: u32 = 0b0;

            /// 0b1: Trigger source is TIM20_TRGO
            pub const Tim20: u32 = 0b1;
        }
    }

    /// Controls the Input trigger of ADC34 regular channel EXT6
    pub mod ADC34_EXT6_RMP {
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

            /// 0b0: Trigger source is TIM4_CC1
            pub const Tim4: u32 = 0b0;

            /// 0b1: Trigger source is TIM20_TRGO2
            pub const Tim20: u32 = 0b1;
        }
    }

    /// Controls the Input trigger of ADC34 regular channel EXT15
    pub mod ADC34_EXT15_RMP {
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

            /// 0b0: Trigger source is TIM2_CC1
            pub const Tim2: u32 = 0b0;

            /// 0b1: Trigger source is TIM20_CC1
            pub const Tim20: u32 = 0b1;
        }
    }

    /// Controls the Input trigger of ADC34 injected channel JEXT5
    pub mod ADC34_JEXT5_RMP {
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

            /// 0b0: Trigger source is TIM4_CC3
            pub const Tim4: u32 = 0b0;

            /// 0b1: Trigger source is TIM20_TRGO
            pub const Tim20: u32 = 0b1;
        }
    }

    /// Controls the Input trigger of ADC34 injected channel JEXT11
    pub mod ADC34_JEXT11_RMP {
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

            /// 0b0: Trigger source is TIM1_CC3
            pub const Tim1: u32 = 0b0;

            /// 0b1: Trigger source is TIM20_TRGO2
            pub const Tim20: u32 = 0b1;
        }
    }

    /// Controls the Input trigger of ADC34 injected channel JEXT14
    pub mod ADC34_JEXT14_RMP {
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

            /// 0b0: Trigger source is TIM7_TRGO
            pub const Tim7: u32 = 0b0;

            /// 0b1: Trigger source is TIM20_CC2
            pub const Tim20: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// configuration register 1
    pub CFGR1: RWRegister<u32>,

    /// CCM SRAM protection register
    pub RCR: RWRegister<u32>,

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

    _reserved1: [u32; 11],

    /// configuration register 4
    pub CFGR4: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// configuration register 3
    pub CFGR3: RWRegister<u32>,
}
pub struct ResetValues {
    pub CFGR1: u32,
    pub RCR: u32,
    pub EXTICR1: u32,
    pub EXTICR2: u32,
    pub EXTICR3: u32,
    pub EXTICR4: u32,
    pub CFGR2: u32,
    pub CFGR4: u32,
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
