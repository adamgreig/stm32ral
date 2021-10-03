extern crate bare_metal;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn TAMP_STAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_CH1();
    fn DMA1_CH2();
    fn DMA1_CH3();
    fn DMA1_CH4();
    fn DMA1_CH5();
    fn DMA1_CH6();
    fn DMA1_CH7();
    fn ADC1_2();
    fn CAN1_TX();
    fn CAN1_RX0();
    fn CAN1_RX1();
    fn CAN1_SCE();
    fn EXTI9_5();
    fn TIM15();
    fn TIM16();
    fn TIM1_CC();
    fn TIM2();
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
    fn SDMMC();
    fn SPI3();
    fn TIM6_DAC();
    fn TIM7();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn COMP();
    fn LPTIM1();
    fn LPTIM2();
    fn USB_FS();
    fn DMA2_CH6();
    fn DMA2_CH7();
    fn I2C3_EV();
    fn I2C3_ER();
    fn SAI1();
    fn SWPMI1();
    fn TSC();
    fn LCD();
    fn RNG();
    fn CRS();
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
    Vector {
        _handler: TAMP_STAMP,
    },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector { _handler: DMA1_CH1 },
    Vector { _handler: DMA1_CH2 },
    Vector { _handler: DMA1_CH3 },
    Vector { _handler: DMA1_CH4 },
    Vector { _handler: DMA1_CH5 },
    Vector { _handler: DMA1_CH6 },
    Vector { _handler: DMA1_CH7 },
    Vector { _handler: ADC1_2 },
    Vector { _handler: CAN1_TX },
    Vector { _handler: CAN1_RX0 },
    Vector { _handler: CAN1_RX1 },
    Vector { _handler: CAN1_SCE },
    Vector { _handler: EXTI9_5 },
    Vector { _handler: TIM15 },
    Vector { _handler: TIM16 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
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
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SDMMC },
    Vector { _reserved: 0 },
    Vector { _handler: SPI3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM6_DAC },
    Vector { _handler: TIM7 },
    Vector { _handler: DMA2_CH1 },
    Vector { _handler: DMA2_CH2 },
    Vector { _handler: DMA2_CH3 },
    Vector { _handler: DMA2_CH4 },
    Vector { _handler: DMA2_CH5 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: COMP },
    Vector { _handler: LPTIM1 },
    Vector { _handler: LPTIM2 },
    Vector { _handler: USB_FS },
    Vector { _handler: DMA2_CH6 },
    Vector { _handler: DMA2_CH7 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector { _handler: SAI1 },
    Vector { _reserved: 0 },
    Vector { _handler: SWPMI1 },
    Vector { _handler: TSC },
    Vector { _handler: LCD },
    Vector { _handler: RNG },
    Vector { _handler: CRS },
    Vector { _handler: FPU },
];

