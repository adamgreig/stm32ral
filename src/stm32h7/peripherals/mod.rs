#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod ac;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743v", feature="stm32h753v"))]
pub mod adc_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod adc_common;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod axi;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod can_ccu;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod cec;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod comp1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod crs;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod dac;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod dbgmcu_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod dcmi;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod dlyb;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod dfsdm_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod dma;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod dma2d;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod dmamux1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod dmamux2;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod exti_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod ethernet_mac_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod fdcan;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod fmc;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod fpu;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod fpu_cpacr;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod flash_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod gpio;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod i2c;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod iwdg1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod lptim;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod lptim3;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod lpuart1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod ltdc;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod mdma;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod mpu;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod nvic_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod nvic_stir;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod opamp;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod otg_hs_device;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod otg1_hs_global;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod otg_hs_host;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod otg_hs_pwrclk;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h7b3"))]
pub mod octospii_o_manager;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod pf;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h7b3"))]
pub mod ramecc1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod rng;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod rtc;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h7b3"))]
pub mod sai_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod scb;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod scb_actrl;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod sdmmc_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod spi_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod stk;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod swpmi;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h735"))]
pub mod tim1_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h735", feature="stm32h7b3", feature="stm32h7b3"))]
pub mod tim2_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h735", feature="stm32h743", feature="stm32h743", feature="stm32h743v", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753", feature="stm32h753v", feature="stm32h753v", feature="stm32h7b3", feature="stm32h7b3"))]
pub mod tim6;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h7b3"))]
pub mod tim15_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod tim16;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod tim17;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod usart;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod vrefbuf;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod wwdg1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h735", feature="stm32h7b3", feature="stm32h7b3"))]
pub mod tim3_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h7b3"))]
pub mod tim12_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h735", feature="stm32h7b3", feature="stm32h7b3"))]
pub mod tim13_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod ethernet_mtl_v1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h7b3"))]
pub mod dsihost;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod bdma;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod hsem;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod jpeg;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod quadspi;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod sai_v2;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod sdmmc_v2;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod pwr_v1;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod spdifrx;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h753"))]
pub mod adc_v2;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h753"))]
pub mod rcc_v1;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod syscfg_v1;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod ethernet_dma_v1;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod ethernet_mtl_v2;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod ethernet_mac_v2;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod hrtim_master;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod hrtim_tima;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod hrtim_timb;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod hrtim_timc;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod hrtim_timd;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod hrtim_time;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod hrtim_common_v1;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v"))]
pub mod dfsdm_v2;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod tim15_v2;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743", feature="stm32h743v", feature="stm32h743v", feature="stm32h753", feature="stm32h753", feature="stm32h753v", feature="stm32h753v"))]
pub mod tim1_v2;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod mdios;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743", feature="stm32h743v", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753", feature="stm32h753v", feature="stm32h753v"))]
pub mod tim2_v2;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod nvic_v2;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v"))]
pub mod dbgmcu_v2;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743", feature="stm32h743v", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753", feature="stm32h753v", feature="stm32h753v"))]
pub mod tim3_v2;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod tim12_v2;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743", feature="stm32h743v", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753", feature="stm32h753v", feature="stm32h753v"))]
pub mod tim13_v2;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod ramecc;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod ramecc3;

#[cfg(any(feature="doc", feature="stm32h743v", feature="stm32h753v"))]
pub mod rcc_v2;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod pwr_v2;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h7b3"))]
pub mod spi_v2;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h7b3"))]
pub mod adc_v3;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod rcc_v3;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod syscfg_v2;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod exti_v2;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod hash;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h7b3"))]
pub mod cryp_v1;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h7b3"))]
pub mod hrtim_common_v2;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h747cm7"))]
pub mod tim1_v3;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod art;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod ethernet_dma_v2;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod flash_v2;

#[cfg(any(feature="doc", feature="stm32h753", feature="stm32h753v"))]
pub mod cryp_v2;

#[cfg(any(feature="doc", feature="stm32h753", feature="stm32h753v"))]
pub mod dbgmcu_v3;

#[cfg(any(feature="doc", feature="stm32h7b3", feature="stm32h7b3"))]
pub mod tim1_v4;

