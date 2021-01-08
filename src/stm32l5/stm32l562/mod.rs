//! stm32ral module for stm32l562

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 3;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::dac;
pub use super::instances::dcb;
pub use super::instances::dfsdm1;
pub use super::instances::dma;
pub use super::instances::dmamux1;
pub use super::instances::exti;
pub use super::instances::flash;
pub use super::instances::gpio;
pub use super::instances::gtzc_mpcbb1;
pub use super::instances::gtzc_mpcbb2;
pub use super::instances::i2c;
pub use super::instances::icache;
pub use super::instances::iwdg;
pub use super::instances::lptim;
pub use super::instances::opamp;
pub use super::instances::pwr;
pub use super::instances::rcc;
pub use super::instances::rtc;
pub use super::instances::s;
pub use super::instances::sec_tim1;
pub use super::instances::sec_tim15;
pub use super::instances::sec_tim16;
pub use super::instances::sec_tim17;
pub use super::instances::sec_tim2;
pub use super::instances::sec_tim3;
pub use super::instances::sec_tim4;
pub use super::instances::sec_tim5;
pub use super::instances::sec_tim6;
pub use super::instances::sec_tim7;
pub use super::instances::spi;
pub use super::instances::tamp;
pub use super::instances::tim1;
pub use super::instances::tim15;
pub use super::instances::tim16;
pub use super::instances::tim17;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim4;
pub use super::instances::tim5;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub mod aes;
pub mod otfdec1;
pub mod pka;
pub use super::instances::adc;
pub use super::instances::adc_common;
pub use super::instances::comp;
pub use super::instances::crc;
pub use super::instances::crs;
pub use super::instances::dbgmcu;
pub use super::instances::fdcan1;
pub use super::instances::fmc;
pub use super::instances::gtzc_tzic;
pub use super::instances::gtzc_tzsc;
pub use super::instances::lpuart1;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::octospi1;
pub use super::instances::rng;
pub use super::instances::sdmmc1;
pub use super::instances::sec_tim8;
pub use super::instances::syscfg;
pub use super::instances::tim8;
pub use super::instances::tsc;
pub use super::instances::ucpd1;
pub use super::instances::usart;
pub use super::instances::usb;
pub use super::instances::vrefbuf;
pub use super::instances::wwdg;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub DCB: dcb::Instance,
    pub DFSDM1: dfsdm1::Instance,
    pub SEC_DFSDM1: dfsdm1::Instance,
    pub DMAMUX1: dmamux1::Instance,
    pub SEC_DMAMUX1: dmamux1::Instance,
    pub EXTI: exti::Instance,
    pub SEC_EXTI: exti::Instance,
    pub FLASH: flash::Instance,
    pub SEC_FLASH: flash::Instance,
    pub GPIOA: gpio::Instance,
    pub SEC_GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub SEC_GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub GPIOG: gpio::Instance,
    pub SEC_GPIOC: gpio::Instance,
    pub SEC_GPIOD: gpio::Instance,
    pub SEC_GPIOE: gpio::Instance,
    pub SEC_GPIOF: gpio::Instance,
    pub SEC_GPIOG: gpio::Instance,
    pub GPIOH: gpio::Instance,
    pub SEC_GPIOH: gpio::Instance,
    pub TAMP: tamp::Instance,
    pub SEC_TAMP: tamp::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub I2C4: i2c::Instance,
    pub SEC_I2C1: i2c::Instance,
    pub SEC_I2C2: i2c::Instance,
    pub SEC_I2C3: i2c::Instance,
    pub SEC_I2C4: i2c::Instance,
    pub ICache: icache::Instance,
    pub SEC_ICache: icache::Instance,
    pub IWDG: iwdg::Instance,
    pub SEC_IWDG: iwdg::Instance,
    pub LPTIM1: lptim::Instance,
    pub LPTIM2: lptim::Instance,
    pub LPTIM3: lptim::Instance,
    pub SEC_LPTIM1: lptim::Instance,
    pub SEC_LPTIM2: lptim::Instance,
    pub SEC_LPTIM3: lptim::Instance,
    pub GTZC_MPCBB1: gtzc_mpcbb1::Instance,
    pub SEC_GTZC_MPCBB1: gtzc_mpcbb1::Instance,
    pub GTZC_MPCBB2: gtzc_mpcbb2::Instance,
    pub SEC_GTZC_MPCBB2: gtzc_mpcbb2::Instance,
    pub PWR: pwr::Instance,
    pub SEC_PWR: pwr::Instance,
    pub RCC: rcc::Instance,
    pub SEC_RCC: rcc::Instance,
    pub RTC: rtc::Instance,
    pub SEC_RTC: rtc::Instance,
    pub SAI1: s::Instance,
    pub SAI2: s::Instance,
    pub SEC_SAI1: s::Instance,
    pub SEC_SAI2: s::Instance,
    pub DMA1: dma::Instance,
    pub SEC_DMA1: dma::Instance,
    pub DMA2: dma::Instance,
    pub SEC_DMA2: dma::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub SEC_SPI1: spi::Instance,
    pub SEC_SPI2: spi::Instance,
    pub SEC_SPI3: spi::Instance,
    pub TIM1: tim1::Instance,
    pub SEC_TIM1: sec_tim1::Instance,
    pub TIM15: tim15::Instance,
    pub SEC_TIM15: sec_tim15::Instance,
    pub TIM16: tim16::Instance,
    pub SEC_TIM16: sec_tim16::Instance,
    pub TIM17: tim17::Instance,
    pub SEC_TIM17: sec_tim17::Instance,
    pub TIM2: tim2::Instance,
    pub SEC_TIM2: sec_tim2::Instance,
    pub TIM3: tim3::Instance,
    pub SEC_TIM3: sec_tim3::Instance,
    pub TIM4: tim4::Instance,
    pub SEC_TIM4: sec_tim4::Instance,
    pub TIM5: tim5::Instance,
    pub SEC_TIM5: sec_tim5::Instance,
    pub TIM6: tim6::Instance,
    pub SEC_TIM6: sec_tim6::Instance,
    pub TIM7: tim7::Instance,
    pub SEC_TIM7: sec_tim7::Instance,
    pub DAC: dac::Instance,
    pub SEC_DAC: dac::Instance,
    pub OPAMP: opamp::Instance,
    pub SEC_OPAMP: opamp::Instance,
    pub AES: aes::Instance,
    pub SEC_AES: aes::Instance,
    pub PKA: pka::Instance,
    pub SEC_PKA: pka::Instance,
    pub OTFDEC1: otfdec1::Instance,
    pub SEC_OTFDEC1: otfdec1::Instance,
    pub TIM8: tim8::Instance,
    pub SEC_TIM8: sec_tim8::Instance,
    pub GTZC_TZIC: gtzc_tzic::Instance,
    pub SEC_GTZC_TZIC: gtzc_tzic::Instance,
    pub GTZC_TZSC: gtzc_tzsc::Instance,
    pub SEC_GTZC_TZSC: gtzc_tzsc::Instance,
    pub WWDG: wwdg::Instance,
    pub SEC_WWDG: wwdg::Instance,
    pub SYSCFG: syscfg::Instance,
    pub SEC_SYSCFG: syscfg::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub USB: usb::Instance,
    pub SEC_USB: usb::Instance,
    pub OCTOSPI1: octospi1::Instance,
    pub SEC_OCTOSPI1: octospi1::Instance,
    pub LPUART1: lpuart1::Instance,
    pub SEC_LPUART1: lpuart1::Instance,
    pub COMP: comp::Instance,
    pub SEC_COMP: comp::Instance,
    pub VREFBUF: vrefbuf::Instance,
    pub SEC_VREFBUF: vrefbuf::Instance,
    pub TSC: tsc::Instance,
    pub SEC_TSC: tsc::Instance,
    pub UCPD1: ucpd1::Instance,
    pub SEC_UCPD1: ucpd1::Instance,
    pub FDCAN1: fdcan1::Instance,
    pub SEC_FDCAN1: fdcan1::Instance,
    pub CRC: crc::Instance,
    pub SEC_CRC: crc::Instance,
    pub CRS: crs::Instance,
    pub SEC_CRS: crs::Instance,
    pub USART1: usart::Instance,
    pub SEC_USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub SEC_USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub SEC_USART3: usart::Instance,
    pub UART4: usart::Instance,
    pub UART5: usart::Instance,
    pub SEC_UART4: usart::Instance,
    pub SEC_UART5: usart::Instance,
    pub ADC_Common: adc_common::Instance,
    pub SEC_ADC_Common: adc_common::Instance,
    pub ADC: adc::Instance,
    pub SEC_ADC: adc::Instance,
    pub NVIC: nvic::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub FMC: fmc::Instance,
    pub SEC_FMC: fmc::Instance,
    pub RNG: rng::Instance,
    pub SEC_RNG: rng::Instance,
    pub SDMMC1: sdmmc1::Instance,
    pub SEC_SDMMC1: sdmmc1::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            DCB: dcb::DCB::steal(),
            DFSDM1: dfsdm1::DFSDM1::steal(),
            SEC_DFSDM1: dfsdm1::SEC_DFSDM1::steal(),
            DMAMUX1: dmamux1::DMAMUX1::steal(),
            SEC_DMAMUX1: dmamux1::SEC_DMAMUX1::steal(),
            EXTI: exti::EXTI::steal(),
            SEC_EXTI: exti::SEC_EXTI::steal(),
            FLASH: flash::FLASH::steal(),
            SEC_FLASH: flash::SEC_FLASH::steal(),
            GPIOA: gpio::GPIOA::steal(),
            SEC_GPIOA: gpio::SEC_GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            SEC_GPIOB: gpio::SEC_GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOE: gpio::GPIOE::steal(),
            GPIOF: gpio::GPIOF::steal(),
            GPIOG: gpio::GPIOG::steal(),
            SEC_GPIOC: gpio::SEC_GPIOC::steal(),
            SEC_GPIOD: gpio::SEC_GPIOD::steal(),
            SEC_GPIOE: gpio::SEC_GPIOE::steal(),
            SEC_GPIOF: gpio::SEC_GPIOF::steal(),
            SEC_GPIOG: gpio::SEC_GPIOG::steal(),
            GPIOH: gpio::GPIOH::steal(),
            SEC_GPIOH: gpio::SEC_GPIOH::steal(),
            TAMP: tamp::TAMP::steal(),
            SEC_TAMP: tamp::SEC_TAMP::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C3: i2c::I2C3::steal(),
            I2C4: i2c::I2C4::steal(),
            SEC_I2C1: i2c::SEC_I2C1::steal(),
            SEC_I2C2: i2c::SEC_I2C2::steal(),
            SEC_I2C3: i2c::SEC_I2C3::steal(),
            SEC_I2C4: i2c::SEC_I2C4::steal(),
            ICache: icache::ICache::steal(),
            SEC_ICache: icache::SEC_ICache::steal(),
            IWDG: iwdg::IWDG::steal(),
            SEC_IWDG: iwdg::SEC_IWDG::steal(),
            LPTIM1: lptim::LPTIM1::steal(),
            LPTIM2: lptim::LPTIM2::steal(),
            LPTIM3: lptim::LPTIM3::steal(),
            SEC_LPTIM1: lptim::SEC_LPTIM1::steal(),
            SEC_LPTIM2: lptim::SEC_LPTIM2::steal(),
            SEC_LPTIM3: lptim::SEC_LPTIM3::steal(),
            GTZC_MPCBB1: gtzc_mpcbb1::GTZC_MPCBB1::steal(),
            SEC_GTZC_MPCBB1: gtzc_mpcbb1::SEC_GTZC_MPCBB1::steal(),
            GTZC_MPCBB2: gtzc_mpcbb2::GTZC_MPCBB2::steal(),
            SEC_GTZC_MPCBB2: gtzc_mpcbb2::SEC_GTZC_MPCBB2::steal(),
            PWR: pwr::PWR::steal(),
            SEC_PWR: pwr::SEC_PWR::steal(),
            RCC: rcc::RCC::steal(),
            SEC_RCC: rcc::SEC_RCC::steal(),
            RTC: rtc::RTC::steal(),
            SEC_RTC: rtc::SEC_RTC::steal(),
            SAI1: s::SAI1::steal(),
            SAI2: s::SAI2::steal(),
            SEC_SAI1: s::SEC_SAI1::steal(),
            SEC_SAI2: s::SEC_SAI2::steal(),
            DMA1: dma::DMA1::steal(),
            SEC_DMA1: dma::SEC_DMA1::steal(),
            DMA2: dma::DMA2::steal(),
            SEC_DMA2: dma::SEC_DMA2::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            SEC_SPI1: spi::SEC_SPI1::steal(),
            SEC_SPI2: spi::SEC_SPI2::steal(),
            SEC_SPI3: spi::SEC_SPI3::steal(),
            TIM1: tim1::TIM1::steal(),
            SEC_TIM1: sec_tim1::SEC_TIM1::steal(),
            TIM15: tim15::TIM15::steal(),
            SEC_TIM15: sec_tim15::SEC_TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            SEC_TIM16: sec_tim16::SEC_TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            SEC_TIM17: sec_tim17::SEC_TIM17::steal(),
            TIM2: tim2::TIM2::steal(),
            SEC_TIM2: sec_tim2::SEC_TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            SEC_TIM3: sec_tim3::SEC_TIM3::steal(),
            TIM4: tim4::TIM4::steal(),
            SEC_TIM4: sec_tim4::SEC_TIM4::steal(),
            TIM5: tim5::TIM5::steal(),
            SEC_TIM5: sec_tim5::SEC_TIM5::steal(),
            TIM6: tim6::TIM6::steal(),
            SEC_TIM6: sec_tim6::SEC_TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            SEC_TIM7: sec_tim7::SEC_TIM7::steal(),
            DAC: dac::DAC::steal(),
            SEC_DAC: dac::SEC_DAC::steal(),
            OPAMP: opamp::OPAMP::steal(),
            SEC_OPAMP: opamp::SEC_OPAMP::steal(),
            AES: aes::AES::steal(),
            SEC_AES: aes::SEC_AES::steal(),
            PKA: pka::PKA::steal(),
            SEC_PKA: pka::SEC_PKA::steal(),
            OTFDEC1: otfdec1::OTFDEC1::steal(),
            SEC_OTFDEC1: otfdec1::SEC_OTFDEC1::steal(),
            TIM8: tim8::TIM8::steal(),
            SEC_TIM8: sec_tim8::SEC_TIM8::steal(),
            GTZC_TZIC: gtzc_tzic::GTZC_TZIC::steal(),
            SEC_GTZC_TZIC: gtzc_tzic::SEC_GTZC_TZIC::steal(),
            GTZC_TZSC: gtzc_tzsc::GTZC_TZSC::steal(),
            SEC_GTZC_TZSC: gtzc_tzsc::SEC_GTZC_TZSC::steal(),
            WWDG: wwdg::WWDG::steal(),
            SEC_WWDG: wwdg::SEC_WWDG::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            SEC_SYSCFG: syscfg::SEC_SYSCFG::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            USB: usb::USB::steal(),
            SEC_USB: usb::SEC_USB::steal(),
            OCTOSPI1: octospi1::OCTOSPI1::steal(),
            SEC_OCTOSPI1: octospi1::SEC_OCTOSPI1::steal(),
            LPUART1: lpuart1::LPUART1::steal(),
            SEC_LPUART1: lpuart1::SEC_LPUART1::steal(),
            COMP: comp::COMP::steal(),
            SEC_COMP: comp::SEC_COMP::steal(),
            VREFBUF: vrefbuf::VREFBUF::steal(),
            SEC_VREFBUF: vrefbuf::SEC_VREFBUF::steal(),
            TSC: tsc::TSC::steal(),
            SEC_TSC: tsc::SEC_TSC::steal(),
            UCPD1: ucpd1::UCPD1::steal(),
            SEC_UCPD1: ucpd1::SEC_UCPD1::steal(),
            FDCAN1: fdcan1::FDCAN1::steal(),
            SEC_FDCAN1: fdcan1::SEC_FDCAN1::steal(),
            CRC: crc::CRC::steal(),
            SEC_CRC: crc::SEC_CRC::steal(),
            CRS: crs::CRS::steal(),
            SEC_CRS: crs::SEC_CRS::steal(),
            USART1: usart::USART1::steal(),
            SEC_USART1: usart::SEC_USART1::steal(),
            USART2: usart::USART2::steal(),
            SEC_USART2: usart::SEC_USART2::steal(),
            USART3: usart::USART3::steal(),
            SEC_USART3: usart::SEC_USART3::steal(),
            UART4: usart::UART4::steal(),
            UART5: usart::UART5::steal(),
            SEC_UART4: usart::SEC_UART4::steal(),
            SEC_UART5: usart::SEC_UART5::steal(),
            ADC_Common: adc_common::ADC_Common::steal(),
            SEC_ADC_Common: adc_common::SEC_ADC_Common::steal(),
            ADC: adc::ADC::steal(),
            SEC_ADC: adc::SEC_ADC::steal(),
            NVIC: nvic::NVIC::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            FMC: fmc::FMC::steal(),
            SEC_FMC: fmc::SEC_FMC::steal(),
            RNG: rng::RNG::steal(),
            SEC_RNG: rng::SEC_RNG::steal(),
            SDMMC1: sdmmc1::SDMMC1::steal(),
            SEC_SDMMC1: sdmmc1::SEC_SDMMC1::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
