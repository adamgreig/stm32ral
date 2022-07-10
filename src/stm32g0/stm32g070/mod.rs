//! stm32ral module for stm32g070

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 2;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::dbg_g070_g071_g07x_g081 as dbg;
pub use super::instances::flash_g030_g031_g041_g070_g071_g081 as flash;
pub use super::instances::iwdg_g070_g071_g081 as iwdg;
pub use super::instances::wwdg;
pub mod rcc;
pub use super::instances::pwr_g070_g071_g07x_g081 as pwr;
pub mod dma;
pub use super::instances::crc;
pub use super::instances::dmamux_g070_g071_g081 as dmamux;
pub use super::instances::exti_g070_g071 as exti;
pub use super::instances::gpio_g030_g031_g041_g070_g071_g081 as gpio;
pub mod tim15;
pub use super::instances::tim16_g030_g031_g041_g070_g081 as tim16;
pub use super::instances::tim17_g030_g031_g041_g070_g081 as tim17;
pub mod usart;
pub use super::instances::spi_g070_g071_g081 as spi;
pub mod tim1;
pub use super::instances::adc_g070_g071_g081 as adc;
pub mod syscfg;
pub use super::instances::i2c_g030_g031_g041_g050_g051_g061_g070_g071_g081_g0b0 as i2c;
pub use super::instances::rtc_g070_g071_g081 as rtc;
pub use super::instances::tamp_g070_g071_g07x_g081 as tamp;
pub use super::instances::tim14_g030_g031_g041_g070_g071_g081 as tim14;
pub use super::instances::tim6_g070_g071_g081 as tim6;
pub mod tim3;
pub use super::instances::mpu;
pub use super::instances::nvic_g070_g071_g07x_g081 as nvic;
pub use super::instances::scb;
pub use super::instances::stk;
pub use super::instances::tim7_g070_g071_g081 as tim7;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub FLASH: flash::Instance,
    pub DBG: dbg::Instance,
    pub RCC: rcc::Instance,
    pub PWR: pwr::Instance,
    pub DMA: dma::Instance,
    pub DMAMUX: dmamux::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub CRC: crc::Instance,
    pub EXTI: exti::Instance,
    pub TIM15: tim15::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART4: usart::Instance,
    pub USART3: usart::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub TIM1: tim1::Instance,
    pub ADC: adc::Instance,
    pub SYSCFG: syscfg::Instance,
    pub TAMP: tamp::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub RTC: rtc::Instance,
    pub TIM14: tim14::Instance,
    pub TIM6: tim6::Instance,
    pub TIM3: tim3::Instance,
    pub NVIC: nvic::Instance,
    pub MPU: mpu::Instance,
    pub STK: stk::Instance,
    pub SCB: scb::Instance,
    pub TIM7: tim7::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            FLASH: flash::FLASH::steal(),
            DBG: dbg::DBG::steal(),
            RCC: rcc::RCC::steal(),
            PWR: pwr::PWR::steal(),
            DMA: dma::DMA::steal(),
            DMAMUX: dmamux::DMAMUX::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOF: gpio::GPIOF::steal(),
            CRC: crc::CRC::steal(),
            EXTI: exti::EXTI::steal(),
            TIM15: tim15::TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART4: usart::USART4::steal(),
            USART3: usart::USART3::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            TIM1: tim1::TIM1::steal(),
            ADC: adc::ADC::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            TAMP: tamp::TAMP::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            RTC: rtc::RTC::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM3: tim3::TIM3::steal(),
            NVIC: nvic::NVIC::steal(),
            MPU: mpu::MPU::steal(),
            STK: stk::STK::steal(),
            SCB: scb::SCB::steal(),
            TIM7: tim7::TIM7::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
