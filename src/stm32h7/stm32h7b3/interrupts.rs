#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD_PVM();
    fn RTC_TAMP_STAMP_CSS_LSE();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA_STR0();
    fn DMA_STR1();
    fn DMA_STR2();
    fn DMA_STR3();
    fn DMA_STR4();
    fn DMA_STR5();
    fn DMA_STR6();
    fn ADC1_2();
    fn FDCAN1_IT0();
    fn FDCAN2_IT0();
    fn FDCAN1_IT1();
    fn FDCAN2_IT1();
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
    fn RTC_ALARM();
    fn TIM8_BRK_TIM12();
    fn TIM8_UP_TIM13();
    fn TIM8_TRG_COM_TIM14();
    fn TIM8_CC();
    fn DMA1_STR7();
    fn FMC();
    fn SDMMC1();
    fn TIM5();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6_DAC();
    fn TIM7();
    fn DMA2_STR0();
    fn DMA2_STR1();
    fn DMA2_STR2();
    fn DMA2_STR3();
    fn DMA2_STR4();
    fn DMA2_STR5();
    fn DMA2_STR6();
    fn DMA2_STR7();
    fn USART6();
    fn I2C3_EV();
    fn I2C3_ER();
    fn OTG_HS_EP1_OUT();
    fn OTG_HS_EP1_IN();
    fn OTG_HS_WKUP();
    fn OTG_HS();
    fn DCMI_PSSI();
    fn CRYP();
    fn HASH_RNG();
    fn FPU();
    fn UART7();
    fn UART8();
    fn SPI4();
    fn SPI5();
    fn SPI6();
    fn SAI1();
    fn LTDC();
    fn LTDC_ER();
    fn DMA2D();
    fn SAI2();
    fn QUADSPI();
    fn LPTIM1();
    fn CEC();
    fn I2C4_EV();
    fn I2C4_ER();
    fn SPDIFRX();
    fn DMAMUX1_OV();
    fn DFSDM1_FLT0();
    fn DFSDM1_FLT1();
    fn DFSDM1_FLT2();
    fn DFSDM1_FLT3();
    fn SWPMI1();
    fn TIM15();
    fn TIM16();
    fn TIM17();
    fn MDIOS_WKUP();
    fn MDIOS();
    fn JPEG();
    fn MDMA();
    fn SDMMC();
    fn HSEM0();
    fn DAC2();
    fn DMAMUX2_OVR();
    fn BDMA_CH1();
    fn BDMA_CH2();
    fn BDMA_CH3();
    fn BDMA_CH4();
    fn BDMA_CH5();
    fn BDMA_CH6();
    fn BDMA_CH7();
    fn BDMA_CH8();
    fn COMP();
    fn LPTIM2();
    fn LPTIM3();
    fn UART9();
    fn USART10();
    fn LPUART();
    fn WWDG1_RST();
    fn CRS();
    fn RAMECC();
    fn WKUP();
    fn OCTOSPI2();
    fn OTFDEC1();
    fn OTFDEC2();
    fn BDMA1();
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
pub static __INTERRUPTS: [Vector; 155] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD_PVM },
    Vector {
        _handler: RTC_TAMP_STAMP_CSS_LSE,
    },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector { _handler: DMA_STR0 },
    Vector { _handler: DMA_STR1 },
    Vector { _handler: DMA_STR2 },
    Vector { _handler: DMA_STR3 },
    Vector { _handler: DMA_STR4 },
    Vector { _handler: DMA_STR5 },
    Vector { _handler: DMA_STR6 },
    Vector { _handler: ADC1_2 },
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
    Vector {
        _handler: RTC_ALARM,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: TIM8_BRK_TIM12,
    },
    Vector {
        _handler: TIM8_UP_TIM13,
    },
    Vector {
        _handler: TIM8_TRG_COM_TIM14,
    },
    Vector { _handler: TIM8_CC },
    Vector {
        _handler: DMA1_STR7,
    },
    Vector { _handler: FMC },
    Vector { _handler: SDMMC1 },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: TIM6_DAC },
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
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
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
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector {
        _handler: OTG_HS_EP1_OUT,
    },
    Vector {
        _handler: OTG_HS_EP1_IN,
    },
    Vector {
        _handler: OTG_HS_WKUP,
    },
    Vector { _handler: OTG_HS },
    Vector {
        _handler: DCMI_PSSI,
    },
    Vector { _handler: CRYP },
    Vector { _handler: HASH_RNG },
    Vector { _handler: FPU },
    Vector { _handler: UART7 },
    Vector { _handler: UART8 },
    Vector { _handler: SPI4 },
    Vector { _handler: SPI5 },
    Vector { _handler: SPI6 },
    Vector { _handler: SAI1 },
    Vector { _handler: LTDC },
    Vector { _handler: LTDC_ER },
    Vector { _handler: DMA2D },
    Vector { _handler: SAI2 },
    Vector { _handler: QUADSPI },
    Vector { _handler: LPTIM1 },
    Vector { _handler: CEC },
    Vector { _handler: I2C4_EV },
    Vector { _handler: I2C4_ER },
    Vector { _handler: SPDIFRX },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMAMUX1_OV,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
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
    Vector { _reserved: 0 },
    Vector { _handler: SWPMI1 },
    Vector { _handler: TIM15 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector {
        _handler: MDIOS_WKUP,
    },
    Vector { _handler: MDIOS },
    Vector { _handler: JPEG },
    Vector { _handler: MDMA },
    Vector { _reserved: 0 },
    Vector { _handler: SDMMC },
    Vector { _handler: HSEM0 },
    Vector { _reserved: 0 },
    Vector { _handler: DAC2 },
    Vector {
        _handler: DMAMUX2_OVR,
    },
    Vector { _handler: BDMA_CH1 },
    Vector { _handler: BDMA_CH2 },
    Vector { _handler: BDMA_CH3 },
    Vector { _handler: BDMA_CH4 },
    Vector { _handler: BDMA_CH5 },
    Vector { _handler: BDMA_CH6 },
    Vector { _handler: BDMA_CH7 },
    Vector { _handler: BDMA_CH8 },
    Vector { _handler: COMP },
    Vector { _handler: LPTIM2 },
    Vector { _handler: LPTIM3 },
    Vector { _handler: UART9 },
    Vector { _handler: USART10 },
    Vector { _handler: LPUART },
    Vector {
        _handler: WWDG1_RST,
    },
    Vector { _handler: CRS },
    Vector { _handler: RAMECC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: WKUP },
    Vector { _handler: OCTOSPI2 },
    Vector { _handler: OTFDEC1 },
    Vector { _handler: OTFDEC2 },
    Vector { _reserved: 0 },
    Vector { _handler: BDMA1 },
];

