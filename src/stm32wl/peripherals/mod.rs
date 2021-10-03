#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod aes;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod comp;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod dac;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod dma;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod dmamux;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4"))]
pub mod exti;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4"))]
pub mod flash;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod gpio;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod gpioc;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod gpioh;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4"))]
pub mod hsem;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod i2c;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod iwdg;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod lptim1;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod lptim;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod lpuart;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod mpu;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod nvic_stir;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod pka;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4"))]
pub mod pwr;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4"))]
pub mod rcc;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod rng;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod rtc;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod scb;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod scb_actrl;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod spi;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod stk;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4"))]
pub mod syscfg;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod tamp;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod tim1;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod tim2;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod tim16;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod tim17;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4"))]
pub mod tzic;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4"))]
pub mod tzsc;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod usart;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod vrefbuf;

#[cfg(any(feature="doc", feature="stm32wl5x_cm0p", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod wwdg;

#[cfg(any(feature="doc", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod adc;

#[cfg(any(feature="doc", feature="stm32wl5x_cm4", feature="stm32wle5"))]
pub mod nvic;

