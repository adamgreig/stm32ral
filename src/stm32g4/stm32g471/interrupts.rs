#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD_PVM();
    fn RTC_TAMP_CSS_LSE();
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
    fn USB_HP();
    fn USB_LP();
    fn fdcan1_intr1_it();
    fn fdcan1_intr0_it();
    fn EXTI9_5();
    fn TIM1_BRK_TIM15();
    fn TIM1_UP_TIM16();
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
    fn RTC_ALARM();
    fn USBWakeUP();
    fn TIM8_BRK();
    fn TIM8_UP();
    fn TIM8_TRG_COM();
    fn TIM8_CC();
    fn ADC3();
    fn LPTIM1();
    fn TIM5();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6_DACUNDER();
    fn TIM7();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn UCPD1();
    fn COMP1_2_3();
    fn COMP4();
    fn CRS();
    fn SAI();
    fn FPU();
    fn I2C4_EV();
    fn I2C4_ER();
    fn SPI4();
    fn AES();
    fn FDCAN2_intr0();
    fn FDCAN2_intr1();
    fn RNG();
    fn LPUART();
    fn I2C3_EV();
    fn I2C3_ER();
    fn DMAMUX_OVR();
    fn DMA1_CH8();
    fn DMA2_CH6();
    fn DMA2_CH7();
    fn DMA2_CH8();
    fn Cordic();
    fn FMAC();
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
pub static __INTERRUPTS: [Vector; 102] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD_PVM },
    Vector {
        _handler: RTC_TAMP_CSS_LSE,
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
    Vector { _handler: USB_HP },
    Vector { _handler: USB_LP },
    Vector {
        _handler: fdcan1_intr1_it,
    },
    Vector {
        _handler: fdcan1_intr0_it,
    },
    Vector { _handler: EXTI9_5 },
    Vector {
        _handler: TIM1_BRK_TIM15,
    },
    Vector {
        _handler: TIM1_UP_TIM16,
    },
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
    Vector {
        _handler: RTC_ALARM,
    },
    Vector {
        _handler: USBWakeUP,
    },
    Vector { _handler: TIM8_BRK },
    Vector { _handler: TIM8_UP },
    Vector {
        _handler: TIM8_TRG_COM,
    },
    Vector { _handler: TIM8_CC },
    Vector { _handler: ADC3 },
    Vector { _reserved: 0 },
    Vector { _handler: LPTIM1 },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector {
        _handler: TIM6_DACUNDER,
    },
    Vector { _handler: TIM7 },
    Vector { _handler: DMA2_CH1 },
    Vector { _handler: DMA2_CH2 },
    Vector { _handler: DMA2_CH3 },
    Vector { _handler: DMA2_CH4 },
    Vector { _handler: DMA2_CH5 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: UCPD1 },
    Vector {
        _handler: COMP1_2_3,
    },
    Vector { _handler: COMP4 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: CRS },
    Vector { _handler: SAI },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FPU },
    Vector { _handler: I2C4_EV },
    Vector { _handler: I2C4_ER },
    Vector { _handler: SPI4 },
    Vector { _handler: AES },
    Vector {
        _handler: FDCAN2_intr0,
    },
    Vector {
        _handler: FDCAN2_intr1,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: RNG },
    Vector { _handler: LPUART },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector {
        _handler: DMAMUX_OVR,
    },
    Vector { _reserved: 0 },
    Vector { _handler: DMA1_CH8 },
    Vector { _handler: DMA2_CH6 },
    Vector { _handler: DMA2_CH7 },
    Vector { _handler: DMA2_CH8 },
    Vector { _handler: Cordic },
    Vector { _handler: FMAC },
];

