#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod gpio;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod tsc;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x8"))]
pub mod dma;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim5;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod tim3_f301_f373_f3x4_f3x8;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim4_f301_f373_f3x8;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim19;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim15_f301_f373_f3x8;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim16_f301_f373_f3x8;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim17_f301_f373_f3x8;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x4"))]
pub mod usart_f301_f373_f3x4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x8"))]
pub mod usart_f302_f303_f3x8;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod spi;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f3x8"))]
pub mod exti_f301_f3x8;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod cec;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod pwr_f301_f373;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod can;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod usb;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod usb_fs;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod i2c_f301_f373;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod i2c_f302_f303_f3x4_f3x8;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod iwdg;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod wwdg;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod rtc_f301_f373_f3x8;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod sdadc;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod dac2;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim6_f301_f373_f3x8;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim7_f301_f373_f3x8;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim18;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim13;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim14;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim12;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod dac1;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod dbgmcu;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod syscfg_comp_opamp;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod nvic;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod fpu;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod mpu;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod stk;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod scb;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod nvic_stir;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod fpu_cpacr;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod scb_actrl;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod flash_f302_f303_f3x4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim2_f302_f303_f3x4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod tim3_f302_f303;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod tim4_f302_f303;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim15_f302_f303_f3x4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim16_f302_f303_f3x4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim17_f302_f303_f3x4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod exti_f302_f303_f3x4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod pwr_f302_f303;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod rtc_f302_f303_f3x4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim6_f302_f303_f3x4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim7_f302_f303_f3x4;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod dac;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod tim1;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod tim20;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod tim8;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod adc;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod adc_common;

#[cfg(any(feature="doc", feature="stm32f303", feature="stm32f3x8"))]
pub mod fmc;

#[cfg(any(feature="doc", feature="stm32f373", feature="stm32f3x8"))]
pub mod flash_f373_f3x8;

#[cfg(any(feature="doc", feature="stm32f373", feature="stm32f3x8"))]
pub mod rcc;

#[cfg(any(feature="doc", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim2_f373_f3x8;

