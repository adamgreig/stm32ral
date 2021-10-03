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
    fn TIM3_IRQ();
    fn I2C1_EV_EXTI23();
    fn I2C1_ER();
    fn SPI1();
    fn USART1_EXTI25();
    fn USART2_EXTI26();
    fn USART3_EXTI28();
    fn EXTI15_10();
    fn RTCAlarm();
    fn TIM6_DAC1();
    fn TIM7_DAC2();
    fn COMP1_2_3();
    fn COMP4_5_6();
    fn HRTIM_MST();
    fn HRTIM_TIMA();
    fn HRTIM_TIMB();
    fn HRTIM_TIMC();
    fn HRTIM_TIMD();
    fn HRTIM_TIME();
    fn HRTIM_FLT();
    fn FPU();
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
pub static __INTERRUPTS: [Vector; 82] = [
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
    Vector { _handler: TIM3_IRQ },
    Vector { _reserved: 0 },
    Vector {
        _handler: I2C1_EV_EXTI23,
    },
    Vector { _handler: I2C1_ER },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI1 },
    Vector { _reserved: 0 },
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
    Vector {
        _handler: TIM6_DAC1,
    },
    Vector {
        _handler: TIM7_DAC2,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: COMP1_2_3,
    },
    Vector {
        _handler: COMP4_5_6,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: HRTIM_MST,
    },
    Vector {
        _handler: HRTIM_TIMA,
    },
    Vector {
        _handler: HRTIM_TIMB,
    },
    Vector {
        _handler: HRTIM_TIMC,
    },
    Vector {
        _handler: HRTIM_TIMD,
    },
    Vector {
        _handler: HRTIM_TIME,
    },
    Vector {
        _handler: HRTIM_FLT,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FPU },
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
    /// 29: Timer 3 global interrupt
    TIM3_IRQ = 29,
    /// 31: I2C1 event interrupt and EXTI Line23 interrupt
    I2C1_EV_EXTI23 = 31,
    /// 32: I2C1 error interrupt
    I2C1_ER = 32,
    /// 35: SPI1 global interrupt
    SPI1 = 35,
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
    /// 54: TIM6 global and DAC12 underrun interrupts
    TIM6_DAC1 = 54,
    /// 55: TIM7 global interrupt
    TIM7_DAC2 = 55,
    /// 64: COMP1_2_3 interrupt combined with EXTI lines 21, 22, 29
    COMP1_2_3 = 64,
    /// 65: COMP4_5_6 interrupt combined with EXTI lines 30, 31, 32
    COMP4_5_6 = 65,
    /// 67: HRTIM1 master timer interrupt
    HRTIM_MST = 67,
    /// 68: HRTIM1 timer A interrupt
    HRTIM_TIMA = 68,
    /// 69: HRTIM1 timer B interrupt
    HRTIM_TIMB = 69,
    /// 70: HRTIM1 timer C interrupt
    HRTIM_TIMC = 70,
    /// 71: HRTIM1 timer D interrupt
    HRTIM_TIMD = 71,
    /// 72: HRTIM1 timer E interrupt
    HRTIM_TIME = 72,
    /// 73: HRTIM1 fault interrupt
    HRTIM_FLT = 73,
    /// 81: Floating point unit
    FPU = 81,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
