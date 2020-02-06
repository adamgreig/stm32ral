#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod gpio_v1;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod tsc;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod dma;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim5;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f301", feature="stm32f301", feature="stm32f373", feature="stm32f373", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8", feature="stm32f3x8", feature="stm32f3x8"))]
pub mod tim3_v1;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim15_v1;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f301", feature="stm32f373", feature="stm32f373", feature="stm32f3x8", feature="stm32f3x8", feature="stm32f3x8"))]
pub mod tim16_v1;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod usart;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod spi_v1;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f3x8"))]
pub mod exti_v1;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod cec;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod pwr_v1;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod can;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x8"))]
pub mod usb;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod i2c;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod iwdg;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod wwdg;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod rtc_v1;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod sdadc;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f301", feature="stm32f301", feature="stm32f373", feature="stm32f373", feature="stm32f373", feature="stm32f3x8", feature="stm32f3x8", feature="stm32f3x8"))]
pub mod tim6_v1;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f301", feature="stm32f373", feature="stm32f373", feature="stm32f3x8", feature="stm32f3x8"))]
pub mod tim13;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim12;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod dbgmcu;

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

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f302", feature="stm32f303", feature="stm32f303", feature="stm32f3x4", feature="stm32f3x8", feature="stm32f3x8"))]
pub mod tim1;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f3x4"))]
pub mod opamp_v1;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod flash_v1;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim2_v1;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim15_v2;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim16_v2;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim17;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod exti_v2;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod pwr_v2;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod rtc_v2;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f302", feature="stm32f303", feature="stm32f303", feature="stm32f3x4", feature="stm32f3x4"))]
pub mod tim6_v2;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod dac;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod tim8;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod adc;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod adc_common;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x8"))]
pub mod fmc;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f302", feature="stm32f303", feature="stm32f303"))]
pub mod tim3_v2;

#[cfg(any(feature="doc", feature="stm32f303", feature="stm32f3x8"))]
pub mod syscfg;

#[cfg(any(feature="doc", feature="stm32f303", feature="stm32f3x8"))]
pub mod opamp_v2;

#[cfg(any(feature="doc", feature="stm32f303", feature="stm32f3x8"))]
pub mod comp;

#[cfg(any(feature="doc", feature="stm32f373", feature="stm32f3x8"))]
pub mod gpio_v2;

#[cfg(any(feature="doc", feature="stm32f373", feature="stm32f3x8"))]
pub mod gpioc;

#[cfg(any(feature="doc", feature="stm32f373", feature="stm32f3x8"))]
pub mod flash_v2;

#[cfg(any(feature="doc", feature="stm32f373", feature="stm32f3x8"))]
pub mod tim2_v2;

#[cfg(any(feature="doc", feature="stm32f373", feature="stm32f3x8"))]
pub mod spi_v2;

#[cfg(any(feature="doc", feature="stm32f373", feature="stm32f3x8"))]
pub mod dac2;

#[cfg(any(feature="doc", feature="stm32f373", feature="stm32f3x8"))]
pub mod dac1;

