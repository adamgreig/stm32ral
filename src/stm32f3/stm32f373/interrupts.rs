extern crate bare_metal;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn TAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2_TS();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_CH1();
    fn DMA1_CH2();
    fn DMA1_CH3();
    fn DMA1_CH4();
    fn DMA1_CH5();
    fn DMA1_CH6();
    fn DMA1_CH7();
    fn ADC1();
    fn CAN_TX();
    fn CAN_RXD();
    fn CAN_RXI();
    fn CAN_SCE();
    fn EXTI5_9();
    fn TIM15();
    fn TIM16();
    fn TIM17();
    fn TIM18_DAC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn CEC();
    fn TIM12();
    fn TIM13();
    fn TIM14();
    fn TIM5();
    fn SPI3();
    fn TIM6_DAC1();
    fn TIM7();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn SDADC1();
    fn SDADC2();
    fn SDADC3();
    fn COMP1_2();
    fn USB_HP();
    fn USB_LP();
    fn USB_WAKEUP();
    fn TIM19();
    fn FPU();
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
pub static __INTERRUPTS: [Vector; 82] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector { _handler: TAMP },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2_TS },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector { _handler: DMA1_CH1 },
    Vector { _handler: DMA1_CH2 },
    Vector { _handler: DMA1_CH3 },
    Vector { _handler: DMA1_CH4 },
    Vector { _handler: DMA1_CH5 },
    Vector { _handler: DMA1_CH6 },
    Vector { _handler: DMA1_CH7 },
    Vector { _handler: ADC1 },
    Vector { _handler: CAN_TX },
    Vector { _handler: CAN_RXD },
    Vector { _handler: CAN_RXI },
    Vector { _handler: CAN_SCE },
    Vector { _handler: EXTI5_9 },
    Vector { _handler: TIM15 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector {
        _handler: TIM18_DAC,
    },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector { _handler: CEC },
    Vector { _handler: TIM12 },
    Vector { _handler: TIM13 },
    Vector { _handler: TIM14 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: TIM6_DAC1,
    },
    Vector { _handler: TIM7 },
    Vector { _handler: DMA2_CH1 },
    Vector { _handler: DMA2_CH2 },
    Vector { _handler: DMA2_CH3 },
    Vector { _handler: DMA2_CH4 },
    Vector { _handler: DMA2_CH5 },
    Vector { _handler: SDADC1 },
    Vector { _handler: SDADC2 },
    Vector { _handler: SDADC3 },
    Vector { _handler: COMP1_2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: USB_HP },
    Vector { _handler: USB_LP },
    Vector {
        _handler: USB_WAKEUP,
    },
    Vector { _reserved: 0 },
    Vector { _handler: TIM19 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FPU },
];

