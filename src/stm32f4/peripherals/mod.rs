#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413"))]
pub mod adc_common_v1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413"))]
pub mod adc1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413"))]
pub mod dbgmcu_v1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod exti;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f411", feature="stm32f412", feature="stm32f413"))]
pub mod flash_v1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod iwdg;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f427", feature="stm32f429"))]
pub mod otg_fs_device_v1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f427", feature="stm32f429"))]
pub mod otg_fs_global_v1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod otg_fs_host;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod otg_fs_pwrclk;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f411", feature="stm32f412", feature="stm32f413"))]
pub mod pwr_v1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413"))]
pub mod rtc_v1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod sdio;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f411"))]
pub mod syscfg_v1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f401", feature="stm32f405", feature="stm32f405", feature="stm32f407", feature="stm32f407", feature="stm32f413", feature="stm32f413", feature="stm32f427", feature="stm32f427", feature="stm32f429", feature="stm32f429", feature="stm32f446", feature="stm32f446", feature="stm32f469", feature="stm32f469"))]
pub mod tim1_v1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f411", feature="stm32f412", feature="stm32f412", feature="stm32f412", feature="stm32f413", feature="stm32f413", feature="stm32f413", feature="stm32f427", feature="stm32f427", feature="stm32f427", feature="stm32f429", feature="stm32f429", feature="stm32f429", feature="stm32f446", feature="stm32f446", feature="stm32f446", feature="stm32f469", feature="stm32f469", feature="stm32f469"))]
pub mod tim10_v1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod tim11_v1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod tim2;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f401", feature="stm32f405", feature="stm32f405", feature="stm32f407", feature="stm32f407", feature="stm32f411", feature="stm32f411", feature="stm32f412", feature="stm32f412", feature="stm32f413", feature="stm32f413", feature="stm32f427", feature="stm32f427", feature="stm32f429", feature="stm32f429", feature="stm32f446", feature="stm32f446", feature="stm32f469", feature="stm32f469"))]
pub mod tim3;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod tim5;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f405", feature="stm32f407", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f427", feature="stm32f429", feature="stm32f429", feature="stm32f446", feature="stm32f446"))]
pub mod tim9_v1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod usart;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod wwdg;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod dma;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod gpio;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412"))]
pub mod i2c_v1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod spi;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413"))]
pub mod nvic_v1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod fpu;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod mpu;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod stk;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod scb;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod nvic_stir;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod fpu_cpacr;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod scb_actrl;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod rng;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod dcmi;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f412"))]
pub mod fsmc;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod dbgmcu_v2;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod rcc;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod syscfg_v2;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod adc;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod dac;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod pwr_v2;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod rtc_v2;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod uart;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod adc_common_v2;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f405", feature="stm32f405", feature="stm32f407", feature="stm32f407", feature="stm32f407"))]
pub mod tim10_v2;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod tim11_v2;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f405", feature="stm32f407", feature="stm32f407", feature="stm32f410", feature="stm32f427", feature="stm32f427", feature="stm32f429", feature="stm32f429", feature="stm32f446", feature="stm32f446", feature="stm32f469", feature="stm32f469"))]
pub mod tim6_v1;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod ethernet_ptp;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod can;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429"))]
pub mod otg_hs_global_v1;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod otg_hs_host;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429"))]
pub mod otg_hs_device_v1;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod sai1;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod ltdc;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod hash;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod cryp;

#[cfg(any(feature="doc", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod ethernet_mac;

#[cfg(any(feature="doc", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod ethernet_mmc;

#[cfg(any(feature="doc", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod ethernet_dma;

#[cfg(any(feature="doc", feature="stm32f410", feature="stm32f410"))]
pub mod tim1_v2;

#[cfg(any(feature="doc", feature="stm32f411", feature="stm32f411", feature="stm32f412", feature="stm32f412"))]
pub mod tim1_v3;

#[cfg(any(feature="doc", feature="stm32f412", feature="stm32f413"))]
pub mod syscfg_v3;

#[cfg(any(feature="doc", feature="stm32f412", feature="stm32f412"))]
pub mod tim6_v2;

#[cfg(any(feature="doc", feature="stm32f412", feature="stm32f413"))]
pub mod tim12;

#[cfg(any(feature="doc", feature="stm32f412", feature="stm32f413"))]
pub mod otg_fs_global_v2;

#[cfg(any(feature="doc", feature="stm32f412", feature="stm32f446", feature="stm32f469"))]
pub mod otg_fs_device_v2;

#[cfg(any(feature="doc", feature="stm32f412", feature="stm32f413", feature="stm32f446", feature="stm32f469"))]
pub mod quadspi;

#[cfg(any(feature="doc", feature="stm32f413", feature="stm32f413"))]
pub mod tim7;

#[cfg(any(feature="doc", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod i2c_v2;

#[cfg(any(feature="doc", feature="stm32f413", feature="stm32f427", feature="stm32f429"))]
pub mod sai;

#[cfg(any(feature="doc", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod dbgmcu_v3;

#[cfg(any(feature="doc", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod nvic_v2;

#[cfg(any(feature="doc", feature="stm32f427", feature="stm32f429"))]
pub mod flash_v2;

#[cfg(any(feature="doc", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod syscfg_v4;

#[cfg(any(feature="doc", feature="stm32f429", feature="stm32f469"))]
pub mod pwr_v3;

#[cfg(any(feature="doc", feature="stm32f429", feature="stm32f469"))]
pub mod dma2d;

#[cfg(any(feature="doc", feature="stm32f446", feature="stm32f469"))]
pub mod otg_hs_global_v2;

#[cfg(any(feature="doc", feature="stm32f446", feature="stm32f469"))]
pub mod otg_hs_device_v2;

#[cfg(any(feature="doc", feature="stm32f446", feature="stm32f469"))]
pub mod otg_hs_pwrclk;

#[cfg(any(feature="doc", feature="stm32f469", feature="stm32f469"))]
pub mod tim9_v2;

