#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital-to-analog converter
//!
//! Used by: stm32wl5x_cm0p, stm32wl5x_cm4, stm32wle5

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register
pub mod CR {

    /// DAC Channel 1 calibration enable
    pub mod CEN1 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC Channel X Normal operating mode
            pub const Normal: u32 = 0b0;

            /// 0b1: DAC Channel X calibration mode
            pub const Calibration: u32 = 0b1;
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
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC Channel X DMA Underrun Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC Channel X DMA Underrun Interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC Channel X DMA mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC Channel X DMA mode enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0000: Unmask bit0 of LFSR/ triangle amplitude equal to 1
            pub const Amp1: u32 = 0b0000;

            /// 0b0001: Unmask bits\[1:0\] of LFSR/ triangle amplitude equal to 3
            pub const Amp3: u32 = 0b0001;

            /// 0b0010: Unmask bits\[2:0\] of LFSR/ triangle amplitude equal to 7
            pub const Amp7: u32 = 0b0010;

            /// 0b0011: Unmask bits\[3:0\] of LFSR/ triangle amplitude equal to 15
            pub const Amp15: u32 = 0b0011;

            /// 0b0100: Unmask bits\[4:0\] of LFSR/ triangle amplitude equal to 31
            pub const Amp31: u32 = 0b0100;

            /// 0b0101: Unmask bits\[5:0\] of LFSR/ triangle amplitude equal 63
            pub const Amp63: u32 = 0b0101;

            /// 0b0110: Unmask bits\[6:0\] of LFSR/ triangle amplitude equal to 127
            pub const Amp127: u32 = 0b0110;

            /// 0b0111: Unmask bits\[7:0\] of LFSR/ triangle amplitude equal to 255
            pub const Amp255: u32 = 0b0111;

            /// 0b1000: Unmask bits\[8:0\] of LFSR/ triangle amplitude equal to 511
            pub const Amp511: u32 = 0b1000;

            /// 0b1001: Unmask bits\[9:0\] of LFSR/ triangle amplitude equal to 1023
            pub const Amp1023: u32 = 0b1001;

            /// 0b1010: Unmask bits\[10:0\] of LFSR/ triangle amplitude equal to 2047
            pub const Amp2047: u32 = 0b1010;

            /// 0b1011: Unmask bits\[11:0\] of LFSR/ triangle amplitude equal to 4095
            pub const Amp4095: u32 = 0b1011;
        }
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

    /// DAC channel1 trigger enable
    pub mod TEN1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC Channel X trigger disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC Channel X trigger enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC Channel X disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC Channel X enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DAC channel1 trigger selection
    pub mod TSEL1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (4 bits: 0b1111 << 2)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: SWTRIG1
            pub const SWTRIG: u32 = 0b0000;

            /// 0b0001: dac_chx_trg1
            pub const TIM1_TRGO: u32 = 0b0001;

            /// 0b0010: dac_chx_trg2
            pub const TIM2_TRGO: u32 = 0b0010;

            /// 0b0011: dac_chx_trg3
            pub const TRG3: u32 = 0b0011;

            /// 0b0100: dac_chx_trg4
            pub const TRG4: u32 = 0b0100;

            /// 0b0101: dac_chx_trg5
            pub const TRG5: u32 = 0b0101;

            /// 0b0110: dac_chx_trg6
            pub const TRG6: u32 = 0b0110;

            /// 0b0111: dac_chx_trg7
            pub const TRG7: u32 = 0b0111;

            /// 0b1000: dac_chx_trg8
            pub const TRG8: u32 = 0b1000;

            /// 0b1001: dac_chx_trg9
            pub const TRG9: u32 = 0b1001;

            /// 0b1010: dac_chx_trg10
            pub const TRG10: u32 = 0b1010;

            /// 0b1011: dac_chx_trg11
            pub const LPTIM1_OUT: u32 = 0b1011;

            /// 0b1100: dac_chx_trg12
            pub const LPTIM2_OUT: u32 = 0b1100;

            /// 0b1101: dac_chx_trg13
            pub const LPTIM3_OUT: u32 = 0b1101;

            /// 0b1110: dac_chx_trg14
            pub const EXTI9: u32 = 0b1110;

            /// 0b1111: dac_chx_trg15
            pub const TRG15: u32 = 0b1111;
        }
    }
}

/// software trigger register
pub mod SWTRGR {

    /// DAC channel1 software trigger
    pub mod SWTRIG1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: No trigger
            pub const NoTrigger: u32 = 0b0;

            /// 0b1: Trigger
            pub const Trigger: u32 = 0b1;
        }
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

/// Dual DAC 12-bit right-aligned data holding register
pub mod DHR12RD {
    pub use super::DHR12R1::DACC1DHR;
}

