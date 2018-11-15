extern crate bare_metal;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn TAMPER_STAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_Channel1();
    fn DMA1_Channel2();
    fn DMA1_Channel3();
    fn DMA1_Channel4();
    fn DMA1_Channel5();
    fn DMA1_Channel6();
    fn DMA1_Channel7();
    fn ADC1();
    fn USB_HP();
    fn USB_LP();
    fn DAC();
    fn COMP_CA();
    fn EXTI9_5();
    fn LCD();
    fn TIM9();
    fn TIM10();
    fn TIM11();
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
    fn RTC_Alarm();
    fn USB_FS_WKUP();
    fn TIM6();
    fn TIM7();
    fn SDIO();
    fn TIM5();
    fn SPI3();
    fn USART4();
    fn USART5();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn AES();
    fn COMP_ACQ();
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
pub static __INTERRUPTS: [Vector; 57] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector {
        _handler: TAMPER_STAMP,
    },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector {
        _handler: DMA1_Channel1,
    },
    Vector {
        _handler: DMA1_Channel2,
    },
    Vector {
        _handler: DMA1_Channel3,
    },
    Vector {
        _handler: DMA1_Channel4,
    },
    Vector {
        _handler: DMA1_Channel5,
    },
    Vector {
        _handler: DMA1_Channel6,
    },
    Vector {
        _handler: DMA1_Channel7,
    },
    Vector { _handler: ADC1 },
    Vector { _handler: USB_HP },
    Vector { _handler: USB_LP },
    Vector { _handler: DAC },
    Vector { _handler: COMP_CA },
    Vector { _handler: EXTI9_5 },
    Vector { _handler: LCD },
    Vector { _handler: TIM9 },
    Vector { _handler: TIM10 },
    Vector { _handler: TIM11 },
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
        _handler: RTC_Alarm,
    },
    Vector {
        _handler: USB_FS_WKUP,
    },
    Vector { _handler: TIM6 },
    Vector { _handler: TIM7 },
    Vector { _handler: SDIO },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: USART4 },
    Vector { _handler: USART5 },
    Vector { _handler: DMA2_CH1 },
    Vector { _handler: DMA2_CH2 },
    Vector { _handler: DMA2_CH3 },
    Vector { _handler: DMA2_CH4 },
    Vector { _handler: DMA2_CH5 },
    Vector { _handler: AES },
    Vector { _handler: COMP_ACQ },
];

/// Available interrupts for this device
#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window Watchdog interrupt
    WWDG = 0,
    /// 1: PVD through EXTI Line detection interrupt
    PVD = 1,
    /// 2: Tamper and TimeStamp through EXTI line interrupts
    TAMPER_STAMP = 2,
    /// 3: RTC Wakeup through EXTI line interrupt
    RTC_WKUP = 3,
    /// 4: Flash global interrupt
    FLASH = 4,
    /// 5: RCC global interrupt
    RCC = 5,
    /// 6: EXTI Line0 interrupt
    EXTI0 = 6,
    /// 7: EXTI Line1 interrupt
    EXTI1 = 7,
    /// 8: EXTI Line2 interrupt
    EXTI2 = 8,
    /// 9: EXTI Line3 interrupt
    EXTI3 = 9,
    /// 10: EXTI Line4 interrupt
    EXTI4 = 10,
    /// 11: DMA1 Channel1 global interrupt
    DMA1_Channel1 = 11,
    /// 12: DMA1 Channel2 global interrupt
    DMA1_Channel2 = 12,
    /// 13: DMA1 Channel3 global interrupt
    DMA1_Channel3 = 13,
    /// 14: DMA1 Channel4 global interrupt
    DMA1_Channel4 = 14,
    /// 15: DMA1 Channel5 global interrupt
    DMA1_Channel5 = 15,
    /// 16: DMA1 Channel6 global interrupt
    DMA1_Channel6 = 16,
    /// 17: DMA1 Channel7 global interrupt
    DMA1_Channel7 = 17,
    /// 18: ADC1 global interrupt
    ADC1 = 18,
    /// 19: USB High priority interrupt
    USB_HP = 19,
    /// 20: USB Low priority interrupt
    USB_LP = 20,
    /// 21: DAC interrupt
    DAC = 21,
    /// 22: Comparator wakeup through EXTI line (21 and 22) interrupt/Channel acquisition interrupt
    COMP_CA = 22,
    /// 23: EXTI Line\[9:5\] interrupts
    EXTI9_5 = 23,
    /// 24: LCD global interrupt
    LCD = 24,
    /// 25: TIM9 global interrupt
    TIM9 = 25,
    /// 26: TIM10 global interrupt
    TIM10 = 26,
    /// 27: TIM11 global interrupt
    TIM11 = 27,
    /// 28: TIM2 global interrupt
    TIM2 = 28,
    /// 29: TIM3 global interrupt
    TIM3 = 29,
    /// 30: TIM4 global interrupt
    TIM4 = 30,
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
    /// 40: EXTI Line\[15:10\] interrupts
    EXTI15_10 = 40,
    /// 41: RTC Alarms (A and B) through EXTI line interrupt
    RTC_Alarm = 41,
    /// 42: USB Device FS Wakeup through EXTI line interrupt
    USB_FS_WKUP = 42,
    /// 43: TIM6 global interrupt
    TIM6 = 43,
    /// 44: TIM7 global interrupt
    TIM7 = 44,
    /// 45: SDIO Global interrupt
    SDIO = 45,
    /// 46: TIM5 Global interrupt
    TIM5 = 46,
    /// 47: SPI3 global interrupt
    SPI3 = 47,
    /// 48: USART4 global interrupt
    USART4 = 48,
    /// 49: USART5 global interrupt
    USART5 = 49,
    /// 50: DMA2 Channel 1 interrupt
    DMA2_CH1 = 50,
    /// 51: DMA2 Channel 2 interrupt
    DMA2_CH2 = 51,
    /// 52: DMA2 Channel 3 interrupt
    DMA2_CH3 = 52,
    /// 53: DMA2 Channel 4 interrupt
    DMA2_CH4 = 53,
    /// 54: DMA2 Channel 5 interrupt
    DMA2_CH5 = 54,
    /// 55: AES global interrupt
    AES = 55,
    /// 56: Comparator Channel Acquisition interrupt
    COMP_ACQ = 56,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
