//! stm32ral module for stm32g471

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::aes;
pub use super::instances::comp_g431_g441_g471 as comp;
pub use super::instances::crc;
pub use super::instances::dac;
pub use super::instances::dbgmcu;
pub use super::instances::dma;
pub use super::instances::dmamux;
pub use super::instances::exti;
pub use super::instances::flash;
pub use super::instances::gpio;
pub use super::instances::i2c_g471_g473_g474_g483_g484 as i2c;
pub use super::instances::iwdg;
pub use super::instances::lptimer1;
pub use super::instances::lpuart1;
pub use super::instances::opamp_g431_g441_g471 as opamp;
pub use super::instances::pwr;
pub use super::instances::rcc;
pub use super::instances::rng;
pub use super::instances::rtc;
pub use super::instances::spi_g471_g473_g474_g483_g484 as spi;
pub use super::instances::syscfg;
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
pub use super::instances::tim8;
pub use super::instances::usart_g471_g473_g474_g483_g484 as usart;
pub use super::instances::vrefbuf;
pub use super::instances::wwdg;
pub mod adc;
pub mod adc12_common;
pub use super::instances::cordic;
pub use super::instances::fmac;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::sai;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;
pub use super::instances::tamp;
pub mod fdcan;
pub use super::instances::crs;
pub use super::instances::ucpd1;
pub use super::instances::usb_fs_device;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub CRC: crc::Instance,
    pub WWDG: wwdg::Instance,
    pub IWDG: iwdg::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub I2C4: i2c::Instance,
    pub FLASH: flash::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub RCC: rcc::Instance,
    pub PWR: pwr::Instance,
    pub RNG: rng::Instance,
    pub AES: aes::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub GPIOG: gpio::Instance,
    pub TIM15: tim15::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub TIM1: tim1::Instance,
    pub TIM8: tim8::Instance,
    pub TIM2: tim2::Instance,
    pub TIM3: tim3::Instance,
    pub TIM4: tim4::Instance,
    pub TIM5: tim5::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub LPTIMER1: lptimer1::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub UART4: usart::Instance,
    pub UART5: usart::Instance,
    pub LPUART1: lpuart1::Instance,
    pub SPI1: spi::Instance,
    pub SPI4: spi::Instance,
    pub SPI3: spi::Instance,
    pub SPI2: spi::Instance,
    pub EXTI: exti::Instance,
    pub RTC: rtc::Instance,
    pub DMA1: dma::Instance,
    pub DMA2: dma::Instance,
    pub DMAMUX: dmamux::Instance,
    pub SYSCFG: syscfg::Instance,
    pub VREFBUF: vrefbuf::Instance,
    pub COMP: comp::Instance,
    pub OPAMP: opamp::Instance,
    pub DAC1: dac::Instance,
    pub DAC2: dac::Instance,
    pub DAC3: dac::Instance,
    pub DAC4: dac::Instance,
    pub ADC1: adc::Instance,
    pub ADC2: adc::Instance,
    pub ADC3: adc::Instance,
    pub ADC12_Common: adc12_common::Instance,
    pub ADC345_Common: adc12_common::Instance,
    pub FMAC: fmac::Instance,
    pub CORDIC: cordic::Instance,
    pub SAI: sai::Instance,
    pub TAMP: tamp::Instance,
    pub FPU: fpu::Instance,
    pub MPU: mpu::Instance,
    pub STK: stk::Instance,
    pub SCB: scb::Instance,
    pub NVIC: nvic::Instance,
    pub FPU_CPACR: fpu_cpacr::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub FDCAN: fdcan::Instance,
    pub FDCAN1: fdcan::Instance,
    pub FDCAN2: fdcan::Instance,
    pub UCPD1: ucpd1::Instance,
    pub USB_FS_device: usb_fs_device::Instance,
    pub CRS: crs::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            CRC: crc::CRC::steal(),
            WWDG: wwdg::WWDG::steal(),
            IWDG: iwdg::IWDG::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C3: i2c::I2C3::steal(),
            I2C4: i2c::I2C4::steal(),
            FLASH: flash::FLASH::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            RCC: rcc::RCC::steal(),
            PWR: pwr::PWR::steal(),
            RNG: rng::RNG::steal(),
            AES: aes::AES::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOE: gpio::GPIOE::steal(),
            GPIOF: gpio::GPIOF::steal(),
            GPIOG: gpio::GPIOG::steal(),
            TIM15: tim15::TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM8: tim8::TIM8::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM4: tim4::TIM4::steal(),
            TIM5: tim5::TIM5::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            LPTIMER1: lptimer1::LPTIMER1::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            UART4: usart::UART4::steal(),
            UART5: usart::UART5::steal(),
            LPUART1: lpuart1::LPUART1::steal(),
            SPI1: spi::SPI1::steal(),
            SPI4: spi::SPI4::steal(),
            SPI3: spi::SPI3::steal(),
            SPI2: spi::SPI2::steal(),
            EXTI: exti::EXTI::steal(),
            RTC: rtc::RTC::steal(),
            DMA1: dma::DMA1::steal(),
            DMA2: dma::DMA2::steal(),
            DMAMUX: dmamux::DMAMUX::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            VREFBUF: vrefbuf::VREFBUF::steal(),
            COMP: comp::COMP::steal(),
            OPAMP: opamp::OPAMP::steal(),
            DAC1: dac::DAC1::steal(),
            DAC2: dac::DAC2::steal(),
            DAC3: dac::DAC3::steal(),
            DAC4: dac::DAC4::steal(),
            ADC1: adc::ADC1::steal(),
            ADC2: adc::ADC2::steal(),
            ADC3: adc::ADC3::steal(),
            ADC12_Common: adc12_common::ADC12_Common::steal(),
            ADC345_Common: adc12_common::ADC345_Common::steal(),
            FMAC: fmac::FMAC::steal(),
            CORDIC: cordic::CORDIC::steal(),
            SAI: sai::SAI::steal(),
            TAMP: tamp::TAMP::steal(),
            FPU: fpu::FPU::steal(),
            MPU: mpu::MPU::steal(),
            STK: stk::STK::steal(),
            SCB: scb::SCB::steal(),
            NVIC: nvic::NVIC::steal(),
            FPU_CPACR: fpu_cpacr::FPU_CPACR::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            FDCAN: fdcan::FDCAN::steal(),
            FDCAN1: fdcan::FDCAN1::steal(),
            FDCAN2: fdcan::FDCAN2::steal(),
            UCPD1: ucpd1::UCPD1::steal(),
            USB_FS_device: usb_fs_device::USB_FS_device::steal(),
            CRS: crs::CRS::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
