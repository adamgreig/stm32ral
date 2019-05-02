//! stm32ral module for stm32f215

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::crc;
pub use super::instances::dcmi;
pub use super::instances::fsmc;
pub use super::instances::rng;
pub mod dbgmcu;
pub use super::instances::adc;
pub use super::instances::adc_common;
pub use super::instances::can;
pub use super::instances::cryp;
pub use super::instances::dac;
pub use super::instances::dma;
pub use super::instances::ethernet_dma;
pub use super::instances::ethernet_mac;
pub use super::instances::ethernet_mmc;
pub use super::instances::ethernet_ptp;
pub use super::instances::exti;
pub use super::instances::flash;
pub use super::instances::gpio;
pub use super::instances::hash;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::otg_fs_device;
pub use super::instances::otg_fs_global;
pub use super::instances::otg_fs_host;
pub use super::instances::otg_fs_pwrclk;
pub use super::instances::otg_hs_device;
pub use super::instances::otg_hs_global;
pub use super::instances::otg_hs_host;
pub use super::instances::otg_hs_pwrclk;
pub use super::instances::pwr;
pub use super::instances::rcc;
pub use super::instances::rtc;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::sdio;
pub use super::instances::spi;
pub use super::instances::stk;
pub use super::instances::syscfg;
pub use super::instances::tim1;
pub use super::instances::tim10;
pub use super::instances::tim11;
pub use super::instances::tim12;
pub use super::instances::tim13;
pub use super::instances::tim14;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim4;
pub use super::instances::tim5;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub use super::instances::tim8;
pub use super::instances::tim9;
pub use super::instances::uart;
pub use super::instances::usart;
pub use super::instances::wwdg;

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub RNG: rng::Instance,
    pub CRC: crc::Instance,
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
    pub GPIOB: gpio::Instance,
    pub GPIOA: gpio::Instance,
    pub SYSCFG: syscfg::Instance,
    pub SPI1: spi::Instance,
    pub SPI3: spi::Instance,
    pub SPI2: spi::Instance,
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
    pub CAN2: can::Instance,
    pub CAN1: can::Instance,
    pub I2C3: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C1: i2c::Instance,
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub RTC: rtc::Instance,
    pub UART4: uart::Instance,
    pub UART5: uart::Instance,
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
    pub TIM11: tim11::Instance,
    pub TIM13: tim13::Instance,
    pub TIM14: tim14::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub Ethernet_MAC: ethernet_mac::Instance,
    pub Ethernet_MMC: ethernet_mmc::Instance,
    pub Ethernet_PTP: ethernet_ptp::Instance,
    pub Ethernet_DMA: ethernet_dma::Instance,
    pub OTG_FS_GLOBAL: otg_fs_global::Instance,
    pub OTG_FS_HOST: otg_fs_host::Instance,
    pub OTG_FS_DEVICE: otg_fs_device::Instance,
    pub OTG_FS_PWRCLK: otg_fs_pwrclk::Instance,
    pub EXTI: exti::Instance,
    pub FLASH: flash::Instance,
    pub HASH: hash::Instance,
    pub CRYP: cryp::Instance,
    pub OTG_HS_GLOBAL: otg_hs_global::Instance,
    pub OTG_HS_HOST: otg_hs_host::Instance,
    pub OTG_HS_DEVICE: otg_hs_device::Instance,
    pub OTG_HS_PWRCLK: otg_hs_pwrclk::Instance,
    pub NVIC: nvic::Instance,
    pub MPU: mpu::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub SCB: scb::Instance,
    pub STK: stk::Instance,
}

#[cfg(all(feature = "rtfm", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            RNG: rng::RNG::steal(),
            CRC: crc::CRC::steal(),
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
            GPIOB: gpio::GPIOB::steal(),
            GPIOA: gpio::GPIOA::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            SPI1: spi::SPI1::steal(),
            SPI3: spi::SPI3::steal(),
            SPI2: spi::SPI2::steal(),
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
            CAN2: can::CAN2::steal(),
            CAN1: can::CAN1::steal(),
            I2C3: i2c::I2C3::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C1: i2c::I2C1::steal(),
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            RTC: rtc::RTC::steal(),
            UART4: uart::UART4::steal(),
            UART5: uart::UART5::steal(),
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
            TIM11: tim11::TIM11::steal(),
            TIM13: tim13::TIM13::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            Ethernet_MAC: ethernet_mac::Ethernet_MAC::steal(),
            Ethernet_MMC: ethernet_mmc::Ethernet_MMC::steal(),
            Ethernet_PTP: ethernet_ptp::Ethernet_PTP::steal(),
            Ethernet_DMA: ethernet_dma::Ethernet_DMA::steal(),
            OTG_FS_GLOBAL: otg_fs_global::OTG_FS_GLOBAL::steal(),
            OTG_FS_HOST: otg_fs_host::OTG_FS_HOST::steal(),
            OTG_FS_DEVICE: otg_fs_device::OTG_FS_DEVICE::steal(),
            OTG_FS_PWRCLK: otg_fs_pwrclk::OTG_FS_PWRCLK::steal(),
            EXTI: exti::EXTI::steal(),
            FLASH: flash::FLASH::steal(),
            HASH: hash::HASH::steal(),
            CRYP: cryp::CRYP::steal(),
            OTG_HS_GLOBAL: otg_hs_global::OTG_HS_GLOBAL::steal(),
            OTG_HS_HOST: otg_hs_host::OTG_HS_HOST::steal(),
            OTG_HS_DEVICE: otg_hs_device::OTG_HS_DEVICE::steal(),
            OTG_HS_PWRCLK: otg_hs_pwrclk::OTG_HS_PWRCLK::steal(),
            NVIC: nvic::NVIC::steal(),
            MPU: mpu::MPU::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            SCB: scb::SCB::steal(),
            STK: stk::STK::steal(),
        }
    }
}

#[cfg(all(feature = "rtfm", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
