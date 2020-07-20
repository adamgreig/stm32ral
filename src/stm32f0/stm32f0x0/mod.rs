//! stm32ral module for stm32f0x0

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 2;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::crc;
pub mod gpio;
pub mod pwr;
pub mod spi;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::wwdg;
pub mod tim1;
pub use super::instances::dma1;
pub use super::instances::exti;
pub use super::instances::nvic;
pub use super::instances::tim14;
pub use super::instances::tim3;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub mod adc;
pub mod rcc;
pub mod syscfg;
pub mod usart;
pub use super::instances::rtc;
pub use super::instances::tim15;
pub use super::instances::tim16;
pub use super::instances::tim17;
pub mod dbgmcu;
pub mod flash;
pub use super::instances::scb;
pub use super::instances::stk;
pub use super::instances::usb;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub CRC: crc::Instance,
    pub GPIOF: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOA: gpio::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub PWR: pwr::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub TIM1: tim1::Instance,
    pub TIM3: tim3::Instance,
    pub TIM14: tim14::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub EXTI: exti::Instance,
    pub NVIC: nvic::Instance,
    pub DMA1: dma1::Instance,
    pub RCC: rcc::Instance,
    pub SYSCFG: syscfg::Instance,
    pub ADC: adc::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub USART4: usart::Instance,
    pub USART6: usart::Instance,
    pub USART5: usart::Instance,
    pub RTC: rtc::Instance,
    pub TIM15: tim15::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub Flash: flash::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub USB: usb::Instance,
    pub SCB: scb::Instance,
    pub STK: stk::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            CRC: crc::CRC::steal(),
            GPIOF: gpio::GPIOF::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOA: gpio::GPIOA::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            PWR: pwr::PWR::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            EXTI: exti::EXTI::steal(),
            NVIC: nvic::NVIC::steal(),
            DMA1: dma1::DMA1::steal(),
            RCC: rcc::RCC::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            ADC: adc::ADC::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            USART4: usart::USART4::steal(),
            USART6: usart::USART6::steal(),
            USART5: usart::USART5::steal(),
            RTC: rtc::RTC::steal(),
            TIM15: tim15::TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            Flash: flash::Flash::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            USB: usb::USB::steal(),
            SCB: scb::SCB::steal(),
            STK: stk::STK::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
