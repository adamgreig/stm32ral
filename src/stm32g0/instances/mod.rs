#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod iwdg_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod wwdg_g030_g031_g041_g070_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod flash;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod pwr_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod dma_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod dmamux_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod gpio_g030_g031_g041_g070_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod gpio_g050_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod crc_g030_g031_g041_g070_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod exti_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g081"))]
pub mod tim16_g030_g031_g041_g070_g081;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g081"))]
pub mod tim17_g030_g031_g041_g070_g081;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod usart_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod usart_g071_g07x_g081;

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

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod i2c_g030_g031_g041_g070_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod rtc_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g081"))]
pub mod tim14_g030_g031_g041_g070_g071_g081;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g071", feature="stm32g081"))]
pub mod tim2_g030_g031_g041_g071_g081;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g081"))]
pub mod tim3_g030_g031_g041_g070_g071_g081;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod nvic_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g07x", feature="stm32g081"))]
pub mod mpu;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod stk;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod scb;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod vrefbuf_g030_g031_g041;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod nvic_stir;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod scb_actrl;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod fpu_cpacr;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041"))]
pub mod fpu;

#[cfg(any(feature="doc", feature="stm32g031", feature="stm32g041", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod lptim_g031_g041_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g031", feature="stm32g041"))]
pub mod lpuart_g031_g041;

#[cfg(any(feature="doc", feature="stm32g031", feature="stm32g041"))]
pub mod dbg_g031_g041;

#[cfg(any(feature="doc", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod rng;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061"))]
pub mod adc_g050_g051_g061;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod crc_g050_g051_g061_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod i2c_g050_g051_g061_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod iwdg_g050_g051_g061_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod tim6_g050_g051_g061_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod tim7_g050_g051_g061_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod tim14_g050_g051_g061_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061"))]
pub mod usart_g050_g051_g061;

#[cfg(any(feature="doc", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod usart_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g050", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod wwdg_g050_g051_g061_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod dac_g051_g061_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod dmamux_g051_g061_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod hdmi_cec_g051_g061_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod lptim_g051_g061_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod tim1_g051_g061_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod tim2_g051_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod tim3_g051_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod tim15;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod tim16_g051_g061_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod tim17_g051_g061_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g051", feature="stm32g061", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod vrefbuf_g051_g061_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod iwdg_g070_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod dbg_g070_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod pwr_g070_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g081"))]
pub mod dma_g070_g081;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod dmamux_g070_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071"))]
pub mod exti_g070_g071;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod spi_g070_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g081"))]
pub mod tim1_g070_g071_g081;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod adc_g070_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod tamp_g070_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod rtc_g070_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod tim6_g070_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod tim7_g070_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g070", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod nvic_g070_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod comp;

#[cfg(any(feature="doc", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod syscfg_vrefbuf;

#[cfg(any(feature="doc", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod ucpd;

#[cfg(any(feature="doc", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod lpuart_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod hdmi_cec_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g071", feature="stm32g07x", feature="stm32g081"))]
pub mod dac_g071_g07x_g081;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod rcc;

#[cfg(any(feature="doc", feature="stm32g07x", feature="stm32g081"))]
pub mod aes;

#[cfg(any(feature="doc", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod dbg_g0b0_g0b1_g0c1;

#[cfg(any(feature="doc", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod dma1;

#[cfg(any(feature="doc", feature="stm32g0b0", feature="stm32g0b1", feature="stm32g0c1"))]
pub mod dma2;

