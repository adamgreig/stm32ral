//! stm32ral module for stm32l0x0

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 2;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::crc;
pub use super::instances::dma1;
pub use super::instances::gpio;
pub use super::instances::lptim;
pub use super::instances::rtc;
pub mod usart2;
pub use super::instances::fw;
pub use super::instances::iwdg;
pub use super::instances::wwdg;
pub mod i2c1;
pub mod rcc;
pub mod spi1;
pub mod syscfg;
pub use super::instances::pwr_l0x0_l0x1 as pwr;
pub mod flash;
pub use super::instances::exti;
pub mod adc;
pub use super::instances::dbg;
pub use super::instances::lpuart1;
pub use super::instances::nvic;
pub use super::instances::tim2;
pub use super::instances::tim21;
pub use super::instances::tim22;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub DMA1: dma1::Instance,
    pub CRC: crc::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOH: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub LPTIM: lptim::Instance,
    pub RTC: rtc::Instance,
    pub USART2: usart2::Instance,
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub FW: fw::Instance,
    pub RCC: rcc::Instance,
    pub SYSCFG: syscfg::Instance,
    pub SPI1: spi1::Instance,
    pub I2C1: i2c1::Instance,
    pub PWR: pwr::Instance,
    pub FLASH: flash::Instance,
    pub EXTI: exti::Instance,
    pub ADC: adc::Instance,
    pub DBG: dbg::Instance,
    pub TIM2: tim2::Instance,
    pub TIM21: tim21::Instance,
    pub TIM22: tim22::Instance,
    pub LPUART1: lpuart1::Instance,
    pub NVIC: nvic::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            DMA1: dma1::DMA1::steal(),
            CRC: crc::CRC::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOH: gpio::GPIOH::steal(),
            GPIOE: gpio::GPIOE::steal(),
            LPTIM: lptim::LPTIM::steal(),
            RTC: rtc::RTC::steal(),
            USART2: usart2::USART2::steal(),
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            FW: fw::FW::steal(),
            RCC: rcc::RCC::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            SPI1: spi1::SPI1::steal(),
            I2C1: i2c1::I2C1::steal(),
            PWR: pwr::PWR::steal(),
            FLASH: flash::FLASH::steal(),
            EXTI: exti::EXTI::steal(),
            ADC: adc::ADC::steal(),
            DBG: dbg::DBG::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM21: tim21::TIM21::steal(),
            TIM22: tim22::TIM22::steal(),
            LPUART1: lpuart1::LPUART1::steal(),
            NVIC: nvic::NVIC::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
