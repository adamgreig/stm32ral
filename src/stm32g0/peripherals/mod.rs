#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod iwdg_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod wwdg_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod flash;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod pwr_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod dma_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod dmamux_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g050", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod gpio;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod crc_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod exti_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g030", feature="stm32g031", feature="stm32g031", feature="stm32g041", feature="stm32g041", feature="stm32g070", feature="stm32g070", feature="stm32g081", feature="stm32g081", feature="stm32g081"))]
pub mod tim16_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod usart_v1;

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

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod i2c_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod rtc_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g081"))]
pub mod tim14_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g030", feature="stm32g031", feature="stm32g031", feature="stm32g041", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g071", feature="stm32g081", feature="stm32g081"))]
pub mod tim2_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod nvic_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g07x", feature="stm32g081"))]
pub mod mpu;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod stk;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod scb;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod vrefbuf_v1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod nvic_stir;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod scb_actrl;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod fpu_cpacr;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod fpu;

#[cfg(any(feature="doc", feature="stm32g031", feature="stm32g041", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod lptim_v1;

#[cfg(any(feature="doc", feature="stm32g031", feature="stm32g041"))]
pub mod lpuart_v1;

#[cfg(any(feature="doc", feature="stm32g031", feature="stm32g041"))]
pub mod dbg_v1;

#[cfg(any(feature="doc", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod rng;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061"))]
pub mod adc_v2;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod crc_v2;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod i2c_v2;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod iwdg_v2;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g050", feature="stm32g051", feature="stm32g051", feature="stm32g061", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0b1", feature="stm32g0c1", feature="stm32g0c1"))]
pub mod tim6_v1;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod tim14_v2;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g050"))]
pub mod tim16_v2;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod usart_v2;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod wwdg_v2;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod dac_v1;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod dmamux_v2;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod hdmi_cec_v1;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod lptim_v2;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod tim1_v2;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g051", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0b1", feature="stm32g0c1", feature="stm32g0c1"))]
pub mod tim2_v2;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod tim15;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g051", feature="stm32g061", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0b1", feature="stm32g0c1", feature="stm32g0c1"))]
pub mod tim16_v3;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod vrefbuf_v2;

#[cfg(any(feature="doc", feature="stm32g061", feature="stm32g061"))]
pub mod tim2_v3;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod iwdg_v3;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod dbg_v2;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod pwr_v2;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g081"))]
pub mod dma_v2;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod dmamux_v3;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071"))]
pub mod exti_v2;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod spi_v2;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g081"))]
pub mod tim1_v3;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod adc_v3;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod tamp_v2;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod rtc_v2;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g070", feature="stm32g071", feature="stm32g071", feature="stm32g07x", feature="stm32g07x", feature="stm32g081", feature="stm32g081"))]
pub mod tim6_v2;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod nvic_v2;

#[cfg(any(feature="doc", feature="stm32g071", feature="stm32g071"))]
pub mod tim16_v4;

#[cfg(any(feature="doc", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod comp;

#[cfg(any(feature="doc", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod syscfg_vrefbuf;

#[cfg(any(feature="doc", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod ucpd;

#[cfg(any(feature="doc", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod lpuart_v2;

#[cfg(any(feature="doc", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod hdmi_cec_v2;

#[cfg(any(feature="doc", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod dac_v2;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod rcc;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod aes;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g07x", feature="stm32g07x"))]
pub mod tim16_v5;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g07x"))]
pub mod tim2_v4;

#[cfg(any(feature="doc", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod dbg_v3;

#[cfg(any(feature="doc", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod dma1;

#[cfg(any(feature="doc", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod dma2;

