//! stm32ral module for stm32h743v

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::ac;
pub use super::instances::adc_common;
pub use super::instances::adc_h743v_h753v as adc;
pub use super::instances::axi_h743_h743v_h753_h753v as axi;
pub use super::instances::bdma;
pub use super::instances::can_ccu;
pub use super::instances::cec;
pub use super::instances::comp1;
pub use super::instances::crc;
pub use super::instances::crs;
pub use super::instances::dac;
pub use super::instances::dbgmcu_h743_h743v_h753_h753v_h757cm7 as dbgmcu;
pub use super::instances::dcmi;
pub use super::instances::dfsdm;
pub use super::instances::dlyb;
pub use super::instances::dma2d;
pub use super::instances::dma_h743_h743v_h747cm7_h753_h753v as dma;
pub use super::instances::dmamux1;
pub use super::instances::dmamux2;
pub use super::instances::ethernet_dma_h743_h743v_h753_h753v as ethernet_dma;
pub use super::instances::ethernet_mac_h743_h743v_h753_h753v as ethernet_mac;
pub use super::instances::ethernet_mtl_h743_h743v_h753_h753v as ethernet_mtl;
pub use super::instances::exti;
pub use super::instances::fdcan;
pub use super::instances::flash_h743_h743v_h753_h753v as flash;
pub use super::instances::fmc;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::gpio;
pub use super::instances::hrtim_common_h743_h743v_h753_h753v as hrtim_common;
pub use super::instances::hrtim_master;
pub use super::instances::hrtim_tima;
pub use super::instances::hrtim_timb;
pub use super::instances::hrtim_timc;
pub use super::instances::hrtim_timd;
pub use super::instances::hrtim_time;
pub use super::instances::hsem;
pub use super::instances::i2c;
pub use super::instances::iwdg_h743_h743v_h753_h753v as iwdg;
pub use super::instances::jpeg;
pub use super::instances::lptim;
pub use super::instances::lptim3;
pub use super::instances::lpuart1;
pub use super::instances::ltdc;
pub use super::instances::mdios;
pub use super::instances::mdma;
pub use super::instances::mpu;
pub use super::instances::nvic_h743_h743v_h753_h753v as nvic;
pub use super::instances::nvic_stir;
pub use super::instances::opamp;
pub use super::instances::otg_hs_device;
pub use super::instances::otg_hs_global;
pub use super::instances::otg_hs_host;
pub use super::instances::otg_hs_pwrclk;
pub use super::instances::pf;
pub use super::instances::pwr_h743_h743v_h753_h753v as pwr;
pub use super::instances::quadspi;
pub use super::instances::rcc_h743v_h753v as rcc;
pub use super::instances::rng;
pub use super::instances::rtc;
pub use super::instances::sai_h743_h743v_h753_h753v as sai;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::sdmmc_h743_h743v_h753_h753v as sdmmc;
pub use super::instances::spdifrx;
pub use super::instances::spi_h743_h743v_h753_h753v as spi;
pub use super::instances::stk;
pub use super::instances::swpmi;
pub use super::instances::syscfg_h743_h743v_h753_h753v as syscfg;
pub use super::instances::tim12;
pub use super::instances::tim13;
pub use super::instances::tim14;
pub use super::instances::tim15;
pub use super::instances::tim16;
pub use super::instances::tim17;
pub use super::instances::tim1_h743_h743v_h753_h753v as tim1;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim4;
pub use super::instances::tim5;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub use super::instances::tim8_h743_h743v_h753_h753v as tim8;
pub use super::instances::usart;
pub use super::instances::vrefbuf;
pub use super::instances::wwdg_h743_h743v_h753_h753v as wwdg;

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub COMP1: comp1::Instance,
    pub CRS: crs::Instance,
    pub DAC: dac::Instance,
    pub BDMA: bdma::Instance,
    pub DMA2D: dma2d::Instance,
    pub DMAMUX2: dmamux2::Instance,
    pub FMC: fmc::Instance,
    pub CEC: cec::Instance,
    pub HSEM: hsem::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub I2C4: i2c::Instance,
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
    pub JPEG: jpeg::Instance,
    pub MDMA: mdma::Instance,
    pub QUADSPI: quadspi::Instance,
    pub RNG: rng::Instance,
    pub RTC: rtc::Instance,
    pub SAI4: sai::Instance,
    pub SAI1: sai::Instance,
    pub SAI2: sai::Instance,
    pub SAI3: sai::Instance,
    pub SDMMC1: sdmmc::Instance,
    pub SDMMC2: sdmmc::Instance,
    pub VREFBUF: vrefbuf::Instance,
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub PWR: pwr::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub SPI4: spi::Instance,
    pub SPI5: spi::Instance,
    pub SPI6: spi::Instance,
    pub LTDC: ltdc::Instance,
    pub SPDIFRX: spdifrx::Instance,
    pub ADC3: adc::Instance,
    pub ADC1: adc::Instance,
    pub ADC2: adc::Instance,
    pub ADC3_Common: adc_common::Instance,
    pub ADC12_Common: adc_common::Instance,
    pub DMAMUX1: dmamux1::Instance,
    pub CRC: crc::Instance,
    pub RCC: rcc::Instance,
    pub LPTIM1: lptim::Instance,
    pub LPTIM2: lptim::Instance,
    pub LPTIM3: lptim3::Instance,
    pub LPTIM4: lptim3::Instance,
    pub LPTIM5: lptim3::Instance,
    pub LPUART1: lpuart1::Instance,
    pub SYSCFG: syscfg::Instance,
    pub EXTI: exti::Instance,
    pub DELAY_Block_SDMMC1: dlyb::Instance,
    pub DELAY_Block_QUADSPI: dlyb::Instance,
    pub DELAY_Block_SDMMC2: dlyb::Instance,
    pub FLASH: flash::Instance,
    pub AXI: axi::Instance,
    pub DCMI: dcmi::Instance,
    pub OTG1_HS_GLOBAL: otg_hs_global::Instance,
    pub OTG2_HS_GLOBAL: otg_hs_global::Instance,
    pub OTG1_HS_HOST: otg_hs_host::Instance,
    pub OTG2_HS_HOST: otg_hs_host::Instance,
    pub OTG1_HS_DEVICE: otg_hs_device::Instance,
    pub OTG2_HS_DEVICE: otg_hs_device::Instance,
    pub OTG1_HS_PWRCLK: otg_hs_pwrclk::Instance,
    pub OTG2_HS_PWRCLK: otg_hs_pwrclk::Instance,
    pub Ethernet_DMA: ethernet_dma::Instance,
    pub Ethernet_MTL: ethernet_mtl::Instance,
    pub Ethernet_MAC: ethernet_mac::Instance,
    pub DMA1: dma::Instance,
    pub DMA2: dma::Instance,
    pub HRTIM_Master: hrtim_master::Instance,
    pub HRTIM_TIMA: hrtim_tima::Instance,
    pub HRTIM_TIMB: hrtim_timb::Instance,
    pub HRTIM_TIMC: hrtim_timc::Instance,
    pub HRTIM_TIMD: hrtim_timd::Instance,
    pub HRTIM_TIME: hrtim_time::Instance,
    pub HRTIM_Common: hrtim_common::Instance,
    pub DFSDM: dfsdm::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub TIM15: tim15::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub UART4: usart::Instance,
    pub UART5: usart::Instance,
    pub USART6: usart::Instance,
    pub UART7: usart::Instance,
    pub UART8: usart::Instance,
    pub TIM1: tim1::Instance,
    pub TIM8: tim8::Instance,
    pub FDCAN1: fdcan::Instance,
    pub FDCAN2: fdcan::Instance,
    pub CAN_CCU: can_ccu::Instance,
    pub MDIOS: mdios::Instance,
    pub OPAMP: opamp::Instance,
    pub SWPMI: swpmi::Instance,
    pub TIM2: tim2::Instance,
    pub TIM5: tim5::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub NVIC: nvic::Instance,
    pub MPU: mpu::Instance,
    pub STK: stk::Instance,
    pub NVIC_STIR: nvic_stir::Instance,
    pub FPU_CPACR: fpu_cpacr::Instance,
    pub SCB_ACTRL: scb_actrl::Instance,
    pub FPU: fpu::Instance,
    pub SCB: scb::Instance,
    pub PF: pf::Instance,
    pub AC: ac::Instance,
    pub TIM3: tim3::Instance,
    pub TIM4: tim4::Instance,
    pub TIM12: tim12::Instance,
    pub TIM13: tim13::Instance,
    pub TIM14: tim14::Instance,
    pub DBGMCU: dbgmcu::Instance,
}

