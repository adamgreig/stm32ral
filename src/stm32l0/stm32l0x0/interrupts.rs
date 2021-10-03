extern crate bare_metal;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn RTC();
    fn FLASH();
    fn RCC();
    fn EXTI0_1();
    fn EXTI2_3();
    fn EXTI4_15();
    fn DMA1_Channel1();
    fn DMA1_Channel2_3();
    fn DMA1_Channel4_7();
    fn ADC();
    fn LPTIM1();
    fn TIM2();
    fn TIM21();
    fn TIM22();
    fn I2C1();
    fn SPI1();
    fn USART2();
    fn LPUART1();
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
pub static __INTERRUPTS: [Vector; 30] = [
    Vector { _handler: WWDG },
    Vector { _reserved: 0 },
    Vector { _handler: RTC },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0_1 },
    Vector { _handler: EXTI2_3 },
    Vector { _handler: EXTI4_15 },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMA1_Channel1,
    },
    Vector {
        _handler: DMA1_Channel2_3,
    },
    Vector {
        _handler: DMA1_Channel4_7,
    },
    Vector { _handler: ADC },
    Vector { _handler: LPTIM1 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM21 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM22 },
    Vector { _handler: I2C1 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: USART2 },
    Vector { _handler: LPUART1 },
];

/// Available interrupts for this device
#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window Watchdog interrupt
    WWDG = 0,
    /// 2: RTC global interrupt
    RTC = 2,
    /// 3: Flash global interrupt
    FLASH = 3,
    /// 4: RCC global interrupt
    RCC = 4,
    /// 5: EXTI Line\[1:0\] interrupts
    EXTI0_1 = 5,
    /// 6: EXTI Line\[3:2\] interrupts
    EXTI2_3 = 6,
    /// 7: EXTI Line15 and EXTI4 interrupts
    EXTI4_15 = 7,
    /// 9: DMA1 Channel1 global interrupt
    DMA1_Channel1 = 9,
    /// 10: DMA1 Channel2 and 3 interrupts
    DMA1_Channel2_3 = 10,
    /// 11: DMA1 Channel4 to 7 interrupts
    DMA1_Channel4_7 = 11,
    /// 12: ADC
    ADC = 12,
    /// 13: LPTIMER1 interrupt through EXTI29
    LPTIM1 = 13,
    /// 15: TIM2 global interrupt
    TIM2 = 15,
    /// 20: TIMER21 global interrupt
    TIM21 = 20,
    /// 22: TIMER22 global interrupt
    TIM22 = 22,
    /// 23: I2C1 global interrupt
    I2C1 = 23,
    /// 25: SPI1_global_interrupt
    SPI1 = 25,
    /// 28: USART2 global interrupt
    USART2 = 28,
    /// 29: LPUART1 global interrupt through
    LPUART1 = 29,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
