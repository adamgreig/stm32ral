//! stm32ral module for stm32h7b3

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::ac;
pub mod adc;
pub mod adc12_common;
pub use super::instances::axi;
pub mod bdma;
pub mod can_ccu;
pub use super::instances::cec;
pub mod comp1;
pub mod crc;
pub use super::instances::crs;
pub use super::instances::cryp_h747cm4_h747cm7_h7b3 as cryp;
pub mod dac;
pub mod dbgmcu;
pub use super::instances::dcmi;
pub use super::instances::dlyb_h735_h7b3 as dlyb;
pub mod dfsdm;
pub use super::instances::dma;
pub use super::instances::dma2d;
pub use super::instances::dmamux1;
pub use super::instances::dmamux2;
pub use super::instances::exti_h735_h743_h743v_h753_h753v_h7b3 as exti;
pub mod fdcan;
pub use super::instances::fmc;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub mod flash;
pub use super::instances::gpio;
pub use super::instances::hash;
pub use super::instances::hrtim_common_h747cm4_h747cm7_h7b3 as hrtim_common;
pub use super::instances::hrtim_master;
pub use super::instances::hrtim_tima;
pub use super::instances::hrtim_timb;
pub use super::instances::hrtim_timc;
pub use super::instances::hrtim_timd;
pub use super::instances::hrtim_time;
pub mod hsem;
pub use super::instances::i2c;
pub use super::instances::iwdg_h743_h743v_h753_h753v_h7b3 as iwdg;
pub use super::instances::jpeg;
pub use super::instances::lptim;
pub mod lptim3;
pub use super::instances::lpuart1;
pub use super::instances::ltdc;
pub use super::instances::mdios;
pub use super::instances::mdma;
pub use super::instances::mpu;
pub mod nvic;
pub use super::instances::nvic_stir;
pub mod octospi2;
pub use super::instances::opamp;
pub mod otfdec;
pub mod otg1_hs_device;
pub use super::instances::otg1_hs_global;
pub mod otg1_hs_host;
pub mod otg1_hs_pwrclk;
pub use super::instances::octospii_o_manager;
pub use super::instances::pf;
pub mod pwr;
pub mod ramecc;
pub mod rcc;
pub use super::instances::rng;
pub use super::instances::rtc;
pub mod sai;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::sdmmc_h743_h743v_h753_h753v_h7b3 as sdmmc;
pub use super::instances::spdifrx;
pub use super::instances::spi_h747cm4_h747cm7_h7b3 as spi;
pub use super::instances::stk;
pub use super::instances::swpmi;
pub mod syscfg;
pub mod tim1;
pub use super::instances::tim2_h735_h7b3 as tim2;
pub use super::instances::tim5_h735_h7b3 as tim5;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub mod tim8;
pub use super::instances::tim15_h735_h7b3 as tim15;
pub use super::instances::tim16;
pub use super::instances::tim17;
pub mod usart;
pub use super::instances::dsihost;
pub use super::instances::tim12_h735_h7b3 as tim12;
pub use super::instances::tim13_h735_h7b3 as tim13;
pub use super::instances::tim14_h735_h7b3 as tim14;
pub use super::instances::tim3_h735_h7b3 as tim3;
pub use super::instances::tim4_h735_h7b3 as tim4;
pub use super::instances::vrefbuf;
pub use super::instances::wwdg_h743_h743v_h753_h753v_h7b3 as wwdg;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub AC: ac::Instance,
    pub ADC1: adc::Instance,
    pub ADC2: adc::Instance,
    pub ADC12_Common: adc12_common::Instance,
    pub AXI: axi::Instance,
    pub BDMA1: bdma::Instance,
    pub BDMA2: bdma::Instance,
    pub CAN_CCU: can_ccu::Instance,
    pub CEC: cec::Instance,
    pub COMP1: comp1::Instance,
    pub CRC: crc::Instance,
    pub CRS: crs::Instance,
    pub CRYP: cryp::Instance,
    pub DAC1: dac::Instance,
    pub DAC2: dac::Instance,
    pub DBGMCU: dbgmcu::Instance,
    pub DCMI: dcmi::Instance,
    pub DELAY_Block_SDMMC1: dlyb::Instance,
    pub DELAY_Block_SDMMC2: dlyb::Instance,
    pub Delay_Block_OCTOSPI1: dlyb::Instance,
    pub Delay_Block_OCTOSPI2: dlyb::Instance,
    pub DFSDM1: dfsdm::Instance,
    pub DFSDM2: dfsdm::Instance,
    pub DMA1: dma::Instance,
    pub DMA2: dma::Instance,
    pub DMA2D: dma2d::Instance,
    pub DMAMUX1: dmamux1::Instance,
    pub DMAMUX2: dmamux2::Instance,
    pub EXTI: exti::Instance,
    pub FDCAN2: fdcan::Instance,
    pub FDCAN1: fdcan::Instance,
    pub FMC: fmc::Instance,
    pub FPU: fpu::Instance,
    pub FPU_CPACR: fpu_cpacr::Instance,
    pub FLASH: flash::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOE: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub GPIOG: gpio::Instance,
    pub GPIOH: gpio::Instance,
    pub GPIOI: gpio::Instance,
    pub GPIOJ: gpio::Instance,
    pub GPIOK: gpio::Instance,
    pub HASH: hash::Instance,
    pub HRTIM_Common: hrtim_common::Instance,
    pub HRTIM_Master: hrtim_master::Instance,
    pub HRTIM_TIMA: hrtim_tima::Instance,
    pub HRTIM_TIMB: hrtim_timb::Instance,
    pub HRTIM_TIMC: hrtim_timc::Instance,
    pub HRTIM_TIMD: hrtim_timd::Instance,
    pub HRTIM_TIME: hrtim_time::Instance,
    pub HSEM: hsem::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub I2C4: i2c::Instance,
    pub IWDG: iwdg::Instance,
    pub JPEG: jpeg::Instance,
    pub LPTIM1: lptim::Instance,
    pub LPTIM2: lptim::Instance,
    pub LPTIM3: lptim3::Instance,
    pub LPUART1: lpuart1::Instance,
    pub LTDC: ltdc::Instance,
    pub MDIOS: mdios::Instance,
    pub MDMA: mdma::Instance,
    pub MPU: mpu::Instance,
    pub NVIC: nvic::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub OCTOSPI2: octospi2::Instance,
    pub OCTOSPI1_CONTROL_REGISTER: octospi2::Instance,
    pub OPAMP: opamp::Instance,
    pub OTFDEC1: otfdec::Instance,
    pub OTFDEC2: otfdec::Instance,
    pub OTG1_HS_DEVICE: otg1_hs_device::Instance,
    pub OTG1_HS_GLOBAL: otg1_hs_global::Instance,
    pub OTG1_HS_HOST: otg1_hs_host::Instance,
    pub OTG1_HS_PWRCLK: otg1_hs_pwrclk::Instance,
    pub OctoSPII_O_Manager: octospii_o_manager::Instance,
    pub PF: pf::Instance,
    pub PWR: pwr::Instance,
    pub RAMECC: ramecc::Instance,
    pub RCC: rcc::Instance,
    pub RNG: rng::Instance,
    pub RTC: rtc::Instance,
    pub SAI1: sai::Instance,
    pub SAI2: sai::Instance,
    pub SCB: scb::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub SDMMC1: sdmmc::Instance,
    pub SDMMC2: sdmmc::Instance,
    pub SPDIFRX: spdifrx::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub SPI4: spi::Instance,
    pub SPI5: spi::Instance,
    pub SPI6: spi::Instance,
    pub STK: stk::Instance,
    pub SWPMI: swpmi::Instance,
    pub SYSCFG: syscfg::Instance,
    pub TIM1: tim1::Instance,
    pub TIM2: tim2::Instance,
    pub TIM5: tim5::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub TIM8: tim8::Instance,
    pub TIM15: tim15::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub USART6: usart::Instance,
    pub UART4: usart::Instance,
    pub UART5: usart::Instance,
    pub UART7: usart::Instance,
    pub UART8: usart::Instance,
    pub UART9: usart::Instance,
    pub USART10: usart::Instance,
    pub VREFBUF: vrefbuf::Instance,
    pub WWDG: wwdg::Instance,
    pub TIM3: tim3::Instance,
    pub TIM4: tim4::Instance,
    pub TIM12: tim12::Instance,
    pub TIM13: tim13::Instance,
    pub TIM14: tim14::Instance,
    pub DSIHOST: dsihost::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            AC: ac::AC::steal(),
            ADC1: adc::ADC1::steal(),
            ADC2: adc::ADC2::steal(),
            ADC12_Common: adc12_common::ADC12_Common::steal(),
            AXI: axi::AXI::steal(),
            BDMA1: bdma::BDMA1::steal(),
            BDMA2: bdma::BDMA2::steal(),
            CAN_CCU: can_ccu::CAN_CCU::steal(),
            CEC: cec::CEC::steal(),
            COMP1: comp1::COMP1::steal(),
            CRC: crc::CRC::steal(),
            CRS: crs::CRS::steal(),
            CRYP: cryp::CRYP::steal(),
            DAC1: dac::DAC1::steal(),
            DAC2: dac::DAC2::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
            DCMI: dcmi::DCMI::steal(),
            DELAY_Block_SDMMC1: dlyb::DELAY_Block_SDMMC1::steal(),
            DELAY_Block_SDMMC2: dlyb::DELAY_Block_SDMMC2::steal(),
            Delay_Block_OCTOSPI1: dlyb::Delay_Block_OCTOSPI1::steal(),
            Delay_Block_OCTOSPI2: dlyb::Delay_Block_OCTOSPI2::steal(),
            DFSDM1: dfsdm::DFSDM1::steal(),
            DFSDM2: dfsdm::DFSDM2::steal(),
            DMA1: dma::DMA1::steal(),
            DMA2: dma::DMA2::steal(),
            DMA2D: dma2d::DMA2D::steal(),
            DMAMUX1: dmamux1::DMAMUX1::steal(),
            DMAMUX2: dmamux2::DMAMUX2::steal(),
            EXTI: exti::EXTI::steal(),
            FDCAN2: fdcan::FDCAN2::steal(),
            FDCAN1: fdcan::FDCAN1::steal(),
            FMC: fmc::FMC::steal(),
            FPU: fpu::FPU::steal(),
            FPU_CPACR: fpu_cpacr::FPU_CPACR::steal(),
            FLASH: flash::FLASH::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOE: gpio::GPIOE::steal(),
            GPIOF: gpio::GPIOF::steal(),
            GPIOG: gpio::GPIOG::steal(),
            GPIOH: gpio::GPIOH::steal(),
            GPIOI: gpio::GPIOI::steal(),
            GPIOJ: gpio::GPIOJ::steal(),
            GPIOK: gpio::GPIOK::steal(),
            HASH: hash::HASH::steal(),
            HRTIM_Common: hrtim_common::HRTIM_Common::steal(),
            HRTIM_Master: hrtim_master::HRTIM_Master::steal(),
            HRTIM_TIMA: hrtim_tima::HRTIM_TIMA::steal(),
            HRTIM_TIMB: hrtim_timb::HRTIM_TIMB::steal(),
            HRTIM_TIMC: hrtim_timc::HRTIM_TIMC::steal(),
            HRTIM_TIMD: hrtim_timd::HRTIM_TIMD::steal(),
            HRTIM_TIME: hrtim_time::HRTIM_TIME::steal(),
            HSEM: hsem::HSEM::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C3: i2c::I2C3::steal(),
            I2C4: i2c::I2C4::steal(),
            IWDG: iwdg::IWDG::steal(),
            JPEG: jpeg::JPEG::steal(),
            LPTIM1: lptim::LPTIM1::steal(),
            LPTIM2: lptim::LPTIM2::steal(),
            LPTIM3: lptim3::LPTIM3::steal(),
            LPUART1: lpuart1::LPUART1::steal(),
            LTDC: ltdc::LTDC::steal(),
            MDIOS: mdios::MDIOS::steal(),
            MDMA: mdma::MDMA::steal(),
            MPU: mpu::MPU::steal(),
            NVIC: nvic::NVIC::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            OCTOSPI2: octospi2::OCTOSPI2::steal(),
            OCTOSPI1_CONTROL_REGISTER: octospi2::OCTOSPI1_CONTROL_REGISTER::steal(),
            OPAMP: opamp::OPAMP::steal(),
            OTFDEC1: otfdec::OTFDEC1::steal(),
            OTFDEC2: otfdec::OTFDEC2::steal(),
            OTG1_HS_DEVICE: otg1_hs_device::OTG1_HS_DEVICE::steal(),
            OTG1_HS_GLOBAL: otg1_hs_global::OTG1_HS_GLOBAL::steal(),
            OTG1_HS_HOST: otg1_hs_host::OTG1_HS_HOST::steal(),
            OTG1_HS_PWRCLK: otg1_hs_pwrclk::OTG1_HS_PWRCLK::steal(),
            OctoSPII_O_Manager: octospii_o_manager::OctoSPII_O_Manager::steal(),
            PF: pf::PF::steal(),
            PWR: pwr::PWR::steal(),
            RAMECC: ramecc::RAMECC::steal(),
            RCC: rcc::RCC::steal(),
            RNG: rng::RNG::steal(),
            RTC: rtc::RTC::steal(),
            SAI1: sai::SAI1::steal(),
            SAI2: sai::SAI2::steal(),
            SCB: scb::SCB::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            SDMMC1: sdmmc::SDMMC1::steal(),
            SDMMC2: sdmmc::SDMMC2::steal(),
            SPDIFRX: spdifrx::SPDIFRX::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            SPI4: spi::SPI4::steal(),
            SPI5: spi::SPI5::steal(),
            SPI6: spi::SPI6::steal(),
            STK: stk::STK::steal(),
            SWPMI: swpmi::SWPMI::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM5: tim5::TIM5::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            TIM8: tim8::TIM8::steal(),
            TIM15: tim15::TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            USART6: usart::USART6::steal(),
            UART4: usart::UART4::steal(),
            UART5: usart::UART5::steal(),
            UART7: usart::UART7::steal(),
            UART8: usart::UART8::steal(),
            UART9: usart::UART9::steal(),
            USART10: usart::USART10::steal(),
            VREFBUF: vrefbuf::VREFBUF::steal(),
            WWDG: wwdg::WWDG::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM4: tim4::TIM4::steal(),
            TIM12: tim12::TIM12::steal(),
            TIM13: tim13::TIM13::steal(),
            TIM14: tim14::TIM14::steal(),
            DSIHOST: dsihost::DSIHOST::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
