//! stm32ral module for stm32f100

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::fsmc_f100_f102 as fsmc;
pub use super::instances::pwr;
pub mod rcc;
pub use super::instances::gpio;
pub mod afio;
pub mod exti;
pub use super::instances::dma;
pub mod rtc;
pub use super::instances::bkp;
pub use super::instances::iwdg;
pub use super::instances::tim1_f100_f101_f102 as tim1;
pub use super::instances::wwdg;
pub mod tim12;
pub mod tim2;
pub mod tim3;
pub mod tim4;
pub mod tim5;
pub use super::instances::i2c;
pub use super::instances::tim13_f100_f102_f107 as tim13;
pub use super::instances::tim14_f100_f102_f107 as tim14;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub mod spi;
pub use super::instances::usart;
pub mod adc1;
pub use super::instances::crc;
pub use super::instances::dac_f100_f102 as dac;
pub use super::instances::dbgmcu;
pub use super::instances::uart;
pub mod cec;
pub mod flash;
pub mod tim15;
pub mod tim16;
pub mod tim17;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub FSMC: fsmc::Instance,
    pub PWR: pwr::Instance,
    pub RCC: rcc::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub GPIOG: gpio::Instance,
    pub AFIO: afio::Instance,
    pub EXTI: exti::Instance,
    pub DMA1: dma::Instance,
    pub DMA2: dma::Instance,
    pub RTC: rtc::Instance,
    pub BKP: bkp::Instance,
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub TIM1: tim1::Instance,
    pub TIM2: tim2::Instance,
    pub TIM3: tim3::Instance,
    pub TIM4: tim4::Instance,
    pub TIM5: tim5::Instance,
    pub TIM12: tim12::Instance,
    pub TIM13: tim13::Instance,
    pub TIM14: tim14::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub ADC1: adc1::Instance,
    pub DAC: dac::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub UART4: uart::Instance,
    pub UART5: uart::Instance,
    pub CRC: crc::Instance,
    pub FLASH: flash::Instance,
    pub TIM15: tim15::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub CEC: cec::Instance,
    pub NVIC: nvic::Instance,
    pub MPU: mpu::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
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
            FSMC: fsmc::FSMC::steal(),
            PWR: pwr::PWR::steal(),
            RCC: rcc::RCC::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOE: gpio::GPIOE::steal(),
            GPIOF: gpio::GPIOF::steal(),
            GPIOG: gpio::GPIOG::steal(),
            AFIO: afio::AFIO::steal(),
            EXTI: exti::EXTI::steal(),
            DMA1: dma::DMA1::steal(),
            DMA2: dma::DMA2::steal(),
            RTC: rtc::RTC::steal(),
            BKP: bkp::BKP::steal(),
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM4: tim4::TIM4::steal(),
            TIM5: tim5::TIM5::steal(),
            TIM12: tim12::TIM12::steal(),
            TIM13: tim13::TIM13::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            ADC1: adc1::ADC1::steal(),
            DAC: dac::DAC::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            UART4: uart::UART4::steal(),
            UART5: uart::UART5::steal(),
            CRC: crc::CRC::steal(),
            FLASH: flash::FLASH::steal(),
            TIM15: tim15::TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            CEC: cec::CEC::steal(),
            NVIC: nvic::NVIC::steal(),
            MPU: mpu::MPU::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
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
