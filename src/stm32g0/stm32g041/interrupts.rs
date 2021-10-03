#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn RTC_TAMP();
    fn FLASH();
    fn RCC();
    fn EXTI0_1();
    fn EXTI2_3();
    fn EXTI4_15();
    fn DMA_Channel1();
    fn DMA_Channel2_3();
    fn DMA_Channel4_5_6_7();
    fn ADC_COMP();
    fn TIM1_BRK_UP_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM14();
    fn TIM16();
    fn TIM17();
    fn I2C1();
    fn I2C2();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3_USART4_LPUART1();
    fn CEC();
    fn AES_RNG();
}

#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}

#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector { _handler: RTC_TAMP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0_1 },
    Vector { _handler: EXTI2_3 },
    Vector { _handler: EXTI4_15 },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMA_Channel1,
    },
    Vector {
        _handler: DMA_Channel2_3,
    },
    Vector {
        _handler: DMA_Channel4_5_6_7,
    },
    Vector { _handler: ADC_COMP },
    Vector {
        _handler: TIM1_BRK_UP_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM14 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector { _handler: I2C1 },
    Vector { _handler: I2C2 },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector {
        _handler: USART3_USART4_LPUART1,
    },
    Vector { _handler: CEC },
    Vector { _handler: AES_RNG },
];

/// Available interrupts for this device
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window watchdog interrupt
    WWDG = 0,
    /// 1: Power voltage detector interrupt
    PVD = 1,
    /// 2: RTC and TAMP interrupts
    RTC_TAMP = 2,
    /// 3: Flash global interrupt
    FLASH = 3,
    /// 4: RCC global interrupt
    RCC = 4,
    /// 5: EXTI line 0 & 1 interrupt
    EXTI0_1 = 5,
    /// 6: EXTI line 2 & 3 interrupt
    EXTI2_3 = 6,
    /// 7: EXTI line 4 to 15 interrupt
    EXTI4_15 = 7,
    /// 9: DMA channel 1 interrupt
    DMA_Channel1 = 9,
    /// 10: DMA channel 2 & 3 interrupts
    DMA_Channel2_3 = 10,
    /// 11: DMA channel 4, 5, 6 & 7 and DMAMUX
    DMA_Channel4_5_6_7 = 11,
    /// 12: ADC and COMP interrupts
    ADC_COMP = 12,
    /// 13: TIM1 break, update, trigger
    TIM1_BRK_UP_TRG_COM = 13,
    /// 14: TIM1 Capture Compare interrupt
    TIM1_CC = 14,
    /// 15: TIM2 global interrupt
    TIM2 = 15,
    /// 16: TIM3 global interrupt
    TIM3 = 16,
    /// 19: TIM14 global interrupt
    TIM14 = 19,
    /// 21: TIM16 global interrupt
    TIM16 = 21,
    /// 22: TIM17 global interrupt
    TIM17 = 22,
    /// 23: I2C1 global interrupt
    I2C1 = 23,
    /// 24: I2C2 global interrupt
    I2C2 = 24,
    /// 25: SPI1 global interrupt
    SPI1 = 25,
    /// 26: SPI2 global interrupt
    SPI2 = 26,
    /// 27: USART1 global interrupt
    USART1 = 27,
    /// 28: USART2 global interrupt
    USART2 = 28,
    /// 29: USART3 + USART4 + LPUART1
    USART3_USART4_LPUART1 = 29,
    /// 30: CEC global interrupt
    CEC = 30,
    /// 31: AES and RNG global interrupts
    AES_RNG = 31,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
