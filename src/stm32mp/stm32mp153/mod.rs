//! stm32ral module for stm32mp153

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::adc;
pub use super::instances::adc2;
pub use super::instances::adc_common;
pub use super::instances::aximc_mx;
pub use super::instances::bsec;
pub use super::instances::ccu;
pub use super::instances::crc;
pub use super::instances::cryp;
pub use super::instances::dac1;
pub use super::instances::dcmi;
pub use super::instances::ddrctrl;
pub use super::instances::ddrperfm;
pub use super::instances::ddrphyc;
pub use super::instances::dfsdm1;
pub use super::instances::dlyb;
pub use super::instances::dma;
pub use super::instances::dmamux1;
pub use super::instances::dts;
pub use super::instances::eth_dma;
pub use super::instances::eth_mac_mmc;
pub use super::instances::eth_mtl;
pub use super::instances::etzpc;
pub use super::instances::exti;
pub use super::instances::fdcan;
pub use super::instances::fmc;
pub use super::instances::gicc;
pub use super::instances::gicd;
pub use super::instances::gich;
pub use super::instances::gicv;
pub use super::instances::gpioa;
pub use super::instances::gpiob;
pub use super::instances::gpioc;
pub use super::instances::gpiod;
pub use super::instances::gpioe;
pub use super::instances::gpiof;
pub use super::instances::gpiog;
pub use super::instances::gpioh;
pub use super::instances::gpioi;
pub use super::instances::gpioj;
pub use super::instances::gpiok;
pub use super::instances::gpioz;
pub use super::instances::hash;
pub use super::instances::hdmi_cec;
pub use super::instances::hdp;
pub use super::instances::hsem;
pub use super::instances::i2c;
pub use super::instances::ipcc;
pub use super::instances::iwdg;
pub use super::instances::lptim;
pub use super::instances::ltdc;
pub use super::instances::mdios;
pub use super::instances::mdma;
pub use super::instances::nvic;
pub use super::instances::otg;
pub use super::instances::pwr;
pub use super::instances::quadspi;
pub use super::instances::rcc;
pub use super::instances::rng;
pub use super::instances::rtc;
pub use super::instances::sai;
pub use super::instances::sdmmc;
pub use super::instances::spdifrx;
pub use super::instances::spi;
pub use super::instances::stgenc;
pub use super::instances::stgenr;
pub use super::instances::syscfg;
pub use super::instances::tamp;
pub use super::instances::tim1;
pub use super::instances::tim12;
pub use super::instances::tim13;
pub use super::instances::tim14;
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
pub use super::instances::tzc;
pub use super::instances::usart;
pub use super::instances::usbphyc;
pub use super::instances::vrefbuf;
pub use super::instances::wwdg1;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub ADC: adc::Instance,
    pub ADC2: adc2::Instance,
    pub ADC_common: adc_common::Instance,
    pub AXIMC_Mx: aximc_mx::Instance,
    pub BSEC: bsec::Instance,
    pub CCU: ccu::Instance,
    pub CRC1: crc::Instance,
    pub CRC2: crc::Instance,
    pub CRYP1: cryp::Instance,
    pub CRYP2: cryp::Instance,
    pub DAC1: dac1::Instance,
    pub DCMI: dcmi::Instance,
    pub DDRCTRL: ddrctrl::Instance,
    pub DDRPERFM: ddrperfm::Instance,
    pub DDRPHYC: ddrphyc::Instance,
    pub DFSDM1: dfsdm1::Instance,
    pub DLYBQS: dlyb::Instance,
    pub DLYBSD1: dlyb::Instance,
    pub DLYBSD2: dlyb::Instance,
    pub DLYBSD3: dlyb::Instance,
    pub DMA1: dma::Instance,
    pub DMA2: dma::Instance,
    pub DMAMUX1: dmamux1::Instance,
    pub DTS: dts::Instance,
    pub ETH_MAC_MMC: eth_mac_mmc::Instance,
    pub ETH_MTL: eth_mtl::Instance,
    pub ETH_DMA: eth_dma::Instance,
    pub ETZPC: etzpc::Instance,
    pub EXTI: exti::Instance,
    pub FDCAN1: fdcan::Instance,
    pub FDCAN2: fdcan::Instance,
    pub FMC: fmc::Instance,
    pub GICC: gicc::Instance,
    pub GICD: gicd::Instance,
    pub GICH: gich::Instance,
    pub GICV: gicv::Instance,
    pub GPIOA: gpioa::Instance,
    pub GPIOB: gpiob::Instance,
    pub GPIOC: gpioc::Instance,
    pub GPIOD: gpiod::Instance,
    pub GPIOE: gpioe::Instance,
    pub GPIOF: gpiof::Instance,
    pub GPIOG: gpiog::Instance,
    pub GPIOH: gpioh::Instance,
    pub GPIOI: gpioi::Instance,
    pub GPIOJ: gpioj::Instance,
    pub GPIOK: gpiok::Instance,
    pub GPIOZ: gpioz::Instance,
    pub HASH1: hash::Instance,
    pub HASH2: hash::Instance,
    pub HDMI_CEC: hdmi_cec::Instance,
    pub HDP: hdp::Instance,
    pub HSEM: hsem::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub I2C3: i2c::Instance,
    pub I2C4: i2c::Instance,
    pub I2C5: i2c::Instance,
    pub I2C6: i2c::Instance,
    pub IPCC: ipcc::Instance,
    pub IWDG1: iwdg::Instance,
    pub IWDG2: iwdg::Instance,
    pub LPTIM1: lptim::Instance,
    pub LPTIM2: lptim::Instance,
    pub LPTIM3: lptim::Instance,
    pub LPTIM4: lptim::Instance,
    pub LPTIM5: lptim::Instance,
    pub LTDC: ltdc::Instance,
    pub MDIOS: mdios::Instance,
    pub MDMA: mdma::Instance,
    pub NVIC: nvic::Instance,
    pub OTG: otg::Instance,
    pub PWR: pwr::Instance,
    pub QUADSPI: quadspi::Instance,
    pub RCC: rcc::Instance,
    pub RNG1: rng::Instance,
    pub RNG2: rng::Instance,
    pub RTC: rtc::Instance,
    pub SAI1: sai::Instance,
    pub SAI2: sai::Instance,
    pub SAI3: sai::Instance,
    pub SAI4: sai::Instance,
    pub SDMMC1: sdmmc::Instance,
    pub SDMMC2: sdmmc::Instance,
    pub SDMMC3: sdmmc::Instance,
    pub SPDIFRX: spdifrx::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub SPI3: spi::Instance,
    pub SPI4: spi::Instance,
    pub SPI5: spi::Instance,
    pub SPI6: spi::Instance,
    pub STGENC: stgenc::Instance,
    pub STGENR: stgenr::Instance,
    pub SYSCFG: syscfg::Instance,
    pub TAMP: tamp::Instance,
    pub TIM1: tim1::Instance,
    pub TIM2: tim2::Instance,
    pub TIM3: tim3::Instance,
    pub TIM4: tim4::Instance,
    pub TIM5: tim5::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub TIM8: tim8::Instance,
    pub TIM12: tim12::Instance,
    pub TIM13: tim13::Instance,
    pub TIM14: tim14::Instance,
    pub TIM15: tim15::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub TZC: tzc::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub USART4: usart::Instance,
    pub USART5: usart::Instance,
    pub USART6: usart::Instance,
    pub USART7: usart::Instance,
    pub USART8: usart::Instance,
    pub USBPHYC: usbphyc::Instance,
    pub VREFBUF: vrefbuf::Instance,
    pub WWDG1: wwdg1::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            ADC: adc::ADC::steal(),
            ADC2: adc2::ADC2::steal(),
            ADC_common: adc_common::ADC_common::steal(),
            AXIMC_Mx: aximc_mx::AXIMC_Mx::steal(),
            BSEC: bsec::BSEC::steal(),
            CCU: ccu::CCU::steal(),
            CRC1: crc::CRC1::steal(),
            CRC2: crc::CRC2::steal(),
            CRYP1: cryp::CRYP1::steal(),
            CRYP2: cryp::CRYP2::steal(),
            DAC1: dac1::DAC1::steal(),
            DCMI: dcmi::DCMI::steal(),
            DDRCTRL: ddrctrl::DDRCTRL::steal(),
            DDRPERFM: ddrperfm::DDRPERFM::steal(),
            DDRPHYC: ddrphyc::DDRPHYC::steal(),
            DFSDM1: dfsdm1::DFSDM1::steal(),
            DLYBQS: dlyb::DLYBQS::steal(),
            DLYBSD1: dlyb::DLYBSD1::steal(),
            DLYBSD2: dlyb::DLYBSD2::steal(),
            DLYBSD3: dlyb::DLYBSD3::steal(),
            DMA1: dma::DMA1::steal(),
            DMA2: dma::DMA2::steal(),
            DMAMUX1: dmamux1::DMAMUX1::steal(),
            DTS: dts::DTS::steal(),
            ETH_MAC_MMC: eth_mac_mmc::ETH_MAC_MMC::steal(),
            ETH_MTL: eth_mtl::ETH_MTL::steal(),
            ETH_DMA: eth_dma::ETH_DMA::steal(),
            ETZPC: etzpc::ETZPC::steal(),
            EXTI: exti::EXTI::steal(),
            FDCAN1: fdcan::FDCAN1::steal(),
            FDCAN2: fdcan::FDCAN2::steal(),
            FMC: fmc::FMC::steal(),
            GICC: gicc::GICC::steal(),
            GICD: gicd::GICD::steal(),
            GICH: gich::GICH::steal(),
            GICV: gicv::GICV::steal(),
            GPIOA: gpioa::GPIOA::steal(),
            GPIOB: gpiob::GPIOB::steal(),
            GPIOC: gpioc::GPIOC::steal(),
            GPIOD: gpiod::GPIOD::steal(),
            GPIOE: gpioe::GPIOE::steal(),
            GPIOF: gpiof::GPIOF::steal(),
            GPIOG: gpiog::GPIOG::steal(),
            GPIOH: gpioh::GPIOH::steal(),
            GPIOI: gpioi::GPIOI::steal(),
            GPIOJ: gpioj::GPIOJ::steal(),
            GPIOK: gpiok::GPIOK::steal(),
            GPIOZ: gpioz::GPIOZ::steal(),
            HASH1: hash::HASH1::steal(),
            HASH2: hash::HASH2::steal(),
            HDMI_CEC: hdmi_cec::HDMI_CEC::steal(),
            HDP: hdp::HDP::steal(),
            HSEM: hsem::HSEM::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            I2C3: i2c::I2C3::steal(),
            I2C4: i2c::I2C4::steal(),
            I2C5: i2c::I2C5::steal(),
            I2C6: i2c::I2C6::steal(),
            IPCC: ipcc::IPCC::steal(),
            IWDG1: iwdg::IWDG1::steal(),
            IWDG2: iwdg::IWDG2::steal(),
            LPTIM1: lptim::LPTIM1::steal(),
            LPTIM2: lptim::LPTIM2::steal(),
            LPTIM3: lptim::LPTIM3::steal(),
            LPTIM4: lptim::LPTIM4::steal(),
            LPTIM5: lptim::LPTIM5::steal(),
            LTDC: ltdc::LTDC::steal(),
            MDIOS: mdios::MDIOS::steal(),
            MDMA: mdma::MDMA::steal(),
            NVIC: nvic::NVIC::steal(),
            OTG: otg::OTG::steal(),
            PWR: pwr::PWR::steal(),
            QUADSPI: quadspi::QUADSPI::steal(),
            RCC: rcc::RCC::steal(),
            RNG1: rng::RNG1::steal(),
            RNG2: rng::RNG2::steal(),
            RTC: rtc::RTC::steal(),
            SAI1: sai::SAI1::steal(),
            SAI2: sai::SAI2::steal(),
            SAI3: sai::SAI3::steal(),
            SAI4: sai::SAI4::steal(),
            SDMMC1: sdmmc::SDMMC1::steal(),
            SDMMC2: sdmmc::SDMMC2::steal(),
            SDMMC3: sdmmc::SDMMC3::steal(),
            SPDIFRX: spdifrx::SPDIFRX::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            SPI3: spi::SPI3::steal(),
            SPI4: spi::SPI4::steal(),
            SPI5: spi::SPI5::steal(),
            SPI6: spi::SPI6::steal(),
            STGENC: stgenc::STGENC::steal(),
            STGENR: stgenr::STGENR::steal(),
            SYSCFG: syscfg::SYSCFG::steal(),
            TAMP: tamp::TAMP::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM4: tim4::TIM4::steal(),
            TIM5: tim5::TIM5::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            TIM8: tim8::TIM8::steal(),
            TIM12: tim12::TIM12::steal(),
            TIM13: tim13::TIM13::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM15: tim15::TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            TZC: tzc::TZC::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            USART4: usart::USART4::steal(),
            USART5: usart::USART5::steal(),
            USART6: usart::USART6::steal(),
            USART7: usart::USART7::steal(),
            USART8: usart::USART8::steal(),
            USBPHYC: usbphyc::USBPHYC::steal(),
            VREFBUF: vrefbuf::VREFBUF::steal(),
            WWDG1: wwdg1::WWDG1::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
