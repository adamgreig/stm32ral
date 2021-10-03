#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn RTC();
    fn RCC();
    fn EXTI0_1();
    fn EXTI2_3();
    fn EXTI4_15();
    fn TSC();
    fn DMA1_Channel1();
    fn DMA1_Channel2_3();
    fn DMA1_Channel4_7();
    fn ADC_COMP();
    fn LPTIM1();
    fn USART4_USART5();
    fn TIM2();
    fn TIM3();
    fn TIM6_DAC();
    fn TIM7();
    fn TIM21();
    fn I2C3();
    fn TIM22();
    fn I2C1();
    fn I2C2();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn AES_RNG_LPUART1();
    fn LCD();
    fn USB();
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
    Vector { _handler: RTC },
    Vector { _reserved: 0 },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0_1 },
    Vector { _handler: EXTI2_3 },
    Vector { _handler: EXTI4_15 },
    Vector { _handler: TSC },
    Vector {
        _handler: DMA1_Channel1,
    },
    Vector {
        _handler: DMA1_Channel2_3,
    },
    Vector {
        _handler: DMA1_Channel4_7,
    },
    Vector { _handler: ADC_COMP },
    Vector { _handler: LPTIM1 },
    Vector {
        _handler: USART4_USART5,
    },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM6_DAC },
    Vector { _handler: TIM7 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM21 },
    Vector { _handler: I2C3 },
    Vector { _handler: TIM22 },
    Vector { _handler: I2C1 },
    Vector { _handler: I2C2 },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector {
        _handler: AES_RNG_LPUART1,
    },
    Vector { _handler: LCD },
    Vector { _handler: USB },
];

/// Available interrupts for this device
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window Watchdog interrupt
    WWDG = 0,
    /// 1: PVD through EXTI line detection
    PVD = 1,
    /// 2: RTC global interrupt
    RTC = 2,
    /// 4: RCC global interrupt
    RCC = 4,
    /// 5: EXTI Line\[1:0\] interrupts
    EXTI0_1 = 5,
    /// 6: EXTI Line\[3:2\] interrupts
    EXTI2_3 = 6,
    /// 7: EXTI Line15 and EXTI4 interrupts
    EXTI4_15 = 7,
    /// 8: Touch sensing interrupt
    TSC = 8,
    /// 9: DMA1 Channel1 global interrupt
    DMA1_Channel1 = 9,
    /// 10: DMA1 Channel2 and 3 interrupts
    DMA1_Channel2_3 = 10,
    /// 11: DMA1 Channel4 to 7 interrupts
    DMA1_Channel4_7 = 11,
    /// 12: ADC and comparator 1 and 2
    ADC_COMP = 12,
    /// 13: LPTIMER1 interrupt through EXTI29
    LPTIM1 = 13,
    /// 14: USART4/USART5 global interrupt
    USART4_USART5 = 14,
    /// 15: TIM2 global interrupt
    TIM2 = 15,
    /// 16: TIM3 global interrupt
    TIM3 = 16,
    /// 17: TIM6 global interrupt and DAC
    TIM6_DAC = 17,
    /// 18: TIM7 global interrupt and DAC
    TIM7 = 18,
    /// 20: TIMER21 global interrupt
    TIM21 = 20,
    /// 21: I2C3 global interrupt
    I2C3 = 21,
    /// 22: TIMER22 global interrupt
    TIM22 = 22,
    /// 23: I2C1 global interrupt
    I2C1 = 23,
    /// 24: I2C2 global interrupt
    I2C2 = 24,
    /// 25: SPI1_global_interrupt
    SPI1 = 25,
    /// 26: SPI2 global interrupt
    SPI2 = 26,
    /// 27: USART1 global interrupt
    USART1 = 27,
    /// 28: USART2 global interrupt
    USART2 = 28,
    /// 29: AES global interrupt RNG global interrupt and LPUART1 global interrupt through
    AES_RNG_LPUART1 = 29,
    /// 30: LCD global interrupt
    LCD = 30,
    /// 31: USB event interrupt through EXTI18
    USB = 31,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
