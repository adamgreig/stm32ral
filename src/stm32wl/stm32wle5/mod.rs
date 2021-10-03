//! stm32ral module for stm32wle5

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::adc;
pub use super::instances::aes;
pub use super::instances::comp;
pub use super::instances::crc;
pub use super::instances::dac;
pub mod dbgmcu;
pub use super::instances::dma;
pub use super::instances::dmamux;
pub mod exti;
pub mod flash;
pub use super::instances::gpio;
pub use super::instances::gpioc;
pub use super::instances::gpioh;
pub mod hsem;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::lptim;
pub use super::instances::lptim1;
pub use super::instances::lpuart;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::pka;
pub mod pwr;
pub mod rcc;
pub use super::instances::rng;
pub use super::instances::rtc;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::spi;
pub use super::instances::stk;
pub mod syscfg;
pub use super::instances::tamp;
pub use super::instances::tim1;
pub use super::instances::tim16;
pub use super::instances::tim17;
pub use super::instances::tim2;
pub use super::instances::usart;
pub use super::instances::vrefbuf;
pub use super::instances::wwdg;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub ADC: adc::Instance,
    pub AES: aes::Instance,
    pub COMP: comp::Instance,
    pub CRC: crc::Instance,
    pub DAC: dac::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub DMA1: dma::Instance,
    pub DMA2: dma::Instance,
    pub DMAMUX: dmamux::Instance,
    pub EXTI: exti::Instance,
    pub FLASH: flash::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpioc::Instance,
    pub GPIOH: gpioh::Instance,
    pub HSEM: hsem::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub IWDG: iwdg::Instance,
    pub LPTIM1: lptim1::Instance,
    pub LPTIM2: lptim::Instance,
    pub LPTIM3: lptim::Instance,
    pub LPUART: lpuart::Instance,
    pub MPU: mpu::Instance,
    pub NVIC: nvic::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub PKA: pka::Instance,
    pub PWR: pwr::Instance,
    pub RCC: rcc::Instance,
    pub RNG: rng::Instance,
    pub RTC: rtc::Instance,
    pub SCB: scb::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub STK: stk::Instance,
    pub SYSCFG: syscfg::Instance,
    pub TAMP: tamp::Instance,
    pub TIM1: tim1::Instance,
    pub TIM2: tim2::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub VREFBUF: vrefbuf::Instance,
    pub WWDG: wwdg::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            ADC: adc::ADC::steal(),
            AES: aes::AES::steal(),
            COMP: comp::COMP::steal(),
            CRC: crc::CRC::steal(),
            DAC: dac::DAC::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            DMA1: dma::DMA1::steal(),
            DMA2: dma::DMA2::steal(),
            DMAMUX: dmamux::DMAMUX::steal(),
            EXTI: exti::EXTI::steal(),
            FLASH: flash::FLASH::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpioc::GPIOC::steal(),
            GPIOH: gpioh::GPIOH::steal(),
            HSEM: hsem::HSEM::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C3: i2c::I2C3::steal(),
            IWDG: iwdg::IWDG::steal(),
            LPTIM1: lptim1::LPTIM1::steal(),
            LPTIM2: lptim::LPTIM2::steal(),
            LPTIM3: lptim::LPTIM3::steal(),
            LPUART: lpuart::LPUART::steal(),
            MPU: mpu::MPU::steal(),
            NVIC: nvic::NVIC::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            PKA: pka::PKA::steal(),
            PWR: pwr::PWR::steal(),
            RCC: rcc::RCC::steal(),
            RNG: rng::RNG::steal(),
            RTC: rtc::RTC::steal(),
            SCB: scb::SCB::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            STK: stk::STK::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            TAMP: tamp::TAMP::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            VREFBUF: vrefbuf::VREFBUF::steal(),
            WWDG: wwdg::WWDG::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
