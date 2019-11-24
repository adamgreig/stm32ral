#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413"))]
pub mod adc_common_f401_f410_f411_f412_f413;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413"))]
pub mod adc1;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413"))]
pub mod dbgmcu_f401_f410_f411_f412_f413;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod exti;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f411", feature="stm32f412", feature="stm32f413"))]
pub mod flash_f401_f405_f411_f412_f413;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod iwdg;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f427", feature="stm32f429"))]
pub mod otg_fs_device_f401_f405_f407_f411_f427_f429;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f427", feature="stm32f429"))]
pub mod otg_fs_global_f401_f405_f407_f411_f427_f429;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod otg_fs_host;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f446", feature="stm32f469"))]
pub mod otg_fs_pwrclk;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429"))]
pub mod otg_s_pwrclk;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f411", feature="stm32f412", feature="stm32f413"))]
pub mod pwr_f401_f411_f412_f413;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413"))]
pub mod rtc_f401_f410_f411_f412_f413;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod sdio;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f411"))]
pub mod syscfg_f401_f411;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod tim1_f401_f405_f407_f413_f427_f429_f446_f469;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod tim8_f401_f405_f407_f413_f427_f429_f446_f469;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod tim10_f401_f411_f412_f413_f427_f429_f446_f469;

#[cfg(any(feature="doc", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod tim13_f412_f413_f427_f429_f446_f469;

#[cfg(any(feature="doc", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod tim14_f412_f413_f427_f429_f446_f469;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod tim11_f401_f410_f411_f412_f413_f427_f429_f446_f469;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod tim2;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod tim3;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod tim4;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod tim5;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446"))]
pub mod tim9;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f446"))]
pub mod tim12_f405_f407_f427_f429_f446;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f410", feature="stm32f411"))]
pub mod usart_f401_f410_f411;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f446"))]
pub mod usart_f405_f407_f446;

#[cfg(any(feature="doc", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod usart_f427_f429_f469;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod wwdg;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod dma;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f411"))]
pub mod gpio_f401_f411;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427"))]
pub mod gpio_f405_f407_f427;

#[cfg(any(feature="doc", feature="stm32f429", feature="stm32f469"))]
pub mod gpio_f429_f469;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f411", feature="stm32f412"))]
pub mod i2c_f401_f405_f407_f411_f412;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod spi_f405_f407_f427_f429_f469;

#[cfg(any(feature="doc", feature="stm32f411", feature="stm32f412"))]
pub mod spi_f411_f412;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413"))]
pub mod nvic_f401_f405_f407_f410_f411_f412_f413;

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

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod rng;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod dcmi;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod fsmc;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod dbgmcu_f405_f407;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod rcc;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod syscfg_f405_f407;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod adc;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod dac;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod pwr_f405_f407;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod rtc_f405_f407_f427_f429_f446_f469;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod uart_f405_f407;

#[cfg(any(feature="doc", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod uart_f427_f429_f446_f469;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod adc_common_f405_f407_f427_f429_f446_f469;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod tim10_f405_f407;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod tim13_f405_f407;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod tim14_f405_f407;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407"))]
pub mod tim11_f405_f407;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod tim6;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod tim7;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod ethernet_ptp;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f412", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod can;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429"))]
pub mod otg_hs_global_f405_f407_f427_f429;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod otg_hs_host;

#[cfg(any(feature="doc", feature="stm32f405", feature="stm32f407", feature="stm32f427", feature="stm32f429"))]
pub mod otg_hs_device_f405_f407_f427_f429;

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

#[cfg(any(feature="doc", feature="stm32f411", feature="stm32f412"))]
pub mod tim1_f411_f412;

#[cfg(any(feature="doc", feature="stm32f411", feature="stm32f412"))]
pub mod tim8_f411_f412;

#[cfg(any(feature="doc", feature="stm32f412", feature="stm32f413"))]
pub mod syscfg_f412_f413;

#[cfg(any(feature="doc", feature="stm32f412", feature="stm32f413"))]
pub mod tim12_f412_f413;

#[cfg(any(feature="doc", feature="stm32f412", feature="stm32f413"))]
pub mod otg_fs_global_f412_f413;

#[cfg(any(feature="doc", feature="stm32f412", feature="stm32f446", feature="stm32f469"))]
pub mod otg_fs_device_f412_f446_f469;

#[cfg(any(feature="doc", feature="stm32f412", feature="stm32f413", feature="stm32f446", feature="stm32f469"))]
pub mod quadspi;

#[cfg(any(feature="doc", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod i2c_f427_f429_f446_f469;

#[cfg(any(feature="doc", feature="stm32f413", feature="stm32f429"))]
pub mod sai;

#[cfg(any(feature="doc", feature="stm32f427", feature="stm32f429", feature="stm32f469"))]
pub mod dbgmcu_f427_f429_f469;

#[cfg(any(feature="doc", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod nvic_f427_f429_f446_f469;

#[cfg(any(feature="doc", feature="stm32f427", feature="stm32f429"))]
pub mod flash_f427_f429;

#[cfg(any(feature="doc", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod syscfg_f429_f446_f469;

#[cfg(any(feature="doc", feature="stm32f429", feature="stm32f469"))]
pub mod pwr_f429_f469;

#[cfg(any(feature="doc", feature="stm32f429", feature="stm32f469"))]
pub mod dma2d;

#[cfg(any(feature="doc", feature="stm32f446", feature="stm32f469"))]
pub mod otg_hs_global_f446_f469;

#[cfg(any(feature="doc", feature="stm32f446", feature="stm32f469"))]
pub mod otg_hs_device_f446_f469;

#[cfg(any(feature="doc", feature="stm32f446", feature="stm32f469"))]
pub mod otg_hs_pwrclk;

