//! stm32ral module for stm32f413

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub mod aes;
pub use super::instances::adc1;
pub use super::instances::tim1_f401_f405_f407_f413_f427_f429_f446_f469 as tim1;
pub use super::instances::tim8_f401_f405_f407_f413_f427_f429_f446_f469 as tim8;
pub mod can;
pub mod tim6;
pub mod tim7;
pub use super::instances::crc;
pub use super::instances::dbgmcu_f401_f410_f411_f412_f413 as dbgmcu;
pub mod dfsdm;
pub use super::instances::dac;
pub mod dma;
pub use super::instances::exti;
pub mod fmpi2c;
pub use super::instances::flash_f401_f405_f411_f412_f413 as flash;
pub use super::instances::tim12_f412_f413 as tim12;
pub use super::instances::tim13_f412_f413_f427_f429_f446_f469 as tim13;
pub use super::instances::tim14_f412_f413_f427_f429_f446_f469 as tim14;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim4;
pub use super::instances::tim9;
pub mod gpio;
pub use super::instances::iwdg;
pub use super::instances::tim10_f401_f411_f412_f413_f427_f429_f446_f469 as tim10;
pub use super::instances::tim11_f401_f410_f411_f412_f413_f427_f429_f446_f469 as tim11;
pub use super::instances::tim5;
pub mod i2c;
pub mod lptim;
pub use super::instances::nvic_f401_f405_f407_f410_f411_f412_f413 as nvic;
pub use super::instances::pwr_f401_f411_f412_f413 as pwr;
pub use super::instances::quadspi;
pub use super::instances::rng;
pub use super::instances::rtc_f401_f410_f411_f412_f413 as rtc;
pub mod rcc;
pub use super::instances::sai;
pub use super::instances::sdio;
pub mod spi;
pub use super::instances::syscfg_f412_f413 as syscfg;
pub mod otg_fs_device;
pub mod usart;
pub use super::instances::otg_fs_global_f412_f413 as otg_fs_global;
pub use super::instances::otg_fs_host;
pub use super::instances::otg_fs_pwrclk;
pub use super::instances::wwdg;
pub mod fsmc;
pub use super::instances::adc_common_f410_f412_f413 as adc_common;
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
    pub AES: aes::Instance,
    pub TIM1: tim1::Instance,
    pub TIM8: tim8::Instance,
    pub ADC1: adc1::Instance,
    pub TIM7: tim7::Instance,
    pub TIM6: tim6::Instance,
    pub CAN1: can::Instance,
    pub CAN2: can::Instance,
    pub CAN3: can::Instance,
    pub CRC: crc::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub DFSDM2: dfsdm::Instance,
    pub DFSDM1: dfsdm::Instance,
    pub DAC: dac::Instance,
    pub DMA1: dma::Instance,
    pub DMA2: dma::Instance,
    pub EXTI: exti::Instance,
    pub FMPI2C: fmpi2c::Instance,
    pub FLASH: flash::Instance,
    pub TIM12: tim12::Instance,
    pub TIM13: tim13::Instance,
    pub TIM14: tim14::Instance,
    pub TIM9: tim9::Instance,
    pub TIM3: tim3::Instance,
    pub TIM4: tim4::Instance,
    pub TIM2: tim2::Instance,
    pub GPIOF: gpio::Instance,
    pub GPIOH: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOG: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOA: gpio::Instance,
    pub TIM10: tim10::Instance,
    pub TIM11: tim11::Instance,
    pub TIM5: tim5::Instance,
    pub IWDG: iwdg::Instance,
    pub I2C2: i2c::Instance,
    pub I2C1: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub LPTIM: lptim::Instance,
    pub NVIC: nvic::Instance,
    pub PWR: pwr::Instance,
    pub QUADSPI: quadspi::Instance,
    pub RNG: rng::Instance,
    pub RTC: rtc::Instance,
    pub RCC: rcc::Instance,
    pub SDIO: sdio::Instance,
    pub SAI: sai::Instance,
    pub SPI5: spi::Instance,
    pub SPI4: spi::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI6: spi::Instance,
    pub SPI3: spi::Instance,
    pub SYSCFG: syscfg::Instance,
    pub USART3: usart::Instance,
    pub UART4: usart::Instance,
    pub USART6: usart::Instance,
    pub USART2: usart::Instance,
    pub UART5: usart::Instance,
    pub UART7: usart::Instance,
    pub UART8: usart::Instance,
    pub UART10: usart::Instance,
    pub USART1: usart::Instance,
    pub UART9: usart::Instance,
    pub OTG_FS_DEVICE: otg_fs_device::Instance,
    pub OTG_FS_HOST: otg_fs_host::Instance,
    pub OTG_FS_PWRCLK: otg_fs_pwrclk::Instance,
    pub OTG_FS_GLOBAL: otg_fs_global::Instance,
    pub WWDG: wwdg::Instance,
    pub FSMC: fsmc::Instance,
    pub ADC_Common: adc_common::Instance,
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
            AES: aes::AES::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM8: tim8::TIM8::steal(),
            ADC1: adc1::ADC1::steal(),
            TIM7: tim7::TIM7::steal(),
            TIM6: tim6::TIM6::steal(),
            CAN1: can::CAN1::steal(),
            CAN2: can::CAN2::steal(),
            CAN3: can::CAN3::steal(),
            CRC: crc::CRC::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            DFSDM2: dfsdm::DFSDM2::steal(),
            DFSDM1: dfsdm::DFSDM1::steal(),
            DAC: dac::DAC::steal(),
            DMA1: dma::DMA1::steal(),
            DMA2: dma::DMA2::steal(),
            EXTI: exti::EXTI::steal(),
            FMPI2C: fmpi2c::FMPI2C::steal(),
            FLASH: flash::FLASH::steal(),
            TIM12: tim12::TIM12::steal(),
            TIM13: tim13::TIM13::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM9: tim9::TIM9::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM4: tim4::TIM4::steal(),
            TIM2: tim2::TIM2::steal(),
            GPIOF: gpio::GPIOF::steal(),
            GPIOH: gpio::GPIOH::steal(),
            GPIOE: gpio::GPIOE::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOG: gpio::GPIOG::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOA: gpio::GPIOA::steal(),
            TIM10: tim10::TIM10::steal(),
            TIM11: tim11::TIM11::steal(),
            TIM5: tim5::TIM5::steal(),
            IWDG: iwdg::IWDG::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C3: i2c::I2C3::steal(),
            LPTIM: lptim::LPTIM::steal(),
            NVIC: nvic::NVIC::steal(),
            PWR: pwr::PWR::steal(),
            QUADSPI: quadspi::QUADSPI::steal(),
            RNG: rng::RNG::steal(),
            RTC: rtc::RTC::steal(),
            RCC: rcc::RCC::steal(),
            SDIO: sdio::SDIO::steal(),
            SAI: sai::SAI::steal(),
            SPI5: spi::SPI5::steal(),
            SPI4: spi::SPI4::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI6: spi::SPI6::steal(),
            SPI3: spi::SPI3::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            USART3: usart::USART3::steal(),
            UART4: usart::UART4::steal(),
            USART6: usart::USART6::steal(),
            USART2: usart::USART2::steal(),
            UART5: usart::UART5::steal(),
            UART7: usart::UART7::steal(),
            UART8: usart::UART8::steal(),
            UART10: usart::UART10::steal(),
            USART1: usart::USART1::steal(),
            UART9: usart::UART9::steal(),
            OTG_FS_DEVICE: otg_fs_device::OTG_FS_DEVICE::steal(),
            OTG_FS_HOST: otg_fs_host::OTG_FS_HOST::steal(),
            OTG_FS_PWRCLK: otg_fs_pwrclk::OTG_FS_PWRCLK::steal(),
            OTG_FS_GLOBAL: otg_fs_global::OTG_FS_GLOBAL::steal(),
            WWDG: wwdg::WWDG::steal(),
            FSMC: fsmc::FSMC::steal(),
            ADC_Common: adc_common::ADC_Common::steal(),
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
