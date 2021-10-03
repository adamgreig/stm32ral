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
    fn ADC1_2();
    fn USB_HP_CAN_TX();
    fn USB_LP_CAN_RX0();
    fn CAN_RX1();
    fn CAN_SCE();
    fn EXTI9_5();
    fn TIM1_BRK_TIM15();
    fn TIM1_UP_TIM16();
    fn TIM1_TRG_COM_TIM17();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV_EXTI23();
    fn I2C1_ER();
    fn I2C2_EV_EXTI24();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1_EXTI25();
    fn USART2_EXTI26();
    fn USART3_EXTI28();
    fn EXTI15_10();
    fn RTCAlarm();
    fn USB_WKUP();
    fn TIM8_BRK();
    fn TIM8_UP();
    fn TIM8_TRG_COM();
    fn TIM8_CC();
    fn ADC3();
    fn FMC();
    fn SPI3();
    fn UART4_EXTI34();
    fn UART5_EXTI35();
    fn TIM6_DACUNDER();
    fn TIM7();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn ADC4();
    fn COMP123();
    fn COMP456();
    fn COMP7();
    fn I2C3_EV();
    fn I2C3_ER();
    fn USB_HP();
    fn USB_LP();
    fn USB_WKUP_EXTI();
    fn TIM20_BRK();
    fn TIM20_UP();
    fn TIM20_TRG_COM();
    fn TIM20_CC();
    fn FPU();
    fn SPI4();
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
pub static __INTERRUPTS: [Vector; 85] = [
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
    Vector { _handler: ADC1_2 },
    Vector {
        _handler: USB_HP_CAN_TX,
    },
    Vector {
        _handler: USB_LP_CAN_RX0,
    },
    Vector { _handler: CAN_RX1 },
    Vector { _handler: CAN_SCE },
    Vector { _handler: EXTI9_5 },
    Vector {
        _handler: TIM1_BRK_TIM15,
    },
    Vector {
        _handler: TIM1_UP_TIM16,
    },
    Vector {
        _handler: TIM1_TRG_COM_TIM17,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector {
        _handler: I2C1_EV_EXTI23,
    },
    Vector { _handler: I2C1_ER },
    Vector {
        _handler: I2C2_EV_EXTI24,
    },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector {
        _handler: USART1_EXTI25,
    },
    Vector {
        _handler: USART2_EXTI26,
    },
    Vector {
        _handler: USART3_EXTI28,
    },
    Vector {
        _handler: EXTI15_10,
    },
    Vector { _handler: RTCAlarm },
    Vector { _handler: USB_WKUP },
    Vector { _handler: TIM8_BRK },
    Vector { _handler: TIM8_UP },
    Vector {
        _handler: TIM8_TRG_COM,
    },
    Vector { _handler: TIM8_CC },
    Vector { _handler: ADC3 },
    Vector { _handler: FMC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI3 },
    Vector {
        _handler: UART4_EXTI34,
    },
    Vector {
        _handler: UART5_EXTI35,
    },
    Vector {
        _handler: TIM6_DACUNDER,
    },
    Vector { _handler: TIM7 },
    Vector { _handler: DMA2_CH1 },
    Vector { _handler: DMA2_CH2 },
    Vector { _handler: DMA2_CH3 },
    Vector { _handler: DMA2_CH4 },
    Vector { _handler: DMA2_CH5 },
    Vector { _handler: ADC4 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: COMP123 },
    Vector { _handler: COMP456 },
    Vector { _handler: COMP7 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector { _handler: USB_HP },
    Vector { _handler: USB_LP },
    Vector {
        _handler: USB_WKUP_EXTI,
    },
    Vector {
        _handler: TIM20_BRK,
    },
    Vector { _handler: TIM20_UP },
    Vector {
        _handler: TIM20_TRG_COM,
    },
    Vector { _handler: TIM20_CC },
    Vector { _handler: FPU },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI4 },
];

/// Available interrupts for this device
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window Watchdog interrupt
    WWDG = 0,
    /// 1: PVD through EXTI line detection interrupt
    PVD = 1,
    /// 2: Tamper and TimeStamp interrupts
    TAMP_STAMP = 2,
    /// 3: RTC Wakeup interrupt through the EXTI line
    RTC_WKUP = 3,
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
    /// 17: DMA1 channel 7interrupt
    DMA1_CH7 = 17,
    /// 18: ADC1 and ADC2 global interrupt
    ADC1_2 = 18,
    /// 19: USB High Priority/CAN_TX interrupts
    USB_HP_CAN_TX = 19,
    /// 20: USB Low Priority/CAN_RX0 interrupts
    USB_LP_CAN_RX0 = 20,
    /// 21: CAN_RX1 interrupt
    CAN_RX1 = 21,
    /// 22: CAN_SCE interrupt
    CAN_SCE = 22,
    /// 23: EXTI Line5 to Line9 interrupts
    EXTI9_5 = 23,
    /// 24: TIM1 Break/TIM15 global interruts
    TIM1_BRK_TIM15 = 24,
    /// 25: TIM1 Update/TIM16 global interrupts
    TIM1_UP_TIM16 = 25,
    /// 26: TIM1 trigger and commutation/TIM17 interrupts
    TIM1_TRG_COM_TIM17 = 26,
    /// 27: TIM1 capture compare interrupt
    TIM1_CC = 27,
    /// 28: TIM2 global interrupt
    TIM2 = 28,
    /// 29: TIM3 global interrupt
    TIM3 = 29,
    /// 30: TIM4 global interrupt
    TIM4 = 30,
    /// 31: I2C1 event interrupt and EXTI Line23 interrupt
    I2C1_EV_EXTI23 = 31,
    /// 32: I2C1 error interrupt
    I2C1_ER = 32,
    /// 33: I2C2 event interrupt & EXTI Line24 interrupt
    I2C2_EV_EXTI24 = 33,
    /// 34: I2C2 error interrupt
    I2C2_ER = 34,
    /// 35: SPI1 global interrupt
    SPI1 = 35,
    /// 36: SPI2 global interrupt
    SPI2 = 36,
    /// 37: USART1 global interrupt and EXTI Line 25 interrupt
    USART1_EXTI25 = 37,
    /// 38: USART2 global interrupt and EXTI Line 26 interrupt
    USART2_EXTI26 = 38,
    /// 39: USART3 global interrupt and EXTI Line 28 interrupt
    USART3_EXTI28 = 39,
    /// 40: EXTI Line15 to Line10 interrupts
    EXTI15_10 = 40,
    /// 41: RTC alarm interrupt
    RTCAlarm = 41,
    /// 42: USB wakeup from Suspend
    USB_WKUP = 42,
    /// 43: TIM8 break interrupt
    TIM8_BRK = 43,
    /// 44: TIM8 update interrupt
    TIM8_UP = 44,
    /// 45: TIM8 Trigger and commutation interrupts
    TIM8_TRG_COM = 45,
    /// 46: TIM8 capture compare interrupt
    TIM8_CC = 46,
    /// 47: ADC3 global interrupt
    ADC3 = 47,
    /// 48: FSMC global interrupt
    FMC = 48,
    /// 51: SPI3 global interrupt
    SPI3 = 51,
    /// 52: UART4 global and EXTI Line 34 interrupts
    UART4_EXTI34 = 52,
    /// 53: UART5 global and EXTI Line 35 interrupts
    UART5_EXTI35 = 53,
    /// 54: TIM6 global and DAC12 underrun interrupts
    TIM6_DACUNDER = 54,
    /// 55: TIM7 global interrupt
    TIM7 = 55,
    /// 56: DMA2 channel1 global interrupt
    DMA2_CH1 = 56,
    /// 57: DMA2 channel2 global interrupt
    DMA2_CH2 = 57,
    /// 58: DMA2 channel3 global interrupt
    DMA2_CH3 = 58,
    /// 59: DMA2 channel4 global interrupt
    DMA2_CH4 = 59,
    /// 60: DMA2 channel5 global interrupt
    DMA2_CH5 = 60,
    /// 61: ADC4 global interrupt
    ADC4 = 61,
    /// 64: COMP1 & COMP2 & COMP3 interrupts combined with EXTI Lines 21, 22 and 29 interrupts
    COMP123 = 64,
    /// 65: COMP4 & COMP5 & COMP6 interrupts combined with EXTI Lines 30, 31 and 32 interrupts
    COMP456 = 65,
    /// 66: COMP7 interrupt combined with EXTI Line 33 interrupt
    COMP7 = 66,
    /// 72: I2C3 Event interrupt
    I2C3_EV = 72,
    /// 73: I2C3 Error interrupt
    I2C3_ER = 73,
    /// 74: USB High priority interrupt
    USB_HP = 74,
    /// 75: USB Low priority interrupt
    USB_LP = 75,
    /// 76: USB wakeup from Suspend and EXTI Line 18
    USB_WKUP_EXTI = 76,
    /// 77: TIM20 Break interrupt
    TIM20_BRK = 77,
    /// 78: TIM20 Upgrade interrupt
    TIM20_UP = 78,
    /// 79: TIM20 Trigger and Commutation interrupt
    TIM20_TRG_COM = 79,
    /// 80: TIM20 Capture Compare interrupt
    TIM20_CC = 80,
    /// 81: Floating point unit
    FPU = 81,
    /// 84: SPI4 Global interrupt
    SPI4 = 84,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
