//! stm32ral module for stm32f7x2

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::ac;
pub use super::instances::adc_common_f7x2_f7x3 as adc_common;
pub use super::instances::adc_f7x2_f7x3 as adc;
pub use super::instances::aes;
pub use super::instances::can1;
pub use super::instances::crc_f7x2_f7x3 as crc;
pub use super::instances::dac;
pub use super::instances::dbgmcu_f745_f765_f7x2_f7x3_f7x6 as dbgmcu;
pub use super::instances::dma_f7x2_f7x3 as dma;
pub use super::instances::exti_f7x2_f7x3 as exti;
pub use super::instances::flash_f7x2_f7x3 as flash;
pub use super::instances::fmc_f7x2_f7x3 as fmc;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::gpio_f7x2_f7x3 as gpio;
pub use super::instances::i2c_f7x2_f7x3 as i2c;
pub use super::instances::iwdg_f7x2_f7x3 as iwdg;
pub use super::instances::lptim1;
pub use super::instances::mpu;
pub use super::instances::nvic_f7x2_f7x3 as nvic;
pub use super::instances::nvic_stir;
pub use super::instances::otg_fs_device_f7x2_f7x3 as otg_fs_device;
pub use super::instances::otg_fs_global_f7x2_f7x3 as otg_fs_global;
pub use super::instances::otg_fs_host_f7x2_f7x3 as otg_fs_host;
pub use super::instances::otg_fs_pwrclk;
pub use super::instances::otg_hs_device_f7x2_f7x3 as otg_hs_device;
pub use super::instances::otg_hs_global_f7x2_f7x3 as otg_hs_global;
pub use super::instances::otg_hs_host_f7x2_f7x3 as otg_hs_host;
pub use super::instances::otg_hs_pwrclk;
pub use super::instances::pf;
pub use super::instances::pwr_f7x2_f7x3 as pwr;
pub use super::instances::quadspi;
pub use super::instances::rcc_f7x2_f7x3 as rcc;
pub use super::instances::rng;
pub use super::instances::rtc_f7x2_f7x3 as rtc;
pub use super::instances::sai_f7x2_f7x3 as sai;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::sdmmc_f7x2_f7x3 as sdmmc;
pub use super::instances::spi_f7x2_f7x3 as spi;
pub use super::instances::stk;
pub use super::instances::syscfg_f7x2_f7x3 as syscfg;
pub use super::instances::tim10_f7x2_f7x3 as tim10;
pub use super::instances::tim11_f7x2_f7x3 as tim11;
pub use super::instances::tim12_f7x2_f7x3 as tim12;
pub use super::instances::tim13_f7x2_f7x3 as tim13;
pub use super::instances::tim14_f7x2_f7x3 as tim14;
pub use super::instances::tim1_f7x2_f7x3 as tim1;
pub use super::instances::tim2_f7x2_f7x3 as tim2;
pub use super::instances::tim3_f7x2_f7x3 as tim3;
pub use super::instances::tim4_f7x2_f7x3 as tim4;
pub use super::instances::tim5_f7x2_f7x3 as tim5;
pub use super::instances::tim6_f7x2_f7x3 as tim6;
pub use super::instances::tim7_f7x2_f7x3 as tim7;
pub use super::instances::tim8_f7x2_f7x3 as tim8;
pub use super::instances::tim9_f7x2_f7x3 as tim9;
pub use super::instances::usart_f7x2_f7x3 as usart;
pub use super::instances::usbphyc;
pub use super::instances::wwdg;

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub TIM1: tim1::Instance,
    pub TIM8: tim8::Instance,
    pub ADC2: adc::Instance,
    pub ADC1: adc::Instance,
    pub ADC3: adc::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub ADC_Common: adc_common::Instance,
    pub CAN1: can1::Instance,
    pub CRC: crc::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub DAC: dac::Instance,
    pub DMA2: dma::Instance,
    pub DMA1: dma::Instance,
    pub EXTI: exti::Instance,
    pub FLASH: flash::Instance,
    pub FMC: fmc::Instance,
    pub TIM9: tim9::Instance,
    pub TIM12: tim12::Instance,
    pub TIM2: tim2::Instance,
    pub TIM3: tim3::Instance,
    pub TIM4: tim4::Instance,
    pub TIM5: tim5::Instance,
    pub GPIOH: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub GPIOG: gpio::Instance,
    pub GPIOI: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOA: gpio::Instance,
    pub TIM13: tim13::Instance,
    pub TIM14: tim14::Instance,
    pub TIM10: tim10::Instance,
    pub TIM11: tim11::Instance,
    pub IWDG: iwdg::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub LPTIM1: lptim1::Instance,
    pub PWR: pwr::Instance,
    pub QUADSPI: quadspi::Instance,
    pub RNG: rng::Instance,
    pub RTC: rtc::Instance,
    pub RCC: rcc::Instance,
    pub SDMMC1: sdmmc::Instance,
    pub SDMMC2: sdmmc::Instance,
    pub SAI1: sai::Instance,
    pub SAI2: sai::Instance,
    pub SPI5: spi::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI4: spi::Instance,
    pub SPI3: spi::Instance,
    pub SYSCFG: syscfg::Instance,
    pub USART3: usart::Instance,
    pub USART6: usart::Instance,
    pub UART8: usart::Instance,
    pub USART2: usart::Instance,
    pub UART7: usart::Instance,
    pub UART4: usart::Instance,
    pub UART5: usart::Instance,
    pub USART1: usart::Instance,
    pub OTG_FS_GLOBAL: otg_fs_global::Instance,
    pub OTG_FS_HOST: otg_fs_host::Instance,
    pub OTG_FS_DEVICE: otg_fs_device::Instance,
    pub OTG_FS_PWRCLK: otg_fs_pwrclk::Instance,
    pub OTG_HS_HOST: otg_hs_host::Instance,
    pub OTG_HS_GLOBAL: otg_hs_global::Instance,
    pub OTG_HS_PWRCLK: otg_hs_pwrclk::Instance,
    pub OTG_HS_DEVICE: otg_hs_device::Instance,
    pub WWDG: wwdg::Instance,
    pub NVIC: nvic::Instance,
    pub MPU: mpu::Instance,
    pub STK: stk::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub FPU_CPACR: fpu_cpacr::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub FPU: fpu::Instance,
    pub SCB: scb::Instance,
    pub PF: pf::Instance,
    pub AC: ac::Instance,
    pub AES: aes::Instance,
    pub USBPHYC: usbphyc::Instance,
}