/// Available interrupts for this device
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window Watchdog interrupt
    WWDG = 0,
    /// 1: PVD through EXTI line
    PVD_PVM = 1,
    /// 2: RTC tamper, timestamp/CSS LSE
    RTC_TAMP_STAMP_CSS_LSE = 2,
    /// 3: RTC Wakeup interrupt through the EXTI linet
    RTC_WKUP = 3,
    /// 4: Flash memory global interrupt
    FLASH = 4,
    /// 5: RCC global interrupt
    RCC = 5,
    /// 6: EXTI Line 0 interrupt
    EXTI0 = 6,
    /// 7: EXTI Line 1 interrupt
    EXTI1 = 7,
    /// 8: EXTI Line 2 interrupt
    EXTI2 = 8,
    /// 9: EXTI Line 3interrupt
    EXTI3 = 9,
    /// 10: EXTI Line 4interrupt
    EXTI4 = 10,
    /// 11: DMA1 Stream0 global interrupt
    DMA_STR0 = 11,
    /// 12: DMA1 Stream1 global interrupt
    DMA_STR1 = 12,
    /// 13: DMA1 Stream2 global interrupt
    DMA_STR2 = 13,
    /// 14: DMA1 Stream3 global interrupt
    DMA_STR3 = 14,
    /// 15: DMA1 Stream4 global interrupt
    DMA_STR4 = 15,
    /// 16: DMA1 Stream5 global interrupt
    DMA_STR5 = 16,
    /// 17: DMA1 Stream6 global interrupt
    DMA_STR6 = 17,
    /// 18: ADC1 and ADC2 global interrupt
    ADC1_2 = 18,
    /// 19: TTCAN Interrupt 0
    FDCAN1_IT0 = 19,
    /// 20: FDCAN Interrupt 0
    FDCAN2_IT0 = 20,
    /// 21: TTCAN Interrupt 1
    FDCAN1_IT1 = 21,
    /// 22: FDCAN Interrupt 1
    FDCAN2_IT1 = 22,
    /// 23: EXTI Line\[9:5\] interrupts
    EXTI9_5 = 23,
    /// 24: TIM1 break interrupt
    TIM1_BRK = 24,
    /// 25: TIM1 update interrupt
    TIM1_UP = 25,
    /// 26: TIM1 trigger and commutation
    TIM1_TRG_COM = 26,
    /// 27: TIM1 capture / compare
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
    /// 41: RTC alarms (A and B) through EXTI Line interrupts)
    RTC_ALARM = 41,
    /// 43: TIM8 and 12 break global
    TIM8_BRK_TIM12 = 43,
    /// 44: TIM8 and 13 update global
    TIM8_UP_TIM13 = 44,
    /// 45: TIM8 and 14 trigger /commutation and global
    TIM8_TRG_COM_TIM14 = 45,
    /// 46: TIM8 capture / compare
    TIM8_CC = 46,
    /// 47: DMA1 Stream7 global interrupt
    DMA1_STR7 = 47,
    /// 48: FMC global interrupt
    FMC = 48,
    /// 49: SDMMC global interrupt
    SDMMC1 = 49,
    /// 50: TIM5 global interrupt
    TIM5 = 50,
    /// 51: SPI3 global interrupt
    SPI3 = 51,
    /// 52: UART4 global interrupt
    UART4 = 52,
    /// 53: UART5 global interrupt
    UART5 = 53,
    /// 54: TIM6 global interrupt
    TIM6_DAC = 54,
    /// 55: TIM7 global interrupt
    TIM7 = 55,
    /// 56: DMA2 Stream0 interrupt
    DMA2_STR0 = 56,
    /// 57: DMA2 Stream1 interrupt
    DMA2_STR1 = 57,
    /// 58: DMA2 Stream2 interrupt
    DMA2_STR2 = 58,
    /// 59: DMA2 Stream3 interrupt
    DMA2_STR3 = 59,
    /// 60: DMA2 Stream4 interrupt
    DMA2_STR4 = 60,
    /// 68: DMA2 Stream5 interrupt
    DMA2_STR5 = 68,
    /// 69: DMA2 Stream6 interrupt
    DMA2_STR6 = 69,
    /// 70: DMA2 Stream7 interrupt
    DMA2_STR7 = 70,
    /// 71: USART6 global interrupt
    USART6 = 71,
    /// 72: I2C3 event interrupt
    I2C3_EV = 72,
    /// 73: I2C3 error interrupt
    I2C3_ER = 73,
    /// 74: OTG_HS out global interrupt
    OTG_HS_EP1_OUT = 74,
    /// 75: OTG_HS in global interrupt
    OTG_HS_EP1_IN = 75,
    /// 76: OTG_HS wakeup interrupt
    OTG_HS_WKUP = 76,
    /// 77: OTG_HS global interrupt
    OTG_HS = 77,
    /// 78: DCMI/PSSI global interrupt
    DCMI_PSSI = 78,
    /// 79: CRYP global interrupt
    CRYP = 79,
    /// 80: HASH and RNG global interrupt
    HASH_RNG = 80,
    /// 81: Floating point unit interrupt
    FPU = 81,
    /// 82: UART7 global interrupt
    UART7 = 82,
    /// 83: UART8 global interrupt
    UART8 = 83,
    /// 84: SPI4 global interrupt
    SPI4 = 84,
    /// 85: SPI5 global interrupt
    SPI5 = 85,
    /// 86: SPI6 global interrupt
    SPI6 = 86,
    /// 87: SAI1 global interrupt
    SAI1 = 87,
    /// 88: LCD-TFT global interrupt
    LTDC = 88,
    /// 89: LCD-TFT error interrupt
    LTDC_ER = 89,
    /// 90: DMA2D global interrupt
    DMA2D = 90,
    /// 91: SAI2 global interrupt
    SAI2 = 91,
    /// 92: QuadSPI global interrupt
    QUADSPI = 92,
    /// 93: LPTIM1 global interrupt
    LPTIM1 = 93,
    /// 94: HDMI-CEC global interrupt
    CEC = 94,
    /// 95: I2C4 event interrupt
    I2C4_EV = 95,
    /// 96: I2C4 error interrupt
    I2C4_ER = 96,
    /// 97: SPDIFRX global interrupt
    SPDIFRX = 97,
    /// 102: DMAMUX1 overrun interrupt
    DMAMUX1_OV = 102,
    /// 110: DFSDM1 filter 0 interrupt
    DFSDM1_FLT0 = 110,
    /// 111: DFSDM1 filter 1 interrupt
    DFSDM1_FLT1 = 111,
    /// 112: DFSDM1 filter 2 interrupt
    DFSDM1_FLT2 = 112,
    /// 113: DFSDM1 filter 3 interrupt
    DFSDM1_FLT3 = 113,
    /// 115: SWPMI global interrupt
    SWPMI1 = 115,
    /// 116: TIM15 global interrupt
    TIM15 = 116,
    /// 117: TIM16 global interrupt
    TIM16 = 117,
    /// 118: TIM17 global interrupt
    TIM17 = 118,
    /// 119: MDIOS wakeup
    MDIOS_WKUP = 119,
    /// 120: MDIOS global interrupt
    MDIOS = 120,
    /// 121: JPEG global interrupt
    JPEG = 121,
    /// 122: MDMA
    MDMA = 122,
    /// 124: SDMMC global interrupt
    SDMMC = 124,
    /// 125: HSEM global interrupt 1
    HSEM0 = 125,
    /// 127: DAC2 underrun interrupt
    DAC2 = 127,
    /// 128: DMAMUX2 overrun interrupt
    DMAMUX2_OVR = 128,
    /// 129: BDMA channel 1 interrupt
    BDMA_CH1 = 129,
    /// 130: BDMA channel 2 interrupt
    BDMA_CH2 = 130,
    /// 131: BDMA channel 3 interrupt
    BDMA_CH3 = 131,
    /// 132: BDMA channel 4 interrupt
    BDMA_CH4 = 132,
    /// 133: BDMA channel 5 interrupt
    BDMA_CH5 = 133,
    /// 134: BDMA channel 6 interrupt
    BDMA_CH6 = 134,
    /// 135: BDMA channel 7 interrupt
    BDMA_CH7 = 135,
    /// 136: BDMA channel 8 interrupt
    BDMA_CH8 = 136,
    /// 137: COMP1 and COMP2
    COMP = 137,
    /// 138: LPTIM2 timer interrupt
    LPTIM2 = 138,
    /// 139: LPTIM2 timer interrupt
    LPTIM3 = 139,
    /// 140: UART9 global interrupt
    UART9 = 140,
    /// 141: USART10 global interrupt
    USART10 = 141,
    /// 142: LPUART global interrupt
    LPUART = 142,
    /// 143: Window Watchdog interrupt
    WWDG1_RST = 143,
    /// 144: Clock Recovery System globa
    CRS = 144,
    /// 145: ECC diagnostic global interrupt
    RAMECC = 145,
    /// 149: WKUP1 to WKUP6 pins
    WKUP = 149,
    /// 150: OCTOSPI2 global interrupt
    OCTOSPI2 = 150,
    /// 151: OTFDEC1 interrupt
    OTFDEC1 = 151,
    /// 152: OTFDEC2 interrupt
    OTFDEC2 = 152,
    /// 154: BDMA1
    BDMA1 = 154,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
