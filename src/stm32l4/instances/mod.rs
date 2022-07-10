#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod dac_l4r5_l4r9;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod dma_l412_l4x1_l4x2_l4x3_l4x5_l4x6;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x5"))]
pub mod lcd_l412_l4x1_l4x2_l4x5;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod tsc;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod iwdg;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod wwdg;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod firewall;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2"))]
pub mod i2c_l412_l4x1_l4x2;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x6"))]
pub mod i2c_l4r5_l4r9_l4x6;

#[cfg(any(feature="doc", feature="stm32l4x3", feature="stm32l4x5"))]
pub mod i2c_l4x3_l4x5;

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

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2"))]
pub mod adc;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3"))]
pub mod sai1;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod sai;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod tim2;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod tim5;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod tim3;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod tim4;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x6"))]
pub mod tim15_l412_l4r5_l4r9_l4x2_l4x3_l4x6;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod tim16;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod tim17;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod tim1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod tim6;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod tim7;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod lptim;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2"))]
pub mod usart_l412_l4x1_l4x2;

#[cfg(any(feature="doc", feature="stm32l4r9", feature="stm32l4x6"))]
pub mod usart_l4r9_l4x6;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2"))]
pub mod usart3;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod lpuart1_l412_l4x1_l4x2_l4x3_l4x5_l4x6;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2"))]
pub mod spi_l412_l4x1_l4x2;

#[cfg(any(feature="doc", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod spi_l4x3_l4x5_l4x6;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5"))]
pub mod sdmmc;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod sdmmc1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod exti;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod vrefbuf;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2"))]
pub mod can1_l412_l4x1_l4x2;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3"))]
pub mod swpmi1;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod opamp_l412_l4r5_l4r9;

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
pub mod dbgmcu_l412_l4x1_l4x2_l4x3;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod fpu;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod mpu_l412_l4x1_l4x2_l4x3_l4x5_l4x6;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod stk;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod scb_l412_l4x1_l4x2_l4x3_l4x5_l4x6;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod nvic_stir;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod fpu_cpacr;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod scb_actrl;

#[cfg(any(feature="doc", feature="stm32l412", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3"))]
pub mod adc_common_l412_l4x1_l4x2_l4x3;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod dma_l4r5_l4r9;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod dmamux1;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod ltcd;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod comp_l4r5_l4r9;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod dbgmcu_l4r5_l4r9_l4x5_l4x6;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod octospi;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod adc1;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x6"))]
pub mod adc_common_l4r5_l4r9_l4x6;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x6"))]
pub mod gpio_l4r5_l4r9_l4x6;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9", feature="stm32l4x6"))]
pub mod gpioi;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod lpuart1_l4r5_l4r9;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod spi_l4r5_l4r9;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod can1_l4r5_l4r9;

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
pub mod mpu_l4r5_l4r9;

#[cfg(any(feature="doc", feature="stm32l4r5", feature="stm32l4r9"))]
pub mod scb_l4r5_l4r9;

#[cfg(any(feature="doc", feature="stm32l4r9", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod tim8;

#[cfg(any(feature="doc", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod dac_l4x1_l4x2_l4x3_l4x5_l4x6;

#[cfg(any(feature="doc", feature="stm32l4x1", feature="stm32l4x2"))]
pub mod comp_l4x1_l4x2;

#[cfg(any(feature="doc", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3"))]
pub mod gpio_l4x1_l4x2_l4x3;

#[cfg(any(feature="doc", feature="stm32l4x1", feature="stm32l4x5"))]
pub mod tim15_l4x1_l4x5;

#[cfg(any(feature="doc", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod opamp_l4x1_l4x2_l4x3_l4x5_l4x6;

#[cfg(any(feature="doc", feature="stm32l4x3", feature="stm32l4x6"))]
pub mod lcd_l4x3_l4x6;

#[cfg(any(feature="doc", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod comp_l4x3_l4x5_l4x6;

#[cfg(any(feature="doc", feature="stm32l4x3", feature="stm32l4x5"))]
pub mod can1_l4x3_l4x5;

