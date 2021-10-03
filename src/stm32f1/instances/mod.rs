#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f102"))]
pub mod fsmc_f100_f102;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod pwr;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod gpio;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod dma;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod bkp;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod iwdg;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod wwdg;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102"))]
pub mod tim1_f100_f101_f102;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f107"))]
pub mod tim8;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f102", feature="stm32f107"))]
pub mod tim13_f100_f102_f107;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f102", feature="stm32f107"))]
pub mod tim14_f100_f102_f107;

#[cfg(any(feature="doc", feature="stm32f102", feature="stm32f107"))]
pub mod tim10_f102_f107;

#[cfg(any(feature="doc", feature="stm32f102", feature="stm32f107"))]
pub mod tim11_f102_f107;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod tim6;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod tim7;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod i2c;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod usart;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f102"))]
pub mod dac_f100_f102;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101"))]
pub mod dbgmcu;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f103", feature="stm32f107"))]
pub mod uart;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod nvic;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod mpu;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod scb_actrl;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod nvic_stir;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod scb;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod stk;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f103"))]
pub mod fsmc_f101_f103;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f103"))]
pub mod afio;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f103"))]
pub mod exti;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod rtc;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod tim2;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod tim3;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod tim4;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod tim5;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod tim9;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod tim12;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f103"))]
pub mod tim10_f101_f103;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f103"))]
pub mod tim11_f101_f103;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f103"))]
pub mod tim13_f101_f103;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f103"))]
pub mod tim14_f101_f103;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f103", feature="stm32f107"))]
pub mod spi;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f103", feature="stm32f107"))]
pub mod dac_f101_f103_f107;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod flash;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod otg_fs_device;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f103"))]
pub mod otg_fs_global;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f103"))]
pub mod otg_fs_host;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod otg_fs_pwrclk;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f103"))]
pub mod ethernet_mmc;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f103"))]
pub mod ethernet_mac;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f103", feature="stm32f107"))]
pub mod ethernet_ptp;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f103"))]
pub mod ethernet_dma;

#[cfg(any(feature="doc", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod usb;

#[cfg(any(feature="doc", feature="stm32f102", feature="stm32f107"))]
pub mod sdio;

#[cfg(any(feature="doc", feature="stm32f102", feature="stm32f107"))]
pub mod adc2;

#[cfg(any(feature="doc", feature="stm32f102", feature="stm32f107"))]
pub mod adc3;

#[cfg(any(feature="doc", feature="stm32f103", feature="stm32f107"))]
pub mod tim1_f103_f107;

