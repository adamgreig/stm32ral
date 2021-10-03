extern crate bare_metal;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG1_IT();
    fn PVD_AVD();
    fn TAMP();
    fn RTC_WKUP_ALARM();
    fn TZC_IT();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_STR0();
    fn DMA1_STR1();
    fn DMA1_STR2();
    fn DMA1_STR3();
    fn DMA1_STR4();
    fn DMA1_STR5();
    fn DMA1_STR6();
    fn ADC1();
    fn FDCAN1_IT0();
    fn FDCAN2_IT0();
    fn FDCAN1_IT1();
    fn FDCAN2_IT1();
    fn EXTI5();
    fn TIM1_BRK();
    fn TIM1_UP();
    fn TIM1_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EVT();
    fn I2C1_ERR();
    fn I2C2_EVT();
    fn I2C2_ERR();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI10();
    fn RTC_TS();
    fn EXTI11();
    fn TIM8_BRK();
    fn TIM8_UP();
    fn TIM8_TRG_COM();
    fn TIM8_CC();
    fn DMA1_STR7();
    fn FMC();
    fn SDMMC1();
    fn TIM5();
    fn SPI3();
    fn USART4();
    fn USART5();
    fn TIM6();
    fn TIM7();
    fn DMA2_STR0();
    fn DMA2_STR1();
    fn DMA2_STR2();
    fn DMA2_STR3();
    fn DMA2_STR4();
    fn ETH1();
    fn ETH1_WKUP();
    fn EXTI6();
    fn EXTI7();
    fn EXTI8();
    fn EXTI9();
    fn DMA2_STR5();
    fn DMA2_STR6();
    fn DMA2_STR7();
    fn USART6();
    fn I2C3_EVT();
    fn I2C3_ERR();
    fn EXTI12();
    fn EXTI13();
    fn DCMI();
    fn CRYP1();
    fn HASH1();
    fn USART7();
    fn USART8();
    fn SPI4();
    fn SPI5();
    fn SPI6();
    fn SAI1();
    fn LTDC();
    fn LTDC_ER();
    fn ADC2();
    fn SAI2();
    fn QUADSPI();
    fn LPTIM1();
    fn CEC();
    fn I2C4_EVT();
    fn I2C4_ERR();
    fn SPDIFRX();
    fn OTG();
    fn IPCC_RX0();
    fn IPCC_TX0();
    fn DMAMUX1_OVR_REQ();
    fn IPCC_RX1();
    fn IPCC_TX1();
    fn CRYP2();
    fn HASH2();
    fn I2C5_EVT();
    fn I2C5_ERR();
    fn DFSDM1_FLT0();
    fn DFSDM1_FLT1();
    fn DFSDM1_FLT2();
    fn DFSDM1_FLT3();
    fn SAI3();
    fn DFSDM1_FLT4();
    fn TIM15();
    fn TIM16();
    fn TIM17();
    fn TIM12();
    fn MDIOS();
    fn EXTI14();
    fn MDMA();
    fn SDMMC2();
    fn HSEM_IT2();
    fn DFSDM1_FLT5();
    fn EXTI15();
    fn TIM13();
    fn TIM14();
    fn DAC();
    fn RNG1();
    fn RNG2();
    fn I2C6_EVT();
    fn I2C6_ERR();
    fn SDMMC3();
    fn LPTIM2();
    fn LPTIM3();
    fn LPTIM4();
    fn LPTIM5();
    fn ETH1_LPI();
    fn RCC_WAKEUP();
    fn SAI4();
    fn DTS();
    fn IWDG1_IT();
    fn IWDG2_IT();
    fn TAMP_S();
    fn RTC_WKUP_ALARM_S();
    fn RTC_TS_S();
    fn DDRPERFM();
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
pub static __INTERRUPTS: [Vector; 214] = [
    Vector { _handler: WWDG1_IT },
    Vector { _handler: PVD_AVD },
    Vector { _handler: TAMP },
    Vector {
        _handler: RTC_WKUP_ALARM,
    },
    Vector { _handler: TZC_IT },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector {
        _handler: DMA1_STR0,
    },
    Vector {
        _handler: DMA1_STR1,
    },
    Vector {
        _handler: DMA1_STR2,
    },
    Vector {
        _handler: DMA1_STR3,
    },
    Vector {
        _handler: DMA1_STR4,
    },
    Vector {
        _handler: DMA1_STR5,
    },
    Vector {
        _handler: DMA1_STR6,
    },
    Vector { _handler: ADC1 },
    Vector {
        _handler: FDCAN1_IT0,
    },
    Vector {
        _handler: FDCAN2_IT0,
    },
    Vector {
        _handler: FDCAN1_IT1,
    },
    Vector {
        _handler: FDCAN2_IT1,
    },
    Vector { _handler: EXTI5 },
    Vector { _handler: TIM1_BRK },
    Vector { _handler: TIM1_UP },
    Vector {
        _handler: TIM1_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: I2C1_EVT },
    Vector { _handler: I2C1_ERR },
    Vector { _handler: I2C2_EVT },
    Vector { _handler: I2C2_ERR },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector { _handler: EXTI10 },
    Vector { _handler: RTC_TS },
    Vector { _handler: EXTI11 },
    Vector { _handler: TIM8_BRK },
    Vector { _handler: TIM8_UP },
    Vector {
        _handler: TIM8_TRG_COM,
    },
    Vector { _handler: TIM8_CC },
    Vector {
        _handler: DMA1_STR7,
    },
    Vector { _handler: FMC },
    Vector { _handler: SDMMC1 },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: USART4 },
    Vector { _handler: USART5 },
    Vector { _handler: TIM6 },
    Vector { _handler: TIM7 },
    Vector {
        _handler: DMA2_STR0,
    },
    Vector {
        _handler: DMA2_STR1,
    },
    Vector {
        _handler: DMA2_STR2,
    },
    Vector {
        _handler: DMA2_STR3,
    },
    Vector {
        _handler: DMA2_STR4,
    },
    Vector { _handler: ETH1 },
    Vector {
        _handler: ETH1_WKUP,
    },
    Vector { _reserved: 0 },
    Vector { _handler: EXTI6 },
    Vector { _handler: EXTI7 },
    Vector { _handler: EXTI8 },
    Vector { _handler: EXTI9 },
    Vector {
        _handler: DMA2_STR5,
    },
    Vector {
        _handler: DMA2_STR6,
    },
    Vector {
        _handler: DMA2_STR7,
    },
    Vector { _handler: USART6 },
    Vector { _handler: I2C3_EVT },
    Vector { _handler: I2C3_ERR },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: EXTI12 },
    Vector { _handler: EXTI13 },
    Vector { _handler: DCMI },
    Vector { _handler: CRYP1 },
    Vector { _handler: HASH1 },
    Vector { _reserved: 0 },
    Vector { _handler: USART7 },
    Vector { _handler: USART8 },
    Vector { _handler: SPI4 },
    Vector { _handler: SPI5 },
    Vector { _handler: SPI6 },
    Vector { _handler: SAI1 },
    Vector { _handler: LTDC },
    Vector { _handler: LTDC_ER },
    Vector { _handler: ADC2 },
    Vector { _handler: SAI2 },
    Vector { _handler: QUADSPI },
    Vector { _handler: LPTIM1 },
    Vector { _handler: CEC },
    Vector { _handler: I2C4_EVT },
    Vector { _handler: I2C4_ERR },
    Vector { _handler: SPDIFRX },
    Vector { _handler: OTG },
    Vector { _reserved: 0 },
    Vector { _handler: IPCC_RX0 },
    Vector { _handler: IPCC_TX0 },
    Vector {
        _handler: DMAMUX1_OVR_REQ,
    },
    Vector { _handler: IPCC_RX1 },
    Vector { _handler: IPCC_TX1 },
    Vector { _handler: CRYP2 },
    Vector { _handler: HASH2 },
    Vector { _handler: I2C5_EVT },
    Vector { _handler: I2C5_ERR },
    Vector { _reserved: 0 },
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
    Vector { _handler: SAI3 },
    Vector {
        _handler: DFSDM1_FLT4,
    },
    Vector { _handler: TIM15 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector { _handler: TIM12 },
    Vector { _handler: MDIOS },
    Vector { _handler: EXTI14 },
    Vector { _handler: MDMA },
    Vector { _reserved: 0 },
    Vector { _handler: SDMMC2 },
    Vector { _handler: HSEM_IT2 },
    Vector {
        _handler: DFSDM1_FLT5,
    },
    Vector { _handler: EXTI15 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM13 },
    Vector { _handler: TIM14 },
    Vector { _handler: DAC },
    Vector { _handler: RNG1 },
    Vector { _handler: RNG2 },
    Vector { _handler: I2C6_EVT },
    Vector { _handler: I2C6_ERR },
    Vector { _handler: SDMMC3 },
    Vector { _handler: LPTIM2 },
    Vector { _handler: LPTIM3 },
    Vector { _handler: LPTIM4 },
    Vector { _handler: LPTIM5 },
    Vector { _handler: ETH1_LPI },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: RCC_WAKEUP,
    },
    Vector { _handler: SAI4 },
    Vector { _handler: DTS },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: IWDG1_IT },
    Vector { _handler: IWDG2_IT },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TAMP_S },
    Vector {
        _handler: RTC_WKUP_ALARM_S,
    },
    Vector { _handler: RTC_TS_S },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: DDRPERFM },
];

