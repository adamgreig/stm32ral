extern crate bare_metal;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn RTC_TAMP();
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
    fn C2SEV();
    fn COMP();
    fn EXTI9_5();
    fn TIM1_BRK();
    fn TIM1_UP();
    fn TIM1_TRG_COM_TIM17();
    fn TIM1_CC();
    fn TIM2();
    fn PKA();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C3_EV();
    fn I2C3_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn LPUART1();
    fn SAI1();
    fn TSC();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn CRS_IT();
    fn PWR_SOTF();
    fn IPCC_C1_RX_IT();
    fn IPCC_C1_TX_IT();
    fn HSEM();
    fn LPTIM1();
    fn LPTIM2();
    fn LCD();
    fn QUADSPI();
    fn AES1();
    fn AES2();
    fn True_RNG();
    fn FPU();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn DMA2_CH6();
    fn DMA2_CH7();
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
pub static __INTERRUPTS: [Vector; 63] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector { _handler: RTC_TAMP },
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
    Vector { _handler: C2SEV },
    Vector { _handler: COMP },
    Vector { _handler: EXTI9_5 },
    Vector { _handler: TIM1_BRK },
    Vector { _handler: TIM1_UP },
    Vector {
        _handler: TIM1_TRG_COM_TIM17,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: PKA },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: LPUART1 },
    Vector { _handler: SAI1 },
    Vector { _handler: TSC },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector { _handler: CRS_IT },
    Vector { _handler: PWR_SOTF },
    Vector {
        _handler: IPCC_C1_RX_IT,
    },
    Vector {
        _handler: IPCC_C1_TX_IT,
    },
    Vector { _handler: HSEM },
    Vector { _handler: LPTIM1 },
    Vector { _handler: LPTIM2 },
    Vector { _handler: LCD },
    Vector { _handler: QUADSPI },
    Vector { _handler: AES1 },
    Vector { _handler: AES2 },
    Vector { _handler: True_RNG },
    Vector { _handler: FPU },
    Vector { _handler: DMA2_CH1 },
    Vector { _handler: DMA2_CH2 },
    Vector { _handler: DMA2_CH3 },
    Vector { _handler: DMA2_CH4 },
    Vector { _handler: DMA2_CH5 },
    Vector { _handler: DMA2_CH6 },
    Vector { _handler: DMA2_CH7 },
    Vector {
        _handler: DMAMUX_OVR,
    },
];

