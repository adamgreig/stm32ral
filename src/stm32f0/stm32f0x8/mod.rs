//! stm32ral module for stm32f0x8

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 2;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::adc;
pub use super::instances::crc;
pub use super::instances::dma;
pub use super::instances::exti;
pub use super::instances::gpio;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::nvic;
pub use super::instances::pwr;
pub use super::instances::rcc;
pub use super::instances::spi;
pub use super::instances::syscfg;
pub use super::instances::tim1;
pub use super::instances::tim14;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub use super::instances::usart;
pub use super::instances::wwdg;
pub mod rtc;
pub use super::instances::can;
pub use super::instances::cec;
pub use super::instances::comp;
pub use super::instances::crs;
pub use super::instances::dac;
pub use super::instances::dbgmcu;
pub use super::instances::flash;
pub use super::instances::scb;
pub use super::instances::stk;
pub use super::instances::tim15;
pub use super::instances::tim16;
pub use super::instances::tim17;
pub use super::instances::tsc;
pub use super::instances::usb;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub CRC: crc::Instance,
    pub GPIOF: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub GPIOA: gpio::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub PWR: pwr::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub TIM1: tim1::Instance,
    pub TIM2: tim2::Instance,
    pub TIM3: tim3::Instance,
    pub TIM14: tim14::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub EXTI: exti::Instance,
    pub NVIC: nvic::Instance,
    pub DMA1: dma::Instance,
    pub DMA2: dma::Instance,
    pub RCC: rcc::Instance,
    pub SYSCFG: syscfg::Instance,
    pub ADC: adc::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub USART4: usart::Instance,
    pub USART6: usart::Instance,
    pub USART7: usart::Instance,
    pub USART8: usart::Instance,
    pub USART5: usart::Instance,
    pub RTC: rtc::Instance,
    pub TIM15: tim15::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub TSC: tsc::Instance,
    pub CEC: cec::Instance,
    pub Flash: flash::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub USB: usb::Instance,
    pub CRS: crs::Instance,
    pub CAN: can::Instance,
    pub DAC: dac::Instance,
    pub SCB: scb::Instance,
    pub STK: stk::Instance,
    pub COMP: comp::Instance,
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
            GPIOE: gpio::GPIOE::steal(),
            GPIOA: gpio::GPIOA::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            PWR: pwr::PWR::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            EXTI: exti::EXTI::steal(),
            NVIC: nvic::NVIC::steal(),
            DMA1: dma::DMA1::steal(),
            DMA2: dma::DMA2::steal(),
            RCC: rcc::RCC::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            ADC: adc::ADC::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            USART4: usart::USART4::steal(),
            USART6: usart::USART6::steal(),
            USART7: usart::USART7::steal(),
            USART8: usart::USART8::steal(),
            USART5: usart::USART5::steal(),
            RTC: rtc::RTC::steal(),
            TIM15: tim15::TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            TSC: tsc::TSC::steal(),
            CEC: cec::CEC::steal(),
            Flash: flash::Flash::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            USB: usb::USB::steal(),
            CRS: crs::CRS::steal(),
            CAN: can::CAN::steal(),
            DAC: dac::DAC::steal(),
            SCB: scb::SCB::steal(),
            STK: stk::STK::steal(),
            COMP: comp::COMP::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