/// Dual DAC 12-bit left aligned data holding register
pub mod DHR12LD {
    pub use super::DHR12L1::DACC1DHR;
}

/// Dual DAC 8-bit right aligned data holding register
pub mod DHR8RD {
    pub use super::DHR8R1::DACC1DHR;
}

/// DAC channel1 data output register
pub mod DOR1 {

    /// DACC1DOR
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

/// status register
pub mod SR {

    /// DAC Channel 1 busy writing sample time flag
    pub mod BWST1 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written
            pub const Idle: u32 = 0b0;

            /// 0b1: There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written
            pub const Busy: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC Channel 1 calibration offset status
    pub mod CAL_FLAG1 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Calibration trimming value is lower than the offset correction value
            pub const Lower: u32 = 0b0;

            /// 0b1: Calibration trimming value is equal or greater than the offset correction value
            pub const Equal_Higher: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
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
        /// Read-write values
        pub mod RW {

            /// 0b0: No DMA underrun error condition occurred for DAC channel x
            pub const NoError: u32 = 0b0;

            /// 0b1: DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)
            pub const Error: u32 = 0b1;
        }
    }
}

/// calibration control register
pub mod CCR {

    /// DAC Channel 1 offset trimming value
    pub mod OTRIM1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// mode control register
pub mod MCR {

    /// DAC Channel 1 mode
    pub mod MODE1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Normal mode - DAC channelx is connected to external pin with Buffer enabled
            pub const NormalPinBuffer: u32 = 0b000;

            /// 0b001: Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
            pub const NormalPinChipBuffer: u32 = 0b001;

            /// 0b010: Normal mode - DAC channelx is connected to external pin with Buffer disabled
            pub const NormalPinNoBuffer: u32 = 0b010;

            /// 0b011: Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled
            pub const NormalChipNoBuffer: u32 = 0b011;

            /// 0b100: S&H mode - DAC channelx is connected to external pin with Buffer enabled
            pub const SHPinBuffer: u32 = 0b100;

            /// 0b101: S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
            pub const SHPinChipBuffer: u32 = 0b101;

            /// 0b110: S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled
            pub const SHPinNoBuffer: u32 = 0b110;

            /// 0b111: S&H mode - DAC channelx is connected to on chip peripherals with Buffer disabled
            pub const SHChipNoBuffer: u32 = 0b111;
        }
    }
}

/// Sample and Hold sample time register 1
pub mod SHSR1 {

    /// DAC Channel 1 sample Time (only valid in Sample and Hold mode)
    pub mod TSAMPLE1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Sample and Hold hold time register
pub mod SHHR {

    /// DAC Channel 1 hold Time (only valid in Sample and Hold mode)
    pub mod THOLD1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Sample and Hold refresh time register
pub mod SHRR {

    /// DAC Channel 1 refresh Time (only valid in Sample and Hold mode)
    pub mod TREFRESH1 {
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
#[repr(C)]
pub struct RegisterBlock {
    /// control register
    pub CR: RWRegister<u32>,

    /// software trigger register
    pub SWTRGR: WORegister<u32>,

    /// channel1 12-bit right-aligned data holding register
    pub DHR12R1: RWRegister<u32>,

    /// channel1 12-bit left aligned data holding register
    pub DHR12L1: RWRegister<u32>,

    /// channel1 8-bit right aligned data holding register
    pub DHR8R1: RWRegister<u32>,

    _reserved1: [u8; 12],

    /// Dual DAC 12-bit right-aligned data holding register
    pub DHR12RD: RWRegister<u32>,

    /// Dual DAC 12-bit left aligned data holding register
    pub DHR12LD: RWRegister<u32>,

    /// Dual DAC 8-bit right aligned data holding register
    pub DHR8RD: RWRegister<u32>,

    /// DAC channel1 data output register
    pub DOR1: RORegister<u32>,

    _reserved2: [u8; 4],

    /// status register
    pub SR: RWRegister<u32>,

    /// calibration control register
    pub CCR: RWRegister<u32>,

    /// mode control register
    pub MCR: RWRegister<u32>,

    /// Sample and Hold sample time register 1
    pub SHSR1: RWRegister<u32>,

    _reserved3: [u8; 4],

    /// Sample and Hold hold time register
    pub SHHR: RWRegister<u32>,

    /// Sample and Hold refresh time register
    pub SHRR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SWTRGR: u32,
    pub DHR12R1: u32,
    pub DHR12L1: u32,
    pub DHR8R1: u32,
    pub DHR12RD: u32,
    pub DHR12LD: u32,
    pub DHR8RD: u32,
    pub DOR1: u32,
    pub SR: u32,
    pub CCR: u32,
    pub MCR: u32,
    pub SHSR1: u32,
    pub SHHR: u32,
    pub SHRR: u32,
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
