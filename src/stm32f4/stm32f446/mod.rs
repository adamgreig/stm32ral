//! stm32ral module for stm32f446

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub mod dbgmcu;
pub mod dcmi;
pub mod fmc;
pub use super::instances::dma_f405_f407_f427_f429_f446_f469 as dma;
pub mod gpio;
pub mod rcc;
pub mod spi;
pub mod syscfg;
pub use super::instances::adc;
pub use super::instances::adc_common_f405_f407_f427_f429_f446_f469 as adc_common;
pub use super::instances::crc;
pub use super::instances::dac;
pub use super::instances::i2c_f401_f411_f412_f427_f429_f446_f469 as i2c;
pub use super::instances::iwdg;
pub use super::instances::rtc_f405_f407_f427_f429_f446_f469 as rtc;
pub use super::instances::tim10_f401_f411_f412_f413_f427_f429_f446_f469 as tim10;
pub use super::instances::tim11_f401_f410_f411_f412_f413_f427_f429_f446_f469 as tim11;
pub use super::instances::tim12_f405_f407_f427_f429_f446 as tim12;
pub use super::instances::tim13_f412_f413_f427_f429_f446_f469 as tim13;
pub use super::instances::tim14_f412_f413_f427_f429_f446_f469 as tim14;
pub use super::instances::tim1_f401_f405_f407_f413_f427_f429_f446_f469 as tim1;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim4;
pub use super::instances::tim5;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub use super::instances::tim8_f405_f407_f413_f427_f429_f446_f469 as tim8;
pub use super::instances::tim9_f401_f405_f407_f410_f411_f427_f429_f446 as tim9;
pub use super::instances::uart_f427_f429_f446_f469 as uart;
pub use super::instances::usart_f405_f407_f446 as usart;
pub use super::instances::wwdg;
pub mod otg_fs_global;
pub use super::instances::otg_fs_host;
pub mod otg_fs_device;
pub use super::instances::can;
pub use super::instances::nvic_f427_f429_f446_f469 as nvic;
pub use super::instances::otg_s_pwrclk;
pub mod flash;
pub use super::instances::exti;
pub use super::instances::otg_hs_device;
pub use super::instances::otg_hs_global;
pub use super::instances::otg_hs_host;
pub mod pwr;
pub mod sai;
pub use super::instances::quadspi;
pub mod hdmi_cec;
pub mod sdio;
pub mod spdifrx;
pub use super::instances::fmpi2c1;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::mpu;
pub use super::instances::nvic_stir;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub DCMI: dcmi::Instance,
    pub FMC: fmc::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub DMA2: dma::Instance,
    pub DMA1: dma::Instance,
    pub RCC: rcc::Instance,
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
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub SPI4: spi::Instance,
    pub ADC1: adc::Instance,
    pub ADC2: adc::Instance,
    pub ADC3: adc::Instance,
    pub USART6: usart::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub DAC: dac::Instance,
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
    pub TIM13: tim13::Instance,
    pub TIM14: tim14::Instance,
    pub TIM11: tim11::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub CRC: crc::Instance,
    pub OTG_FS_GLOBAL: otg_fs_global::Instance,
    pub OTG_FS_HOST: otg_fs_host::Instance,
    pub OTG_FS_DEVICE: otg_fs_device::Instance,
    pub OTG_FS_PWRCLK: otg_s_pwrclk::Instance,
    pub OTG_HS_PWRCLK: otg_s_pwrclk::Instance,
    pub CAN1: can::Instance,
    pub CAN2: can::Instance,
    pub NVIC: nvic::Instance,
    pub FLASH: flash::Instance,
    pub EXTI: exti::Instance,
    pub OTG_HS_GLOBAL: otg_hs_global::Instance,
    pub OTG_HS_HOST: otg_hs_host::Instance,
    pub OTG_HS_DEVICE: otg_hs_device::Instance,
    pub SAI1: sai::Instance,
    pub SAI2: sai::Instance,
    pub PWR: pwr::Instance,
    pub QUADSPI: quadspi::Instance,
    pub SPDIFRX: spdifrx::Instance,
    pub SDIO: sdio::Instance,
    pub HDMI_CEC: hdmi_cec::Instance,
    pub FPU: fpu::Instance,
    pub MPU: mpu::Instance,
    pub STK: stk::Instance,
    pub SCB: scb::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub FPU_CPACR: fpu_cpacr::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub FMPI2C1: fmpi2c1::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            DCMI: dcmi::DCMI::steal(),
            FMC: fmc::FMC::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            DMA2: dma::DMA2::steal(),
            DMA1: dma::DMA1::steal(),
            RCC: rcc::RCC::steal(),
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
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            SPI4: spi::SPI4::steal(),
            ADC1: adc::ADC1::steal(),
            ADC2: adc::ADC2::steal(),
            ADC3: adc::ADC3::steal(),
            USART6: usart::USART6::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            DAC: dac::DAC::steal(),
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
            TIM13: tim13::TIM13::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM11: tim11::TIM11::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            CRC: crc::CRC::steal(),
            OTG_FS_GLOBAL: otg_fs_global::OTG_FS_GLOBAL::steal(),
            OTG_FS_HOST: otg_fs_host::OTG_FS_HOST::steal(),
            OTG_FS_DEVICE: otg_fs_device::OTG_FS_DEVICE::steal(),
            OTG_FS_PWRCLK: otg_s_pwrclk::OTG_FS_PWRCLK::steal(),
            OTG_HS_PWRCLK: otg_s_pwrclk::OTG_HS_PWRCLK::steal(),
            CAN1: can::CAN1::steal(),
            CAN2: can::CAN2::steal(),
            NVIC: nvic::NVIC::steal(),
            FLASH: flash::FLASH::steal(),
            EXTI: exti::EXTI::steal(),
            OTG_HS_GLOBAL: otg_hs_global::OTG_HS_GLOBAL::steal(),
            OTG_HS_HOST: otg_hs_host::OTG_HS_HOST::steal(),
            OTG_HS_DEVICE: otg_hs_device::OTG_HS_DEVICE::steal(),
            SAI1: sai::SAI1::steal(),
            SAI2: sai::SAI2::steal(),
            PWR: pwr::PWR::steal(),
            QUADSPI: quadspi::QUADSPI::steal(),
            SPDIFRX: spdifrx::SPDIFRX::steal(),
            SDIO: sdio::SDIO::steal(),
            HDMI_CEC: hdmi_cec::HDMI_CEC::steal(),
            FPU: fpu::FPU::steal(),
            MPU: mpu::MPU::steal(),
            STK: stk::STK::steal(),
            SCB: scb::SCB::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            FPU_CPACR: fpu_cpacr::FPU_CPACR::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            FMPI2C1: fmpi2c1::FMPI2C1::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
