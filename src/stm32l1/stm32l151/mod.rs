//! stm32ral module for stm32l151

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::aes;
pub use super::instances::comp;
pub use super::instances::crc;
pub use super::instances::dac;
pub use super::instances::dma;
pub use super::instances::exti;
pub use super::instances::flash;
pub use super::instances::fsmc;
pub use super::instances::gpio;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::lcd;
pub use super::instances::opamp;
pub use super::instances::pwr;
pub use super::instances::rcc;
pub use super::instances::ri;
pub use super::instances::rtc;
pub use super::instances::sdio;
pub use super::instances::spi;
pub use super::instances::syscfg;
pub mod tim10;
pub mod tim11;
pub use super::instances::adc;
pub use super::instances::dbgmcu;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim4;
pub use super::instances::tim5;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub use super::instances::tim9;
pub use super::instances::usart;
pub use super::instances::usb;
pub use super::instances::wwdg;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub AES: aes::Instance,
    pub COMP: comp::Instance,
    pub CRC: crc::Instance,
    pub DAC: dac::Instance,
    pub DMA1: dma::Instance,
    pub DMA2: dma::Instance,
    pub EXTI: exti::Instance,
    pub Flash: flash::Instance,
    pub FSMC: fsmc::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub GPIOG: gpio::Instance,
    pub GPIOH: gpio::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub IWDG: iwdg::Instance,
    pub LCD: lcd::Instance,
    pub OPAMP: opamp::Instance,
    pub PWR: pwr::Instance,
    pub RCC: rcc::Instance,
    pub RI: ri::Instance,
    pub RTC: rtc::Instance,
    pub SDIO: sdio::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub SYSCFG: syscfg::Instance,
    pub TIM10: tim10::Instance,
    pub TIM11: tim11::Instance,
    pub TIM2: tim2::Instance,
    pub TIM3: tim3::Instance,
    pub TIM4: tim4::Instance,
    pub TIM5: tim5::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub TIM9: tim9::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub UART4: usart::Instance,
    pub UART5: usart::Instance,
    pub USB: usb::Instance,
    pub USB_SRAM: usb::Instance,
    pub WWDG: wwdg::Instance,
    pub ADC: adc::Instance,
    pub NVIC: nvic::Instance,
    pub DBGMCU: dbgmcu::Instance,
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
            AES: aes::AES::steal(),
            COMP: comp::COMP::steal(),
            CRC: crc::CRC::steal(),
            DAC: dac::DAC::steal(),
            DMA1: dma::DMA1::steal(),
            DMA2: dma::DMA2::steal(),
            EXTI: exti::EXTI::steal(),
            Flash: flash::Flash::steal(),
            FSMC: fsmc::FSMC::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOE: gpio::GPIOE::steal(),
            GPIOF: gpio::GPIOF::steal(),
            GPIOG: gpio::GPIOG::steal(),
            GPIOH: gpio::GPIOH::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            IWDG: iwdg::IWDG::steal(),
            LCD: lcd::LCD::steal(),
            OPAMP: opamp::OPAMP::steal(),
            PWR: pwr::PWR::steal(),
            RCC: rcc::RCC::steal(),
            RI: ri::RI::steal(),
            RTC: rtc::RTC::steal(),
            SDIO: sdio::SDIO::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            TIM10: tim10::TIM10::steal(),
            TIM11: tim11::TIM11::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM4: tim4::TIM4::steal(),
            TIM5: tim5::TIM5::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            TIM9: tim9::TIM9::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            UART4: usart::UART4::steal(),
            UART5: usart::UART5::steal(),
            USB: usb::USB::steal(),
            USB_SRAM: usb::USB_SRAM::steal(),
            WWDG: wwdg::WWDG::steal(),
            ADC: adc::ADC::steal(),
            NVIC: nvic::NVIC::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
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
