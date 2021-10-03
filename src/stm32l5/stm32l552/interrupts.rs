#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD_PVM();
    fn RTC();
    fn RTC_S();
    fn TAMP();
    fn TAMP_S();
    fn FLASH();
    fn FLASH_S();
    fn RCC();
    fn RCC_S();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn EXTI5();
    fn EXTI6();
    fn EXTI7();
    fn EXTI8();
    fn EXTI9();
    fn EXTI10();
    fn EXTI11();
    fn EXTI12();
    fn EXTI13();
    fn EXTI14();
    fn EXTI15();
    fn DMAMUX1_OVR();
    fn DMAMUX1_OVR_S();
    fn DMA1_CH1();
    fn DMA1_CH2();
    fn DMA1_CH3();
    fn DMA1_CH4();
    fn DMA1_CH5();
    fn DMA1_CH6();
    fn DMA1_CH7();
    fn DMA1_Channel8();
    fn ADC1_2();
    fn FDCAN1_IT0();
    fn FDCAN1_IT1();
    fn TIM1_BRK();
    fn TIM1_UP();
    fn TIM1_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn TIM5();
    fn TIM6();
    fn TIM7();
    fn TIM8_BRK();
    fn TIM8_UP();
    fn TIM8_TRG_COM();
    fn TIM8_CC();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn UART4();
    fn UART5();
    fn LPUART1();
    fn LPTIM1();
    fn LPTIM2();
    fn TIM15();
    fn TIM16();
    fn TIM17();
    fn COMP();
    fn USB_FS();
    fn FMC();
    fn OCTOSPI1();
    fn SDMMC1();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn DMA2_CH6();
    fn DMA2_CH7();
    fn DMA2_CH8();
    fn I2C3_EV();
    fn I2C3_ER();
    fn SAI1();
    fn SAI2();
    fn TSC();
    fn RNG();
    fn LPTIM3();
    fn SPI3();
    fn I2C4_ER();
    fn I2C4_EV();
    fn DFSDM1_FLT0();
    fn DFSDM1_FLT1();
    fn DFSDM1_FLT2();
    fn DFSDM1_FLT3();
    fn UCPD1();
    fn ICACHE();
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
pub static __INTERRUPTS: [Vector; 108] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD_PVM },
    Vector { _handler: RTC },
    Vector { _handler: RTC_S },
    Vector { _handler: TAMP },
    Vector { _handler: TAMP_S },
    Vector { _handler: FLASH },
    Vector { _handler: FLASH_S },
    Vector { _reserved: 0 },
    Vector { _handler: RCC },
    Vector { _handler: RCC_S },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector { _handler: EXTI5 },
    Vector { _handler: EXTI6 },
    Vector { _handler: EXTI7 },
    Vector { _handler: EXTI8 },
    Vector { _handler: EXTI9 },
    Vector { _handler: EXTI10 },
    Vector { _handler: EXTI11 },
    Vector { _handler: EXTI12 },
    Vector { _handler: EXTI13 },
    Vector { _handler: EXTI14 },
    Vector { _handler: EXTI15 },
    Vector {
        _handler: DMAMUX1_OVR,
    },
    Vector {
        _handler: DMAMUX1_OVR_S,
    },
    Vector { _handler: DMA1_CH1 },
    Vector { _handler: DMA1_CH2 },
    Vector { _handler: DMA1_CH3 },
    Vector { _handler: DMA1_CH4 },
    Vector { _handler: DMA1_CH5 },
    Vector { _handler: DMA1_CH6 },
    Vector { _handler: DMA1_CH7 },
    Vector {
        _handler: DMA1_Channel8,
    },
    Vector { _handler: ADC1_2 },
    Vector { _reserved: 0 },
    Vector {
        _handler: FDCAN1_IT0,
    },
    Vector {
        _handler: FDCAN1_IT1,
    },
    Vector { _handler: TIM1_BRK },
    Vector { _handler: TIM1_UP },
    Vector {
        _handler: TIM1_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: TIM5 },
    Vector { _handler: TIM6 },
    Vector { _handler: TIM7 },
    Vector { _handler: TIM8_BRK },
    Vector { _handler: TIM8_UP },
    Vector {
        _handler: TIM8_TRG_COM,
    },
    Vector { _handler: TIM8_CC },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: LPUART1 },
    Vector { _handler: LPTIM1 },
    Vector { _handler: LPTIM2 },
    Vector { _handler: TIM15 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector { _handler: COMP },
    Vector { _handler: USB_FS },
    Vector { _reserved: 0 },
    Vector { _handler: FMC },
    Vector { _handler: OCTOSPI1 },
    Vector { _reserved: 0 },
    Vector { _handler: SDMMC1 },
    Vector { _reserved: 0 },
    Vector { _handler: DMA2_CH1 },
    Vector { _handler: DMA2_CH2 },
    Vector { _handler: DMA2_CH3 },
    Vector { _handler: DMA2_CH4 },
    Vector { _handler: DMA2_CH5 },
    Vector { _handler: DMA2_CH6 },
    Vector { _handler: DMA2_CH7 },
    Vector { _handler: DMA2_CH8 },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector { _handler: SAI1 },
    Vector { _handler: SAI2 },
    Vector { _handler: TSC },
    Vector { _reserved: 0 },
    Vector { _handler: RNG },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LPTIM3 },
    Vector { _handler: SPI3 },
    Vector { _handler: I2C4_ER },
    Vector { _handler: I2C4_EV },
    Vector {
        _handler: DFSDM1_FLT0,
    },
    Vector {
        _handler: DFSDM1_FLT1,
    },
    Vector {
        _handler: DFSDM1_FLT2,
    },
    Vector {
        _handler: DFSDM1_FLT3,
    },
    Vector { _handler: UCPD1 },
    Vector { _handler: ICACHE },
];

