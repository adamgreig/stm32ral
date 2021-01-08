//! stm32ral module for stm32wle5

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub mod adc;
pub mod aes;
pub mod comp;
pub mod crc;
pub mod dac;
pub mod dbgmcu;
pub mod dma;
pub mod dmamux;
pub mod exti;
pub mod flash;
pub mod gpio;
pub mod gpioc;
pub mod gpioh;
pub mod hsem;
pub mod i2c;
pub mod iwdg;
pub mod lptim1;
pub mod lptim2;
pub mod lptim3;
pub mod lpuart;
pub mod mpu;
pub mod nvic;
pub mod nvic_stir;
pub mod pka;
pub mod pwr;
pub mod rcc;
pub mod rng;
pub mod rtc;
pub mod scb;
pub mod scb_actrl;
pub mod spi;
pub mod stk;
pub mod syscfg;
pub mod syscfg_continue;
pub mod tamp;
pub mod tim1;
pub mod tim16;
pub mod tim17;
pub mod tim2;
pub mod usart;
pub mod vrefbuf;
pub mod wwdg;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub COMP: comp::Instance,
    pub CRC: crc::Instance,
    pub DAC: dac::Instance,
    pub DMA1: dma::Instance,
    pub DMA2: dma::Instance,
    pub DMAMUX: dmamux::Instance,
    pub PKA: pka::Instance,
    pub PWR: pwr::Instance,
    pub RCC: rcc::Instance,
    pub RNG: rng::Instance,
    pub RTC: rtc::Instance,
    pub SYSCFG: syscfg::Instance,
    pub LPTIM1: lptim1::Instance,
    pub LPTIM2: lptim2::Instance,
    pub LPTIM3: lptim3::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub TIM1: tim1::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub VREFBUF: vrefbuf::Instance,
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub AES: aes::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub HSEM: hsem::Instance,
    pub ADC: adc::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpioc::Instance,
    pub GPIOH: gpioh::Instance,
    pub FLASH: flash::Instance,
    pub EXTI: exti::Instance,
    pub LPUART: lpuart::Instance,
    pub TIM2: tim2::Instance,
    pub TAMP: tamp::Instance,
    pub MPU: mpu::Instance,
    pub STK: stk::Instance,
    pub SCB: scb::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub NVIC: nvic::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub SYSCFG_continue: syscfg_continue::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            COMP: comp::COMP::steal(),
            CRC: crc::CRC::steal(),
            DAC: dac::DAC::steal(),
            DMA1: dma::DMA1::steal(),
            DMA2: dma::DMA2::steal(),
            DMAMUX: dmamux::DMAMUX::steal(),
            PKA: pka::PKA::steal(),
            PWR: pwr::PWR::steal(),
            RCC: rcc::RCC::steal(),
            RNG: rng::RNG::steal(),
            RTC: rtc::RTC::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            LPTIM1: lptim1::LPTIM1::steal(),
            LPTIM2: lptim2::LPTIM2::steal(),
            LPTIM3: lptim3::LPTIM3::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            TIM1: tim1::TIM1::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            VREFBUF: vrefbuf::VREFBUF::steal(),
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            AES: aes::AES::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C3: i2c::I2C3::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            HSEM: hsem::HSEM::steal(),
            ADC: adc::ADC::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpioc::GPIOC::steal(),
            GPIOH: gpioh::GPIOH::steal(),
            FLASH: flash::FLASH::steal(),
            EXTI: exti::EXTI::steal(),
            LPUART: lpuart::LPUART::steal(),
            TIM2: tim2::TIM2::steal(),
            TAMP: tamp::TAMP::steal(),
            MPU: mpu::MPU::steal(),
            STK: stk::STK::steal(),
            SCB: scb::SCB::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            NVIC: nvic::NVIC::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            SYSCFG_continue: syscfg_continue::SYSCFG_continue::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