/// Available interrupts for this device
#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window Watchdog interrupt
    WWDG = 0,
    /// 1: Power voltage detector through EXTI line detection interrupt
    PVD = 1,
    /// 2: Tamper and timestamp through EXTI19 line
    TAMP = 2,
    /// 3: RTC
    RTC_WKUP = 3,
    /// 4: Flash global interrupt
    FLASH = 4,
    /// 5: RCC global interrupt
    RCC = 5,
    /// 6: EXTI Line 0 interrupt
    EXTI0 = 6,
    /// 7: EXTI Line1 interrupt
    EXTI1 = 7,
    /// 8: EXTI Line 2 and routing interface interrupt
    EXTI2_TS = 8,
    /// 9: EXTI Line1 interrupt
    EXTI3 = 9,
    /// 10: EXTI Line4 interrupt
    EXTI4 = 10,
    /// 11: DMA1 channel 1 interrupt
    DMA1_CH1 = 11,
    /// 12: DMA1 channel 2 interrupt
    DMA1_CH2 = 12,
    /// 13: DMA1 channel 3 interrupt
    DMA1_CH3 = 13,
    /// 14: DMA1 channel 4 interrupt
    DMA1_CH4 = 14,
    /// 15: DMA1 channel 5 interrupt
    DMA1_CH5 = 15,
    /// 16: DMA1 channel 6 interrupt
    DMA1_CH6 = 16,
    /// 17: DMA1 channel 7 interrupt
    DMA1_CH7 = 17,
    /// 18: ADC1 interrupt
    ADC1 = 18,
    /// 19: USB high priority/CAN_TX interrupt
    CAN_TX = 19,
    /// 20: USB low priority/CAN_RXD interrupt
    CAN_RXD = 20,
    /// 21: CAN_RXI interrupt
    CAN_RXI = 21,
    /// 22: CAN_SCE interrupt
    CAN_SCE = 22,
    /// 23: EXTI Line\[9:5\] interrupts
    EXTI5_9 = 23,
    /// 24: Timer 15 global interrupt
    TIM15 = 24,
    /// 25: Timer 16 global interrupt
    TIM16 = 25,
    /// 26: Timer 17 global interrupt
    TIM17 = 26,
    /// 27: Timer 18 global interrupt/DAC3 underrun interrupt
    TIM18_DAC = 27,
    /// 28: Timer 2 global interrupt
    TIM2 = 28,
    /// 29: Timer 3 global interrupt
    TIM3 = 29,
    /// 30: Timer 4 global interrupt
    TIM4 = 30,
    /// 31: I2C1_EV global interrupt/EXTI Line\[3:2\] interrupts
    I2C1_EV = 31,
    /// 32: I2C1_ER
    I2C1_ER = 32,
    /// 33: I2C2_EV global interrupt/EXTI Line\[4:2\] interrupts
    I2C2_EV = 33,
    /// 34: I2C2_ER
    I2C2_ER = 34,
    /// 35: SPI1 global interrupt
    SPI1 = 35,
    /// 36: SPI2 global interrupt
    SPI2 = 36,
    /// 37: USART1 global interrupt/EXTI25 (USART1 wakeup event)
    USART1 = 37,
    /// 38: USART2 global interrupt/EXTI26 (USART1 wakeup event)
    USART2 = 38,
    /// 39: USART3 global interrupt/EXTI28 (USART1 wakeup event)
    USART3 = 39,
    /// 40: EXTI Line\[15:10\] interrupts
    EXTI15_10 = 40,
    /// 41: RTC alarm interrupt
    RTC_ALARM = 41,
    /// 42: CEC interrupt
    CEC = 42,
    /// 43: Timer 12 global interrupt
    TIM12 = 43,
    /// 44: Timer 13 global interrupt
    TIM13 = 44,
    /// 45: Timer 14 global interrupt
    TIM14 = 45,
    /// 50: Timer 5 global interrupt
    TIM5 = 50,
    /// 51: SPI3 global interrupt
    SPI3 = 51,
    /// 54: TIM6 global, DAC1 Cahnnel1 and Cahnnel2 underrun error Interrupts
    TIM6_DAC1 = 54,
    /// 55: Timer 7 global interrupt
    TIM7 = 55,
    /// 56: DMA2 channel interrupt
    DMA2_CH1 = 56,
    /// 57: DMA2 channel interrupt
    DMA2_CH2 = 57,
    /// 58: DMA2 channel interrupt
    DMA2_CH3 = 58,
    /// 59: DMA2 channel interrupt
    DMA2_CH4 = 59,
    /// 60: DMA2 channel interrupt
    DMA2_CH5 = 60,
    /// 61: ADC sigma delta 1 (SDADC1) global interrupt
    SDADC1 = 61,
    /// 62: ADC sigma delta 2 (SDADC2) global interrupt
    SDADC2 = 62,
    /// 63: ADC sigma delta 3 (SDADC3) global interrupt
    SDADC3 = 63,
    /// 64: Comparator 1/comparator 2 global
    COMP1_2 = 64,
    /// 74: USB high priority interrupt
    USB_HP = 74,
    /// 75: USB low priority interrupt
    USB_LP = 75,
    /// 76: USB wakeup interrupt
    USB_WAKEUP = 76,
    /// 78: Timer 19 global interrupt
    TIM19 = 78,
    /// 81: Floating point unit interrupt
    FPU = 81,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
