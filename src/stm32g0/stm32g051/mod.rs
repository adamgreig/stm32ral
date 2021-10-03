//! stm32ral module for stm32g051

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 2;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::adc_g050_g051_g061 as adc;
pub use super::instances::crc_g050_g051_g061_g0b0_g0b1_g0c1 as crc;
pub use super::instances::dac_g051_g061_g0b1_g0c1 as dac;
pub use super::instances::dmamux_g051_g061_g0b0_g0b1_g0c1 as dmamux;
pub use super::instances::hdmi_cec_g051_g061_g0b1_g0c1 as hdmi_cec;
pub use super::instances::i2c_g050_g051_g061_g0b0_g0b1_g0c1 as i2c;
pub use super::instances::iwdg_g050_g051_g061_g0b0_g0b1_g0c1 as iwdg;
pub use super::instances::lptim_g051_g061_g0b1_g0c1 as lptim;
pub use super::instances::tim14_g050_g051_g061_g0b0_g0b1_g0c1 as tim14;
pub use super::instances::tim15;
pub use super::instances::tim16_g051_g061_g0b0_g0b1_g0c1 as tim16;
pub use super::instances::tim17_g051_g061_g0b0_g0b1_g0c1 as tim17;
pub use super::instances::tim1_g051_g061_g0b0_g0b1_g0c1 as tim1;
pub use super::instances::tim2_g051_g0b1_g0c1 as tim2;
pub use super::instances::tim3_g051_g0b0_g0b1_g0c1 as tim3;
pub use super::instances::tim6_g050_g051_g061_g0b0_g0b1_g0c1 as tim6;
pub use super::instances::tim7_g050_g051_g061_g0b0_g0b1_g0c1 as tim7;
pub use super::instances::usart_g050_g051_g061 as usart;
pub use super::instances::vrefbuf_g051_g061_g0b0_g0b1_g0c1 as vrefbuf;
pub use super::instances::wwdg_g050_g051_g061_g0b0_g0b1_g0c1 as wwdg;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub ADC: adc::Instance,
    pub CRC: crc::Instance,
    pub DAC: dac::Instance,
    pub DMAMUX: dmamux::Instance,
    pub HDMI_CEC: hdmi_cec::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub IWDG: iwdg::Instance,
    pub LPTIM1: lptim::Instance,
    pub LPTIM2: lptim::Instance,
    pub TIM1: tim1::Instance,
    pub TIM2: tim2::Instance,
    pub TIM3: tim3::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub TIM14: tim14::Instance,
    pub TIM15: tim15::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub VREFBUF: vrefbuf::Instance,
    pub WWDG: wwdg::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            ADC: adc::ADC::steal(),
            CRC: crc::CRC::steal(),
            DAC: dac::DAC::steal(),
            DMAMUX: dmamux::DMAMUX::steal(),
            HDMI_CEC: hdmi_cec::HDMI_CEC::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            IWDG: iwdg::IWDG::steal(),
            LPTIM1: lptim::LPTIM1::steal(),
            LPTIM2: lptim::LPTIM2::steal(),
            TIM1: tim1::TIM1::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM15: tim15::TIM15::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            VREFBUF: vrefbuf::VREFBUF::steal(),
            WWDG: wwdg::WWDG::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
