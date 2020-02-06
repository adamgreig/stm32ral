#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod iwdg_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod wwdg;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod flash;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod rcc_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod pwr_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod dma_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod dmamux_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod gpio;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g041"))]
pub mod aes_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod rng;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod exti_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g030", feature="stm32g031", feature="stm32g031", feature="stm32g041", feature="stm32g041", feature="stm32g07x", feature="stm32g07x", feature="stm32g07x", feature="stm32g081", feature="stm32g081", feature="stm32g081"))]
pub mod tim16;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod usart;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod spi_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod tim1_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod adc_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod syscfg;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod tamp_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod lptim;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod lpuart_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod i2c;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod rtc_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod tim14;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g030", feature="stm32g031", feature="stm32g031", feature="stm32g041", feature="stm32g041", feature="stm32g07x", feature="stm32g07x", feature="stm32g081", feature="stm32g081"))]
pub mod tim2;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod nvic_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod mpu;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod stk;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod scb;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod vrefbuf;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod dbg_v1;

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
pub mod iwdg_v2;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod dbg_v2;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod rcc_v2;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod pwr_v2;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod dma_v2;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod dmamux_v2;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod aes_v2;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod exti_v2;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod spi_v2;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod tim1_v2;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod adc_v2;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod comp;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod syscfg_vrefbuf;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod tamp_v2;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod ucpd;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod lpuart_v2;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod hdmi_cec;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod dac;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod rtc_v2;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g07x", feature="stm32g081", feature="stm32g081"))]
pub mod tim6;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod nvic_v2;

