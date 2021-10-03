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
    fn TIM1_UP_TIM10();
    fn TIM1_TRG_COM_TIM11();
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
    fn EXTI15_10();
    fn RTC_Alarm();
    fn OTG_FS_WKUP();
    fn DMA1_Stream7();
    fn SDIO();
    fn TIM5();
    fn SPI3();
    fn DMA2_Stream0();
    fn DMA2_Stream1();
    fn DMA2_Stream2();
    fn DMA2_Stream3();
    fn DMA2_Stream4();
    fn OTG_FS();
    fn DMA2_Stream5();
    fn DMA2_Stream6();
    fn DMA2_Stream7();
    fn USART6();
    fn I2C3_EV();
    fn I2C3_ER();
    fn FPU();
    fn SPI4();
    fn SPI5();
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
pub static __INTERRUPTS: [Vector; 86] = [
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
    Vector {
        _handler: TIM1_UP_TIM10,
    },
    Vector {
        _handler: TIM1_TRG_COM_TIM11,
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
    Vector { _reserved: 0 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_Alarm,
    },
    Vector {
        _handler: OTG_FS_WKUP,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMA1_Stream7,
    },
    Vector { _reserved: 0 },
    Vector { _handler: SDIO },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
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
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: OTG_FS },
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
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FPU },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI4 },
    Vector { _handler: SPI5 },
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
    /// 25: TIM1 Update interrupt and TIM10 global interrupt
    TIM1_UP_TIM10 = 25,
    /// 26: TIM1 Trigger and Commutation interrupts and TIM11 global interrupt
    TIM1_TRG_COM_TIM11 = 26,
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
    /// 40: EXTI Line\[15:10\] interrupts
    EXTI15_10 = 40,
    /// 41: RTC Alarms (A and B) through EXTI line interrupt
    RTC_Alarm = 41,
    /// 42: USB On-The-Go FS Wakeup through EXTI line interrupt
    OTG_FS_WKUP = 42,
    /// 47: DMA1 stream7 global interrupt
    DMA1_Stream7 = 47,
    /// 49: SDIO global interrupt
    SDIO = 49,
    /// 50: TIM5 global interrupt
    TIM5 = 50,
    /// 51: SPI3 global interrupt
    SPI3 = 51,
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
    /// 67: USB On The Go FS global interrupt
    OTG_FS = 67,
    /// 68: DMA2 stream5 global interrupt
    DMA2_Stream5 = 68,
    /// 69: DMA2 stream6 global interrupt
    DMA2_Stream6 = 69,
    /// 70: DMA2 stream7 global interrupt
    DMA2_Stream7 = 70,
    /// 71: USART6 global interrupt
    USART6 = 71,
    /// 72: I2C3 event interrupt
    I2C3_EV = 72,
    /// 73: I2C3 error interrupt
    I2C3_ER = 73,
    /// 81: FPU interrupt
    FPU = 81,
    /// 84: SPI4 global interrupt
    SPI4 = 84,
    /// 85: SPI5 global interrupt
    SPI5 = 85,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
