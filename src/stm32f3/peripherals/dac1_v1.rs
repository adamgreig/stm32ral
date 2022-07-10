#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital-to-analog converter
//!
//! Used by: stm32f302, stm32f303, stm32f3x4

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register
pub mod CR {

    /// DAC channel2 DMA underrun interrupt enable
    pub mod DMAUDRIE2 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC channel X DMA Underrun Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC channel X DMA Underrun Interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DAC channel2 DMA enable
    pub mod DMAEN2 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC channel X DMA mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC channel X DMA mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DAC channel2 mask/amplitude selector
    pub mod MAMP2 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel2 noise/triangle wave generation enable
    pub mod WAVE2 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Wave generation disabled
            pub const Disabled: u32 = 0b00;

            /// 0b01: Noise wave generation enabled
            pub const Noise: u32 = 0b01;

            /// 0b10: Triangle wave generation enabled
            pub const Triangle: u32 = 0b10;
        }
    }

    /// DAC channel2 trigger selection
    pub mod TSEL2 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (3 bits: 0b111 << 19)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Timer 6 TRGO event
            pub const TIM6_TRGO: u32 = 0b000;

            /// 0b001: Timer 8 TRGO event
            pub const TIM8_TRGO: u32 = 0b001;

            /// 0b010: Timer 7 TRGO event
            pub const TIM7_TRGO: u32 = 0b010;

            /// 0b011: Timer 5 TRGO event
            pub const TIM5_TRGO: u32 = 0b011;

            /// 0b100: Timer 2 TRGO event
            pub const TIM2_TRGO: u32 = 0b100;

            /// 0b101: Timer 4 TRGO event
            pub const TIM4_TRGO: u32 = 0b101;

            /// 0b110: EXTI line9
            pub const EXTI9: u32 = 0b110;

            /// 0b111: Software trigger
            pub const SOFTWARE: u32 = 0b111;
        }
    }

    /// DAC channel2 trigger enable
    pub mod TEN2 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC channel X trigger disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC channel X trigger enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DAC channel2 output buffer disable
    pub mod BOFF2 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC channel X output buffer enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: DAC channel X output buffer disabled
            pub const Disabled: u32 = 0b1;
        }
    }

    /// DAC channel2 enable
    pub mod EN2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC channel X disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC channel X enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DAC channel1 DMA Underrun Interrupt enable
    pub mod DMAUDRIE1 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMAUDRIE2::RW;
    }

    /// DAC channel1 DMA enable
    pub mod DMAEN1 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMAEN2::RW;
    }

    /// DAC channel1 mask/amplitude selector
    pub mod MAMP1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel1 noise/triangle wave generation enable
    pub mod WAVE1 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WAVE2::RW;
    }

    /// DAC channel1 trigger selection
    pub mod TSEL1 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Timer 6 TRGO event
            pub const TIM6_TRGO: u32 = 0b000;

            /// 0b001: Timer 3 TRGO event
            pub const TIM3_TRGO: u32 = 0b001;

            /// 0b010: Timer 7 TRGO event
            pub const TIM7_TRGO: u32 = 0b010;

            /// 0b011: Timer 15 TRGO event
            pub const TIM15_TRGO: u32 = 0b011;

            /// 0b100: Timer 2 TRGO event
            pub const TIM2_TRGO: u32 = 0b100;

            /// 0b110: EXTI line9
            pub const EXTI9: u32 = 0b110;

            /// 0b111: Software trigger
            pub const SOFTWARE: u32 = 0b111;
        }
    }

    /// DAC channel1 trigger enable
    pub mod TEN1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEN2::RW;
    }

    /// DAC channel1 output buffer disable
    pub mod BOFF1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BOFF2::RW;
    }

    /// DAC channel1 enable
    pub mod EN1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EN2::RW;
    }
}

/// software trigger register
pub mod SWTRIGR {

    /// DAC channel2 software trigger
    pub mod SWTRIG2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: DAC channel X software trigger disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC channel X software trigger enabled
            pub const Enabled: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel1 software trigger
    pub mod SWTRIG1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::SWTRIG2::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel1 12-bit right-aligned data holding register
pub mod DHR12R1 {

