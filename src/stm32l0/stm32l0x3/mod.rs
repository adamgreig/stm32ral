//! stm32ral module for stm32l0x3

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 2;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::adc;
pub use super::instances::aes;
pub use super::instances::crc;
pub use super::instances::crs;
pub use super::instances::dac;
pub use super::instances::dbg;
pub use super::instances::dma1;
pub use super::instances::exti;
pub use super::instances::flash;
pub use super::instances::fw;
pub use super::instances::gpio;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::lptim;
pub use super::instances::lpuart1;
pub use super::instances::nvic;
pub use super::instances::pwr;
pub use super::instances::rcc;
pub use super::instances::rng;
pub use super::instances::rtc;
pub use super::instances::spi;
pub use super::instances::syscfg;
pub use super::instances::tim2;
pub use super::instances::tim21;
pub use super::instances::tim22;
pub use super::instances::tim3;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub use super::instances::tsc;
pub use super::instances::usart;
pub use super::instances::usb;
pub use super::instances::wwdg;
pub mod lcd;
pub use super::instances::mpu;
pub use super::instances::scb;
pub use super::instances::stk;

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub AES: aes::Instance,
    pub DAC: dac::Instance,
    pub DMA1: dma1::Instance,
    pub CRC: crc::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOH: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub LPTIM: lptim::Instance,
    pub RNG: rng::Instance,
    pub RTC: rtc::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART4: usart::Instance,
    pub USART5: usart::Instance,
    pub TSC: tsc::Instance,
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub USB: usb::Instance,
    pub CRS: crs::Instance,
    pub FW: fw::Instance,
    pub RCC: rcc::Instance,
    pub SYSCFG: syscfg::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub PWR: pwr::Instance,
    pub FLASH: flash::Instance,
    pub EXTI: exti::Instance,
    pub ADC: adc::Instance,
    pub DBG: dbg::Instance,
    pub TIM2: tim2::Instance,
    pub TIM3: tim3::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub TIM21: tim21::Instance,
    pub TIM22: tim22::Instance,
    pub LPUART1: lpuart1::Instance,
    pub NVIC: nvic::Instance,
    pub LCD: lcd::Instance,
    pub MPU: mpu::Instance,
    pub STK: stk::Instance,
    pub SCB: scb::Instance,
}

#[cfg(all(feature = "rtfm", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            AES: aes::AES::steal(),
            DAC: dac::DAC::steal(),
            DMA1: dma1::DMA1::steal(),
            CRC: crc::CRC::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOH: gpio::GPIOH::steal(),
            GPIOE: gpio::GPIOE::steal(),
            LPTIM: lptim::LPTIM::steal(),
            RNG: rng::RNG::steal(),
            RTC: rtc::RTC::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART4: usart::USART4::steal(),
            USART5: usart::USART5::steal(),
            TSC: tsc::TSC::steal(),
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            USB: usb::USB::steal(),
            CRS: crs::CRS::steal(),
            FW: fw::FW::steal(),
            RCC: rcc::RCC::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C3: i2c::I2C3::steal(),
            PWR: pwr::PWR::steal(),
            FLASH: flash::FLASH::steal(),
            EXTI: exti::EXTI::steal(),
            ADC: adc::ADC::steal(),
            DBG: dbg::DBG::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            TIM21: tim21::TIM21::steal(),
            TIM22: tim22::TIM22::steal(),
            LPUART1: lpuart1::LPUART1::steal(),
            NVIC: nvic::NVIC::steal(),
            LCD: lcd::LCD::steal(),
            MPU: mpu::MPU::steal(),
            STK: stk::STK::steal(),
            SCB: scb::SCB::steal(),
        }
    }
}

#[cfg(all(feature = "rtfm", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
