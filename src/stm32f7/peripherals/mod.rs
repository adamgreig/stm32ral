#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f730", feature="stm32f745", feature="stm32f745", feature="stm32f750", feature="stm32f750", feature="stm32f7x2", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x3"))]
pub mod tim1_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x7"))]
pub mod adc_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f730", feature="stm32f745", feature="stm32f745", feature="stm32f750", feature="stm32f750", feature="stm32f765", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x7", feature="stm32f7x9", feature="stm32f7x9"))]
pub mod tim6;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod adc_common;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod can1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod crc_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6"))]
pub mod dbgmcu_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod dac;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f7x6"))]
pub mod dma_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod exti_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod flash_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod fmc_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f730", feature="stm32f745", feature="stm32f745", feature="stm32f750", feature="stm32f750", feature="stm32f765", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x7", feature="stm32f7x9", feature="stm32f7x9"))]
pub mod tim9;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod tim2;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f730", feature="stm32f745", feature="stm32f745", feature="stm32f750", feature="stm32f750", feature="stm32f765", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x7", feature="stm32f7x9", feature="stm32f7x9"))]
pub mod tim3;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod tim5;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod gpio_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f730", feature="stm32f730", feature="stm32f730", feature="stm32f745", feature="stm32f745", feature="stm32f745", feature="stm32f745", feature="stm32f750", feature="stm32f750", feature="stm32f750", feature="stm32f750", feature="stm32f765", feature="stm32f765", feature="stm32f765", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x2", feature="stm32f7x2", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x3", feature="stm32f7x3", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x6", feature="stm32f7x6", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x7", feature="stm32f7x7", feature="stm32f7x7", feature="stm32f7x9", feature="stm32f7x9", feature="stm32f7x9", feature="stm32f7x9"))]
pub mod tim13;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod iwdg_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod i2c_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod lptim1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod pwr_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod quadspi;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod rng;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod rtc_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod rcc_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod sdmmc;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod sai_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod spi;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod syscfg_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod usart_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod otg_fs_global_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod otg_fs_host_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod otg_fs_device_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod otg_fs_pwrclk;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod otg_hs_host_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod otg_hs_global_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod otg_hs_pwrclk;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod otg_hs_device_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod wwdg;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod nvic_v1;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod mpu;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod stk;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod nvic_stir;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod fpu_cpacr;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod scb_actrl;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod fpu;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod scb;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod pf;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod ac;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod aes;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f7x2", feature="stm32f7x3"))]
pub mod usbphyc;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod hash;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod cryp;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod dcmi;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod fmc_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f7x6"))]
pub mod rcc_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod gpio_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f7x6"))]
pub mod syscfg_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x9"))]
pub mod adc_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod pwr_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod iwdg_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod ethernet_mac;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod ethernet_mmc;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod ethernet_ptp;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod ethernet_dma;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod crc_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod can;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f7x6"))]
pub mod flash_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod exti_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod ltdc;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod sai_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod dma2d;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod cec;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod spdifrx;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod i2c_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod rtc_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod usart_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod otg_fs_global_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod otg_fs_host_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod otg_fs_device_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod otg_hs_global_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod otg_hs_host_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod otg_hs_device_v2;

#[cfg(any(feature="doc", feature="stm32f745", feature="stm32f750", feature="stm32f7x6"))]
pub mod nvic_v2;

#[cfg(any(feature="doc", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod dma_v2;

#[cfg(any(feature="doc", feature="stm32f765", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod rcc_v3;

#[cfg(any(feature="doc", feature="stm32f765", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod syscfg_v3;

#[cfg(any(feature="doc", feature="stm32f765", feature="stm32f765", feature="stm32f7x6", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x7", feature="stm32f7x9", feature="stm32f7x9"))]
pub mod tim1_v2;

#[cfg(any(feature="doc", feature="stm32f765", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod flash_v3;

#[cfg(any(feature="doc", feature="stm32f765", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod jpeg;

#[cfg(any(feature="doc", feature="stm32f765", feature="stm32f7x7"))]
pub mod dfsdm1;

#[cfg(any(feature="doc", feature="stm32f765", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod dsi;

#[cfg(any(feature="doc", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod mdios;

#[cfg(any(feature="doc", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod dbgmcu_v2;