/// Available interrupts for this device
#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window Watchdog interrupt
    WWDG1_IT = 0,
    /// 1: PVD AND AVD detector through EXTI
    PVD_AVD = 1,
    /// 2: Tamper interrupt (include LSECSS interrupts)
    TAMP = 2,
    /// 3: RTC Tamper or TimeStamp
    RTC_WKUP_ALARM = 3,
    /// 4: TrustZone DDR address space controller
    TZC_IT = 4,
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
    /// 11: DMA1 stream0 global interrupt
    DMA1_STR0 = 11,
    /// 12: DMA1 stream1 global interrupt
    DMA1_STR1 = 12,
    /// 13: DMA1 stream2 global interrupt
    DMA1_STR2 = 13,
    /// 14: DMA1 stream3 global interrupt
    DMA1_STR3 = 14,
    /// 15: DMA1 stream4 global interrupt
    DMA1_STR4 = 15,
    /// 16: DMA1 stream5 global interrupt
    DMA1_STR5 = 16,
    /// 17: DMA1 stream6 global interrupt
    DMA1_STR6 = 17,
    /// 18: ADC1 global interrupt
    ADC1 = 18,
    /// 19: FDCAN1 interrupt 0
    FDCAN1_IT0 = 19,
    /// 20: FDCAN2 interrupt 0
    FDCAN2_IT0 = 20,
    /// 21: FDCAN1 interrupt 1
    FDCAN1_IT1 = 21,
    /// 22: FDCAN2 interrupt 1
    FDCAN2_IT1 = 22,
    /// 23: EXTI line 5 interrupt
    EXTI5 = 23,
    /// 24: TIM1 break interrupt
    TIM1_BRK = 24,
    /// 25: TIM1 update interrupt
    TIM1_UP = 25,
    /// 26: TIM1 trigger and commutation interrupt
    TIM1_TRG_COM = 26,
    /// 27: TIM1 capture compare interrupt
    TIM1_CC = 27,
    /// 28: TIM2 global interrupt
    TIM2 = 28,
    /// 29: TIM3 global interrupt
    TIM3 = 29,
    /// 30: TIM4 global interrupt
    TIM4 = 30,
    /// 31: I2C1 event interrupt
    I2C1_EVT = 31,
    /// 32: I2C1 global error interrupt
    I2C1_ERR = 32,
    /// 33: I2C2 event interrupt
    I2C2_EVT = 33,
    /// 34: I2C2 global error interrupt
    I2C2_ERR = 34,
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
    /// 40: EXTI line 10 interrupt
    EXTI10 = 40,
    /// 41: RTC timestamp interrupt
    RTC_TS = 41,
    /// 42: EXTI line 11 interrupt
    EXTI11 = 42,
    /// 43: TIM8 break interrupt
    TIM8_BRK = 43,
    /// 44: TIM8 update interrupt
    TIM8_UP = 44,
    /// 45: TIM8 trigger and commutation interrupt
    TIM8_TRG_COM = 45,
    /// 46: TIM8 capture compare interrupt
    TIM8_CC = 46,
    /// 47: DMA1 stream7 global interrupt
    DMA1_STR7 = 47,
    /// 48: FMC global interrupt
    FMC = 48,
    /// 49: SDMMC1 global interrupt
    SDMMC1 = 49,
    /// 50: TIM5 global interrupt
    TIM5 = 50,
    /// 51: SPI3 global interrupt
    SPI3 = 51,
    /// 52: USART4 global interrupt
    USART4 = 52,
    /// 53: USART5 global interrupt
    USART5 = 53,
    /// 54: TIM6 global interrupt
    TIM6 = 54,
    /// 55: TIM7 global interrupt
    TIM7 = 55,
    /// 56: DMA2 stream0 global interrupt
    DMA2_STR0 = 56,
    /// 57: DMA2 stream1 global interrupt
    DMA2_STR1 = 57,
    /// 58: DMA2 stream2 global interrupt
    DMA2_STR2 = 58,
    /// 59: DMA2 stream3 global interrupt
    DMA2_STR3 = 59,
    /// 60: DMA2 stream4 global interrupt
    DMA2_STR4 = 60,
    /// 61: ETH1 global interrupt
    ETH1 = 61,
    /// 62: ETH1 wakeup interrupt
    ETH1_WKUP = 62,
    /// 64: EXTI line 6 interrupt
    EXTI6 = 64,
    /// 65: EXTI line 7 interrupt
    EXTI7 = 65,
    /// 66: EXTI line 8 interrupt
    EXTI8 = 66,
    /// 67: EXTI line 9 interrupt
    EXTI9 = 67,
    /// 68: DMA2 stream5 global interrupt
    DMA2_STR5 = 68,
    /// 69: DMA2 stream6 global interrupt
    DMA2_STR6 = 69,
    /// 70: DMA2 stream7 global interrupt
    DMA2_STR7 = 70,
    /// 71: USART6 global interrupt
    USART6 = 71,
    /// 72: I2C3 event interrupt
    I2C3_EVT = 72,
    /// 73: I2C3 global error interrupt
    I2C3_ERR = 73,
    /// 76: EXTI line 12 interrupt
    EXTI12 = 76,
    /// 77: EXTI line 13 interrupt
    EXTI13 = 77,
    /// 78: DCMI global interrupt
    DCMI = 78,
    /// 79: CRYP1 global interrupt
    CRYP1 = 79,
    /// 80: HASH1 interrupt
    HASH1 = 80,
    /// 82: USART7 global interrupt
    USART7 = 82,
    /// 83: USART8 global interrupt
    USART8 = 83,
    /// 84: SPI4 global interrupt
    SPI4 = 84,
    /// 85: SPI5 global interrupt
    SPI5 = 85,
    /// 86: SPI6 global interrupt
    SPI6 = 86,
    /// 87: SAI1 global interrupt
    SAI1 = 87,
    /// 88: LTCD global interrupt
    LTDC = 88,
    /// 89: LTCD global error interrupt
    LTDC_ER = 89,
    /// 90: ADC2 global interrupt
    ADC2 = 90,
    /// 91: SAI2 global interrupt
    SAI2 = 91,
    /// 92: QUADSPI global interrupt
    QUADSPI = 92,
    /// 93: LPTIMER1 global interrupt
    LPTIM1 = 93,
    /// 94: HDMI-CEC global interrupt
    CEC = 94,
    /// 95: I2C4 event interrupt
    I2C4_EVT = 95,
    /// 96: I2C4 global error interrupt
    I2C4_ERR = 96,
    /// 97: SPDIFRX global interrupt
    SPDIFRX = 97,
    /// 98: USB On-The-Go global interrupt
    OTG = 98,
    /// 100: IPCC RX0 occupied interrupt
    IPCC_RX0 = 100,
    /// 101: IPCC TX0 free interrupt
    IPCC_TX0 = 101,
    /// 102: DMAMUX1 overrun interrupt
    DMAMUX1_OVR_REQ = 102,
    /// 103: IPCC RX1 occupied interrupt
    IPCC_RX1 = 103,
    /// 104: IPCC TX1 free interrupt
    IPCC_TX1 = 104,
    /// 105: CRYP2 global interrupt
    CRYP2 = 105,
    /// 106: HASH2 interrupt
    HASH2 = 106,
    /// 107: I2C5 event interrupt
    I2C5_EVT = 107,
    /// 108: I2C5 global error interrupt
    I2C5_ERR = 108,
    /// 110: DFSDM1 filter0 Interrupt
    DFSDM1_FLT0 = 110,
    /// 111: DFSDM1 filter1 Interrupt
    DFSDM1_FLT1 = 111,
    /// 112: DFSDM1 filter2 Interrupt
    DFSDM1_FLT2 = 112,
    /// 113: DFSDM1 filter3 Interrupt
    DFSDM1_FLT3 = 113,
    /// 114: SAI3 global interrupt
    SAI3 = 114,
    /// 115: DFSDM1 filter4 Interrupt
    DFSDM1_FLT4 = 115,
    /// 116: TIM15 global interrupt
    TIM15 = 116,
    /// 117: TIM16 global interrupt
    TIM16 = 117,
    /// 118: TIM17 global interrupt
    TIM17 = 118,
    /// 119: TIM12 gloabl interrupt
    TIM12 = 119,
    /// 120: MDIOS global interrupt
    MDIOS = 120,
    /// 121: EXTI line 14 interrupt
    EXTI14 = 121,
    /// 122: MDMA global interrupt
    MDMA = 122,
    /// 124: SDMMC2 global interrupt
    SDMMC2 = 124,
    /// 125: HSEM semaphore interrupt 2
    HSEM_IT2 = 125,
    /// 126: DFSDM1 filter5 Interrupt
    DFSDM1_FLT5 = 126,
    /// 127: EXTI line 15 interrupt
    EXTI15 = 127,
    /// 130: TIM13 global interrupt
    TIM13 = 130,
    /// 131: TIM14 global interrupt
    TIM14 = 131,
    /// 132: DAC1 and DAC2 underrun error interrupts
    DAC = 132,
    /// 133: RNG1 interrupt
    RNG1 = 133,
    /// 134: RNG2 interrupt
    RNG2 = 134,
    /// 135: I2C6 event interrupt
    I2C6_EVT = 135,
    /// 136: I2C6 global error interrupt
    I2C6_ERR = 136,
    /// 137: SDMMC3 global interrupt
    SDMMC3 = 137,
    /// 138: LPTIMER2 global interrupt
    LPTIM2 = 138,
    /// 139: LPTIMER3 global interrupt
    LPTIM3 = 139,
    /// 140: LPTIMER4 global interrupt
    LPTIM4 = 140,
    /// 141: LPTIMER5 global interrupt
    LPTIM5 = 141,
    /// 142: ETH1 LPI interrupt
    ETH1_LPI = 142,
    /// 145: RCC MPU wakeup interrupt
    RCC_WAKEUP = 145,
    /// 146: SAI4 global interrupt
    SAI4 = 146,
    /// 147: Digital temperature sensor interrupt
    DTS = 147,
    /// 150: IWDG1 early wake
    IWDG1_IT = 150,
    /// 151: IWDG2 early wake
    IWDG2_IT = 151,
    /// 197: TAMP tamper secure interrupt
    TAMP_S = 197,
    /// 198: RTC wakeup timer and alarms (A and B) secure interrupt
    RTC_WKUP_ALARM_S = 198,
    /// 199: RTC timestamp secure interrupt
    RTC_TS_S = 199,
    /// 213: DDR performance monitor interrupt
    DDRPERFM = 213,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
