#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod iwdg_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod wwdg;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod flash;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod rcc_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod pwr_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod dma_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod dmamux_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod gpio;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g041"))]
pub mod aes_g030_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod rng;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod exti_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod tim16;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod tim17;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod tim15;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod usart_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod usart_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod spi_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod tim1_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod adc_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod syscfg;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod tamp_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod lptim;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod lpuart_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod i2c;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod rtc_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod tim14;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod tim2;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod tim3;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod nvic_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod mpu;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod stk;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod scb;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod vrefbuf;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod dbg_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod nvic_stir;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod scb_actrl;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod fpu_cpacr;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod fpu;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod syscfg_itline;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod iwdg_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod dbg_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod rcc_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod pwr_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod dma_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod dmamux_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod aes_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod exti_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod spi_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod tim1_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod adc_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod comp;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod syscfg_vrefbuf;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod tamp_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod ucpd;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod lpuart_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod hdmi_cec;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod dac;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod rtc_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod tim6;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod tim7;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod nvic_g07x_g081;

