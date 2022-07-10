//! stm32ral module for stm32g0b0

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 2;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::crc;
pub use super::instances::dbg_g0b0_g0b1_g0c1 as dbg;
pub use super::instances::dma1;
pub use super::instances::dma2;
pub use super::instances::dmamux_g051_g061_g0b0_g0b1_g0c1 as dmamux;
pub use super::instances::gpio_g050_g0b0_g0b1_g0c1 as gpio;
pub use super::instances::i2c_g030_g031_g041_g050_g051_g061_g070_g071_g081_g0b0 as i2c;
pub use super::instances::iwdg_g030_g031_g041_g050_g051_g061_g0b0_g0b1_g0c1 as iwdg;
pub use super::instances::tim14_g050_g051_g061_g0b0_g0b1_g0c1 as tim14;
pub use super::instances::tim15_g0b0_g0b1_g0c1 as tim15;
pub use super::instances::tim16_g051_g061_g0b0_g0b1_g0c1 as tim16;
pub use super::instances::tim17_g051_g061_g0b0_g0b1_g0c1 as tim17;
pub use super::instances::tim1_g051_g061_g0b0 as tim1;
pub use super::instances::tim3_g051_g0b0_g0b1_g0c1 as tim3;
pub use super::instances::tim6_g050_g051_g061_g0b0_g0b1_g0c1 as tim6;
pub use super::instances::tim7_g050_g051_g061_g0b0_g0b1_g0c1 as tim7;
pub use super::instances::usart_g0b0_g0b1_g0c1 as usart;
pub use super::instances::vrefbuf_g051_g061_g0b0_g0b1_g0c1 as vrefbuf;
pub use super::instances::wwdg;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub CRC: crc::Instance,
    pub DBG: dbg::Instance,
    pub DMAMUX: dmamux::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub IWDG: iwdg::Instance,
    pub TIM1: tim1::Instance,
    pub TIM3: tim3::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub TIM14: tim14::Instance,
    pub TIM15: tim15::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub USART4: usart::Instance,
    pub USART5: usart::Instance,
    pub USART6: usart::Instance,
    pub VREFBUF: vrefbuf::Instance,
    pub WWDG: wwdg::Instance,
    pub DMA1: dma1::Instance,
    pub DMA2: dma2::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            CRC: crc::CRC::steal(),
            DBG: dbg::DBG::steal(),
            DMAMUX: dmamux::DMAMUX::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOE: gpio::GPIOE::steal(),
            GPIOF: gpio::GPIOF::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            IWDG: iwdg::IWDG::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM15: tim15::TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            USART4: usart::USART4::steal(),
            USART5: usart::USART5::steal(),
            USART6: usart::USART6::steal(),
            VREFBUF: vrefbuf::VREFBUF::steal(),
            WWDG: wwdg::WWDG::steal(),
            DMA1: dma1::DMA1::steal(),
            DMA2: dma2::DMA2::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
