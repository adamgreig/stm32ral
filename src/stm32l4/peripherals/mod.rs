#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod dac1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod dma_v1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x5"))]
pub mod lcd_v1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod tsc;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod iwdg;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod wwdg;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod firewall;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod i2c;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod flash;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod pwr;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod syscfg;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod rng;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod aes;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5"))]
pub mod adc;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod sai1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x5", feature="stm32l4x6", feature="stm32l4x6"))]
pub mod tim2;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x5", feature="stm32l4x5", feature="stm32l4x6", feature="stm32l4x6"))]
pub mod tim3;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x6"))]
pub mod tim15_v1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x5", feature="stm32l4x6", feature="stm32l4x6"))]
pub mod tim16;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod tim1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x5", feature="stm32l4x6", feature="stm32l4x6"))]
pub mod tim6;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod lptim;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod usart;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2"))]
pub mod usart3;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod lpuart1_v1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod spi_v1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod sdmmc;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod exti;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod vrefbuf;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2"))]
pub mod can1_v1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3"))]
pub mod swpmi1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod opamp_v1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod nvic;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod crs;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x2"))]
pub mod usb;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x5"))]
pub mod dfsdm;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod quadspi;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3"))]
pub mod dbgmcu_v1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod fpu;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod mpu_v1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod stk;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod scb_v1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod nvic_stir;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod fpu_cpacr;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod scb_actrl;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3"))]
pub mod adc_common_v1;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod dma_v2;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod dmamux1;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod ltcd;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod comp_v1;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod dbgmcu_v2;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod octospi;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod adc1;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x6"))]
pub mod adc_common_v2;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x6"))]
pub mod gpio_v1;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x6"))]
pub mod gpioi;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod lpuart1_v2;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod spi_v2;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x6"))]
pub mod can1_v2;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod rtc;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod otg_fs_global;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod otg_fs_host;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod otg_fs_device;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod otg_fs_pwrclk;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x6"))]
pub mod fmc;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x6"))]
pub mod dcmi;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x6"))]
pub mod hash;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x6"))]
pub mod dma2d;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod gfxmmu;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod octospim;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod mpu_v2;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod scb_v2;

#[cfg(any(feature="doc", feature="stm32l4r9", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod tim8;

#[cfg(any(feature="doc", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod dac;

#[cfg(any(feature="doc", feature="stm32l4x1", feature="stm32l4x2"))]
pub mod comp_v2;

#[cfg(any(feature="doc", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3"))]
pub mod gpio_v2;

#[cfg(any(feature="doc", feature="stm32l4x1", feature="stm32l4x5"))]
pub mod tim15_v2;

#[cfg(any(feature="doc", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod opamp_v2;

#[cfg(any(feature="doc", feature="stm32l4x3", feature="stm32l4x6"))]
pub mod lcd_v2;

#[cfg(any(feature="doc", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod comp_v3;

#[cfg(any(feature="doc", feature="stm32l4x3", feature="stm32l4x5"))]
pub mod can1_v3;

