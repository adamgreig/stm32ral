extern crate bare_metal;
#[cfg(feature = "rt")]
extern "C" {
    fn TZIC_ILA();
    fn PVD_PVM_3();
    fn TAMP_RTCSTAMP_LSECSS_RTCALARM_RTCSSRU_RTCWKUP();
    fn FLASH_RCC_C1SEV();
    fn EXTI1_0();
    fn EXTI3_2();
    fn EXTI15_4();
    fn ADC_COMP_DAC();
    fn DMA1_CH3_1();
    fn DMA1_CH7_4();
    fn DMA2_CH7_1_DMAMUX1_OVR();
    fn LPTIM1();
    fn LPTIM2();
    fn LPTIM3();
    fn TIM1_BRK_TIM1_UP_TIM1_TRG_COM_TIM1_CC();
    fn TIM2();
    fn TIM16();
    fn TIM17();
    fn IPCC_C2_RX_IT_IPCC_C2_TX_IT();
    fn HSEM();
    fn True_RNG();
    fn AES_PKA();
    fn I2C1_EV_I2C1_ER();
    fn I2C2_EV_I2C2_ER();
    fn I2C3_EV_I2C3_ER();
    fn SPI1();
    fn SPI2S2();
    fn USART1();
    fn USART2();
    fn LPUART1();
    fn SUBGHZSPI();
    fn Radio_IRQ_Busy();
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
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: TZIC_ILA },
    Vector {
        _handler: PVD_PVM_3,
    },
    Vector {
        _handler: TAMP_RTCSTAMP_LSECSS_RTCALARM_RTCSSRU_RTCWKUP,
    },
    Vector {
        _handler: FLASH_RCC_C1SEV,
    },
    Vector { _handler: EXTI1_0 },
    Vector { _handler: EXTI3_2 },
    Vector { _handler: EXTI15_4 },
    Vector {
        _handler: ADC_COMP_DAC,
    },
    Vector {
        _handler: DMA1_CH3_1,
    },
    Vector {
        _handler: DMA1_CH7_4,
    },
    Vector {
        _handler: DMA2_CH7_1_DMAMUX1_OVR,
    },
    Vector { _handler: LPTIM1 },
    Vector { _handler: LPTIM2 },
    Vector { _handler: LPTIM3 },
    Vector {
        _handler: TIM1_BRK_TIM1_UP_TIM1_TRG_COM_TIM1_CC,
    },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector {
        _handler: IPCC_C2_RX_IT_IPCC_C2_TX_IT,
    },
    Vector { _handler: HSEM },
    Vector { _handler: True_RNG },
    Vector { _handler: AES_PKA },
    Vector {
        _handler: I2C1_EV_I2C1_ER,
    },
    Vector {
        _handler: I2C2_EV_I2C2_ER,
    },
    Vector {
        _handler: I2C3_EV_I2C3_ER,
    },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2S2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: LPUART1 },
    Vector {
        _handler: SUBGHZSPI,
    },
    Vector {
        _handler: Radio_IRQ_Busy,
    },
];

/// Available interrupts for this device
#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Security Interrupt controller illegal access interrupt
    TZIC_ILA = 0,
    /// 1: PVD through EXTI\[16\], PVM\[3\] through EXTI\[34\]
    PVD_PVM_3 = 1,
    /// 2: Tamper, TimeStamp, LSECSS,alarm A and B,SSR underflow,RTC wakeup interrupt
    TAMP_RTCSTAMP_LSECSS_RTCALARM_RTCSSRU_RTCWKUP = 2,
    /// 3: Flash memory global interrupt and Flash memory ECC single error interrupt,RCC global interrupt,CPU1 SEV through EXTI
    FLASH_RCC_C1SEV = 3,
    /// 4: EXTI line 0 interrupt through EXTI
    EXTI1_0 = 4,
    /// 5: EXTI line 1 interrupt through EXTI
    EXTI3_2 = 5,
    /// 6: EXTI line 2 interrupt through EXTI
    EXTI15_4 = 6,
    /// 7: ADC and DAC global interrupt,COMP1 and COMP2 interrupt through EXTI
    ADC_COMP_DAC = 7,
    /// 8: DMA1 channel 3:1 secure and non-secure interrupt (C2IMR2\[2:0\])
    DMA1_CH3_1 = 8,
    /// 9: DMA1 channel 7:4 secure and non-secure interrupt (C2IMR2\[6:3\])
    DMA1_CH7_4 = 9,
    /// 10: DMA2 channel 7:1 secure and non-secure interrupt (C2IMR2\[14:8\]),DMAMUX1 overrun interrupt (C2IMR2\[15\])
    DMA2_CH7_1_DMAMUX1_OVR = 10,
    /// 11: LPtimer 1 global interrupt
    LPTIM1 = 11,
    /// 12: LPtimer 2 global interrupt
    LPTIM2 = 12,
    /// 13: LPtimer 3 global interrupt
    LPTIM3 = 13,
    /// 14: Timer 1 break, update, trigger and communication,capture compare interrupt
    TIM1_BRK_TIM1_UP_TIM1_TRG_COM_TIM1_CC = 14,
    /// 15: Timer 2 global interrupt
    TIM2 = 15,
    /// 16: Timer 16 global interrupt
    TIM16 = 16,
    /// 17: Timer 17 global interrupt
    TIM17 = 17,
    /// 18: IPCC CPU2 RX occupied interrupt, IPCC CPU2 TX free interrupt
    IPCC_C2_RX_IT_IPCC_C2_TX_IT = 18,
    /// 19: Semaphore interrupt 1 to CPU2
    HSEM = 19,
    /// 20: True random number generator interrupt
    True_RNG = 20,
    /// 21: AES global interrupt , Private key accelerator interrupt
    AES_PKA = 21,
    /// 22: I2C1 event interrupt,I2C1 error interrupt
    I2C1_EV_I2C1_ER = 22,
    /// 23: I2C2 event interrupt , I2C2 error interrupt
    I2C2_EV_I2C2_ER = 23,
    /// 24: I2C3 event interrupt , I2C2 error interrupt
    I2C3_EV_I2C3_ER = 24,
    /// 25: SPI 1 global interrupt
    SPI1 = 25,
    /// 26: SPI2S2 global interrupt
    SPI2S2 = 26,
    /// 27: USART1 global interrupt
    USART1 = 27,
    /// 28: USART2 global interrupt
    USART2 = 28,
    /// 29: LPUART1 global interrupt
    LPUART1 = 29,
    /// 30: Sub-GHz radio SPI global interrupt
    SUBGHZSPI = 30,
    /// 31: Radio IRQs, RFBUSY interrupt through EXTI
    Radio_IRQ_Busy = 31,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
