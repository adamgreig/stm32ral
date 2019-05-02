//! stm32ral module for stm32l4x5

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::aes;
pub use super::instances::comp_l4x3_l4x5_l4x6 as comp;
pub use super::instances::crc;
pub use super::instances::dac1;
pub use super::instances::dma;
pub use super::instances::firewall;
pub use super::instances::flash;
pub use super::instances::i2c_l4x3_l4x5 as i2c;
pub use super::instances::iwdg;
pub use super::instances::lcd;
pub use super::instances::pwr;
pub use super::instances::rng;
pub use super::instances::syscfg;
pub use super::instances::tsc;
pub use super::instances::wwdg;
pub mod adc;
pub mod gpio;
pub use super::instances::lptim;
pub use super::instances::sai;
pub use super::instances::tim1;
pub use super::instances::tim15;
pub use super::instances::tim16;
pub use super::instances::tim17;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim4;
pub use super::instances::tim5;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub mod usart;
pub use super::instances::can1_l4x3_l4x5 as can1;
pub use super::instances::exti;
pub use super::instances::lpuart1;
pub use super::instances::rtc;
pub use super::instances::sdmmc;
pub use super::instances::spi_l4x3_l4x5_l4x6 as spi;
pub use super::instances::vrefbuf;
pub mod swpmi1;
pub use super::instances::crs;
pub use super::instances::dfsdm;
pub use super::instances::fmc;
pub use super::instances::nvic;
pub use super::instances::opamp;
pub use super::instances::quadspi;
pub use super::instances::tim8;
pub use super::instances::usb;
pub mod adc123_common;
pub mod rcc;
pub use super::instances::dbgmcu_l4x5_l4x6 as dbgmcu;
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
    pub DAC1: dac1::Instance,
    pub DMA1: dma::Instance,
    pub DMA2: dma::Instance,
    pub CRC: crc::Instance,
    pub LCD: lcd::Instance,
    pub TSC: tsc::Instance,
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub COMP: comp::Instance,
    pub FIREWALL: firewall::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub FLASH: flash::Instance,
    pub PWR: pwr::Instance,
    pub SYSCFG: syscfg::Instance,
    pub RNG: rng::Instance,
    pub AES: aes::Instance,
    pub ADC1: adc::Instance,
    pub ADC2: adc::Instance,
    pub ADC3: adc::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub GPIOH: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub GPIOG: gpio::Instance,
    pub SAI1: sai::Instance,
    pub SAI2: sai::Instance,
    pub TIM2: tim2::Instance,
    pub TIM5: tim5::Instance,
    pub TIM4: tim4::Instance,
    pub TIM3: tim3::Instance,
    pub TIM15: tim15::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub TIM1: tim1::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub LPTIM1: lptim::Instance,
    pub LPTIM2: lptim::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub UART5: usart::Instance,
    pub UART4: usart::Instance,
    pub LPUART1: lpuart1::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub SDMMC: sdmmc::Instance,
    pub EXTI: exti::Instance,
    pub VREFBUF: vrefbuf::Instance,
    pub CAN1: can1::Instance,
    pub RTC: rtc::Instance,
    pub SWPMI1: swpmi1::Instance,
    pub OPAMP: opamp::Instance,
    pub NVIC: nvic::Instance,
    pub CRS: crs::Instance,
    pub USB: usb::Instance,
    pub QUADSPI: quadspi::Instance,
    pub FMC: fmc::Instance,
    pub DFSDM: dfsdm::Instance,
    pub TIM8: tim8::Instance,
    pub RCC: rcc::Instance,
    pub ADC123_Common: adc123_common::Instance,
    pub DBGMCU: dbgmcu::Instance,
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
            DAC1: dac1::DAC1::steal(),
            DMA1: dma::DMA1::steal(),
            DMA2: dma::DMA2::steal(),
            CRC: crc::CRC::steal(),
            LCD: lcd::LCD::steal(),
            TSC: tsc::TSC::steal(),
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            COMP: comp::COMP::steal(),
            FIREWALL: firewall::FIREWALL::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C3: i2c::I2C3::steal(),
            FLASH: flash::FLASH::steal(),
            PWR: pwr::PWR::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            RNG: rng::RNG::steal(),
            AES: aes::AES::steal(),
            ADC1: adc::ADC1::steal(),
            ADC2: adc::ADC2::steal(),
            ADC3: adc::ADC3::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOE: gpio::GPIOE::steal(),
            GPIOH: gpio::GPIOH::steal(),
            GPIOF: gpio::GPIOF::steal(),
            GPIOG: gpio::GPIOG::steal(),
            SAI1: sai::SAI1::steal(),
            SAI2: sai::SAI2::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM5: tim5::TIM5::steal(),
            TIM4: tim4::TIM4::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM15: tim15::TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            LPTIM1: lptim::LPTIM1::steal(),
            LPTIM2: lptim::LPTIM2::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            UART5: usart::UART5::steal(),
            UART4: usart::UART4::steal(),
            LPUART1: lpuart1::LPUART1::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            SDMMC: sdmmc::SDMMC::steal(),
            EXTI: exti::EXTI::steal(),
            VREFBUF: vrefbuf::VREFBUF::steal(),
            CAN1: can1::CAN1::steal(),
            RTC: rtc::RTC::steal(),
            SWPMI1: swpmi1::SWPMI1::steal(),
            OPAMP: opamp::OPAMP::steal(),
            NVIC: nvic::NVIC::steal(),
            CRS: crs::CRS::steal(),
            USB: usb::USB::steal(),
            QUADSPI: quadspi::QUADSPI::steal(),
            FMC: fmc::FMC::steal(),
            DFSDM: dfsdm::DFSDM::steal(),
            TIM8: tim8::TIM8::steal(),
            RCC: rcc::RCC::steal(),
            ADC123_Common: adc123_common::ADC123_Common::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
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
