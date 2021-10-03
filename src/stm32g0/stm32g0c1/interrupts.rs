extern crate bare_metal;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn DMA_Channel4_5_6_7();
    fn TIM1_BRK_UP_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM6_DAC();
    fn TIM7();
    fn TIM14();
    fn TIM15();
    fn TIM16();
    fn TIM17();
    fn I2C1();
    fn I2C2();
    fn USART1();
    fn USART2();
    fn USART3_USART4_USART5_USART6_LPUART1();
    fn CEC();
    fn AES_RNG();
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
    Vector { _handler: WWDG },
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
        _handler: DMA_Channel4_5_6_7,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: TIM1_BRK_UP_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM6_DAC },
    Vector { _handler: TIM7 },
    Vector { _handler: TIM14 },
    Vector { _handler: TIM15 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector { _handler: I2C1 },
    Vector { _handler: I2C2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector {
        _handler: USART3_USART4_USART5_USART6_LPUART1,
    },
    Vector { _handler: CEC },
    Vector { _handler: AES_RNG },
];

/// Available interrupts for this device
#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window watchdog interrupt
    WWDG = 0,
    /// 11: DMA channel 4, 5, 6 & 7 and DMAMUX
    DMA_Channel4_5_6_7 = 11,
    /// 13: TIM1 break, update, trigger and commutation interrupts
    TIM1_BRK_UP_TRG_COM = 13,
    /// 14: TIM1 Capture Compare interrupt
    TIM1_CC = 14,
    /// 15: TIM2 global interrupt
    TIM2 = 15,
    /// 16: TIM3 global interrupt
    TIM3 = 16,
    /// 17: TIM6 + LPTIM1 and DAC global interrupt
    TIM6_DAC = 17,
    /// 18: TIM7 + LPTIM2 global interrupt
    TIM7 = 18,
    /// 19: TIM14 global interrupt
    TIM14 = 19,
    /// 20: Timer 15 global interrupt
    TIM15 = 20,
    /// 21: TIM16 global interrupt
    TIM16 = 21,
    /// 22: TIM17 global interrupt
    TIM17 = 22,
    /// 23: I2C1 global interrupt
    I2C1 = 23,
    /// 24: I2C2 global interrupt
    I2C2 = 24,
    /// 27: USART1 global interrupt
    USART1 = 27,
    /// 28: USART2 global interrupt
    USART2 = 28,
    /// 29: USART3,4,5,6 and LPUART1 global interrupt
    USART3_USART4_USART5_USART6_LPUART1 = 29,
    /// 30: CEC global interrupt
    CEC = 30,
    /// 31: AES and RNG global interrupts
    AES_RNG = 31,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
