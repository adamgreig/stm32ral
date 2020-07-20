//! stm32ral module for stm32f3x4

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub mod gpio;
pub use super::instances::crc;
pub use super::instances::flash_f302_f303_f3x4 as flash;
pub use super::instances::tsc;
pub mod dma1;
pub mod rcc;
pub use super::instances::tim15_f302_f303_f3x4 as tim15;
pub use super::instances::tim16_f302_f303_f3x4 as tim16;
pub use super::instances::tim17_f302_f303_f3x4 as tim17;
pub use super::instances::tim2_f302_f303_f3x4 as tim2;
pub use super::instances::usart_f301_f373_f3x4 as usart;
pub mod spi;
pub use super::instances::exti_f302_f303_f3x4 as exti;
pub mod pwr;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::rtc_f302_f303_f3x4 as rtc;
pub use super::instances::tim6_f302_f303_f3x4 as tim6;
pub use super::instances::tim7_f302_f303_f3x4 as tim7;
pub use super::instances::wwdg;
pub mod dac;
pub use super::instances::adc_f302_f3x4 as adc;
pub use super::instances::dbgmcu;
pub use super::instances::tim1;
pub mod syscfg;
pub use super::instances::can;
pub use super::instances::tim3_f301_f373_f3x4_f3x8 as tim3;
pub mod adc_common;
pub mod hrtim_common;
pub mod hrtim_master;
pub mod hrtim_tima;
pub mod hrtim_timb;
pub mod hrtim_timc;
pub mod hrtim_timd;
pub mod hrtim_time;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::opamp_f301_f3x4 as opamp;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;
pub mod comp;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub TSC: tsc::Instance,
    pub CRC: crc::Instance,
    pub Flash: flash::Instance,
    pub RCC: rcc::Instance,
    pub DMA1: dma1::Instance,
    pub TIM2: tim2::Instance,
    pub TIM15: tim15::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub SPI2: spi::Instance,
    pub I2S2ext: spi::Instance,
    pub I2S3ext: spi::Instance,
    pub SPI3: spi::Instance,
    pub SPI1: spi::Instance,
    pub EXTI: exti::Instance,
    pub PWR: pwr::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub RTC: rtc::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub DAC1: dac::Instance,
    pub DAC2: dac::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub TIM1: tim1::Instance,
    pub ADC1: adc::Instance,
    pub ADC2: adc::Instance,
    pub SYSCFG: syscfg::Instance,
    pub TIM3: tim3::Instance,
    pub CAN: can::Instance,
    pub ADC_Common: adc_common::Instance,
    pub HRTIM_Master: hrtim_master::Instance,
    pub HRTIM_TIMA: hrtim_tima::Instance,
    pub HRTIM_TIMB: hrtim_timb::Instance,
    pub HRTIM_TIMC: hrtim_timc::Instance,
    pub HRTIM_TIMD: hrtim_timd::Instance,
    pub HRTIM_TIME: hrtim_time::Instance,
    pub HRTIM_Common: hrtim_common::Instance,
    pub NVIC: nvic::Instance,
    pub FPU: fpu::Instance,
    pub MPU: mpu::Instance,
    pub STK: stk::Instance,
    pub SCB: scb::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub FPU_CPACR: fpu_cpacr::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub OPAMP: opamp::Instance,
    pub COMP: comp::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOF: gpio::GPIOF::steal(),
            TSC: tsc::TSC::steal(),
            CRC: crc::CRC::steal(),
            Flash: flash::Flash::steal(),
            RCC: rcc::RCC::steal(),
            DMA1: dma1::DMA1::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM15: tim15::TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            SPI2: spi::SPI2::steal(),
            I2S2ext: spi::I2S2ext::steal(),
            I2S3ext: spi::I2S3ext::steal(),
            SPI3: spi::SPI3::steal(),
            SPI1: spi::SPI1::steal(),
            EXTI: exti::EXTI::steal(),
            PWR: pwr::PWR::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C3: i2c::I2C3::steal(),
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            RTC: rtc::RTC::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            DAC1: dac::DAC1::steal(),
            DAC2: dac::DAC2::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            TIM1: tim1::TIM1::steal(),
            ADC1: adc::ADC1::steal(),
            ADC2: adc::ADC2::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            TIM3: tim3::TIM3::steal(),
            CAN: can::CAN::steal(),
            ADC_Common: adc_common::ADC_Common::steal(),
            HRTIM_Master: hrtim_master::HRTIM_Master::steal(),
            HRTIM_TIMA: hrtim_tima::HRTIM_TIMA::steal(),
            HRTIM_TIMB: hrtim_timb::HRTIM_TIMB::steal(),
            HRTIM_TIMC: hrtim_timc::HRTIM_TIMC::steal(),
            HRTIM_TIMD: hrtim_timd::HRTIM_TIMD::steal(),
            HRTIM_TIME: hrtim_time::HRTIM_TIME::steal(),
            HRTIM_Common: hrtim_common::HRTIM_Common::steal(),
            NVIC: nvic::NVIC::steal(),
            FPU: fpu::FPU::steal(),
            MPU: mpu::MPU::steal(),
            STK: stk::STK::steal(),
            SCB: scb::SCB::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            FPU_CPACR: fpu_cpacr::FPU_CPACR::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            OPAMP: opamp::OPAMP::steal(),
            COMP: comp::COMP::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
