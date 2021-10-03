//! stm32ral module for stm32f102

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::pwr;
pub mod rcc;
pub use super::instances::afio;
pub use super::instances::dma;
pub use super::instances::exti;
pub use super::instances::gpio;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::rtc;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::wwdg;
pub mod spi1;
pub use super::instances::usart;
pub mod adc1;
pub use super::instances::crc;
pub use super::instances::flash;
pub mod dbgmcu;
pub use super::instances::adc2;
pub use super::instances::bkp;
pub use super::instances::dac_f100_f102 as dac;
pub use super::instances::fsmc_f100_f102 as fsmc;
pub use super::instances::otg_fs_device;
pub use super::instances::otg_fs_global;
pub use super::instances::otg_fs_host;
pub use super::instances::otg_fs_pwrclk;
pub use super::instances::sdio;
pub use super::instances::tim10_f102_f107 as tim10;
pub use super::instances::tim11_f102_f107 as tim11;
pub use super::instances::tim12;
pub use super::instances::tim1_f100_f101_f102 as tim1;
pub use super::instances::tim8;
pub use super::instances::tim9;
pub mod can;
pub use super::instances::usb;
pub mod spi;
pub mod uart;
pub use super::instances::adc3;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;
pub use super::instances::tim13_f100_f102_f107 as tim13;
pub use super::instances::tim14_f100_f102_f107 as tim14;
pub use super::instances::tim4;
pub use super::instances::tim5;
pub use super::instances::tim6;
pub use super::instances::tim7;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
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
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub TIM2: tim2::Instance,
    pub TIM3: tim3::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub SPI1: spi1::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub ADC1: adc1::Instance,
    pub CRC: crc::Instance,
    pub FLASH: flash::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub BKP: bkp::Instance,
    pub FSMC: fsmc::Instance,
    pub OTG_FS_DEVICE: otg_fs_device::Instance,
    pub OTG_FS_GLOBAL: otg_fs_global::Instance,
    pub OTG_FS_HOST: otg_fs_host::Instance,
    pub OTG_FS_PWRCLK: otg_fs_pwrclk::Instance,
    pub SDIO: sdio::Instance,
    pub TIM10: tim10::Instance,
    pub TIM11: tim11::Instance,
    pub TIM9: tim9::Instance,
    pub TIM12: tim12::Instance,
    pub TIM8: tim8::Instance,
    pub TIM1: tim1::Instance,
    pub ADC2: adc2::Instance,
    pub DAC: dac::Instance,
    pub CAN1: can::Instance,
    pub CAN2: can::Instance,
    pub USB: usb::Instance,
    pub UART5: uart::Instance,
    pub UART4: uart::Instance,
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub TIM4: tim4::Instance,
    pub TIM5: tim5::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub TIM13: tim13::Instance,
    pub TIM14: tim14::Instance,
    pub NVIC: nvic::Instance,
    pub MPU: mpu::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub SCB: scb::Instance,
    pub STK: stk::Instance,
    pub ADC3: adc3::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
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
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            SPI1: spi1::SPI1::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            ADC1: adc1::ADC1::steal(),
            CRC: crc::CRC::steal(),
            FLASH: flash::FLASH::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            BKP: bkp::BKP::steal(),
            FSMC: fsmc::FSMC::steal(),
            OTG_FS_DEVICE: otg_fs_device::OTG_FS_DEVICE::steal(),
            OTG_FS_GLOBAL: otg_fs_global::OTG_FS_GLOBAL::steal(),
            OTG_FS_HOST: otg_fs_host::OTG_FS_HOST::steal(),
            OTG_FS_PWRCLK: otg_fs_pwrclk::OTG_FS_PWRCLK::steal(),
            SDIO: sdio::SDIO::steal(),
            TIM10: tim10::TIM10::steal(),
            TIM11: tim11::TIM11::steal(),
            TIM9: tim9::TIM9::steal(),
            TIM12: tim12::TIM12::steal(),
            TIM8: tim8::TIM8::steal(),
            TIM1: tim1::TIM1::steal(),
            ADC2: adc2::ADC2::steal(),
            DAC: dac::DAC::steal(),
            CAN1: can::CAN1::steal(),
            CAN2: can::CAN2::steal(),
            USB: usb::USB::steal(),
            UART5: uart::UART5::steal(),
            UART4: uart::UART4::steal(),
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            TIM4: tim4::TIM4::steal(),
            TIM5: tim5::TIM5::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            TIM13: tim13::TIM13::steal(),
            TIM14: tim14::TIM14::steal(),
            NVIC: nvic::NVIC::steal(),
            MPU: mpu::MPU::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            SCB: scb::SCB::steal(),
            STK: stk::STK::steal(),
            ADC3: adc3::ADC3::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
