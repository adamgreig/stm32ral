#[cfg(feature = "rt")]
extern "C" {
    fn WWDG_IRQ();
    fn PVD();
    fn TAMP_STAMP();
    fn RTC_WKUP_IRQ();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2_TSC();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_CH1();
    fn DMA1_CH2();
    fn DMA1_CH3();
    fn DMA1_CH4();
    fn DMA1_CH5();
    fn DMA1_CH6();
    fn DMA1_CH7();
    fn ADC1_IRQ();
    fn CAN_TX_IRQ();
    fn CAN_RXD_IRQ();
    fn CAN_RXI_IRQ();
    fn CAN_SCE_IRQ();
    fn EXTI9_5();
    fn TIM15_IRQ();
    fn TIM16_IRQ();
    fn TIM17_IRQ();
    fn TIM18_DAC3_IRQ();
    fn TIM2();
    fn I2C1_EV_IRQ();
    fn I2C1_ER_IRQ();
    fn I2C2_EV_IRQ();
    fn I2C2_ER_IRQ();
    fn SPI1_IRQ();
    fn SPI2_IRQ();
    fn USART1_IRQ();
    fn USART2_IRQ();
    fn USART3_IRQ();
    fn EXTI15_10();
    fn RTC_ALARM_IT_IRQ();
    fn CEC_IRQ();
    fn TIM12_IRQ();
    fn TIM13_IRQ();
    fn TIM14_IRQ();
    fn SPI3_IRQ();
    fn TIM6_DAC1();
    fn TIM7_IRQ();
    fn ADC_SD1_IRQ();
    fn ADC_SD2_IRQ();
    fn ADC_SD3_IRQ();
    fn COMP1_2_3();
    fn COMP4_5_6();
    fn I2C3_EV_IRQ();
    fn I2C3_ER_IRQ();
    fn USB_HP_IRQ();
    fn USB_LP_IRQ();
    fn USB_WAKEUP_IRQ();
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
    Vector { _handler: WWDG_IRQ },
    Vector { _handler: PVD },
    Vector {
        _handler: TAMP_STAMP,
    },
    Vector {
        _handler: RTC_WKUP_IRQ,
    },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector {
        _handler: EXTI2_TSC,
    },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector { _handler: DMA1_CH1 },
    Vector { _handler: DMA1_CH2 },
    Vector { _handler: DMA1_CH3 },
    Vector { _handler: DMA1_CH4 },
    Vector { _handler: DMA1_CH5 },
    Vector { _handler: DMA1_CH6 },
    Vector { _handler: DMA1_CH7 },
    Vector { _handler: ADC1_IRQ },
    Vector {
        _handler: CAN_TX_IRQ,
    },
    Vector {
        _handler: CAN_RXD_IRQ,
    },
    Vector {
        _handler: CAN_RXI_IRQ,
    },
    Vector {
        _handler: CAN_SCE_IRQ,
    },
    Vector { _handler: EXTI9_5 },
    Vector {
        _handler: TIM15_IRQ,
    },
    Vector {
        _handler: TIM16_IRQ,
    },
    Vector {
        _handler: TIM17_IRQ,
    },
    Vector {
        _handler: TIM18_DAC3_IRQ,
    },
    Vector { _handler: TIM2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: I2C1_EV_IRQ,
    },
    Vector {
        _handler: I2C1_ER_IRQ,
    },
    Vector {
        _handler: I2C2_EV_IRQ,
    },
    Vector {
        _handler: I2C2_ER_IRQ,
    },
    Vector { _handler: SPI1_IRQ },
    Vector { _handler: SPI2_IRQ },
    Vector {
        _handler: USART1_IRQ,
    },
    Vector {
        _handler: USART2_IRQ,
    },
    Vector {
        _handler: USART3_IRQ,
    },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM_IT_IRQ,
    },
    Vector { _handler: CEC_IRQ },
    Vector {
        _handler: TIM12_IRQ,
    },
    Vector {
        _handler: TIM13_IRQ,
    },
    Vector {
        _handler: TIM14_IRQ,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI3_IRQ },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: TIM6_DAC1,
    },
    Vector { _handler: TIM7_IRQ },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: ADC_SD1_IRQ,
    },
    Vector {
        _handler: ADC_SD2_IRQ,
    },
    Vector {
        _handler: ADC_SD3_IRQ,
    },
    Vector {
        _handler: COMP1_2_3,
    },
    Vector {
        _handler: COMP4_5_6,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: I2C3_EV_IRQ,
    },
    Vector {
        _handler: I2C3_ER_IRQ,
    },
    Vector {
        _handler: USB_HP_IRQ,
    },
    Vector {
        _handler: USB_LP_IRQ,
    },
    Vector {
        _handler: USB_WAKEUP_IRQ,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FPU },
];