    /// DAC channel1 12-bit right-aligned data
    pub mod DACC1DHR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel1 12-bit left aligned data holding register
pub mod DHR12L1 {

    /// DAC channel1 12-bit left-aligned data
    pub mod DACC1DHR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (12 bits: 0xfff << 4)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel1 8-bit right aligned data holding register
pub mod DHR8R1 {

    /// DAC channel1 8-bit right-aligned data
    pub mod DACC1DHR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel2 12-bit right aligned data holding register
pub mod DHR12R2 {

    /// DAC channel2 12-bit right-aligned data
    pub mod DACC2DHR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel2 12-bit left aligned data holding register
pub mod DHR12L2 {

    /// DAC channel2 12-bit left-aligned data
    pub mod DACC2DHR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (12 bits: 0xfff << 4)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel2 8-bit right-aligned data holding register
pub mod DHR8R2 {

    /// DAC channel2 8-bit right-aligned data
    pub mod DACC2DHR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Dual DAC 12-bit right-aligned data holding register
pub mod DHR12RD {

    /// DAC channel2 12-bit right-aligned data
    pub mod DACC2DHR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel1 12-bit right-aligned data
    pub mod DACC1DHR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DUAL DAC 12-bit left aligned data holding register
pub mod DHR12LD {

    /// DAC channel2 12-bit left-aligned data
    pub mod DACC2DHR {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (12 bits: 0xfff << 20)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel1 12-bit left-aligned data
    pub mod DACC1DHR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (12 bits: 0xfff << 4)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DUAL DAC 8-bit right aligned data holding register
pub mod DHR8RD {

    /// DAC channel2 8-bit right-aligned data
    pub mod DACC2DHR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel1 8-bit right-aligned data
    pub mod DACC1DHR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel1 data output register
pub mod DOR1 {

    /// DAC channel1 data output
    pub mod DACC1DOR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel2 data output register
pub mod DOR2 {

    /// DAC channel2 data output
    pub mod DACC2DOR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// status register
pub mod SR {

    /// DAC channel2 DMA underrun flag
    pub mod DMAUDR2 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No DMA underrun error condition occurred for DAC channel X
            pub const NoUnderrun: u32 = 0b0;

            /// 0b1: DMA underrun error condition occurred for DAC channel X
            pub const Underrun: u32 = 0b1;
        }
    }

    /// DAC channel1 DMA underrun flag
    pub mod DMAUDR1 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DMAUDR2::RW;
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// control register
    pub CR: RWRegister<u32>,

    /// software trigger register
    pub SWTRIGR: WORegister<u32>,

    /// channel1 12-bit right-aligned data holding register
    pub DHR12R1: RWRegister<u32>,

    /// channel1 12-bit left aligned data holding register
    pub DHR12L1: RWRegister<u32>,

    /// channel1 8-bit right aligned data holding register
    pub DHR8R1: RWRegister<u32>,

    /// channel2 12-bit right aligned data holding register
    pub DHR12R2: RWRegister<u32>,

    /// channel2 12-bit left aligned data holding register
    pub DHR12L2: RWRegister<u32>,

    /// channel2 8-bit right-aligned data holding register
    pub DHR8R2: RWRegister<u32>,

    /// Dual DAC 12-bit right-aligned data holding register
    pub DHR12RD: RWRegister<u32>,

    /// DUAL DAC 12-bit left aligned data holding register
    pub DHR12LD: RWRegister<u32>,

    /// DUAL DAC 8-bit right aligned data holding register
    pub DHR8RD: RWRegister<u32>,

    /// channel1 data output register
    pub DOR1: RORegister<u32>,

    /// channel2 data output register
    pub DOR2: RORegister<u32>,

    /// status register
    pub SR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SWTRIGR: u32,
    pub DHR12R1: u32,
    pub DHR12L1: u32,
    pub DHR8R1: u32,
    pub DHR12R2: u32,
    pub DHR12L2: u32,
    pub DHR8R2: u32,
    pub DHR12RD: u32,
    pub DHR12LD: u32,
    pub DHR8RD: u32,
    pub DOR1: u32,
    pub DOR2: u32,
    pub SR: u32,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}
#[cfg(feature = "rtic")]
unsafe impl Send for Instance {}
