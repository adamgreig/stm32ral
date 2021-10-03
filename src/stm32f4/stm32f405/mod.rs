//! stm32ral module for stm32f405

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::adc;
pub use super::instances::adc_common_f405_f407_f427_f429_f446_f469 as adc_common;
pub use super::instances::dac;
pub use super::instances::dbgmcu_f405_f407 as dbgmcu;
pub use super::instances::dcmi;
pub use super::instances::dma_f405_f407_f427_f429_f446_f469 as dma;
pub use super::instances::fsmc;
pub use super::instances::gpio_f405_f407_f427 as gpio;
pub use super::instances::i2c_f405_f407 as i2c;
pub use super::instances::iwdg;
pub use super::instances::pwr_f405_f407 as pwr;
pub use super::instances::rcc;
pub use super::instances::rng_f405_f407_f427_f429 as rng;
pub use super::instances::rtc_f405_f407_f427_f429_f446_f469 as rtc;
pub use super::instances::sdio;
pub use super::instances::spi_f405_f407_f427_f429_f469 as spi;
pub use super::instances::syscfg_f405_f407 as syscfg;
pub use super::instances::tim10_f405_f407 as tim10;
pub use super::instances::tim11_f405_f407 as tim11;
pub use super::instances::tim12_f405_f407_f427_f429_f446 as tim12;
pub use super::instances::tim13_f405_f407 as tim13;
pub use super::instances::tim14_f405_f407 as tim14;
pub use super::instances::tim1_f401_f405_f407_f413_f427_f429_f446_f469 as tim1;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim4;
pub use super::instances::tim5;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub use super::instances::tim8_f405_f407_f413_f427_f429_f446_f469 as tim8;
pub use super::instances::tim9_f401_f405_f407_f410_f411_f427_f429_f446 as tim9;
pub use super::instances::uart_f405_f407 as uart;
pub use super::instances::usart_f405_f407_f446 as usart;
pub use super::instances::wwdg;
pub mod ethernet_mac;
pub mod ethernet_mmc;
pub use super::instances::ethernet_ptp;
pub mod ethernet_dma;
pub use super::instances::can;
pub use super::instances::crc;
pub use super::instances::cryp;
pub use super::instances::exti;
pub use super::instances::flash_f405_f407 as flash;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::hash;
pub use super::instances::ltdc;
pub use super::instances::mpu;
pub use super::instances::nvic_f401_f405_f407_f410_f411_f412_f413 as nvic;
pub use super::instances::nvic_stir;
pub use super::instances::otg_fs_device_f401_f405_f407_f411_f427_f429 as otg_fs_device;
pub use super::instances::otg_fs_global_f401_f405_f407_f411_f427_f429 as otg_fs_global;
pub use super::instances::otg_fs_host;
pub use super::instances::otg_hs_device;
pub use super::instances::otg_hs_global;
pub use super::instances::otg_hs_host;
pub use super::instances::otg_s_pwrclk;
pub use super::instances::sai1;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub RNG: rng::Instance,
    pub DCMI: dcmi::Instance,
    pub FSMC: fsmc::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub DMA2: dma::Instance,
    pub DMA1: dma::Instance,
    pub RCC: rcc::Instance,
    pub GPIOI: gpio::Instance,
    pub GPIOH: gpio::Instance,
    pub GPIOG: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOJ: gpio::Instance,
    pub GPIOK: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOA: gpio::Instance,
    pub SYSCFG: syscfg::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub I2S2ext: spi::Instance,
    pub I2S3ext: spi::Instance,
    pub SPI4: spi::Instance,
    pub SPI5: spi::Instance,
    pub SPI6: spi::Instance,
    pub SDIO: sdio::Instance,
    pub ADC1: adc::Instance,
    pub ADC2: adc::Instance,
    pub ADC3: adc::Instance,
    pub USART6: usart::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub DAC: dac::Instance,
    pub PWR: pwr::Instance,
    pub I2C3: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C1: i2c::Instance,
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub RTC: rtc::Instance,
    pub UART4: uart::Instance,
    pub UART5: uart::Instance,
    pub UART7: uart::Instance,
    pub UART8: uart::Instance,
    pub ADC_Common: adc_common::Instance,
    pub TIM1: tim1::Instance,
    pub TIM8: tim8::Instance,
    pub TIM2: tim2::Instance,
    pub TIM3: tim3::Instance,
    pub TIM4: tim4::Instance,
    pub TIM5: tim5::Instance,
    pub TIM9: tim9::Instance,
    pub TIM12: tim12::Instance,
    pub TIM10: tim10::Instance,
    pub TIM13: tim13::Instance,
    pub TIM14: tim14::Instance,
    pub TIM11: tim11::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub Ethernet_MAC: ethernet_mac::Instance,
    pub Ethernet_MMC: ethernet_mmc::Instance,
    pub Ethernet_PTP: ethernet_ptp::Instance,
    pub Ethernet_DMA: ethernet_dma::Instance,
    pub CRC: crc::Instance,
    pub OTG_FS_GLOBAL: otg_fs_global::Instance,
    pub OTG_FS_HOST: otg_fs_host::Instance,
    pub OTG_FS_DEVICE: otg_fs_device::Instance,
    pub OTG_FS_PWRCLK: otg_s_pwrclk::Instance,
    pub OTG_HS_PWRCLK: otg_s_pwrclk::Instance,
    pub CAN1: can::Instance,
    pub CAN2: can::Instance,
    pub FLASH: flash::Instance,
    pub EXTI: exti::Instance,
    pub OTG_HS_GLOBAL: otg_hs_global::Instance,
    pub OTG_HS_HOST: otg_hs_host::Instance,
    pub OTG_HS_DEVICE: otg_hs_device::Instance,
    pub NVIC: nvic::Instance,
    pub SAI1: sai1::Instance,
    pub LTDC: ltdc::Instance,
    pub HASH: hash::Instance,
    pub CRYP: cryp::Instance,
    pub FPU: fpu::Instance,
    pub MPU: mpu::Instance,
    pub STK: stk::Instance,
    pub SCB: scb::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub FPU_CPACR: fpu_cpacr::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            RNG: rng::RNG::steal(),
            DCMI: dcmi::DCMI::steal(),
            FSMC: fsmc::FSMC::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            DMA2: dma::DMA2::steal(),
            DMA1: dma::DMA1::steal(),
            RCC: rcc::RCC::steal(),
            GPIOI: gpio::GPIOI::steal(),
            GPIOH: gpio::GPIOH::steal(),
            GPIOG: gpio::GPIOG::steal(),
            GPIOF: gpio::GPIOF::steal(),
            GPIOE: gpio::GPIOE::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOJ: gpio::GPIOJ::steal(),
            GPIOK: gpio::GPIOK::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOA: gpio::GPIOA::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            I2S2ext: spi::I2S2ext::steal(),
            I2S3ext: spi::I2S3ext::steal(),
            SPI4: spi::SPI4::steal(),
            SPI5: spi::SPI5::steal(),
            SPI6: spi::SPI6::steal(),
            SDIO: sdio::SDIO::steal(),
            ADC1: adc::ADC1::steal(),
            ADC2: adc::ADC2::steal(),
            ADC3: adc::ADC3::steal(),
            USART6: usart::USART6::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            DAC: dac::DAC::steal(),
            PWR: pwr::PWR::steal(),
            I2C3: i2c::I2C3::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C1: i2c::I2C1::steal(),
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            RTC: rtc::RTC::steal(),
            UART4: uart::UART4::steal(),
            UART5: uart::UART5::steal(),
            UART7: uart::UART7::steal(),
            UART8: uart::UART8::steal(),
            ADC_Common: adc_common::ADC_Common::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM8: tim8::TIM8::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM4: tim4::TIM4::steal(),
            TIM5: tim5::TIM5::steal(),
            TIM9: tim9::TIM9::steal(),
            TIM12: tim12::TIM12::steal(),
            TIM10: tim10::TIM10::steal(),
            TIM13: tim13::TIM13::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM11: tim11::TIM11::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            Ethernet_MAC: ethernet_mac::Ethernet_MAC::steal(),
            Ethernet_MMC: ethernet_mmc::Ethernet_MMC::steal(),
            Ethernet_PTP: ethernet_ptp::Ethernet_PTP::steal(),
            Ethernet_DMA: ethernet_dma::Ethernet_DMA::steal(),
            CRC: crc::CRC::steal(),
            OTG_FS_GLOBAL: otg_fs_global::OTG_FS_GLOBAL::steal(),
            OTG_FS_HOST: otg_fs_host::OTG_FS_HOST::steal(),
            OTG_FS_DEVICE: otg_fs_device::OTG_FS_DEVICE::steal(),
            OTG_FS_PWRCLK: otg_s_pwrclk::OTG_FS_PWRCLK::steal(),
            OTG_HS_PWRCLK: otg_s_pwrclk::OTG_HS_PWRCLK::steal(),
            CAN1: can::CAN1::steal(),
            CAN2: can::CAN2::steal(),
            FLASH: flash::FLASH::steal(),
            EXTI: exti::EXTI::steal(),
            OTG_HS_GLOBAL: otg_hs_global::OTG_HS_GLOBAL::steal(),
            OTG_HS_HOST: otg_hs_host::OTG_HS_HOST::steal(),
            OTG_HS_DEVICE: otg_hs_device::OTG_HS_DEVICE::steal(),
            NVIC: nvic::NVIC::steal(),
            SAI1: sai1::SAI1::steal(),
            LTDC: ltdc::LTDC::steal(),
            HASH: hash::HASH::steal(),
            CRYP: cryp::CRYP::steal(),
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

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
