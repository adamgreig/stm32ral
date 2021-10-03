#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD_PVM_3();
    fn TAMP_RTCSTAMP_LSECSS_RTCSSRU();
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
    fn ADC();
    fn DAC();
    fn COMP();
    fn EXTI9_5();
    fn TIM1_BRK();
    fn TIM1_UP();
    fn TIM1_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM16();
    fn TIM17();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2S2();
    fn USART1();
    fn USART2();
    fn LPUART1();
    fn LPTIM1();
    fn LPTIM2();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn LPTIM3();
    fn HSEM();
    fn I2C3_EV();
    fn I2C3_ER();
    fn Radio_IRQ_Busy();
    fn AES();
    fn True_RNG();
    fn PKA();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn DMA2_CH6();
    fn DMA2_CH7();
    fn DMAMUX1_OVR();
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
pub static __INTERRUPTS: [Vector; 62] = [
    Vector { _handler: WWDG },
    Vector {
        _handler: PVD_PVM_3,
    },
    Vector {
        _handler: TAMP_RTCSTAMP_LSECSS_RTCSSRU,
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
    Vector { _handler: ADC },
    Vector { _handler: DAC },
    Vector { _reserved: 0 },
    Vector { _handler: COMP },
    Vector { _handler: EXTI9_5 },
    Vector { _handler: TIM1_BRK },
    Vector { _handler: TIM1_UP },
    Vector {
        _handler: TIM1_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2S2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: LPUART1 },
    Vector { _handler: LPTIM1 },
    Vector { _handler: LPTIM2 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector { _handler: LPTIM3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: HSEM },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector {
        _handler: Radio_IRQ_Busy,
    },
    Vector { _handler: AES },
    Vector { _handler: True_RNG },
    Vector { _handler: PKA },
    Vector { _handler: DMA2_CH1 },
    Vector { _handler: DMA2_CH2 },
    Vector { _handler: DMA2_CH3 },
    Vector { _handler: DMA2_CH4 },
    Vector { _handler: DMA2_CH5 },
    Vector { _handler: DMA2_CH6 },
    Vector { _handler: DMA2_CH7 },
    Vector {
        _handler: DMAMUX1_OVR,
    },
];

/// Available interrupts for this device
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window watchdog early wakeup interrupt
    WWDG = 0,
    /// 1: PVD through EXTI\[16\], PVM\[3\] through EXTI\[34\]
    PVD_PVM_3 = 1,
    /// 2: Tamper, TimeStamp, LSECSS,RTC_SSRU interrupt
    TAMP_RTCSTAMP_LSECSS_RTCSSRU = 2,
    /// 3: RTC wakeup interrupt
    RTC_WKUP = 3,
    /// 4: Flash memory global interrupt and Flash memory ECC single error interrupt
    FLASH = 4,
    /// 5: RCC global interrupt
    RCC = 5,
    /// 6: EXTI line 0 interrupt through EXTI
    EXTI0 = 6,
    /// 7: EXTI line 1 interrupt through EXTI
    EXTI1 = 7,
    /// 8: EXTI line 2 interrupt through EXTI
    EXTI2 = 8,
    /// 9: EXTI line 3 interrupt through EXTI
    EXTI3 = 9,
    /// 10: EXTI line 4 interrupt through EXTI
    EXTI4 = 10,
    /// 11: DMA1 channel 1 non-secure interrupt
    DMA1_CH1 = 11,
    /// 12: DMA1 channel 2 non-secure interrupt
    DMA1_CH2 = 12,
    /// 13: DMA1 channel 3 non-secure interrupt
    DMA1_CH3 = 13,
    /// 14: DMA1 channel 4 non-secure interrupt
    DMA1_CH4 = 14,
    /// 15: DMA1 channel 5 non-secure interrupt
    DMA1_CH5 = 15,
    /// 16: DMA1 channel 6 non-secure interrupt
    DMA1_CH6 = 16,
    /// 17: DMA1 channel 7 non-secure interrupt
    DMA1_CH7 = 17,
    /// 18: ADC global interrupt
    ADC = 18,
    /// 19: DAC global interrupt
    DAC = 19,
    /// 21: COMP2 and COMP1 interrupt through EXTI\[22:21\]
    COMP = 21,
    /// 22: EXTI line 9_5 interrupt through EXTI
    EXTI9_5 = 22,
    /// 23: Timer 1 break interrupt
    TIM1_BRK = 23,
    /// 24: Timer 1 Update
    TIM1_UP = 24,
    /// 25: Timer 1 trigger and communication
    TIM1_TRG_COM = 25,
    /// 26: Timer 1 capture compare interrupt
    TIM1_CC = 26,
    /// 27: Timer 2 global interrupt
    TIM2 = 27,
    /// 28: Timer 16 global interrupt
    TIM16 = 28,
    /// 29: Timer 17 global interrupt
    TIM17 = 29,
    /// 30: I2C1 event interrupt
    I2C1_EV = 30,
    /// 31: I2C1 event interrupt
    I2C1_ER = 31,
    /// 32: I2C2 event interrupt
    I2C2_EV = 32,
    /// 33: I2C2 error interrupt
    I2C2_ER = 33,
    /// 34: SPI 1 global interrupt
    SPI1 = 34,
    /// 35: SPI2S2 global interrupt
    SPI2S2 = 35,
    /// 36: USART1 global interrupt
    USART1 = 36,
    /// 37: USART2 global interrupt
    USART2 = 37,
    /// 38: LPUART1 global interrupt
    LPUART1 = 38,
    /// 39: LPtimer 1 global interrupt
    LPTIM1 = 39,
    /// 40: LPtimer 2 global interrupt
    LPTIM2 = 40,
    /// 41: EXTI line 15_10\] interrupt through EXTI
    EXTI15_10 = 41,
    /// 42: RTC alarms A and B interrupt
    RTC_ALARM = 42,
    /// 43: LPtimer 3 global interrupt
    LPTIM3 = 43,
    /// 47: Semaphore interrupt 0 to CPU
    HSEM = 47,
    /// 48: I2C3 event interrupt
    I2C3_EV = 48,
    /// 49: I2C3 error interrupt
    I2C3_ER = 49,
    /// 50: Radio IRQs, RFBUSY interrupt through EXTI
    Radio_IRQ_Busy = 50,
    /// 51: AES global interrupt
    AES = 51,
    /// 52: True random number generator interrupt
    True_RNG = 52,
    /// 53: Private key accelerator interrupt
    PKA = 53,
    /// 54: DMA2 channel 1 non-secure interrupt
    DMA2_CH1 = 54,
    /// 55: DMA2 channel 2 non-secure interrupt
    DMA2_CH2 = 55,
    /// 56: DMA2 channel 3 non-secure interrupt
    DMA2_CH3 = 56,
    /// 57: DMA2 channel 4 non-secure interrupt
    DMA2_CH4 = 57,
    /// 58: DMA2 channel 5 non-secure interrupt
    DMA2_CH5 = 58,
    /// 59: DMA2 channel 6 non-secure interrupt
    DMA2_CH6 = 59,
    /// 60: DMA2 channel 7 non-secure interrupt
    DMA2_CH7 = 60,
    /// 61: DMAMUX1 overrun interrupt
    DMAMUX1_OVR = 61,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