/// Available interrupts for this device
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window Watchdog interrupt
    WWDG_IRQ = 0,
    /// 1: PVD through EXTI line detection interrupt
    PVD = 1,
    /// 2: Tamper and TimeStamp interrupts
    TAMP_STAMP = 2,
    /// 3: RTC
    RTC_WKUP_IRQ = 3,
    /// 4: Flash global interrupt
    FLASH = 4,
    /// 5: RCC global interrupt
    RCC = 5,
    /// 6: EXTI Line0 interrupt
    EXTI0 = 6,
    /// 7: EXTI Line3 interrupt
    EXTI1 = 7,
    /// 8: EXTI Line2 and Touch sensing interrupts
    EXTI2_TSC = 8,
    /// 9: EXTI Line3 interrupt
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
    ADC1_IRQ = 18,
    /// 19: USB high priority/CAN_TX interrupt
    CAN_TX_IRQ = 19,
    /// 20: USB low priority/CAN_RXD interrupt
    CAN_RXD_IRQ = 20,
    /// 21: CAN_RXI interrupt
    CAN_RXI_IRQ = 21,
    /// 22: CAN_SCE interrupt
    CAN_SCE_IRQ = 22,
    /// 23: EXTI Line5 to Line9 interrupts
    EXTI9_5 = 23,
    /// 24: Timer 15 global interrupt
    TIM15_IRQ = 24,
    /// 25: Timer 16 global interrupt
    TIM16_IRQ = 25,
    /// 26: Timer 17 global interrupt
    TIM17_IRQ = 26,
    /// 27: Timer 18 global interrupt/DAC3 underrun interrupt
    TIM18_DAC3_IRQ = 27,
    /// 28: Timer 2 global interrupt
    TIM2 = 28,
    /// 31: I2C1_EV global interrupt/EXTI Line\[3:2\] interrupts
    I2C1_EV_IRQ = 31,
    /// 32: I2C1_ER
    I2C1_ER_IRQ = 32,
    /// 33: I2C2_EV global interrupt/EXTI Line\[4:2\] interrupts
    I2C2_EV_IRQ = 33,
    /// 34: I2C2_ER
    I2C2_ER_IRQ = 34,
    /// 35: SPI1 global interrupt
    SPI1_IRQ = 35,
    /// 36: SPI2 global interrupt
    SPI2_IRQ = 36,
    /// 37: USART1 global interrupt/EXTI25 (USART1 wakeup event)
    USART1_IRQ = 37,
    /// 38: USART2 global interrupt/EXTI26 (USART1 wakeup event)
    USART2_IRQ = 38,
    /// 39: USART3 global interrupt/EXTI28 (USART1 wakeup event)
    USART3_IRQ = 39,
    /// 40: EXTI Line15 to Line10 interrupts
    EXTI15_10 = 40,
    /// 41: RTC alarm interrupt
    RTC_ALARM_IT_IRQ = 41,
    /// 42: CEC interrupt
    CEC_IRQ = 42,
    /// 43: Timer 12 global interrupt
    TIM12_IRQ = 43,
    /// 44: Timer 13 global interrupt
    TIM13_IRQ = 44,
    /// 45: Timer 14 global interrupt
    TIM14_IRQ = 45,
    /// 51: SPI3 global interrupt
    SPI3_IRQ = 51,
    /// 54: TIM6 global, DAC1 Cahnnel1 and Cahnnel2 underrun error Interrupts
    TIM6_DAC1 = 54,
    /// 55: Timer 7 global interrupt
    TIM7_IRQ = 55,
    /// 61: ADC sigma delta 1 (SDADC1) global interrupt
    ADC_SD1_IRQ = 61,
    /// 62: ADC sigma delta 2 (SDADC2) global interrupt
    ADC_SD2_IRQ = 62,
    /// 63: ADC sigma delta 3 (SDADC3) global interrupt
    ADC_SD3_IRQ = 63,
    /// 64: COMP1_2_3 interrupt combined with EXTI lines 21, 22, 29
    COMP1_2_3 = 64,
    /// 65: COMP4_5_6 interrupt combined with EXTI lines 30, 31, 32
    COMP4_5_6 = 65,
    /// 72: I2C3 event interrupt / EXTI Line27 interrupt
    I2C3_EV_IRQ = 72,
    /// 73: I2C3 error interrupt
    I2C3_ER_IRQ = 73,
    /// 74: USB high priority interrupt
    USB_HP_IRQ = 74,
    /// 75: USB low priority interrupt
    USB_LP_IRQ = 75,
    /// 76: USB wakeup interrupt
    USB_WAKEUP_IRQ = 76,
    /// 81: Floating point unit interrupt
    FPU = 81,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
