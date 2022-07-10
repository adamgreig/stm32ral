#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod ac;

#[cfg(any(feature="doc", feature="stm32h743v", feature="stm32h753v"))]
pub mod adc_h743v_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod adc_common;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod axi;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod can_ccu;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod cec;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod comp1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod crs;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod dac;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod dcmi;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h7b3"))]
pub mod dlyb_h735_h7b3;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod dlyb_h743_h743v_h747cm4_h747cm7_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod dfsdm_h747cm4_h747cm7_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod dma;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod dma2d;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod dmamux1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod dmamux2;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod exti_h735_h743_h743v_h753_h753v_h7b3;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod ethernet_mac_h735_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod fdcan;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod fmc;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod fpu;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod fpu_cpacr;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod flash_h735_h743_h743v_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod gpio;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod i2c;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod iwdg_h743_h743v_h753_h753v_h7b3;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod iwdg_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod lptim;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod lptim3;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod lpuart1;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod ltdc;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod mdios;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod mdma;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod mpu;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod nvic_h735_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod nvic_stir;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod opamp;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod otg_hs_device;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h7b3"))]
pub mod otg1_hs_global;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod otg_hs_global;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod otg_hs_host;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod otg_hs_pwrclk;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h7b3"))]
pub mod octospii_o_manager;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod pf;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod rng;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod rtc;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod sai_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod scb;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod scb_actrl;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod sdmmc_h735_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod spi_h735_h743_h743v_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod stk;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod swpmi;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h7b3"))]
pub mod tim2_h735_h7b3;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h7b3"))]
pub mod tim5_h735_h7b3;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h7b3"))]
pub mod tim3_h735_h7b3;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h7b3"))]
pub mod tim4_h735_h7b3;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod tim6;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod tim7;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h7b3"))]
pub mod tim12_h735_h7b3;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h7b3"))]
pub mod tim13_h735_h7b3;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h7b3"))]
pub mod tim14_h735_h7b3;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h7b3"))]
pub mod tim15_h735_h7b3;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod tim16;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod tim17;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod usart;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod vrefbuf;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod wwdg_h743_h743v_h753_h753v_h7b3;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod wwdg_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h735", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod ethernet_mtl_h735_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod bdma;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod hsem;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod jpeg;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod quadspi;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod sai_h743_h743v_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod sdmmc_h743_h743v_h753_h753v_h7b3;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod pwr_h743_h743v_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod spdifrx;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h753"))]
pub mod adc_h743_h753;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h753"))]
pub mod rcc_h743_h753;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod syscfg_h743_h743v_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod ethernet_dma_h743_h743v_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod ethernet_mtl_h743_h743v_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod ethernet_mac_h743_h743v_h753_h753v;

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

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod hrtim_common;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v"))]
pub mod dfsdm_h743_h743v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod tim15_h743_h743v_h747cm4_h747cm7_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod tim1_h743_h743v_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod tim8_h743_h743v_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod tim2_h743_h743v_h747cm4_h747cm7_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod tim5_h743_h743v_h747cm4_h747cm7_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod tim3_h743_h743v_h747cm4_h747cm7_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod tim4_h743_h743v_h747cm4_h747cm7_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod tim12_h743_h743v_h747cm4_h747cm7_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod tim13_h743_h743v_h747cm4_h747cm7_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod tim14_h743_h743v_h747cm4_h747cm7_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h753", feature="stm32h753v"))]
pub mod nvic_h743_h743v_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v"))]
pub mod dbgmcu_h743_h743v;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod ramecc;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod ramecc3;

#[cfg(any(feature="doc", feature="stm32h743v", feature="stm32h753v"))]
pub mod rcc_h743v_h753v;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod pwr_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h7b3"))]
pub mod spi_h747cm4_h747cm7_h7b3;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod adc_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod rcc_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod syscfg_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod exti_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v", feature="stm32h7b3"))]
pub mod hash;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h7b3"))]
pub mod cryp_h747cm4_h747cm7_h7b3;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod tim1_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod tim8_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod dbgmcu_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod art;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod ethernet_dma_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod dsihost;

#[cfg(any(feature="doc", feature="stm32h747cm4", feature="stm32h747cm7"))]
pub mod flash_h747cm4_h747cm7;

#[cfg(any(feature="doc", feature="stm32h753", feature="stm32h753v"))]
pub mod cryp_h753_h753v;

#[cfg(any(feature="doc", feature="stm32h753", feature="stm32h753v"))]
pub mod dbgmcu_h753_h753v;