#[cfg(all(feature = "rtfm", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            TIM1: tim1::TIM1::steal(),
            TIM8: tim8::TIM8::steal(),
            ADC2: adc::ADC2::steal(),
            ADC1: adc::ADC1::steal(),
            ADC3: adc::ADC3::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            ADC_Common: adc_common::ADC_Common::steal(),
            CAN1: can1::CAN1::steal(),
            CRC: crc::CRC::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            DAC: dac::DAC::steal(),
            DMA2: dma::DMA2::steal(),
            DMA1: dma::DMA1::steal(),
            EXTI: exti::EXTI::steal(),
            FLASH: flash::FLASH::steal(),
            FMC: fmc::FMC::steal(),
            TIM9: tim9::TIM9::steal(),
            TIM12: tim12::TIM12::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM4: tim4::TIM4::steal(),
            TIM5: tim5::TIM5::steal(),
            GPIOH: gpio::GPIOH::steal(),
            GPIOF: gpio::GPIOF::steal(),
            GPIOG: gpio::GPIOG::steal(),
            GPIOI: gpio::GPIOI::steal(),
            GPIOE: gpio::GPIOE::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOA: gpio::GPIOA::steal(),
            TIM13: tim13::TIM13::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM10: tim10::TIM10::steal(),
            TIM11: tim11::TIM11::steal(),
            IWDG: iwdg::IWDG::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C3: i2c::I2C3::steal(),
            LPTIM1: lptim1::LPTIM1::steal(),
            PWR: pwr::PWR::steal(),
            QUADSPI: quadspi::QUADSPI::steal(),
            RNG: rng::RNG::steal(),
            RTC: rtc::RTC::steal(),
            RCC: rcc::RCC::steal(),
            SDMMC1: sdmmc::SDMMC1::steal(),
            SDMMC2: sdmmc::SDMMC2::steal(),
            SAI1: sai::SAI1::steal(),
            SAI2: sai::SAI2::steal(),
            SPI5: spi::SPI5::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI4: spi::SPI4::steal(),
            SPI3: spi::SPI3::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            USART3: usart::USART3::steal(),
            USART6: usart::USART6::steal(),
            UART8: usart::UART8::steal(),
            USART2: usart::USART2::steal(),
            UART7: usart::UART7::steal(),
            UART4: usart::UART4::steal(),
            UART5: usart::UART5::steal(),
            USART1: usart::USART1::steal(),
            OTG_FS_GLOBAL: otg_fs_global::OTG_FS_GLOBAL::steal(),
            OTG_FS_HOST: otg_fs_host::OTG_FS_HOST::steal(),
            OTG_FS_DEVICE: otg_fs_device::OTG_FS_DEVICE::steal(),
            OTG_FS_PWRCLK: otg_fs_pwrclk::OTG_FS_PWRCLK::steal(),
            OTG_HS_HOST: otg_hs_host::OTG_HS_HOST::steal(),
            OTG_HS_GLOBAL: otg_hs_global::OTG_HS_GLOBAL::steal(),
            OTG_HS_PWRCLK: otg_hs_pwrclk::OTG_HS_PWRCLK::steal(),
            OTG_HS_DEVICE: otg_hs_device::OTG_HS_DEVICE::steal(),
            WWDG: wwdg::WWDG::steal(),
            NVIC: nvic::NVIC::steal(),
            MPU: mpu::MPU::steal(),
            STK: stk::STK::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            FPU_CPACR: fpu_cpacr::FPU_CPACR::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            FPU: fpu::FPU::steal(),
            SCB: scb::SCB::steal(),
            PF: pf::PF::steal(),
            AC: ac::AC::steal(),
            AES: aes::AES::steal(),
            USBPHYC: usbphyc::USBPHYC::steal(),
        }
    }
}

#[cfg(all(feature = "rtfm", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
