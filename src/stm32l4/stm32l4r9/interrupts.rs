#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD_PVM();
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
    fn ADC1();
    fn CAN1_TX();
    fn CAN1_RX0();
    fn CAN1_RX1();
    fn CAN1_SCE();
    fn EXTI9_5();
    fn TIM1_BRK_TIM15();
    fn TIM1_UP_TIM16();
    fn TIM1_TRG_COM_TIM17();
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
    fn DFSDM1_FLT3();
    fn TIM8_BRK();
    fn TIM8_UP();
    fn TIM8_TRG_COM();
    fn TIM8_CC();
    fn ADC3();
    fn FMC();
    fn SDMMC1();
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
    fn DFSDM1_FLT0();
    fn DFSDM1_FLT1();
    fn DFSDM1_FLT2();
    fn COMP();
    fn LPTIM1();
    fn LPTIM2();
    fn OTG_FS();
    fn DMA2_CH6();
    fn DMA2_CH7();
    fn LPUART1();
    fn OCTOSPI1();
    fn I2C3_EV();
    fn I2C3_ER();
    fn SAI1();
    fn SAI2();
    fn OCTOSPI2();
    fn TSC();
    fn DSIHOST();
    fn AES();
    fn RNG_HASH();
    fn FPU();
    fn CRS();
    fn I2C4_ER();
    fn I2C4_EV();
    fn DCMI();
    fn DMA2D();
    fn LCD_TFT();
    fn LCD_TFT_ER();
    fn GFXMMU();
    fn DMAMUX_OVR();
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
pub static __INTERRUPTS: [Vector; 95] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD_PVM },
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
    Vector { _handler: ADC1 },
    Vector { _handler: CAN1_TX },
    Vector { _handler: CAN1_RX0 },
    Vector { _handler: CAN1_RX1 },
    Vector { _handler: CAN1_SCE },
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
        _handler: DFSDM1_FLT3,
    },
    Vector { _handler: TIM8_BRK },
    Vector { _handler: TIM8_UP },
    Vector {
        _handler: TIM8_TRG_COM,
    },
    Vector { _handler: TIM8_CC },
    Vector { _handler: ADC3 },
    Vector { _handler: FMC },
    Vector { _handler: SDMMC1 },
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
    Vector {
        _handler: DFSDM1_FLT0,
    },
    Vector {
        _handler: DFSDM1_FLT1,
    },
    Vector {
        _handler: DFSDM1_FLT2,
    },
    Vector { _handler: COMP },
    Vector { _handler: LPTIM1 },
    Vector { _handler: LPTIM2 },
    Vector { _handler: OTG_FS },
    Vector { _handler: DMA2_CH6 },
    Vector { _handler: DMA2_CH7 },
    Vector { _handler: LPUART1 },
    Vector { _handler: OCTOSPI1 },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector { _handler: SAI1 },
    Vector { _handler: SAI2 },
    Vector { _handler: OCTOSPI2 },
    Vector { _handler: TSC },
    Vector { _handler: DSIHOST },
    Vector { _handler: AES },
    Vector { _handler: RNG_HASH },
    Vector { _handler: FPU },
    Vector { _handler: CRS },
    Vector { _handler: I2C4_ER },
    Vector { _handler: I2C4_EV },
    Vector { _handler: DCMI },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: DMA2D },
    Vector { _handler: LCD_TFT },
    Vector {
        _handler: LCD_TFT_ER,
    },
    Vector { _handler: GFXMMU },
    Vector {
        _handler: DMAMUX_OVR,
    },
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
    /// 10: EXTI Line 4 interrupt
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
    ADC1 = 18,
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
    /// 24: TIM1 Break/TIM15 global interrupts
    TIM1_BRK_TIM15 = 24,
    /// 25: TIM1 Update/TIM16 global interrupts
    TIM1_UP_TIM16 = 25,
    /// 26: TIM1 Trigger and Commutation interrupts and TIM17 global interrupt
    TIM1_TRG_COM_TIM17 = 26,
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
    /// 40: EXTI Lines 10 to 15 interrupts
    EXTI15_10 = 40,
    /// 41: RTC alarms through EXTI line 18 interrupts
    RTC_ALARM = 41,
    /// 42: DFSDM1_FLT3 global interrupt
    DFSDM1_FLT3 = 42,
    /// 43: TIM8 Break Interrupt
    TIM8_BRK = 43,
    /// 44: TIM8 Update Interrupt
    TIM8_UP = 44,
    /// 45: TIM8 Trigger and Commutation Interrupt
    TIM8_TRG_COM = 45,
    /// 46: TIM8 Capture Compare Interrupt
    TIM8_CC = 46,
    /// 47: ADC3 global interrupt
    ADC3 = 47,
    /// 48: FMC global Interrupt
    FMC = 48,
    /// 49: SDMMC global Interrupt
    SDMMC1 = 49,
    /// 50: TIM5 global Interrupt
    TIM5 = 50,
    /// 51: SPI3 global Interrupt
    SPI3 = 51,
    /// 52: UART4 global Interrupt
    UART4 = 52,
    /// 53: UART5 global Interrupt
    UART5 = 53,
    /// 54: TIM6 global and DAC1 and 2 underrun error interrupts
    TIM6_DACUNDER = 54,
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
    /// 61: DFSDM1_FLT0 global interrupt
    DFSDM1_FLT0 = 61,
    /// 62: DFSDM1_FLT1 global interrupt
    DFSDM1_FLT1 = 62,
    /// 63: DFSDM1_FLT2 global interrupt
    DFSDM1_FLT2 = 63,
    /// 64: COMP1 and COMP2 interrupts
    COMP = 64,
    /// 65: LP TIM1 interrupt
    LPTIM1 = 65,
    /// 66: LP TIM2 interrupt
    LPTIM2 = 66,
    /// 67: USB OTG FS global Interrupt
    OTG_FS = 67,
    /// 68: DMA2 Channel 6 global Interrupt
    DMA2_CH6 = 68,
    /// 69: DMA2 Channel 7 global Interrupt
    DMA2_CH7 = 69,
    /// 70: LPUART1 global interrupt
    LPUART1 = 70,
    /// 71: OCTOSPI1 global interrupt
    OCTOSPI1 = 71,
    /// 72: I2C3 event interrupt
    I2C3_EV = 72,
    /// 73: I2C3 error interrupt
    I2C3_ER = 73,
    /// 74: SAI1 global interrupt
    SAI1 = 74,
    /// 75: SAI2 global interrupt
    SAI2 = 75,
    /// 76: OCTOSPI2 global interrupt
    OCTOSPI2 = 76,
    /// 77: TSC global interrupt
    TSC = 77,
    /// 78: DSI global interrupt
    DSIHOST = 78,
    /// 79: AES global interrupt
    AES = 79,
    /// 80: RNG and HASH global interrupt
    RNG_HASH = 80,
    /// 81: Floating point interrupt
    FPU = 81,
    /// 82: CRS global interrupt
    CRS = 82,
    /// 83: I2C4 error interrupt
    I2C4_ER = 83,
    /// 84: I2C4 event interrupt
    I2C4_EV = 84,
    /// 85: DCMI global interrupt
    DCMI = 85,
    /// 90: DMA2D global interrupt
    DMA2D = 90,
    /// 91: LTDC global interrupt
    LCD_TFT = 91,
    /// 92: LTDC global error interrupt
    LCD_TFT_ER = 92,
    /// 93: GFXMMU global error interrupt
    GFXMMU = 93,
    /// 94: DMAMUX Overrun interrupt
    DMAMUX_OVR = 94,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
