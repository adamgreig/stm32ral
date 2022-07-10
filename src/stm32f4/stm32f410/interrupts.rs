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
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_Stream0();
    fn DMA1_Stream1();
    fn DMA1_Stream2();
    fn DMA1_Stream3();
    fn DMA1_Stream4();
    fn DMA1_Stream5();
    fn DMA1_Stream6();
    fn ADC();
    fn EXTI9_5();
    fn TIM1_BRK_TIM9();
    fn PWM1_UP();
    fn TIM1_TRG_COM_TIM11();
    fn TIM1_CC();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn EXTI15_10();
    fn RTC_Alarm();
    fn DMA1_Stream7();
    fn TIM5();
    fn TIM6_DAC1();
    fn DMA2_Stream0();
    fn DMA2_Stream1();
    fn DMA2_Stream2();
    fn DMA2_Stream3();
    fn DMA2_Stream4();
    fn EXTI19();
    fn DMA2_Stream5();
    fn DMA2_Stream6();
    fn DMA2_Stream7();
    fn USART6();
    fn EXTI20();
    fn RNG();
    fn FPU();
    fn SPI5();
    fn I2C4_EV();
    fn I2C4_ER();
    fn LPTIM1();
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
pub static __INTERRUPTS: [Vector; 98] = [
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
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector {
        _handler: DMA1_Stream0,
    },
    Vector {
        _handler: DMA1_Stream1,
    },
    Vector {
        _handler: DMA1_Stream2,
    },
    Vector {
        _handler: DMA1_Stream3,
    },
    Vector {
        _handler: DMA1_Stream4,
    },
    Vector {
        _handler: DMA1_Stream5,
    },
    Vector {
        _handler: DMA1_Stream6,
    },
    Vector { _handler: ADC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: EXTI9_5 },
    Vector {
        _handler: TIM1_BRK_TIM9,
    },
    Vector { _handler: PWM1_UP },
    Vector {
        _handler: TIM1_TRG_COM_TIM11,
    },
    Vector { _handler: TIM1_CC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _reserved: 0 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_Alarm,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMA1_Stream7,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM5 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: TIM6_DAC1,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMA2_Stream0,
    },
    Vector {
        _handler: DMA2_Stream1,
    },
    Vector {
        _handler: DMA2_Stream2,
    },
    Vector {
        _handler: DMA2_Stream3,
    },
    Vector {
        _handler: DMA2_Stream4,
    },
    Vector { _reserved: 0 },
    Vector { _handler: EXTI19 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMA2_Stream5,
    },
    Vector {
        _handler: DMA2_Stream6,
    },
    Vector {
        _handler: DMA2_Stream7,
    },
    Vector { _handler: USART6 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: EXTI20 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: RNG },
    Vector { _handler: FPU },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI5 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C4_EV },
    Vector { _handler: I2C4_ER },
    Vector { _handler: LPTIM1 },
];

/// Available interrupts for this device
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window watchdog interrupt
    WWDG = 0,
    /// 1: PVD through EXTI line detection interrupt
    PVD = 1,
    /// 2: Tamper and TimeStamp interrupts through the EXTI line
    TAMP_STAMP = 2,
    /// 3: RTC Wakeup interrupt through the EXTI line
    RTC_WKUP = 3,
    /// 4: FLASH global interrupt
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
    /// 11: DMA1 stream0 global interrupt
    DMA1_Stream0 = 11,
    /// 12: DMA1 stream1 global interrupt
    DMA1_Stream1 = 12,
    /// 13: DMA1 stream2 global interrupt
    DMA1_Stream2 = 13,
    /// 14: DMA1 stream3 global interrupt
    DMA1_Stream3 = 14,
    /// 15: DMA1 stream4 global interrupt
    DMA1_Stream4 = 15,
    /// 16: DMA1 stream5 global interrupt
    DMA1_Stream5 = 16,
    /// 17: DMA1 stream6 global interrupt
    DMA1_Stream6 = 17,
    /// 18: ADC1 global interrupt
    ADC = 18,
    /// 23: EXTI Line\[9:5\] interrupts
    EXTI9_5 = 23,
    /// 24: TIM1 Break interrupt and TIM9 global interrupt
    TIM1_BRK_TIM9 = 24,
    /// 25: Timer1 Update interrupt
    PWM1_UP = 25,
    /// 26: TIM1 Trigger and Commutation interrupts and TIM11 global interrupt
    TIM1_TRG_COM_TIM11 = 26,
    /// 27: TIM1 Capture Compare interrupt
    TIM1_CC = 27,
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
    /// 40: EXTI Line\[15:10\] interrupts
    EXTI15_10 = 40,
    /// 41: RTC Alarms (A and B) through EXTI line interrupt
    RTC_Alarm = 41,
    /// 47: DMA1 stream7 global interrupt
    DMA1_Stream7 = 47,
    /// 50: TIM5 global interrupt
    TIM5 = 50,
    /// 54: Timer6 and DAC1 global interrupt
    TIM6_DAC1 = 54,
    /// 56: DMA2 stream0 global interrupt
    DMA2_Stream0 = 56,
    /// 57: DMA2 stream1 global interrupt
    DMA2_Stream1 = 57,
    /// 58: DMA2 stream2 global interrupt
    DMA2_Stream2 = 58,
    /// 59: DMA2 stream3 global interrupt
    DMA2_Stream3 = 59,
    /// 60: DMA2 stream4 global interrupt
    DMA2_Stream4 = 60,
    /// 62: EXTI Line 19 interrupt
    EXTI19 = 62,
    /// 68: DMA2 stream5 global interrupt
    DMA2_Stream5 = 68,
    /// 69: DMA2 stream6 global interrupt
    DMA2_Stream6 = 69,
    /// 70: DMA2 stream7 global interrupt
    DMA2_Stream7 = 70,
    /// 71: USART6 global interrupt
    USART6 = 71,
    /// 76: EXTI Line 20 interrupt
    EXTI20 = 76,
    /// 80: RNG global interrupt
    RNG = 80,
    /// 81: Floating point unit interrupt
    FPU = 81,
    /// 85: SPI 5 global interrupt
    SPI5 = 85,
    /// 95: I2C4 event interrupt
    I2C4_EV = 95,
    /// 96: I2C2 error interrupt
    I2C4_ER = 96,
    /// 97: LPTimer global interrupt
    LPTIM1 = 97,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
