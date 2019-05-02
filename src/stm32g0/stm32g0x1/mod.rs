//! stm32ral module for stm32g0x1

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 3;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::adc;
pub use super::instances::aes;
pub use super::instances::comp;
pub use super::instances::crc;
pub use super::instances::dac;
pub use super::instances::dbg;
pub use super::instances::dma;
pub use super::instances::dmamux;
pub use super::instances::exti;
pub use super::instances::flash;
pub use super::instances::gpio;
pub use super::instances::hdmi_cec;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::lptim;
pub use super::instances::lpuart;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::pwr;
pub use super::instances::rcc;
pub use super::instances::rng;
pub use super::instances::rtc;
pub use super::instances::scb;
pub use super::instances::spi;
pub use super::instances::stk;
pub use super::instances::syscfg_vrefbuf;
pub use super::instances::tamp;
pub use super::instances::tim1;
pub use super::instances::tim14;
pub use super::instances::tim15;
pub use super::instances::tim16;
pub use super::instances::tim17;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub use super::instances::ucpd;
pub use super::instances::usart;
pub use super::instances::wwdg;

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub FLASH: flash::Instance,
    pub DBG: dbg::Instance,
    pub RCC: rcc::Instance,
    pub PWR: pwr::Instance,
    pub DMA: dma::Instance,
    pub DMAMUX: dmamux::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub AES: aes::Instance,
    pub RNG: rng::Instance,
    pub CRC: crc::Instance,
    pub EXTI: exti::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub TIM15: tim15::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub USART4: usart::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub TIM1: tim1::Instance,
    pub ADC: adc::Instance,
    pub COMP: comp::Instance,
    pub SYSCFG_VREFBUF: syscfg_vrefbuf::Instance,
    pub TAMP: tamp::Instance,
    pub UCPD1: ucpd::Instance,
    pub UCPD2: ucpd::Instance,
    pub LPTIM1: lptim::Instance,
    pub LPTIM2: lptim::Instance,
    pub LPUART: lpuart::Instance,
    pub HDMI_CEC: hdmi_cec::Instance,
    pub DAC: dac::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub RTC: rtc::Instance,
    pub TIM14: tim14::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub TIM2: tim2::Instance,
    pub TIM3: tim3::Instance,
    pub NVIC: nvic::Instance,
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
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            FLASH: flash::FLASH::steal(),
            DBG: dbg::DBG::steal(),
            RCC: rcc::RCC::steal(),
            PWR: pwr::PWR::steal(),
            DMA: dma::DMA::steal(),
            DMAMUX: dmamux::DMAMUX::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOF: gpio::GPIOF::steal(),
            AES: aes::AES::steal(),
            RNG: rng::RNG::steal(),
            CRC: crc::CRC::steal(),
            EXTI: exti::EXTI::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            TIM15: tim15::TIM15::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            USART4: usart::USART4::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            TIM1: tim1::TIM1::steal(),
            ADC: adc::ADC::steal(),
            COMP: comp::COMP::steal(),
            SYSCFG_VREFBUF: syscfg_vrefbuf::SYSCFG_VREFBUF::steal(),
            TAMP: tamp::TAMP::steal(),
            UCPD1: ucpd::UCPD1::steal(),
            UCPD2: ucpd::UCPD2::steal(),
            LPTIM1: lptim::LPTIM1::steal(),
            LPTIM2: lptim::LPTIM2::steal(),
            LPUART: lpuart::LPUART::steal(),
            HDMI_CEC: hdmi_cec::HDMI_CEC::steal(),
            DAC: dac::DAC::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            RTC: rtc::RTC::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            NVIC: nvic::NVIC::steal(),
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
