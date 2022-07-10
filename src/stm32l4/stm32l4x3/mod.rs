//! stm32ral module for stm32l4x3

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::comp_l4x3_l4x5_l4x6 as comp;
pub use super::instances::crc;
pub use super::instances::dac_l4x1_l4x2_l4x3_l4x5_l4x6 as dac;
pub use super::instances::dma_l412_l4x1_l4x2_l4x3_l4x5_l4x6 as dma;
pub use super::instances::firewall;
pub use super::instances::flash;
pub use super::instances::i2c_l4x3_l4x5 as i2c;
pub use super::instances::iwdg;
pub use super::instances::lcd_l4x3_l4x6 as lcd;
pub use super::instances::tsc;
pub use super::instances::wwdg;
pub mod rcc;
pub use super::instances::aes;
pub use super::instances::pwr;
pub use super::instances::rng;
pub use super::instances::syscfg;
pub mod adc1;
pub use super::instances::gpio_l4x1_l4x2_l4x3 as gpio;
pub use super::instances::lptim;
pub use super::instances::sai1;
pub use super::instances::tim1;
pub use super::instances::tim15_l412_l4r5_l4r9_l4x2_l4x3_l4x6 as tim15;
pub use super::instances::tim16;
pub use super::instances::tim2;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub mod usart;
pub use super::instances::can1_l4x3_l4x5 as can1;
pub use super::instances::crs;
pub use super::instances::exti;
pub use super::instances::lpuart1_l412_l4x1_l4x2_l4x3_l4x5_l4x6 as lpuart1;
pub use super::instances::nvic;
pub use super::instances::opamp_l4x1_l4x2_l4x3_l4x5_l4x6 as opamp;
pub use super::instances::rtc;
pub use super::instances::sdmmc;
pub use super::instances::spi_l4x3_l4x5_l4x6 as spi;
pub use super::instances::swpmi1;
pub use super::instances::vrefbuf;
pub mod usb;
pub use super::instances::adc_common_l412_l4x1_l4x2_l4x3 as adc_common;
pub use super::instances::dbgmcu_l412_l4x1_l4x2_l4x3 as dbgmcu;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::mpu_l412_l4x1_l4x2_l4x3_l4x5_l4x6 as mpu;
pub use super::instances::nvic_stir;
pub use super::instances::scb_actrl;
pub use super::instances::scb_l412_l4x1_l4x2_l4x3_l4x5_l4x6 as scb;
pub use super::instances::stk;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub DAC: dac::Instance,
    pub DMA1: dma::Instance,
    pub DMA2: dma::Instance,
    pub CRC: crc::Instance,
    pub LCD: lcd::Instance,
    pub TSC: tsc::Instance,
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub COMP: comp::Instance,
    pub FIREWALL: firewall::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub FLASH: flash::Instance,
    pub RCC: rcc::Instance,
    pub PWR: pwr::Instance,
    pub SYSCFG: syscfg::Instance,
    pub RNG: rng::Instance,
    pub AES: aes::Instance,
    pub ADC1: adc1::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub GPIOH: gpio::Instance,
    pub SAI1: sai1::Instance,
    pub TIM2: tim2::Instance,
    pub TIM15: tim15::Instance,
    pub TIM16: tim16::Instance,
    pub TIM1: tim1::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub LPTIM1: lptim::Instance,
    pub LPTIM2: lptim::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub LPUART1: lpuart1::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub SDMMC: sdmmc::Instance,
    pub EXTI: exti::Instance,
    pub VREFBUF: vrefbuf::Instance,
    pub CAN1: can1::Instance,
    pub RTC: rtc::Instance,
    pub SWPMI1: swpmi1::Instance,
    pub OPAMP: opamp::Instance,
    pub NVIC: nvic::Instance,
    pub CRS: crs::Instance,
    pub USB: usb::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub FPU: fpu::Instance,
    pub MPU: mpu::Instance,
    pub STK: stk::Instance,
    pub SCB: scb::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub FPU_CPACR: fpu_cpacr::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub ADC_Common: adc_common::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            DAC: dac::DAC::steal(),
            DMA1: dma::DMA1::steal(),
            DMA2: dma::DMA2::steal(),
            CRC: crc::CRC::steal(),
            LCD: lcd::LCD::steal(),
            TSC: tsc::TSC::steal(),
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            COMP: comp::COMP::steal(),
            FIREWALL: firewall::FIREWALL::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C3: i2c::I2C3::steal(),
            FLASH: flash::FLASH::steal(),
            RCC: rcc::RCC::steal(),
            PWR: pwr::PWR::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            RNG: rng::RNG::steal(),
            AES: aes::AES::steal(),
            ADC1: adc1::ADC1::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOE: gpio::GPIOE::steal(),
            GPIOH: gpio::GPIOH::steal(),
            SAI1: sai1::SAI1::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM15: tim15::TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            LPTIM1: lptim::LPTIM1::steal(),
            LPTIM2: lptim::LPTIM2::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            LPUART1: lpuart1::LPUART1::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            SDMMC: sdmmc::SDMMC::steal(),
            EXTI: exti::EXTI::steal(),
            VREFBUF: vrefbuf::VREFBUF::steal(),
            CAN1: can1::CAN1::steal(),
            RTC: rtc::RTC::steal(),
            SWPMI1: swpmi1::SWPMI1::steal(),
            OPAMP: opamp::OPAMP::steal(),
            NVIC: nvic::NVIC::steal(),
            CRS: crs::CRS::steal(),
            USB: usb::USB::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            FPU: fpu::FPU::steal(),
            MPU: mpu::MPU::steal(),
            STK: stk::STK::steal(),
            SCB: scb::SCB::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            FPU_CPACR: fpu_cpacr::FPU_CPACR::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            ADC_Common: adc_common::ADC_Common::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
