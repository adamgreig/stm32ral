//! stm32ral module for stm32g031

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 2;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::adc_g030_g031_g041 as adc;
pub use super::instances::crc;
pub use super::instances::dbg_g030_g031_g041 as dbg;
pub use super::instances::dma_g030_g031_g041 as dma;
pub use super::instances::dmamux_g030_g031_g041 as dmamux;
pub use super::instances::exti_g030_g031_g041 as exti;
pub use super::instances::flash;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::gpio;
pub use super::instances::i2c;
pub use super::instances::iwdg_g030_g031_g041 as iwdg;
pub use super::instances::lptim;
pub use super::instances::lpuart_g030_g031_g041 as lpuart;
pub use super::instances::mpu;
pub use super::instances::nvic_g030_g031_g041 as nvic;
pub use super::instances::nvic_stir;
pub use super::instances::pwr_g030_g031_g041 as pwr;
pub use super::instances::rcc_g030_g031_g041 as rcc;
pub use super::instances::rng;
pub use super::instances::rtc_g030_g031_g041 as rtc;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::spi_g030_g031_g041 as spi;
pub use super::instances::stk;
pub use super::instances::syscfg;
pub use super::instances::syscfg_itline;
pub use super::instances::tamp_g030_g031_g041 as tamp;
pub use super::instances::tim14;
pub use super::instances::tim16;
pub use super::instances::tim17;
pub use super::instances::tim1_g030_g031_g041 as tim1;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::usart_g030_g031_g041 as usart;
pub use super::instances::vrefbuf;
pub use super::instances::wwdg;

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub FLASH: flash::Instance,
    pub RCC: rcc::Instance,
    pub PWR: pwr::Instance,
    pub DMA: dma::Instance,
    pub DMAMUX: dmamux::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub RNG: rng::Instance,
    pub CRC: crc::Instance,
    pub EXTI: exti::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub TIM1: tim1::Instance,
    pub ADC: adc::Instance,
    pub SYSCFG: syscfg::Instance,
    pub TAMP: tamp::Instance,
    pub LPTIM1: lptim::Instance,
    pub LPTIM2: lptim::Instance,
    pub LPUART: lpuart::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub RTC: rtc::Instance,
    pub TIM14: tim14::Instance,
    pub TIM2: tim2::Instance,
    pub TIM3: tim3::Instance,
    pub NVIC: nvic::Instance,
    pub MPU: mpu::Instance,
    pub STK: stk::Instance,
    pub SCB: scb::Instance,
    pub VREFBUF: vrefbuf::Instance,
    pub DBG: dbg::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub FPU_CPACR: fpu_cpacr::Instance,
    pub FPU: fpu::Instance,
    pub SYSCFG_ITLINE: syscfg_itline::Instance,
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
            RCC: rcc::RCC::steal(),
            PWR: pwr::PWR::steal(),
            DMA: dma::DMA::steal(),
            DMAMUX: dmamux::DMAMUX::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOF: gpio::GPIOF::steal(),
            RNG: rng::RNG::steal(),
            CRC: crc::CRC::steal(),
            EXTI: exti::EXTI::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            TIM1: tim1::TIM1::steal(),
            ADC: adc::ADC::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            TAMP: tamp::TAMP::steal(),
            LPTIM1: lptim::LPTIM1::steal(),
            LPTIM2: lptim::LPTIM2::steal(),
            LPUART: lpuart::LPUART::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            RTC: rtc::RTC::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            NVIC: nvic::NVIC::steal(),
            MPU: mpu::MPU::steal(),
            STK: stk::STK::steal(),
            SCB: scb::SCB::steal(),
            VREFBUF: vrefbuf::VREFBUF::steal(),
            DBG: dbg::DBG::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            FPU_CPACR: fpu_cpacr::FPU_CPACR::steal(),
            FPU: fpu::FPU::steal(),
            SYSCFG_ITLINE: syscfg_itline::SYSCFG_ITLINE::steal(),
        }
    }
}

#[cfg(all(feature = "rtfm", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
