#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn TAMPER();
    fn RTC();
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
    fn ADC1_2();
    fn USB_HP_CAN_TX();
    fn USB_LP_CAN_RX0();
    fn CAN_RX1();
    fn CAN_SCE();
    fn EXTI9_5();
    fn TIM1_BRK();
    fn TIM1_UP();
    fn TIM1_TRG_COM();
    fn TIM1_CC();
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
    fn RTCAlarm();
    fn USBWakeup();
    fn TIM8_BRK();
    fn TIM8_UP();
    fn TIM8_TRG_COM();
    fn TIM8_CC();
    fn ADC3();
    fn FSMC();
    fn SDIO();
    fn TIM5();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6();
    fn TIM7();
    fn DMA2_Channel1();
    fn DMA2_Channel2();
    fn DMA2_Channel3();
    fn DMA2_Channel4();
    fn DMA2_Channel5();
    fn ETH();
    fn ETH_WKUP();
    fn CAN2_TX();
    fn CAN2_RX0();
    fn CAN2_RX1();
    fn CAN2_SCE();
    fn OTG_FS();
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
pub static __INTERRUPTS: [Vector; 68] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector { _handler: TAMPER },
    Vector { _handler: RTC },
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
    Vector { _handler: TIM1_BRK },
    Vector { _handler: TIM1_UP },
    Vector {
        _handler: TIM1_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
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
    Vector { _handler: RTCAlarm },
    Vector {
        _handler: USBWakeup,
    },
    Vector { _handler: TIM8_BRK },
    Vector { _handler: TIM8_UP },
    Vector {
        _handler: TIM8_TRG_COM,
    },
    Vector { _handler: TIM8_CC },
    Vector { _handler: ADC3 },
    Vector { _handler: FSMC },
    Vector { _handler: SDIO },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: TIM6 },
    Vector { _handler: TIM7 },
    Vector {
        _handler: DMA2_Channel1,
    },
    Vector {
        _handler: DMA2_Channel2,
    },
    Vector {
        _handler: DMA2_Channel3,
    },
    Vector {
        _handler: DMA2_Channel4,
    },
    Vector {
        _handler: DMA2_Channel5,
    },
    Vector { _handler: ETH },
    Vector { _handler: ETH_WKUP },
    Vector { _handler: CAN2_TX },
    Vector { _handler: CAN2_RX0 },
    Vector { _handler: CAN2_RX1 },
    Vector { _handler: CAN2_SCE },
    Vector { _handler: OTG_FS },
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
    /// 2: Tamper interrupt
    TAMPER = 2,
    /// 3: RTC global interrupt
    RTC = 3,
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
    /// 18: ADC1 and ADC2 global interrupt
    ADC1_2 = 18,
    /// 19: USB High Priority or CAN TX interrupts
    USB_HP_CAN_TX = 19,
    /// 20: USB Low Priority or CAN RX0 interrupts
    USB_LP_CAN_RX0 = 20,
    /// 21: CAN RX1 interrupts
    CAN_RX1 = 21,
    /// 22: CAN SCE interrupt
    CAN_SCE = 22,
    /// 23: EXTI Line\[9:5\] interrupts
    EXTI9_5 = 23,
    /// 24: TIM1 Break interrupt
    TIM1_BRK = 24,
    /// 25: TIM1 Update interrupt
    TIM1_UP = 25,
    /// 26: TIM1 Trigger and Commutation interrupts
    TIM1_TRG_COM = 26,
    /// 27: TIM1 Capture Compare interrupt
    TIM1_CC = 27,
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
    /// 41: RTC Alarms through EXTI line interrupt
    RTCAlarm = 41,
    /// 42: USB wakeup from suspend through EXTI line interrupt
    USBWakeup = 42,
    /// 43: TIM8 Break interrupt
    TIM8_BRK = 43,
    /// 44: TIM8 Update interrupt
    TIM8_UP = 44,
    /// 45: TIM8 Trigger and Commutation interrupts
    TIM8_TRG_COM = 45,
    /// 46: TIM8 Capture Compare interrupt
    TIM8_CC = 46,
    /// 47: ADC3 global interrupt
    ADC3 = 47,
    /// 48: FSMC global interrupt
    FSMC = 48,
    /// 49: SDIO global interrupt
    SDIO = 49,
    /// 50: TIM5 global interrupt
    TIM5 = 50,
    /// 51: SPI3 global interrupt
    SPI3 = 51,
    /// 52: UART4 global interrupt
    UART4 = 52,
    /// 53: UART5 global interrupt
    UART5 = 53,
    /// 54: TIM6 global interrupt
    TIM6 = 54,
    /// 55: TIM7 global interrupt
    TIM7 = 55,
    /// 56: DMA2 Channel1 global interrupt
    DMA2_Channel1 = 56,
    /// 57: DMA2 Channel2 global interrupt
    DMA2_Channel2 = 57,
    /// 58: DMA2 Channel3 global interrupt
    DMA2_Channel3 = 58,
    /// 59: DMA2 Channel4 global interrupt
    DMA2_Channel4 = 59,
    /// 60: DMA2 Channel5 global interrupt
    DMA2_Channel5 = 60,
    /// 61: Ethernet global interrupt
    ETH = 61,
    /// 62: Ethernet Wakeup through EXTI line interrupt
    ETH_WKUP = 62,
    /// 63: CAN2 TX interrupts
    CAN2_TX = 63,
    /// 64: CAN2 RX0 interrupts
    CAN2_RX0 = 64,
    /// 65: CAN2 RX1 interrupts
    CAN2_RX1 = 65,
    /// 66: CAN2 SCE interrupt
    CAN2_SCE = 66,
    /// 67: USB On The Go FS global interrupt
    OTG_FS = 67,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