/// Available interrupts for this device
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window Watchdog interrupt
    WWDG = 0,
    /// 1: PVD/PVM1/PVM2/PVM3/PVM4 through EXTI
    PVD_PVM = 1,
    /// 2: RTC global interrupts (EXTI line 17)
    RTC = 2,
    /// 3: RTC secure global interrupts (EXTI line 18)
    RTC_S = 3,
    /// 4: TAMPTamper global interrupt (EXTI line 19)
    TAMP = 4,
    /// 5: Tamper secure global interrupt (EXTI line 20)
    TAMP_S = 5,
    /// 6: Flash global interrupt
    FLASH = 6,
    /// 7: Flash memory secure global interrupt
    FLASH_S = 7,
    /// 9: RCC global interrupt
    RCC = 9,
    /// 10: RCC SECURE GLOBAL INTERRUPT
    RCC_S = 10,
    /// 11: EXTI line0 interrupt
    EXTI0 = 11,
    /// 12: EXTI line1 interrupt
    EXTI1 = 12,
    /// 13: EXTI line2 interrupt
    EXTI2 = 13,
    /// 14: EXTI line3 interrupt
    EXTI3 = 14,
    /// 15: EXTI line4 interrupt
    EXTI4 = 15,
    /// 16: EXTI line5 interrupt
    EXTI5 = 16,
    /// 17: EXTI line6 interrupt
    EXTI6 = 17,
    /// 18: EXTI line7 interrupt
    EXTI7 = 18,
    /// 19: EXTI line8 interrupt
    EXTI8 = 19,
    /// 20: EXTI line9 interrupt
    EXTI9 = 20,
    /// 21: EXTI line10 interrupt
    EXTI10 = 21,
    /// 22: EXTI line11 interrupt
    EXTI11 = 22,
    /// 23: EXTI line12 interrupt
    EXTI12 = 23,
    /// 24: EXTI line13 interrupt
    EXTI13 = 24,
    /// 25: EXTI line14 interrupt
    EXTI14 = 25,
    /// 26: EXTI line15 interrupt
    EXTI15 = 26,
    /// 27: DMAMUX overrun interrupt
    DMAMUX1_OVR = 27,
    /// 28: DMAMUX1 secure overRun interrupt
    DMAMUX1_OVR_S = 28,
    /// 29: DMA1 Channel1 global interrupt
    DMA1_CH1 = 29,
    /// 30: DMA1 Channel2 global interrupt
    DMA1_CH2 = 30,
    /// 31: DMA1 Channel3 interrupt
    DMA1_CH3 = 31,
    /// 32: DMA1 Channel4 interrupt
    DMA1_CH4 = 32,
    /// 33: DMA1 Channel5 interrupt
    DMA1_CH5 = 33,
    /// 34: DMA1 Channel6 interrupt
    DMA1_CH6 = 34,
    /// 35: DMA1 Channel 7 interrupt
    DMA1_CH7 = 35,
    /// 36: DMA1_Channel8
    DMA1_Channel8 = 36,
    /// 37: ADC1_2 global interrupt
    ADC1_2 = 37,
    /// 39: FDCAN1 Interrupt 0
    FDCAN1_IT0 = 39,
    /// 40: FDCAN1 Interrupt 1
    FDCAN1_IT1 = 40,
    /// 41: TIM1 Break
    TIM1_BRK = 41,
    /// 42: TIM1 Update
    TIM1_UP = 42,
    /// 43: TIM1 Trigger and Commutation
    TIM1_TRG_COM = 43,
    /// 44: TIM1 Capture Compare interrupt
    TIM1_CC = 44,
    /// 45: TIM2 global interrupt
    TIM2 = 45,
    /// 46: TIM3 global interrupt
    TIM3 = 46,
    /// 47: TIM4 global interrupt
    TIM4 = 47,
    /// 48: TIM5 global interrupt
    TIM5 = 48,
    /// 49: TIM6 global interrupt
    TIM6 = 49,
    /// 50: TIM7 global interrupt
    TIM7 = 50,
    /// 51: TIM8 Break Interrupt
    TIM8_BRK = 51,
    /// 52: TIM8 Update Interrupt
    TIM8_UP = 52,
    /// 53: TIM8 Trigger and Commutation Interrupt
    TIM8_TRG_COM = 53,
    /// 54: TIM8 Capture Compare Interrupt
    TIM8_CC = 54,
    /// 55: I2C1 event interrupt
    I2C1_EV = 55,
    /// 56: I2C1 error interrupt
    I2C1_ER = 56,
    /// 57: I2C2 event interrupt
    I2C2_EV = 57,
    /// 58: I2C2 error interrupt
    I2C2_ER = 58,
    /// 59: SPI1 global interrupt
    SPI1 = 59,
    /// 60: SPI2 global interrupt
    SPI2 = 60,
    /// 61: USART1 global interrupt
    USART1 = 61,
    /// 62: USART2 global interrupt
    USART2 = 62,
    /// 63: USART3 global interrupt
    USART3 = 63,
    /// 64: UART4 global interrupt
    UART4 = 64,
    /// 65: UART5 global interrupt
    UART5 = 65,
    /// 66: LPUART1 global interrupt
    LPUART1 = 66,
    /// 67: LP TIM1 interrupt
    LPTIM1 = 67,
    /// 68: LP TIM2 interrupt
    LPTIM2 = 68,
    /// 69: TIM15 global interrupt
    TIM15 = 69,
    /// 70: TIM16 global interrupt
    TIM16 = 70,
    /// 71: TIM17 global interrupt
    TIM17 = 71,
    /// 72: COMP1 and COMP2 interrupts
    COMP = 72,
    /// 73: USB FS global interrupt
    USB_FS = 73,
    /// 75: FMC global interrupt
    FMC = 75,
    /// 76: OCTOSPI1 global interrupt
    OCTOSPI1 = 76,
    /// 78: SDMMC1 global interrupt
    SDMMC1 = 78,
    /// 80: DMA2_CH1
    DMA2_CH1 = 80,
    /// 81: DMA2_CH2
    DMA2_CH2 = 81,
    /// 82: DMA2_CH3
    DMA2_CH3 = 82,
    /// 83: DMA2_CH4
    DMA2_CH4 = 83,
    /// 84: DMA2_CH5
    DMA2_CH5 = 84,
    /// 85: DMA2_CH6
    DMA2_CH6 = 85,
    /// 86: DMA2_CH7
    DMA2_CH7 = 86,
    /// 87: DMA2_CH8
    DMA2_CH8 = 87,
    /// 88: I2C3 event interrupt
    I2C3_EV = 88,
    /// 89: I2C3 error interrupt
    I2C3_ER = 89,
    /// 90: SAI1 global interrupt
    SAI1 = 90,
    /// 91: SAI2 global interrupt
    SAI2 = 91,
    /// 92: TSC global interrupt
    TSC = 92,
    /// 94: RNG global interrupt
    RNG = 94,
    /// 98: LPTIM3
    LPTIM3 = 98,
    /// 99: SPI3
    SPI3 = 99,
    /// 100: I2C4 error interrupt
    I2C4_ER = 100,
    /// 101: I2C4 event interrupt
    I2C4_EV = 101,
    /// 102: DFSDM1_FLT0 global interrupt
    DFSDM1_FLT0 = 102,
    /// 103: DFSDM1_FLT1 global interrupt
    DFSDM1_FLT1 = 103,
    /// 104: DFSDM1_FLT2 global interrupt
    DFSDM1_FLT2 = 104,
    /// 105: DFSDM1_FLT3 global interrupt
    DFSDM1_FLT3 = 105,
    /// 106: UCPD global interrupt
    UCPD1 = 106,
    /// 107: ICACHE
    ICACHE = 107,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
