#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn RTC_TAMP();
    fn FLASH();
    fn RCC_CRS();
    fn EXTI0_1();
    fn EXTI2_3();
    fn EXTI4_15();
    fn UCPD1_UCPD2_USB();
    fn ADC_COMP();
    fn TIM1_BRK_UP_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3_TIM4();
    fn TIM6_DAC();
    fn TIM7();
    fn TIM14();
    fn TIM15();
    fn TIM16();
    fn TIM17();
    fn I2C1();
    fn I2C2_I2C3();
    fn SPI1();
    fn SPI2_SPI3();
    fn USART1();
    fn USART2_LPUART2();
    fn USART3_USART4_USART5_USART6_LPUART1();
    fn CEC();
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
pub static __INTERRUPTS: [Vector; 31] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector { _handler: RTC_TAMP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC_CRS },
    Vector { _handler: EXTI0_1 },
    Vector { _handler: EXTI2_3 },
    Vector { _handler: EXTI4_15 },
    Vector {
        _handler: UCPD1_UCPD2_USB,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ADC_COMP },
    Vector {
        _handler: TIM1_BRK_UP_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector {
        _handler: TIM3_TIM4,
    },
    Vector { _handler: TIM6_DAC },
    Vector { _handler: TIM7 },
    Vector { _handler: TIM14 },
    Vector { _handler: TIM15 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector { _handler: I2C1 },
    Vector {
        _handler: I2C2_I2C3,
    },
    Vector { _handler: SPI1 },
    Vector {
        _handler: SPI2_SPI3,
    },
    Vector { _handler: USART1 },
    Vector {
        _handler: USART2_LPUART2,
    },
    Vector {
        _handler: USART3_USART4_USART5_USART6_LPUART1,
    },
    Vector { _handler: CEC },
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
    RCC_CRS = 4,
    /// 5: EXTI line 0 and 1 interrupt
    EXTI0_1 = 5,
    /// 6: EXTI line 2 and 3 interrupt
    EXTI2_3 = 6,
    /// 7: EXTI line 4 to 15 interrupt
    EXTI4_15 = 7,
    /// 8: UCPD and USB global interrupt
    UCPD1_UCPD2_USB = 8,
    /// 12: ADC and COMP interrupts (ADC combined with EXTI 17 and 18)
    ADC_COMP = 12,
    /// 13: TIM1 break, update, trigger and commutation interrupts
    TIM1_BRK_UP_TRG_COM = 13,
    /// 14: TIM1 Capture Compare interrupt
    TIM1_CC = 14,
    /// 15: TIM2 global interrupt
    TIM2 = 15,
    /// 16: TIM3 global interrupt
    TIM3_TIM4 = 16,
    /// 17: TIM6 + LPTIM1 and DAC global interrupt
    TIM6_DAC = 17,
    /// 18: TIM7 + LPTIM2 global interrupt
    TIM7 = 18,
    /// 19: TIM14 global interrupt
    TIM14 = 19,
    /// 20: Timer 15 global interrupt
    TIM15 = 20,
    /// 21: TIM16 global interrupt
    TIM16 = 21,
    /// 22: TIM17 global interrupt
    TIM17 = 22,
    /// 23: I2C1 global interrupt
    I2C1 = 23,
    /// 24: I2C2 and I2C3 global interrupt
    I2C2_I2C3 = 24,
    /// 25: SPI1 gloabl interrupt
    SPI1 = 25,
    /// 26: SPI2 gloabl interrupt
    SPI2_SPI3 = 26,
    /// 27: USART1 global interrupt
    USART1 = 27,
    /// 28: USART2 and LPUART2 global interrupt (combined with EXTI 26)
    USART2_LPUART2 = 28,
    /// 29: USART3,4,5,6 and LPUART1 global interrupt (combined with EXTI 28)
    USART3_USART4_USART5_USART6_LPUART1 = 29,
    /// 30: CEC global interrupt
    CEC = 30,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
