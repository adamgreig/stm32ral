//! stm32ral module for stm32wb55

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub mod adc;
pub mod aes1;
pub mod aes2;
pub mod comp;
pub mod crc;
pub mod crs;
pub mod dbgmcu;
pub mod dma1;
pub mod dma2;
pub mod dmamux1;
pub mod exti;
pub mod flash;
pub mod fpu;
pub mod fpu_cpacr;
pub mod gpio;
pub mod gpioe;
pub mod gpioh;
pub mod hsem;
pub mod i2c;
pub mod ipcc;
pub mod iwdg;
pub mod lcd;
pub mod lptim;
pub mod mpu;
pub mod nvic;
pub mod nvic_stir;
pub mod pka;
pub mod pwr;
pub mod quadspi;
pub mod rcc;
pub mod rng;
pub mod rtc;
pub mod sai1;
pub mod scb;
pub mod scb_actrl;
pub mod spi;
pub mod stk;
pub mod syscfg;
pub mod tim1;
pub mod tim16;
pub mod tim17;
pub mod tim2;
pub mod tsc;
pub mod usart1;
pub mod usb;
pub mod vrefbuf;
pub mod wwdg;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub DMA1: dma1::Instance,
    pub DMA2: dma2::Instance,
    pub DMAMUX1: dmamux1::Instance,
    pub CRC: crc::Instance,
    pub LCD: lcd::Instance,
    pub TSC: tsc::Instance,
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub COMP: comp::Instance,
    pub I2C1: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub Flash: flash::Instance,
    pub QUADSPI: quadspi::Instance,
    pub RCC: rcc::Instance,
    pub PWR: pwr::Instance,
    pub SYSCFG: syscfg::Instance,
    pub RNG: rng::Instance,
    pub AES1: aes1::Instance,
    pub AES2: aes2::Instance,
    pub HSEM: hsem::Instance,
    pub ADC: adc::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOE: gpioe::Instance,
    pub GPIOH: gpioh::Instance,
    pub SAI1: sai1::Instance,
    pub TIM2: tim2::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub TIM1: tim1::Instance,
    pub LPTIM1: lptim::Instance,
    pub LPTIM2: lptim::Instance,
    pub USART1: usart1::Instance,
    pub LPUART1: usart1::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub VREFBUF: vrefbuf::Instance,
    pub RTC: rtc::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub PKA: pka::Instance,
    pub IPCC: ipcc::Instance,
    pub EXTI: exti::Instance,
    pub CRS: crs::Instance,
    pub USB: usb::Instance,
    pub SCB: scb::Instance,
    pub STK: stk::Instance,
    pub MPU: mpu::Instance,
    pub FPU: fpu::Instance,
    pub NVIC: nvic::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub FPU_CPACR: fpu_cpacr::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            DMA1: dma1::DMA1::steal(),
            DMA2: dma2::DMA2::steal(),
            DMAMUX1: dmamux1::DMAMUX1::steal(),
            CRC: crc::CRC::steal(),
            LCD: lcd::LCD::steal(),
            TSC: tsc::TSC::steal(),
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            COMP: comp::COMP::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C3: i2c::I2C3::steal(),
            Flash: flash::Flash::steal(),
            QUADSPI: quadspi::QUADSPI::steal(),
            RCC: rcc::RCC::steal(),
            PWR: pwr::PWR::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            RNG: rng::RNG::steal(),
            AES1: aes1::AES1::steal(),
            AES2: aes2::AES2::steal(),
            HSEM: hsem::HSEM::steal(),
            ADC: adc::ADC::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOE: gpioe::GPIOE::steal(),
            GPIOH: gpioh::GPIOH::steal(),
            SAI1: sai1::SAI1::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            TIM1: tim1::TIM1::steal(),
            LPTIM1: lptim::LPTIM1::steal(),
            LPTIM2: lptim::LPTIM2::steal(),
            USART1: usart1::USART1::steal(),
            LPUART1: usart1::LPUART1::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            VREFBUF: vrefbuf::VREFBUF::steal(),
            RTC: rtc::RTC::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            PKA: pka::PKA::steal(),
            IPCC: ipcc::IPCC::steal(),
            EXTI: exti::EXTI::steal(),
            CRS: crs::CRS::steal(),
            USB: usb::USB::steal(),
            SCB: scb::SCB::steal(),
            STK: stk::STK::steal(),
            MPU: mpu::MPU::steal(),
            FPU: fpu::FPU::steal(),
            NVIC: nvic::NVIC::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            FPU_CPACR: fpu_cpacr::FPU_CPACR::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
