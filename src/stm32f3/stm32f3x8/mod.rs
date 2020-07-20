//! stm32ral module for stm32f3x8

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub mod gpio;
pub mod gpioc;
pub use super::instances::crc;
pub use super::instances::flash_f373_f3x8 as flash;
pub use super::instances::tsc;
pub mod rcc;
pub use super::instances::dma;
pub use super::instances::tim15_f301_f373_f3x8 as tim15;
pub use super::instances::tim16_f301_f373_f3x8 as tim16;
pub use super::instances::tim17_f301_f373_f3x8 as tim17;
pub use super::instances::tim19;
pub use super::instances::tim2_f373_f3x8 as tim2;
pub use super::instances::tim3_f301_f373_f3x4_f3x8 as tim3;
pub use super::instances::tim4_f301_f373_f3x8 as tim4;
pub use super::instances::tim5;
pub mod tim20;
pub use super::instances::usart_f302_f303_f3x8 as usart;
pub mod spi;
pub use super::instances::exti_f301_f3x8 as exti;
pub mod pwr;
pub use super::instances::can;
pub use super::instances::dac1;
pub use super::instances::dac2;
pub use super::instances::dbgmcu;
pub use super::instances::fmc;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::rtc_f301_f373_f3x8 as rtc;
pub use super::instances::sdadc;
pub use super::instances::tim1;
pub use super::instances::tim12;
pub use super::instances::tim13;
pub use super::instances::tim14;
pub use super::instances::tim18;
pub use super::instances::tim6_f301_f373_f3x8 as tim6;
pub use super::instances::tim7_f301_f373_f3x8 as tim7;
pub use super::instances::usb;
pub use super::instances::wwdg;
pub mod syscfg;
pub mod tim8;
pub use super::instances::adc_common;
pub use super::instances::adc_f303_f3x8 as adc;
pub use super::instances::comp;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::opamp_f303_f3x8 as opamp;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub GPIOA: gpio::Instance,
    pub GPIOH: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOC: gpioc::Instance,
    pub GPIOE: gpioc::Instance,
    pub GPIOF: gpioc::Instance,
    pub GPIOG: gpioc::Instance,
    pub TSC: tsc::Instance,
    pub CRC: crc::Instance,
    pub Flash: flash::Instance,
    pub RCC: rcc::Instance,
    pub DMA1: dma::Instance,
    pub DMA2: dma::Instance,
    pub TIM2: tim2::Instance,
    pub TIM5: tim5::Instance,
    pub TIM3: tim3::Instance,
    pub TIM4: tim4::Instance,
    pub TIM19: tim19::Instance,
    pub TIM15: tim15::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub TIM20: tim20::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub UART4: usart::Instance,
    pub UART5: usart::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub I2S2ext: spi::Instance,
    pub I2S3ext: spi::Instance,
    pub SPI4: spi::Instance,
    pub EXTI: exti::Instance,
    pub PWR: pwr::Instance,
    pub CAN: can::Instance,
    pub USB: usb::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub RTC: rtc::Instance,
    pub SDADC1: sdadc::Instance,
    pub SDADC2: sdadc::Instance,
    pub SDADC3: sdadc::Instance,
    pub DAC2: dac2::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub TIM18: tim18::Instance,
    pub TIM13: tim13::Instance,
    pub TIM14: tim14::Instance,
    pub TIM12: tim12::Instance,
    pub DAC1: dac1::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub FMC: fmc::Instance,
    pub TIM1: tim1::Instance,
    pub TIM8: tim8::Instance,
    pub SYSCFG: syscfg::Instance,
    pub NVIC: nvic::Instance,
    pub FPU: fpu::Instance,
    pub MPU: mpu::Instance,
    pub STK: stk::Instance,
    pub SCB: scb::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub FPU_CPACR: fpu_cpacr::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub ADC1: adc::Instance,
    pub ADC2: adc::Instance,
    pub ADC3: adc::Instance,
    pub ADC4: adc::Instance,
    pub ADC1_2: adc_common::Instance,
    pub ADC3_4: adc_common::Instance,
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
            GPIOH: gpio::GPIOH::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOC: gpioc::GPIOC::steal(),
            GPIOE: gpioc::GPIOE::steal(),
            GPIOF: gpioc::GPIOF::steal(),
            GPIOG: gpioc::GPIOG::steal(),
            TSC: tsc::TSC::steal(),
            CRC: crc::CRC::steal(),
            Flash: flash::Flash::steal(),
            RCC: rcc::RCC::steal(),
            DMA1: dma::DMA1::steal(),
            DMA2: dma::DMA2::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM5: tim5::TIM5::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM4: tim4::TIM4::steal(),
            TIM19: tim19::TIM19::steal(),
            TIM15: tim15::TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            TIM20: tim20::TIM20::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            UART4: usart::UART4::steal(),
            UART5: usart::UART5::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            I2S2ext: spi::I2S2ext::steal(),
            I2S3ext: spi::I2S3ext::steal(),
            SPI4: spi::SPI4::steal(),
            EXTI: exti::EXTI::steal(),
            PWR: pwr::PWR::steal(),
            CAN: can::CAN::steal(),
            USB: usb::USB::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C3: i2c::I2C3::steal(),
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            RTC: rtc::RTC::steal(),
            SDADC1: sdadc::SDADC1::steal(),
            SDADC2: sdadc::SDADC2::steal(),
            SDADC3: sdadc::SDADC3::steal(),
            DAC2: dac2::DAC2::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            TIM18: tim18::TIM18::steal(),
            TIM13: tim13::TIM13::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM12: tim12::TIM12::steal(),
            DAC1: dac1::DAC1::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            FMC: fmc::FMC::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM8: tim8::TIM8::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            NVIC: nvic::NVIC::steal(),
            FPU: fpu::FPU::steal(),
            MPU: mpu::MPU::steal(),
            STK: stk::STK::steal(),
            SCB: scb::SCB::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            FPU_CPACR: fpu_cpacr::FPU_CPACR::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            ADC1: adc::ADC1::steal(),
            ADC2: adc::ADC2::steal(),
            ADC3: adc::ADC3::steal(),
            ADC4: adc::ADC4::steal(),
            ADC1_2: adc_common::ADC1_2::steal(),
            ADC3_4: adc_common::ADC3_4::steal(),
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
