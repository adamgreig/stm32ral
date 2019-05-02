//! stm32ral module for stm32f410

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::adc1;
pub use super::instances::adc_common_f401_f410_f411_f412_f413 as adc_common;
pub use super::instances::crc;
pub use super::instances::dbgmcu_f401_f410_f411_f412_f413 as dbgmcu;
pub use super::instances::exti;
pub mod flash;
pub use super::instances::iwdg;
pub mod pwr;
pub mod rcc;
pub use super::instances::rtc_f401_f410_f411_f412_f413 as rtc;
pub mod syscfg;
pub use super::instances::dma;
pub use super::instances::tim1;
pub use super::instances::tim11_f401_f410_f411_f412_f413_f427_f429_f446_f469 as tim11;
pub use super::instances::tim5;
pub use super::instances::tim8;
pub use super::instances::tim9;
pub use super::instances::usart_f401_f410_f411 as usart;
pub use super::instances::wwdg;
pub mod gpio;
pub mod i2c;
pub mod spi;
pub use super::instances::nvic_f401_f405_f407_f410_f411_f412_f413 as nvic;
pub use super::instances::tim6;
pub mod rng;
pub use super::instances::dac;
pub mod fmpi2c4;
pub mod lptim1;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::mpu;
pub use super::instances::nvic_stir;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub ADC_Common: adc_common::Instance,
    pub ADC1: adc1::Instance,
    pub CRC: crc::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub EXTI: exti::Instance,
    pub FLASH: flash::Instance,
    pub IWDG: iwdg::Instance,
    pub PWR: pwr::Instance,
    pub RCC: rcc::Instance,
    pub RTC: rtc::Instance,
    pub SYSCFG: syscfg::Instance,
    pub TIM1: tim1::Instance,
    pub TIM8: tim8::Instance,
    pub TIM11: tim11::Instance,
    pub TIM5: tim5::Instance,
    pub TIM9: tim9::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART6: usart::Instance,
    pub WWDG: wwdg::Instance,
    pub DMA2: dma::Instance,
    pub DMA1: dma::Instance,
    pub GPIOH: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOA: gpio::Instance,
    pub I2C2: i2c::Instance,
    pub I2C1: i2c::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI5: spi::Instance,
    pub NVIC: nvic::Instance,
    pub TIM6: tim6::Instance,
    pub RNG: rng::Instance,
    pub DAC: dac::Instance,
    pub LPTIM1: lptim1::Instance,
    pub FMPI2C4: fmpi2c4::Instance,
    pub FPU: fpu::Instance,
    pub MPU: mpu::Instance,
    pub STK: stk::Instance,
    pub SCB: scb::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub FPU_CPACR: fpu_cpacr::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
}

#[cfg(all(feature = "rtfm", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            ADC_Common: adc_common::ADC_Common::steal(),
            ADC1: adc1::ADC1::steal(),
            CRC: crc::CRC::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            EXTI: exti::EXTI::steal(),
            FLASH: flash::FLASH::steal(),
            IWDG: iwdg::IWDG::steal(),
            PWR: pwr::PWR::steal(),
            RCC: rcc::RCC::steal(),
            RTC: rtc::RTC::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM8: tim8::TIM8::steal(),
            TIM11: tim11::TIM11::steal(),
            TIM5: tim5::TIM5::steal(),
            TIM9: tim9::TIM9::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART6: usart::USART6::steal(),
            WWDG: wwdg::WWDG::steal(),
            DMA2: dma::DMA2::steal(),
            DMA1: dma::DMA1::steal(),
            GPIOH: gpio::GPIOH::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOA: gpio::GPIOA::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C1: i2c::I2C1::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI5: spi::SPI5::steal(),
            NVIC: nvic::NVIC::steal(),
            TIM6: tim6::TIM6::steal(),
            RNG: rng::RNG::steal(),
            DAC: dac::DAC::steal(),
            LPTIM1: lptim1::LPTIM1::steal(),
            FMPI2C4: fmpi2c4::FMPI2C4::steal(),
            FPU: fpu::FPU::steal(),
            MPU: mpu::MPU::steal(),
            STK: stk::STK::steal(),
            SCB: scb::SCB::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            FPU_CPACR: fpu_cpacr::FPU_CPACR::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
        }
    }
}

#[cfg(all(feature = "rtfm", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