/// Available interrupts for this device
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window Watchdog interrupt
    WWDG = 0,
    /// 1: PVD through EXTI line detection
    PVD_PVM = 1,
    /// 2: RTC_TAMP_CSS_LSE
    RTC_TAMP_CSS_LSE = 2,
    /// 3: RTC Wakeup timer
    RTC_WKUP = 3,
    /// 4: FLASH
    FLASH = 4,
    /// 5: RCC
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
    /// 18: ADC1 and ADC2 global interrupt
    ADC1_2 = 18,
    /// 19: USB_HP
    USB_HP = 19,
    /// 20: USB_LP
    USB_LP = 20,
    /// 21: fdcan1_intr1_it
    fdcan1_intr1_it = 21,
    /// 22: fdcan1_intr0_it
    fdcan1_intr0_it = 22,
    /// 23: EXTI9_5
    EXTI9_5 = 23,
    /// 24: TIM1_BRK_TIM15
    TIM1_BRK_TIM15 = 24,
    /// 25: TIM1_UP_TIM16
    TIM1_UP_TIM16 = 25,
    /// 26: TIM1_TRG_COM/
    TIM1_TRG_COM = 26,
    /// 27: TIM1 capture compare interrupt
    TIM1_CC = 27,
    /// 28: TIM2
    TIM2 = 28,
    /// 29: TIM3
    TIM3 = 29,
    /// 30: TIM4
    TIM4 = 30,
    /// 31: I2C1_EV
    I2C1_EV = 31,
    /// 32: I2C1_ER
    I2C1_ER = 32,
    /// 33: I2C2_EV
    I2C2_EV = 33,
    /// 34: I2C2_ER
    I2C2_ER = 34,
    /// 35: SPI1
    SPI1 = 35,
    /// 36: SPI2
    SPI2 = 36,
    /// 37: USART1
    USART1 = 37,
    /// 38: USART2
    USART2 = 38,
    /// 39: USART3
    USART3 = 39,
    /// 40: EXTI15_10
    EXTI15_10 = 40,
    /// 41: RTC_ALARM
    RTC_ALARM = 41,
    /// 42: USBWakeUP
    USBWakeUP = 42,
    /// 43: TIM8_BRK
    TIM8_BRK = 43,
    /// 44: TIM8_UP
    TIM8_UP = 44,
    /// 45: TIM8_TRG_COM
    TIM8_TRG_COM = 45,
    /// 46: TIM8_CC
    TIM8_CC = 46,
    /// 47: ADC3
    ADC3 = 47,
    /// 49: LPTIM1
    LPTIM1 = 49,
    /// 50: TIM5
    TIM5 = 50,
    /// 51: SPI3
    SPI3 = 51,
    /// 52: UART4
    UART4 = 52,
    /// 53: UART5
    UART5 = 53,
    /// 54: TIM6_DACUNDER
    TIM6_DACUNDER = 54,
    /// 55: TIM7
    TIM7 = 55,
    /// 56: DMA2_CH1
    DMA2_CH1 = 56,
    /// 57: DMA2_CH2
    DMA2_CH2 = 57,
    /// 58: DMA2_CH3
    DMA2_CH3 = 58,
    /// 59: DMA2_CH4
    DMA2_CH4 = 59,
    /// 60: DMA2_CH5
    DMA2_CH5 = 60,
    /// 63: UCPD1
    UCPD1 = 63,
    /// 64: COMP1_2_3
    COMP1_2_3 = 64,
    /// 65: COMP4_5_6
    COMP4 = 65,
    /// 75: CRS
    CRS = 75,
    /// 76: SAI
    SAI = 76,
    /// 81: Floating point unit interrupt
    FPU = 81,
    /// 82: I2C4_EV
    I2C4_EV = 82,
    /// 83: I2C4_ER
    I2C4_ER = 83,
    /// 84: SPI4
    SPI4 = 84,
    /// 85: AES
    AES = 85,
    /// 86: FDCAN2_intr0
    FDCAN2_intr0 = 86,
    /// 87: FDCAN2_intr1
    FDCAN2_intr1 = 87,
    /// 90: RNG
    RNG = 90,
    /// 91: LPUART
    LPUART = 91,
    /// 92: I2C3_EV
    I2C3_EV = 92,
    /// 93: I2C3_ER
    I2C3_ER = 93,
    /// 94: DMAMUX_OVR
    DMAMUX_OVR = 94,
    /// 96: DMA1_CH8
    DMA1_CH8 = 96,
    /// 97: DMA2_CH6
    DMA2_CH6 = 97,
    /// 98: DMA2_CH7
    DMA2_CH7 = 98,
    /// 99: DMA2_CH8
    DMA2_CH8 = 99,
    /// 100: Cordic
    Cordic = 100,
    /// 101: FMAC
    FMAC = 101,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