/// Available interrupts for this device
#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window Watchdog interrupt
    WWDG = 0,
    /// 1: PVD through EXTI line detection
    PVD = 1,
    /// 2: Tamper and TimeStamp interrupts
    TAMP_STAMP = 2,
    /// 3: RTC Tamper or TimeStamp /CSS on LSE through EXTI line 19 interrupts
    RTC_WKUP = 3,
    /// 4: Flash global interrupt
    FLASH = 4,
    /// 5: RCC global interrupt
    RCC = 5,
    /// 6: EXTI Line 0 interrupt
    EXTI0 = 6,
    /// 7: EXTI Line 1 interrupt
    EXTI1 = 7,
    /// 8: EXTI Line 2 interrupt
    EXTI2 = 8,
    /// 9: EXTI Line 3 interrupt
    EXTI3 = 9,
    /// 10: EXTI Line4 interrupt
    EXTI4 = 10,
    /// 11: DMA1 Channel1 global interrupt
    DMA1_CH1 = 11,
    /// 12: DMA1 Channel2 global interrupt
    DMA1_CH2 = 12,
    /// 13: DMA1 Channel3 interrupt
    DMA1_CH3 = 13,
    /// 14: DMA1 Channel4 interrupt
    DMA1_CH4 = 14,
    /// 15: DMA1 Channel5 interrupt
    DMA1_CH5 = 15,
    /// 16: DMA1 Channel6 interrupt
    DMA1_CH6 = 16,
    /// 17: DMA1 Channel 7 interrupt
    DMA1_CH7 = 17,
    /// 18: ADC1 and ADC2 global interrupt
    ADC1_2 = 18,
    /// 19: CAN1 TX interrupts
    CAN1_TX = 19,
    /// 20: CAN1 RX0 interrupts
    CAN1_RX0 = 20,
    /// 21: CAN1 RX1 interrupts
    CAN1_RX1 = 21,
    /// 22: CAN1 SCE interrupt
    CAN1_SCE = 22,
    /// 23: EXTI Line5 to Line9 interrupts
    EXTI9_5 = 23,
    /// 24: Timer 15 global interrupt
    TIM15 = 24,
    /// 25: Timer 16 global interrupt
    TIM16 = 25,
    /// 27: TIM1 Capture Compare interrupt
    TIM1_CC = 27,
    /// 28: TIM2 global interrupt
    TIM2 = 28,
    /// 31: I2C1 event interrupt
    I2C1_EV = 31,
    /// 32: I2C1 error interrupt
    I2C1_ER = 32,
    /// 33: I2C2 event interrupt
    I2C2_EV = 33,
    /// 34: I2C2 error interrupt
    I2C2_ER = 34,
    /// 35: SPI1 global interrupt
    SPI1 = 35,
    /// 36: SPI2 global interrupt
    SPI2 = 36,
    /// 37: USART1 global interrupt
    USART1 = 37,
    /// 38: USART2 global interrupt
    USART2 = 38,
    /// 39: USART3 global interrupt
    USART3 = 39,
    /// 40: EXTI Lines 10 to 15 interrupts
    EXTI15_10 = 40,
    /// 41: RTC alarms through EXTI line 18 interrupts
    RTC_ALARM = 41,
    /// 49: SDMMC global Interrupt
    SDMMC = 49,
    /// 51: SPI3 global Interrupt
    SPI3 = 51,
    /// 54: TIM6 global and DAC1 and 2 underrun error interrupts
    TIM6_DAC = 54,
    /// 55: TIM7 global interrupt
    TIM7 = 55,
    /// 56: DMA2 Channel 1 global Interrupt
    DMA2_CH1 = 56,
    /// 57: DMA2 Channel 2 global Interrupt
    DMA2_CH2 = 57,
    /// 58: DMA2 Channel 3 global Interrupt
    DMA2_CH3 = 58,
    /// 59: DMA2 Channel 4 global Interrupt
    DMA2_CH4 = 59,
    /// 60: DMA2 Channel 5 global Interrupt
    DMA2_CH5 = 60,
    /// 64: COMP1 and COMP2 interrupts
    COMP = 64,
    /// 65: LP TIM1 interrupt
    LPTIM1 = 65,
    /// 66: LP TIM2 interrupt
    LPTIM2 = 66,
    /// 67: USB event interrupt through EXTI
    USB_FS = 67,
    /// 68: DMA2 Channel 6 global Interrupt
    DMA2_CH6 = 68,
    /// 69: DMA2 Channel 7 global Interrupt
    DMA2_CH7 = 69,
    /// 72: I2C3 event interrupt
    I2C3_EV = 72,
    /// 73: I2C3 error interrupt
    I2C3_ER = 73,
    /// 74: SAI1 global interrupt
    SAI1 = 74,
    /// 76: SWPMI1 global interrupt
    SWPMI1 = 76,
    /// 77: TSC global interrupt
    TSC = 77,
    /// 78: LCD global interrupt
    LCD = 78,
    /// 79: RNG global interrupt
    RNG = 79,
    /// 80: CRS global interrupt
    CRS = 80,
    /// 81: USB global interrupt
    FPU = 81,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