/// Available interrupts for this device
#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window Watchdog interrupt
    WWDG = 0,
    /// 1: PVD through EXTI\[16\] (C1IMR2\[20\])
    PVD = 1,
    /// 2: RTC/TAMP/CSS on LSE through EXTI line 19 interrupt
    RTC_TAMP = 2,
    /// 3: RTC wakeup interrupt through EXTI\[19\]
    RTC_WKUP = 3,
    /// 4: Flash global interrupt
    FLASH = 4,
    /// 5: RCC global interrupt
    RCC = 5,
    /// 6: EXTI line 0 interrupt through EXTI\[0\]
    EXTI0 = 6,
    /// 7: EXTI line 0 interrupt through EXTI\[1\]
    EXTI1 = 7,
    /// 8: EXTI line 0 interrupt through EXTI\[2\]
    EXTI2 = 8,
    /// 9: EXTI line 0 interrupt through EXTI\[3\]
    EXTI3 = 9,
    /// 10: EXTI line 0 interrupt through EXTI\[4\]
    EXTI4 = 10,
    /// 11: DMA1 Channel1 global interrupt
    DMA1_Channel1 = 11,
    /// 12: DMA1 Channel2 global interrupt
    DMA1_Channel2 = 12,
    /// 13: DMA1 Channel3 interrupt
    DMA1_Channel3 = 13,
    /// 14: DMA1 Channel4 interrupt
    DMA1_Channel4 = 14,
    /// 15: DMA1 Channel5 interrupt
    DMA1_Channel5 = 15,
    /// 16: DMA1 Channel6 interrupt
    DMA1_Channel6 = 16,
    /// 17: DMA1 Channel 7 interrupt
    DMA1_Channel7 = 17,
    /// 18: ADC1 global interrupt
    ADC1 = 18,
    /// 19: USB high priority interrupt
    USB_HP = 19,
    /// 20: USB low priority interrupt (including USB wakeup)
    USB_LP = 20,
    /// 21: CPU2 SEV through EXTI\[40\]
    C2SEV = 21,
    /// 22: COMP2 & COMP1 interrupt through AIEC\[21:20\]
    COMP = 22,
    /// 23: EXTI line \[9:5\] interrupt through EXTI\[9:5\]
    EXTI9_5 = 23,
    /// 24: Timer 1 break interrupt
    TIM1_BRK = 24,
    /// 25: Timer 1 Update
    TIM1_UP = 25,
    /// 26: TIM1 Trigger and Commutation interrupts and TIM17 global interrupt
    TIM1_TRG_COM_TIM17 = 26,
    /// 27: TIM1 Capture Compare interrupt
    TIM1_CC = 27,
    /// 28: TIM2 global interrupt
    TIM2 = 28,
    /// 29: Private key accelerator interrupt
    PKA = 29,
    /// 30: I2C1 event interrupt
    I2C1_EV = 30,
    /// 31: I2C1 error interrupt
    I2C1_ER = 31,
    /// 32: I2C3 event interrupt
    I2C3_EV = 32,
    /// 33: I2C3 error interrupt
    I2C3_ER = 33,
    /// 34: SPI 1 global interrupt
    SPI1 = 34,
    /// 35: SPI1 global interrupt
    SPI2 = 35,
    /// 36: USART1 global interrupt
    USART1 = 36,
    /// 37: LPUART1 global interrupt
    LPUART1 = 37,
    /// 38: SAI1 global interrupt
    SAI1 = 38,
    /// 39: TSC global interrupt
    TSC = 39,
    /// 40: EXTI line \[15:10\] interrupt through EXTI\[15:10\]
    EXTI15_10 = 40,
    /// 41: RTC Alarms (A and B) interrupt through AIEC
    RTC_ALARM = 41,
    /// 42: CRS interrupt
    CRS_IT = 42,
    /// 43: PWR switching on the fly interrupt
    PWR_SOTF = 43,
    /// 44: IPCC CPU1 RX occupied interrupt
    IPCC_C1_RX_IT = 44,
    /// 45: IPCC CPU1 TX free interrupt
    IPCC_C1_TX_IT = 45,
    /// 46: Semaphore interrupt 0 to CPU1
    HSEM = 46,
    /// 47: LPtimer 1 global interrupt
    LPTIM1 = 47,
    /// 48: LPtimer 2 global interrupt
    LPTIM2 = 48,
    /// 49: LCD global interrupt
    LCD = 49,
    /// 50: QSPI global interrupt
    QUADSPI = 50,
    /// 51: AES1 global interrupt
    AES1 = 51,
    /// 52: AES2 global interrupt
    AES2 = 52,
    /// 53: True random number generator interrupt
    True_RNG = 53,
    /// 54: Floating point unit interrupt
    FPU = 54,
    /// 55: DMA2 channel 1 interrupt
    DMA2_CH1 = 55,
    /// 56: DMA2 channel 2 interrupt
    DMA2_CH2 = 56,
    /// 57: DMA2 channel 3 interrupt
    DMA2_CH3 = 57,
    /// 58: DMA2 channel 4 interrupt
    DMA2_CH4 = 58,
    /// 59: DMA2 channel 5 interrupt
    DMA2_CH5 = 59,
    /// 60: DMA2 channel 6 interrupt
    DMA2_CH6 = 60,
    /// 61: DMA2 channel 7 interrupt
    DMA2_CH7 = 61,
    /// 62: DMAMUX overrun interrupt
    DMAMUX_OVR = 62,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