#[cfg(all(feature = "rtfm", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            COMP1: comp1::COMP1::steal(),
            CRS: crs::CRS::steal(),
            DAC: dac::DAC::steal(),
            BDMA: bdma::BDMA::steal(),
            DMA2D: dma2d::DMA2D::steal(),
            DMAMUX2: dmamux2::DMAMUX2::steal(),
            FMC: fmc::FMC::steal(),
            CEC: cec::CEC::steal(),
            HSEM: hsem::HSEM::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C3: i2c::I2C3::steal(),
            I2C4: i2c::I2C4::steal(),
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
            JPEG: jpeg::JPEG::steal(),
            MDMA: mdma::MDMA::steal(),
            QUADSPI: quadspi::QUADSPI::steal(),
            RNG: rng::RNG::steal(),
            RTC: rtc::RTC::steal(),
            SAI4: sai::SAI4::steal(),
            SAI1: sai::SAI1::steal(),
            SAI2: sai::SAI2::steal(),
            SAI3: sai::SAI3::steal(),
            SDMMC1: sdmmc::SDMMC1::steal(),
            SDMMC2: sdmmc::SDMMC2::steal(),
            VREFBUF: vrefbuf::VREFBUF::steal(),
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            PWR: pwr::PWR::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            SPI4: spi::SPI4::steal(),
            SPI5: spi::SPI5::steal(),
            SPI6: spi::SPI6::steal(),
            LTDC: ltdc::LTDC::steal(),
            SPDIFRX: spdifrx::SPDIFRX::steal(),
            ADC3: adc::ADC3::steal(),
            ADC1: adc::ADC1::steal(),
            ADC2: adc::ADC2::steal(),
            ADC3_Common: adc_common::ADC3_Common::steal(),
            ADC12_Common: adc_common::ADC12_Common::steal(),
            DMAMUX1: dmamux1::DMAMUX1::steal(),
            CRC: crc::CRC::steal(),
            RCC: rcc::RCC::steal(),
            LPTIM1: lptim::LPTIM1::steal(),
            LPTIM2: lptim::LPTIM2::steal(),
            LPTIM3: lptim3::LPTIM3::steal(),
            LPTIM4: lptim3::LPTIM4::steal(),
            LPTIM5: lptim3::LPTIM5::steal(),
            LPUART1: lpuart1::LPUART1::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            EXTI: exti::EXTI::steal(),
            DELAY_Block_SDMMC1: dlyb::DELAY_Block_SDMMC1::steal(),
            DELAY_Block_QUADSPI: dlyb::DELAY_Block_QUADSPI::steal(),
            DELAY_Block_SDMMC2: dlyb::DELAY_Block_SDMMC2::steal(),
            FLASH: flash::FLASH::steal(),
            AXI: axi::AXI::steal(),
            DCMI: dcmi::DCMI::steal(),
            OTG1_HS_GLOBAL: otg_hs_global::OTG1_HS_GLOBAL::steal(),
            OTG2_HS_GLOBAL: otg_hs_global::OTG2_HS_GLOBAL::steal(),
            OTG1_HS_HOST: otg_hs_host::OTG1_HS_HOST::steal(),
            OTG2_HS_HOST: otg_hs_host::OTG2_HS_HOST::steal(),
            OTG1_HS_DEVICE: otg_hs_device::OTG1_HS_DEVICE::steal(),
            OTG2_HS_DEVICE: otg_hs_device::OTG2_HS_DEVICE::steal(),
            OTG1_HS_PWRCLK: otg_hs_pwrclk::OTG1_HS_PWRCLK::steal(),
            OTG2_HS_PWRCLK: otg_hs_pwrclk::OTG2_HS_PWRCLK::steal(),
            Ethernet_DMA: ethernet_dma::Ethernet_DMA::steal(),
            Ethernet_MTL: ethernet_mtl::Ethernet_MTL::steal(),
            Ethernet_MAC: ethernet_mac::Ethernet_MAC::steal(),
            DMA1: dma::DMA1::steal(),
            DMA2: dma::DMA2::steal(),
            HRTIM_Master: hrtim_master::HRTIM_Master::steal(),
            HRTIM_TIMA: hrtim_tima::HRTIM_TIMA::steal(),
            HRTIM_TIMB: hrtim_timb::HRTIM_TIMB::steal(),
            HRTIM_TIMC: hrtim_timc::HRTIM_TIMC::steal(),
            HRTIM_TIMD: hrtim_timd::HRTIM_TIMD::steal(),
            HRTIM_TIME: hrtim_time::HRTIM_TIME::steal(),
            HRTIM_Common: hrtim_common::HRTIM_Common::steal(),
            DFSDM: dfsdm::DFSDM::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            TIM15: tim15::TIM15::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            UART4: usart::UART4::steal(),
            UART5: usart::UART5::steal(),
            USART6: usart::USART6::steal(),
            UART7: usart::UART7::steal(),
            UART8: usart::UART8::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM8: tim8::TIM8::steal(),
            FDCAN1: fdcan::FDCAN1::steal(),
            FDCAN2: fdcan::FDCAN2::steal(),
            CAN_CCU: can_ccu::CAN_CCU::steal(),
            MDIOS: mdios::MDIOS::steal(),
            OPAMP: opamp::OPAMP::steal(),
            SWPMI: swpmi::SWPMI::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM5: tim5::TIM5::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            NVIC: nvic::NVIC::steal(),
            MPU: mpu::MPU::steal(),
            STK: stk::STK::steal(),
            NVIC_STIR: nvic_stir::NVIC_STIR::steal(),
            FPU_CPACR: fpu_cpacr::FPU_CPACR::steal(),
            SCB_ACTRL: scb_actrl::SCB_ACTRL::steal(),
            FPU: fpu::FPU::steal(),
            SCB: scb::SCB::steal(),
            PF: pf::PF::steal(),
            AC: ac::AC::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM4: tim4::TIM4::steal(),
            TIM12: tim12::TIM12::steal(),
            TIM13: tim13::TIM13::steal(),
            TIM14: tim14::TIM14::steal(),
            DBGMCU: dbgmcu::DBGMCU::steal(),
        }
    }
}

#[cfg(all(feature = "rtfm", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
