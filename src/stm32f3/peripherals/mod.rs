#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod gpio;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod tsc;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x8"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod dma;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f301", feature="stm32f373", feature="stm32f373"))]
pub mod tim16_v1;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod usart;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod spi;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod cec;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373"))]
pub mod pwr;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod can;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373"))]
pub mod usb;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod i2c;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
pub mod iwdg;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod wwdg;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod rtc_v1;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f373", feature="stm32f3x8"))]
pub mod sdadc;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f301", feature="stm32f301", feature="stm32f373", feature="stm32f373", feature="stm32f373"))]
pub mod tim6_v1;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod dbgmcu;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod nvic;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod fpu;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4"))]
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

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod exti;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f302", feature="stm32f303", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim1_v1;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f3x4"))]
pub mod opamp;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod flash;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim2;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f302", feature="stm32f303", feature="stm32f303"))]
pub mod tim3_v1;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim15;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim16_v2;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod tim17;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod rtc_v2;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f302", feature="stm32f303", feature="stm32f303", feature="stm32f3x4", feature="stm32f3x4"))]
pub mod tim6_v2;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod dac1_v1;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod tim8;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x4"))]
pub mod adc;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303"))]
pub mod adc1_2;

#[cfg(any(feature="doc", feature="stm32f302", feature="stm32f303", feature="stm32f3x8"))]
pub mod fmc;

#[cfg(any(feature="doc", feature="stm32f373", feature="stm32f373", feature="stm32f373"))]
pub mod tim3_v2;

#[cfg(any(feature="doc", feature="stm32f373", feature="stm32f3x8"))]
pub mod dac2;

#[cfg(any(feature="doc", feature="stm32f373", feature="stm32f373"))]
pub mod tim13_v1;

#[cfg(any(feature="doc", feature="stm32f373", feature="stm32f3x8"))]
pub mod dac1_v2;

#[cfg(any(feature="doc", feature="stm32f3x8", feature="stm32f3x8", feature="stm32f3x8"))]
pub mod tim3_v3;

#[cfg(any(feature="doc", feature="stm32f3x8", feature="stm32f3x8", feature="stm32f3x8"))]
pub mod tim16_v3;

#[cfg(any(feature="doc", feature="stm32f3x8", feature="stm32f3x8", feature="stm32f3x8"))]
pub mod tim6_v3;

#[cfg(any(feature="doc", feature="stm32f3x8", feature="stm32f3x8"))]
pub mod tim13_v2;

#[cfg(any(feature="doc", feature="stm32f3x8", feature="stm32f3x8"))]
pub mod tim1_v2;

