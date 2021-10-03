#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital filter for sigma delta modulators

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DFSDM channel 0 configuration register
pub mod CFGR10 {

    /// Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register).
    pub mod SITP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: SPI with rising edge to strobe data
            pub const B_0x0: u32 = 0b00;

            /// 0b01: SPI with falling edge to strobe data
            pub const B_0x1: u32 = 0b01;

            /// 0b10: Manchester coded input on DATINy pin: rising edge = logic 0, falling edge = logic 1
            pub const B_0x2: u32 = 0b10;

            /// 0b11: Manchester coded input on DATINy pin: rising edge = logic 1, falling edge = logic 0
            pub const B_0x3: u32 = 0b11;
        }
    }

    /// SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Î£â modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Î£â modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
    pub mod SPICKSEL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: clock coming from external CKINy input - sampling point according SITP\[1:0\]
            pub const B_0x0: u32 = 0b00;

            /// 0b01: clock coming from internal CKOUT output - sampling point according SITP\[1:0\]
            pub const B_0x1: u32 = 0b01;
        }
    }

    /// Short-circuit detector enable on channel y
    pub mod SCDEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input channel y will not be guarded by the short-circuit detector
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Input channel y will be continuously guarded by the short-circuit detector
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Clock absence detector enable on channel y
    pub mod CKABEN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Clock absence detector disabled on channel y
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Clock absence detector enabled on channel y
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting.
    pub mod CHEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Channel y disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Channel y enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
    pub mod CHINSEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Channel inputs are taken from pins of the same channel y.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Channel inputs are taken from pins of the following channel (channel (y+1) modulo 8).
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\[1:0\] bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
    pub mod DATMPX {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Data to channel y are taken from external serial inputs as 1-bit values. DFSDM_CHyDATINR register is write protected.
            pub const B_0x0: u32 = 0b00;

            /// 0b01: Data to channel y are taken from internal analog to digital converter ADCy+1 output register update as 16-bit values (if ADCy+1 is available). Data from ADCs are written into INDAT0\[15:0\] part of DFSDM_CHyDATINR register.
            pub const B_0x1: u32 = 0b01;
        }
    }

    /// Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\[15:0\] (assigned to channel y) second sample INDAT1\[15:0\] (assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\[15:0\] part is read as first sample and then INDAT1\[15:0\] part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\[15:0\] (assigned to channel y) second sample INDAT1\[15:0\] (assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\[1:0\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
    pub mod DATPACK {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Standard: input data in DFSDM_CHyDATINR register are stored only in INDAT0\[15:0\]. To empty DFSDM_CHyDATINR register one sample must be read by the DFSDM filter from channel y.
            pub const B_0x0: u32 = 0b00;

            /// 0b01: Interleaved: input data in DFSDM_CHyDATINR register are stored as two samples:
            pub const B_0x1: u32 = 0b01;
        }
    }

    /// Output serial clock divider Â 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2Â -
    pub mod CKOUTDIV {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: Output clock generation is disabled (CKOUT signal is set to low state)
            pub const B_0x0: u32 = 0b00000000;
        }
    }

    /// Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)
    pub mod CKOUTSRC {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Source for output clock is from system clock
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Source for output clock is from audio clock
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)
    pub mod DFSDMEN {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DFSDM interface disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DFSDM interface enabled
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// DFSDM channel 0 configuration register
pub mod CFGR20 {

    /// Data right bit-shift for channel y will be performed to have final results. Bit-shift is performed before offset correction. The data shift is rounding the result to nearest integer value. The sign of shifted result is maintained (to have valid 24-bit signed format of result data). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). 0-31: Defines the shift of the data result coming from the integrator - how many bit shifts to the right
    pub mod DTRBS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (5 bits: 0b11111 << 3)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 24-bit calibration offset for channel y For channel y, OFFSET is applied to the results of each conversion from this channel. This value is set by software.
    pub mod OFFSET {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM channel 0 analog watchdog and short-circuit detector register
pub mod AWSCDR0 {

    /// short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel.
    pub mod SCDT {
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

    /// Break signal assignment for short-circuit detector on channel y BKSCD\[i\] = 0: Break i signal not assigned to short-circuit detector on channel y BKSCD\[i\] = 1: Break i signal assigned to short-circuit detector on channel y
    pub mod BKSCD {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1). This number is
    pub mod AWFOSR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog Sinc filter order on channel y 2: Sinc2 filter type 3: Sinc3 filter type Sincx filter type transfer function: FastSinc filter type transfer function: This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
    pub mod AWFORD {
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

            /// 0b00: FastSinc filter type
            pub const B_0x0: u32 = 0b00;

            /// 0b01: Sinc1 filter type
            pub const B_0x1: u32 = 0b01;
        }
    }
}

/// DFSDM channel 0 watchdog filter data register
pub mod WDATR0 {

    /// Input channel y watchdog data Data converted by the analog watchdog filter for input channel y. This data is continuously converted (no trigger) for this channel, with a limited resolution (OSR=1..32/sinc order = 1..3).
    pub mod WDATA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM channel 0 data input register
pub mod DATINR0 {

    /// Input data for channel y Input parallel channel data to be processed by the digital filter if DATMPX\[1:0\]=1 or DATMPX\[1:0\]=2. Data can be written by CPU/DMA (if DATMPX\[1:0\]=2) or directly by internal ADC (if DATMPX\[1:0\]=1). If DATPACK\[1:0\]=0 (standard mode) Channel y data sample is stored into INDAT0\[15:0\]. If DATPACK\[1:0\]=1 (interleaved mode) First channel y data sample is stored into INDAT0\[15:0\]. Second channel y data sample is stored into INDAT1\[15:0\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\[1:0\]=2 (dual mode). For even y channels: Channel y data sample is stored into INDAT0\[15:0\]. For odd y channels: INDAT0\[15:0\] is write protected. See for more details. INDAT0\[15:0\] is in the16-bit signed format.
    pub mod INDAT0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input data for channel y or channel y+1 Input parallel channel data to be processed by the digital filter if DATMPX\[1:0\]=1 or DATMPX\[1:0\]=2. Data can be written by CPU/DMA (if DATMPX\[1:0\]=2) or directly by internal ADC (if DATMPX\[1:0\]=1). If DATPACK\[1:0\]=0 (standard mode) INDAT0\[15:0\] is write protected (not used for input sample). If DATPACK\[1:0\]=1 (interleaved mode) Second channel y data sample is stored into INDAT1\[15:0\]. First channel y data sample is stored into INDAT0\[15:0\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\[1:0\]=2 (dual mode). For even y channels: sample in INDAT1\[15:0\] is automatically copied into INDAT0\[15:0\] of channel (y+1). For odd y channels: INDAT1\[15:0\] is write protected. See for more details. INDAT0\[15:1\] is in the16-bit signed format.
    pub mod INDAT1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

///
pub mod DLYR0 {

    /// Pulses to skip for input data skipping function immediately after writing to this field. Reading of PLSSKP\[5:0\] returns current value of pulses which will be skipped. If PLSSKP\[5:0\]=0 then all required data samples were already skipped. Note: User can update PLSSKP\[5:0\] also when PLSSKP\[5:0\] is not zero. 0-63: Defines the number of serial input samples that will be skipped. Skipping is applied
    pub mod PLSSKP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DFSDM channel 0 configuration register
pub mod CFGR11 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// DFSDM channel 0 configuration register
pub mod CFGR21 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// DFSDM channel 0 analog watchdog and short-circuit detector register
pub mod AWSCDR1 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// DFSDM channel 0 watchdog filter data register
pub mod WDATR1 {
    pub use super::WDATR0::WDATA;
}

/// DFSDM channel 0 data input register
pub mod DATINR1 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

///
pub mod DLYR1 {
    pub use super::DLYR0::PLSSKP;
}

/// DFSDM channel 0 configuration register
pub mod CFGR12 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// DFSDM channel 0 configuration register
pub mod CFGR22 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// DFSDM channel 0 analog watchdog and short-circuit detector register
pub mod AWSCDR2 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// DFSDM channel 0 watchdog filter data register
pub mod WDATR2 {
    pub use super::WDATR0::WDATA;
}

/// DFSDM channel 0 data input register
pub mod DATINR2 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

///
pub mod DLYR2 {
    pub use super::DLYR0::PLSSKP;
}

/// DFSDM channel 0 configuration register
pub mod CFGR13 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// DFSDM channel 0 configuration register
pub mod CFGR23 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// DFSDM channel 0 analog watchdog and short-circuit detector register
pub mod AWSCDR3 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// DFSDM channel 0 watchdog filter data register
pub mod WDATR3 {
    pub use super::WDATR0::WDATA;
}

/// DFSDM channel 0 data input register
pub mod DATINR3 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

///
pub mod DLYR3 {
    pub use super::DLYR0::PLSSKP;
}

/// DFSDM channel 0 configuration register
pub mod CFGR14 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// DFSDM channel 0 configuration register
pub mod CFGR24 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// DFSDM channel 0 analog watchdog and short-circuit detector register
pub mod AWSCDR4 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// DFSDM channel 0 watchdog filter data register
pub mod WDATR4 {
    pub use super::WDATR0::WDATA;
}

/// DFSDM channel 0 data input register
pub mod DATINR4 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

///
pub mod DLYR4 {
    pub use super::DLYR0::PLSSKP;
}

/// DFSDM channel 0 configuration register
pub mod CFGR15 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// DFSDM channel 0 configuration register
pub mod CFGR25 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// DFSDM channel 0 analog watchdog and short-circuit detector register
pub mod AWSCDR5 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// DFSDM channel 0 watchdog filter data register
pub mod WDATR5 {
    pub use super::WDATR0::WDATA;
}

/// DFSDM channel 0 data input register
pub mod DATINR5 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

///
pub mod DLYR5 {
    pub use super::DLYR0::PLSSKP;
}

/// DFSDM channel 0 configuration register
pub mod CFGR16 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// DFSDM channel 0 configuration register
pub mod CFGR26 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// DFSDM channel 0 analog watchdog and short-circuit detector register
pub mod AWSCDR6 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// DFSDM channel 0 watchdog filter data register
pub mod WDATR6 {
    pub use super::WDATR0::WDATA;
}

/// DFSDM channel 0 data input register
pub mod DATINR6 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

///
pub mod DLYR6 {
    pub use super::DLYR0::PLSSKP;
}

/// DFSDM channel 0 configuration register
pub mod CFGR17 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// DFSDM channel 0 configuration register
pub mod CFGR27 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// DFSDM channel 0 analog watchdog and short-circuit detector register
pub mod AWSCDR7 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// DFSDM channel 0 watchdog filter data register
pub mod WDATR7 {
    pub use super::WDATR0::WDATA;
}

/// DFSDM channel 0 data input register
pub mod DATINR7 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

///
pub mod DLYR7 {
    pub use super::DLYR0::PLSSKP;
}

///
pub mod CR10 {

    /// DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state
    pub mod DFEN {
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

            /// 0b0: DFSDM_FLTx is disabled. All conversions of given DFSDM_FLTx are stopped immediately and all DFSDM_FLTx functions are stopped.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DFSDM_FLTx is enabled. If DFSDM_FLTx is enabled, then DFSDM_FLTx starts operating according to its setting.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Start a conversion of the injected group of channels This bit is always read as '0â.
    pub mod JSWSTART {
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

            /// 0b0: Writing '0â has no effect.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Writing '1â makes a request to convert the channels in the injected conversion group, causing JCIP to become '1â at the same time. If JCIP=1 already, then writing to JSWSTART has no effect. Writing '1â has no effect if JSYNC=1.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    pub mod JSYNC {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Do not launch an injected conversion synchronously with DFSDM_FLT0
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Launch an injected conversion in this DFSDM_FLTx at the very moment when an injected conversion is launched in DFSDM_FLT0 by its JSWSTART trigger
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel.
    pub mod JSCAN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: One channel conversion is performed from the injected channel group and next the selected channel from this group is selected.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The series of conversions for the injected group channels is executed, starting over with the lowest selected channel.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    pub mod JDMAEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The DMA channel is not enabled to read injected data
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The DMA channel is enabled to read injected data
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger).
    pub mod JEXTSEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    pub mod JEXTEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Trigger detection is disabled
            pub const B_0x0: u32 = 0b00;

            /// 0b01: Each rising edge on the selected trigger makes a request to launch an injected conversion
            pub const B_0x1: u32 = 0b01;

            /// 0b10: Each falling edge on the selected trigger makes a request to launch an injected conversion
            pub const B_0x2: u32 = 0b10;

            /// 0b11: Both rising edges and falling edges on the selected trigger make requests to launch injected conversions
            pub const B_0x3: u32 = 0b11;
        }
    }

    /// Software start of a conversion on the regular channel This bit is always read as '0â.
    pub mod RSWSTART {
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

            /// 0b0: Writing '0â has no effect
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Writing '1â makes a request to start a conversion on the regular channel and causes RCIP to become '1â. If RCIP=1 already, writing to RSWSTART has no effect. Writing '1â has no effect if RSYNC=1.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Continuous mode selection for regular conversions Writing '0â to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately.
    pub mod RCONT {
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

            /// 0b0: The regular channel is converted just once for each conversion request
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The regular channel is converted repeatedly after each conversion request
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    pub mod RSYNC {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Do not launch a regular conversion synchronously with DFSDM_FLT0
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Launch a regular conversion in this DFSDM_FLTx at the very moment when a regular conversion is launched in DFSDM_FLT0
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    pub mod RDMAEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The DMA channel is not enabled to read regular data
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The DMA channel is enabled to read regular data
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion).
    pub mod RCH {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Channel 0 is selected as the regular channel
            pub const B_0x0: u32 = 0b000;

            /// 0b001: Channel 1 is selected as the regular channel
            pub const B_0x1: u32 = 0b001;
        }
    }

    /// Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \[FOSR * (IOSR-1 + FORD) + FORD\] / fCKIN ..... for Sincx filters t = \[FOSR * (IOSR-1 + 4) + 2\] / fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \[FOSR * IOSR\] / fCKIN in case if FOSR = FOSR\[9:0\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input.
    pub mod FAST {
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

            /// 0b0: Fast conversion mode disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Fast conversion mode enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Analog watchdog fast mode select
    pub mod AWFSEL {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Analog watchdog on data output value (after the digital filter). The comparison is done after offset correction and shift
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Analog watchdog on channel transceivers value (after watchdog filter)
            pub const B_0x1: u32 = 0b1;
        }
    }
}

///
pub mod CR20 {

    /// Injected end of conversion interrupt enable Please see the explanation of JEOCF in DFSDM_FLTxISR.
    pub mod JEOCIE {
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

            /// 0b0: Injected end of conversion interrupt is disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Injected end of conversion interrupt is enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Regular end of conversion interrupt enable Please see the explanation of REOCF in DFSDM_FLTxISR.
    pub mod REOCIE {
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

            /// 0b0: Regular end of conversion interrupt is disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Regular end of conversion interrupt is enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Injected data overrun interrupt enable Please see the explanation of JOVRF in DFSDM_FLTxISR.
    pub mod JOVRIE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Injected data overrun interrupt is disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Injected data overrun interrupt is enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Regular data overrun interrupt enable Please see the explanation of ROVRF in DFSDM_FLTxISR.
    pub mod ROVRIE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Regular data overrun interrupt is disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Regular data overrun interrupt is enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Analog watchdog interrupt enable Please see the explanation of AWDF in DFSDM_FLTxISR.
    pub mod AWDIE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Analog watchdog interrupt is disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Analog watchdog interrupt is enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Short-circuit detector interrupt enable Please see the explanation of SCDF\[7:0\] in DFSDM_FLTxISR. Note: SCDIE is present only in DFSDM_FLT0CR2 register (filter x=0)
    pub mod SCDIE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: short-circuit detector interrupt is disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: short-circuit detector interrupt is enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Clock absence interrupt enable Please see the explanation of CKABF\[7:0\] in DFSDM_FLTxISR. Note: CKABIE is present only in DFSDM_FLT0CR2 register (filter x=0)
    pub mod CKABIE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Detection of channel input clock absence interrupt is disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Detection of channel input clock absence interrupt is enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Extremes detector channel selection These bits select the input channels to be taken by the Extremes detector. EXCH\[y\] = 0: Extremes detector does not accept data from channel y EXCH\[y\] = 1: Extremes detector accepts data from channel y
    pub mod EXCH {
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

    /// Analog watchdog channel selection These bits select the input channel to be guarded continuously by the analog watchdog. AWDCH\[y\] = 0: Analog watchdog is disabled on channel y AWDCH\[y\] = 1: Analog watchdog is enabled on channel y
    pub mod AWDCH {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

///
pub mod ISR0 {

    /// End of injected conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxJDATAR.
    pub mod JEOCF {
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

            /// 0b0: No injected conversion has completed
            pub const B_0x0: u32 = 0b0;

            /// 0b1: An injected conversion has completed and its data may be read
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// End of regular conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxRDATAR.
    pub mod REOCF {
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

            /// 0b0: No regular conversion has completed
            pub const B_0x0: u32 = 0b0;

            /// 0b1: A regular conversion has completed and its data may be read
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Injected conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRJOVRF bit in the DFSDM_FLTxICR register.
    pub mod JOVRF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No injected conversion overrun has occurred
            pub const B_0x0: u32 = 0b0;

            /// 0b1: An injected conversion overrun has occurred, which means that an injected conversion finished while JEOCF was already '1â. JDATAR is not affected by overruns
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Regular conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRROVRF bit in the DFSDM_FLTxICR register.
    pub mod ROVRF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No regular conversion overrun has occurred
            pub const B_0x0: u32 = 0b0;

            /// 0b1: A regular conversion overrun has occurred, which means that a regular conversion finished while REOCF was already '1â. RDATAR is not affected by overruns
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Analog watchdog This bit is set by hardware. It is cleared by software by clearing all source flag bits AWHTF\[7:0\] and AWLTF\[7:0\] in DFSDM_FLTxAWSR register (by writing '1â into the clear bits in DFSDM_FLTxAWCFR register).
    pub mod AWDF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Analog watchdog event occurred
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The analog watchdog block detected voltage which crosses the value programmed in the DFSDM_FLTxAWLTR or DFSDM_FLTxAWHTR registers.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Injected conversion in progress status A request to start an injected conversion is ignored when JCIP=1.
    pub mod JCIP {
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

            /// 0b0: No request to convert the injected channel group (neither by software nor by trigger) has been issued
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The conversion of the injected channel group is in progress or a request for a injected conversion is pending, due either to '1â being written to JSWSTART or to a trigger detection
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Regular conversion in progress status A request to start a regular conversion is ignored when RCIP=1.
    pub mod RCIP {
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

            /// 0b0: No request to convert the regular channel has been issued
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The conversion of the regular channel is in progress or a request for a regular conversion is pending
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Clock absence flag CKABF\[y\]=0: Clock signal on channel y is present. CKABF\[y\]=1: Clock signal on channel y is not present. Given y bit is set by hardware when clock absence is detected on channel y. It is held at CKABF\[y\]=1 state by hardware when CHEN=0 (see DFSDM_CHyCFGR1 register). It is held at CKABF\[y\]=1 state by hardware when the transceiver is not yet synchronized.It can be cleared by software using the corresponding CLRCKABF\[y\] bit in the DFSDM_FLTxICR register. Note: CKABF\[7:0\] is present only in DFSDM_FLT0ISR register (filter x=0)
    pub mod CKABF {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// short-circuit detector flag SDCF\[y\]=0: No short-circuit detector event occurred on channel y SDCF\[y\]=1: The short-circuit detector counter reaches, on channel y, the value programmed in the DFSDM_CHyAWSCDR registers This bit is set by hardware. It can be cleared by software using the corresponding CLRSCDF\[y\] bit in the DFSDM_FLTxICR register. SCDF\[y\] is cleared also by hardware when CHEN\[y\] = 0 (given channel is disabled). Note: SCDF\[7:0\] is present only in DFSDM_FLT0ISR register (filter x=0)
    pub mod SCDF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

///
pub mod ICR0 {

    /// Clear the injected conversion overrun flag
    pub mod CLRJOVRF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Writing '0â has no effect
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Writing '1â clears the JOVRF bit in the DFSDM_FLTxISR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Clear the regular conversion overrun flag
    pub mod CLRROVRF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Writing '0â has no effect
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Writing '1â clears the ROVRF bit in the DFSDM_FLTxISR register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Clear the clock absence flag CLRCKABF\[y\]=0: Writing '0â has no effect CLRCKABF\[y\]=1: Writing '1â to position y clears the corresponding CKABF\[y\] bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\[y\]. Note: CLRCKABF\[7:0\] is present only in DFSDM_FLT0ICR register (filter x=0)
    pub mod CLRCKABF {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear the short-circuit detector flag CLRSCDF\[y\]=0: Writing '0â has no effect CLRSCDF\[y\]=1: Writing '1â to position y clears the corresponding SCDF\[y\] bit in the DFSDM_FLTxISR register Note: CLRSCDF\[7:0\] is present only in DFSDM_FLT0ICR register (filter x=0)
    pub mod CLRSCDF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

///
pub mod JCHGR0 {

    /// Injected channel group selection JCHG\[y\]=0: channel y is not part of the injected group JCHG\[y\]=1: channel y is part of the injected group If JSCAN=1, each of the selected channels is converted, one after another. The lowest channel (channel 0, if selected) is converted first and the sequence ends at the highest selected channel. If JSCAN=0, then only one channel is converted from the selected channels, and the channel selection is moved to the next channel. Writing JCHG, if JSCAN=0, resets the channel selection to the lowest selected channel. At least one channel must always be selected for the injected group. Writes causing all JCHG bits to be zero are ignored.
    pub mod JCHG {
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

///
pub mod FCR0 {

    /// Integrator oversampling ratio (averaging length) from Sinc filter will be summed into one output data sample from the integrator. The output data rate from the integrator will be decreased by this number (additional data decimation ratio). This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If IOSR = 0, then the Integrator has no effect (Integrator bypass). 0- 255: The length of the Integrator in the range 1 - 256 (IOSR + 1). Defines how many samples
    pub mod IOSR {
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

    /// Sinc filter oversampling ratio (decimation rate) number is also the decimation ratio of the output data rate from filter. This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If FOSR = 0, then the filter has no effect (filter bypass). 0 - 1023: Defines the length of the Sinc type filter in the range 1 - 1024 (FOSR = FOSR\[9:0\] +1). This
    pub mod FOSR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (10 bits: 0x3ff << 16)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Sinc filter order 2: Sinc2 filter type 3: Sinc3 filter type 4: Sinc4 filter type 5: Sinc5 filter type 6-7: Reserved Sincx filter type transfer function: FastSinc filter type transfer function: This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1).
    pub mod FORD {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (3 bits: 0b111 << 29)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: FastSinc filter type
            pub const B_0x0: u32 = 0b000;

            /// 0b001: Sinc1 filter type
            pub const B_0x1: u32 = 0b001;
        }
    }
}

///
pub mod JDATAR0 {

    /// Injected channel most recently converted When each conversion of a channel in the injected group finishes, JDATACH\[2:0\] is updated to indicate which channel was converted. Thus, JDATA\[23:0\] holds the data that corresponds to the channel indicated by JDATACH\[2:0\].
    pub mod JDATACH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected group conversion data When each conversion of a channel in the injected group finishes, its resulting data is stored in this field. The data is valid when JEOCF=1. Reading this register clears the corresponding JEOCF.
    pub mod JDATA {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

///
pub mod RDATAR0 {

    /// Regular channel most recently converted When each regular conversion finishes, RDATACH\[2:0\] is updated to indicate which channel was converted (because regular channel selection RCH\[2:0\] in DFSDM_FLTxCR1 register can be updated during regular conversion). Thus RDATA\[23:0\] holds the data that corresponds to the channel indicated by RDATACH\[2:0\].
    pub mod RDATACH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Regular channel pending data Regular data in RDATA\[23:0\] was delayed due to an injected channel trigger during the conversion
    pub mod RPEND {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Regular channel conversion data When each regular conversion finishes, its data is stored in this register. The data is valid when REOCF=1. Reading this register clears the corresponding REOCF.
    pub mod RDATA {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

///
pub mod AWHTR0 {

    /// Break signal assignment to analog watchdog high threshold event BKAWH\[i\] = 0: Break i signal is not assigned to an analog watchdog high threshold event BKAWH\[i\] = 1: Break i signal is assigned to an analog watchdog high threshold event
    pub mod BKAWH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog high threshold These bits are written by software to define the high threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), the higher 16 bits (AWHT\[23:8\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWHT\[7:0\] are not taken into comparison in this case.
    pub mod AWHT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

///
pub mod AWLTR0 {

    /// Break signal assignment to analog watchdog low threshold event BKAWL\[i\] = 0: Break i signal is not assigned to an analog watchdog low threshold event BKAWL\[i\] = 1: Break i signal is assigned to an analog watchdog low threshold event
    pub mod BKAWL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog watchdog low threshold These bits are written by software to define the low threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), only the higher 16 bits (AWLT\[23:8\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWLT\[7:0\] are not taken into comparison in this case.
    pub mod AWLT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

///
pub mod AWSR0 {

    /// Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    pub mod AWLTF {
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

    /// Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\] bit in the DFSDM_FLTxAWCFR register.
    pub mod AWHTF {
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
}

///
pub mod AWCFR0 {

    /// Clear the analog watchdog low threshold flag CLRAWLTF\[y\]=0: Writing '0â has no effect CLRAWLTF\[y\]=1: Writing '1â to position y clears the corresponding AWLTF\[y\] bit in the DFSDM_FLTxAWSR register
    pub mod CLRAWLTF {
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

    /// Clear the analog watchdog high threshold flag CLRAWHTF\[y\]=0: Writing '0â has no effect CLRAWHTF\[y\]=1: Writing '1â to position y clears the corresponding AWHTF\[y\] bit in the DFSDM_FLTxAWSR register
    pub mod CLRAWHTF {
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
}

///
pub mod EXMAX0 {

    /// Extremes detector maximum data channel. These bits contains information about the channel on which the data is stored into EXMAX\[23:0\]. Bits are cleared by reading of this register.
    pub mod EXMAXCH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Extremes detector maximum value These bits are set by hardware and indicate the highest value converted by DFSDM_FLTx. EXMAX\[23:0\] bits are reset to value (0x800000) by reading of this register.
    pub mod EXMAX {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

///
pub mod EXMIN0 {

    /// Extremes detector minimum data channel These bits contain information about the channel on which the data is stored into EXMIN\[23:0\]. Bits are cleared by reading of this register.
    pub mod EXMINCH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN\[23:0\] bits are reset to value (0x7FFFFF) by reading of this register.
    pub mod EXMIN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

///
pub mod FLTCNVTIMR0 {

    /// 28-bit timer counting conversion time t = CNVCNT\[27:0\] / fDFSDMCLK The timer has an input clock from DFSDM clock (system clock fDFSDMCLK). Conversion time measurement is started on each conversion start and stopped when conversion finishes (interval between first and last serial sample). Only in case of filter bypass (FOSR\[9:0\] = 0) is the conversion time measurement stopped and CNVCNT\[27:0\] = 0. The counted time is: if FAST=0 (or first conversion in continuous mode if FAST=1): t = \[FOSR * (IOSR-1 + FORD) + FORD\] / fCKIN ..... for Sincx filters t = \[FOSR * (IOSR-1 + 4) + 2\] / fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \[FOSR * IOSR\] / fCKIN in case if FOSR = FOSR\[9:0\]+1 = 1 (filter bypassed, active only integrator): CNVCNT = 0 (counting is stopped, conversion time: t = IOSR / fCKIN) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input (from internal ADC or from CPU/DMA write) Note: When conversion is interrupted (e.g. by disable/enable selected channel) the timer counts also this interruption time.
    pub mod CNVCNT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (28 bits: 0xfffffff << 4)
        pub const mask: u32 = 0xfffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

///
pub mod CR11 {
    pub use super::CR10::AWFSEL;
    pub use super::CR10::DFEN;
    pub use super::CR10::FAST;
    pub use super::CR10::JDMAEN;
    pub use super::CR10::JEXTEN;
    pub use super::CR10::JEXTSEL;
    pub use super::CR10::JSCAN;
    pub use super::CR10::JSWSTART;
    pub use super::CR10::JSYNC;
    pub use super::CR10::RCH;
    pub use super::CR10::RCONT;
    pub use super::CR10::RDMAEN;
    pub use super::CR10::RSWSTART;
    pub use super::CR10::RSYNC;
}

///
pub mod CR21 {
    pub use super::CR20::AWDCH;
    pub use super::CR20::AWDIE;
    pub use super::CR20::CKABIE;
    pub use super::CR20::EXCH;
    pub use super::CR20::JEOCIE;
    pub use super::CR20::JOVRIE;
    pub use super::CR20::REOCIE;
    pub use super::CR20::ROVRIE;
    pub use super::CR20::SCDIE;
}

///
pub mod ISR1 {
    pub use super::ISR0::AWDF;
    pub use super::ISR0::CKABF;
    pub use super::ISR0::JCIP;
    pub use super::ISR0::JEOCF;
    pub use super::ISR0::JOVRF;
    pub use super::ISR0::RCIP;
    pub use super::ISR0::REOCF;
    pub use super::ISR0::ROVRF;
    pub use super::ISR0::SCDF;
}

///
pub mod ICR1 {
    pub use super::ICR0::CLRCKABF;
    pub use super::ICR0::CLRJOVRF;
    pub use super::ICR0::CLRROVRF;
    pub use super::ICR0::CLRSCDF;
}

///
pub mod JCHGR1 {
    pub use super::JCHGR0::JCHG;
}

///
pub mod FCR1 {
    pub use super::FCR0::FORD;
    pub use super::FCR0::FOSR;
    pub use super::FCR0::IOSR;
}

///
pub mod JDATAR1 {
    pub use super::JDATAR0::JDATA;
    pub use super::JDATAR0::JDATACH;
}

///
pub mod RDATAR1 {
    pub use super::RDATAR0::RDATA;
    pub use super::RDATAR0::RDATACH;
    pub use super::RDATAR0::RPEND;
}

///
pub mod AWHTR1 {
    pub use super::AWHTR0::AWHT;
    pub use super::AWHTR0::BKAWH;
}

///
pub mod AWLTR1 {
    pub use super::AWLTR0::AWLT;
    pub use super::AWLTR0::BKAWL;
}

///
pub mod AWSR1 {
    pub use super::AWSR0::AWHTF;
    pub use super::AWSR0::AWLTF;
}

///
pub mod AWCFR1 {
    pub use super::AWCFR0::CLRAWHTF;
    pub use super::AWCFR0::CLRAWLTF;
}

///
pub mod EXMAX1 {
    pub use super::EXMAX0::EXMAX;
    pub use super::EXMAX0::EXMAXCH;
}

///
pub mod EXMIN1 {
    pub use super::EXMIN0::EXMIN;
    pub use super::EXMIN0::EXMINCH;
}

///
pub mod FLTCNVTIMR1 {
    pub use super::FLTCNVTIMR0::CNVCNT;
}

///
pub mod CR12 {
    pub use super::CR10::AWFSEL;
    pub use super::CR10::DFEN;
    pub use super::CR10::FAST;
    pub use super::CR10::JDMAEN;
    pub use super::CR10::JEXTEN;
    pub use super::CR10::JEXTSEL;
    pub use super::CR10::JSCAN;
    pub use super::CR10::JSWSTART;
    pub use super::CR10::JSYNC;
    pub use super::CR10::RCH;
    pub use super::CR10::RCONT;
    pub use super::CR10::RDMAEN;
    pub use super::CR10::RSWSTART;
    pub use super::CR10::RSYNC;
}

///
pub mod CR22 {
    pub use super::CR20::AWDCH;
    pub use super::CR20::AWDIE;
    pub use super::CR20::CKABIE;
    pub use super::CR20::EXCH;
    pub use super::CR20::JEOCIE;
    pub use super::CR20::JOVRIE;
    pub use super::CR20::REOCIE;
    pub use super::CR20::ROVRIE;
    pub use super::CR20::SCDIE;
}

///
pub mod ISR2 {
    pub use super::ISR0::AWDF;
    pub use super::ISR0::CKABF;
    pub use super::ISR0::JCIP;
    pub use super::ISR0::JEOCF;
    pub use super::ISR0::JOVRF;
    pub use super::ISR0::RCIP;
    pub use super::ISR0::REOCF;
    pub use super::ISR0::ROVRF;
    pub use super::ISR0::SCDF;
}

///
pub mod ICR2 {
    pub use super::ICR0::CLRCKABF;
    pub use super::ICR0::CLRJOVRF;
    pub use super::ICR0::CLRROVRF;
    pub use super::ICR0::CLRSCDF;
}

///
pub mod JCHGR2 {
    pub use super::JCHGR0::JCHG;
}

///
pub mod FCR2 {
    pub use super::FCR0::FORD;
    pub use super::FCR0::FOSR;
    pub use super::FCR0::IOSR;
}

///
pub mod JDATAR2 {
    pub use super::JDATAR0::JDATA;
    pub use super::JDATAR0::JDATACH;
}

///
pub mod RDATAR2 {
    pub use super::RDATAR0::RDATA;
    pub use super::RDATAR0::RDATACH;
    pub use super::RDATAR0::RPEND;
}

///
pub mod AWHTR2 {
    pub use super::AWHTR0::AWHT;
    pub use super::AWHTR0::BKAWH;
}

///
pub mod AWLTR2 {
    pub use super::AWLTR0::AWLT;
    pub use super::AWLTR0::BKAWL;
}

///
pub mod AWSR2 {
    pub use super::AWSR0::AWHTF;
    pub use super::AWSR0::AWLTF;
}

///
pub mod AWCFR2 {
    pub use super::AWCFR0::CLRAWHTF;
    pub use super::AWCFR0::CLRAWLTF;
}

///
pub mod EXMAX2 {
    pub use super::EXMAX0::EXMAX;
    pub use super::EXMAX0::EXMAXCH;
}

///
pub mod EXMIN2 {
    pub use super::EXMIN0::EXMIN;
    pub use super::EXMIN0::EXMINCH;
}

///
pub mod FLTCNVTIMR2 {
    pub use super::FLTCNVTIMR0::CNVCNT;
}

///
pub mod CR13 {
    pub use super::CR10::AWFSEL;
    pub use super::CR10::DFEN;
    pub use super::CR10::FAST;
    pub use super::CR10::JDMAEN;
    pub use super::CR10::JEXTEN;
    pub use super::CR10::JEXTSEL;
    pub use super::CR10::JSCAN;
    pub use super::CR10::JSWSTART;
    pub use super::CR10::JSYNC;
    pub use super::CR10::RCH;
    pub use super::CR10::RCONT;
    pub use super::CR10::RDMAEN;
    pub use super::CR10::RSWSTART;
    pub use super::CR10::RSYNC;
}

///
pub mod CR23 {
    pub use super::CR20::AWDCH;
    pub use super::CR20::AWDIE;
    pub use super::CR20::CKABIE;
    pub use super::CR20::EXCH;
    pub use super::CR20::JEOCIE;
    pub use super::CR20::JOVRIE;
    pub use super::CR20::REOCIE;
    pub use super::CR20::ROVRIE;
    pub use super::CR20::SCDIE;
}

///
pub mod ISR3 {
    pub use super::ISR0::AWDF;
    pub use super::ISR0::CKABF;
    pub use super::ISR0::JCIP;
    pub use super::ISR0::JEOCF;
    pub use super::ISR0::JOVRF;
    pub use super::ISR0::RCIP;
    pub use super::ISR0::REOCF;
    pub use super::ISR0::ROVRF;
    pub use super::ISR0::SCDF;
}

///
pub mod ICR3 {
    pub use super::ICR0::CLRCKABF;
    pub use super::ICR0::CLRJOVRF;
    pub use super::ICR0::CLRROVRF;
    pub use super::ICR0::CLRSCDF;
}

///
pub mod JCHGR3 {
    pub use super::JCHGR0::JCHG;
}

///
pub mod FCR3 {
    pub use super::FCR0::FORD;
    pub use super::FCR0::FOSR;
    pub use super::FCR0::IOSR;
}

///
pub mod JDATAR3 {
    pub use super::JDATAR0::JDATA;
    pub use super::JDATAR0::JDATACH;
}

///
pub mod RDATAR3 {
    pub use super::RDATAR0::RDATA;
    pub use super::RDATAR0::RDATACH;
    pub use super::RDATAR0::RPEND;
}

///
pub mod AWHTR3 {
    pub use super::AWHTR0::AWHT;
    pub use super::AWHTR0::BKAWH;
}

///
pub mod AWLTR3 {
    pub use super::AWLTR0::AWLT;
    pub use super::AWLTR0::BKAWL;
}

///
pub mod AWSR3 {
    pub use super::AWSR0::AWHTF;
    pub use super::AWSR0::AWLTF;
}

///
pub mod AWCFR3 {
    pub use super::AWCFR0::CLRAWHTF;
    pub use super::AWCFR0::CLRAWLTF;
}

///
pub mod EXMAX3 {
    pub use super::EXMAX0::EXMAX;
    pub use super::EXMAX0::EXMAXCH;
}

///
pub mod EXMIN3 {
    pub use super::EXMIN0::EXMIN;
    pub use super::EXMIN0::EXMINCH;
}

///
pub mod FLTCNVTIMR3 {
    pub use super::FLTCNVTIMR0::CNVCNT;
}

///
pub mod CR14 {
    pub use super::CR10::AWFSEL;
    pub use super::CR10::DFEN;
    pub use super::CR10::FAST;
    pub use super::CR10::JDMAEN;
    pub use super::CR10::JEXTEN;
    pub use super::CR10::JEXTSEL;
    pub use super::CR10::JSCAN;
    pub use super::CR10::JSWSTART;
    pub use super::CR10::JSYNC;
    pub use super::CR10::RCH;
    pub use super::CR10::RCONT;
    pub use super::CR10::RDMAEN;
    pub use super::CR10::RSWSTART;
    pub use super::CR10::RSYNC;
}

///
pub mod CR24 {
    pub use super::CR20::AWDCH;
    pub use super::CR20::AWDIE;
    pub use super::CR20::CKABIE;
    pub use super::CR20::EXCH;
    pub use super::CR20::JEOCIE;
    pub use super::CR20::JOVRIE;
    pub use super::CR20::REOCIE;
    pub use super::CR20::ROVRIE;
    pub use super::CR20::SCDIE;
}

///
pub mod ISR4 {
    pub use super::ISR0::AWDF;
    pub use super::ISR0::CKABF;
    pub use super::ISR0::JCIP;
    pub use super::ISR0::JEOCF;
    pub use super::ISR0::JOVRF;
    pub use super::ISR0::RCIP;
    pub use super::ISR0::REOCF;
    pub use super::ISR0::ROVRF;
    pub use super::ISR0::SCDF;
}

///
pub mod ICR4 {
    pub use super::ICR0::CLRCKABF;
    pub use super::ICR0::CLRJOVRF;
    pub use super::ICR0::CLRROVRF;
    pub use super::ICR0::CLRSCDF;
}

///
pub mod JCHGR4 {
    pub use super::JCHGR0::JCHG;
}

///
pub mod FCR4 {
    pub use super::FCR0::FORD;
    pub use super::FCR0::FOSR;
    pub use super::FCR0::IOSR;
}

///
pub mod JDATAR4 {
    pub use super::JDATAR0::JDATA;
    pub use super::JDATAR0::JDATACH;
}

///
pub mod RDATAR4 {
    pub use super::RDATAR0::RDATA;
    pub use super::RDATAR0::RDATACH;
    pub use super::RDATAR0::RPEND;
}

///
pub mod AWHTR4 {
    pub use super::AWHTR0::AWHT;
    pub use super::AWHTR0::BKAWH;
}

///
pub mod AWLTR4 {
    pub use super::AWLTR0::AWLT;
    pub use super::AWLTR0::BKAWL;
}

///
pub mod AWSR4 {
    pub use super::AWSR0::AWHTF;
    pub use super::AWSR0::AWLTF;
}

///
pub mod AWCFR4 {
    pub use super::AWCFR0::CLRAWHTF;
    pub use super::AWCFR0::CLRAWLTF;
}

///
pub mod EXMAX4 {
    pub use super::EXMAX0::EXMAX;
    pub use super::EXMAX0::EXMAXCH;
}

///
pub mod EXMIN4 {
    pub use super::EXMIN0::EXMIN;
    pub use super::EXMIN0::EXMINCH;
}

///
pub mod FLTCNVTIMR4 {
    pub use super::FLTCNVTIMR0::CNVCNT;
}

///
pub mod CR15 {
    pub use super::CR10::AWFSEL;
    pub use super::CR10::DFEN;
    pub use super::CR10::FAST;
    pub use super::CR10::JDMAEN;
    pub use super::CR10::JEXTEN;
    pub use super::CR10::JEXTSEL;
    pub use super::CR10::JSCAN;
    pub use super::CR10::JSWSTART;
    pub use super::CR10::JSYNC;
    pub use super::CR10::RCH;
    pub use super::CR10::RCONT;
    pub use super::CR10::RDMAEN;
    pub use super::CR10::RSWSTART;
    pub use super::CR10::RSYNC;
}

///
pub mod CR25 {
    pub use super::CR20::AWDCH;
    pub use super::CR20::AWDIE;
    pub use super::CR20::CKABIE;
    pub use super::CR20::EXCH;
    pub use super::CR20::JEOCIE;
    pub use super::CR20::JOVRIE;
    pub use super::CR20::REOCIE;
    pub use super::CR20::ROVRIE;
    pub use super::CR20::SCDIE;
}

///
pub mod ISR5 {
    pub use super::ISR0::AWDF;
    pub use super::ISR0::CKABF;
    pub use super::ISR0::JCIP;
    pub use super::ISR0::JEOCF;
    pub use super::ISR0::JOVRF;
    pub use super::ISR0::RCIP;
    pub use super::ISR0::REOCF;
    pub use super::ISR0::ROVRF;
    pub use super::ISR0::SCDF;
}

///
pub mod ICR5 {
    pub use super::ICR0::CLRCKABF;
    pub use super::ICR0::CLRJOVRF;
    pub use super::ICR0::CLRROVRF;
    pub use super::ICR0::CLRSCDF;
}

///
pub mod JCHGR5 {
    pub use super::JCHGR0::JCHG;
}

///
pub mod FCR5 {
    pub use super::FCR0::FORD;
    pub use super::FCR0::FOSR;
    pub use super::FCR0::IOSR;
}

///
pub mod JDATAR5 {
    pub use super::JDATAR0::JDATA;
    pub use super::JDATAR0::JDATACH;
}

///
pub mod RDATAR5 {
    pub use super::RDATAR0::RDATA;
    pub use super::RDATAR0::RDATACH;
    pub use super::RDATAR0::RPEND;
}

///
pub mod AWHTR5 {
    pub use super::AWHTR0::AWHT;
    pub use super::AWHTR0::BKAWH;
}

///
pub mod AWLTR5 {
    pub use super::AWLTR0::AWLT;
    pub use super::AWLTR0::BKAWL;
}

///
pub mod AWSR5 {
    pub use super::AWSR0::AWHTF;
    pub use super::AWSR0::AWLTF;
}

///
pub mod AWCFR5 {
    pub use super::AWCFR0::CLRAWHTF;
    pub use super::AWCFR0::CLRAWLTF;
}

///
pub mod EXMAX5 {
    pub use super::EXMAX0::EXMAX;
    pub use super::EXMAX0::EXMAXCH;
}

///
pub mod EXMIN5 {
    pub use super::EXMIN0::EXMIN;
    pub use super::EXMIN0::EXMINCH;
}

///
pub mod FLTCNVTIMR5 {
    pub use super::FLTCNVTIMR0::CNVCNT;
}

///
pub mod CR16 {
    pub use super::CR10::AWFSEL;
    pub use super::CR10::DFEN;
    pub use super::CR10::FAST;
    pub use super::CR10::JDMAEN;
    pub use super::CR10::JEXTEN;
    pub use super::CR10::JEXTSEL;
    pub use super::CR10::JSCAN;
    pub use super::CR10::JSWSTART;
    pub use super::CR10::JSYNC;
    pub use super::CR10::RCH;
    pub use super::CR10::RCONT;
    pub use super::CR10::RDMAEN;
    pub use super::CR10::RSWSTART;
    pub use super::CR10::RSYNC;
}

///
pub mod CR26 {
    pub use super::CR20::AWDCH;
    pub use super::CR20::AWDIE;
    pub use super::CR20::CKABIE;
    pub use super::CR20::EXCH;
    pub use super::CR20::JEOCIE;
    pub use super::CR20::JOVRIE;
    pub use super::CR20::REOCIE;
    pub use super::CR20::ROVRIE;
    pub use super::CR20::SCDIE;
}

///
pub mod ISR6 {
    pub use super::ISR0::AWDF;
    pub use super::ISR0::CKABF;
    pub use super::ISR0::JCIP;
    pub use super::ISR0::JEOCF;
    pub use super::ISR0::JOVRF;
    pub use super::ISR0::RCIP;
    pub use super::ISR0::REOCF;
    pub use super::ISR0::ROVRF;
    pub use super::ISR0::SCDF;
}

///
pub mod ICR6 {
    pub use super::ICR0::CLRCKABF;
    pub use super::ICR0::CLRJOVRF;
    pub use super::ICR0::CLRROVRF;
    pub use super::ICR0::CLRSCDF;
}

///
pub mod JCHGR6 {
    pub use super::JCHGR0::JCHG;
}

///
pub mod FCR6 {
    pub use super::FCR0::FORD;
    pub use super::FCR0::FOSR;
    pub use super::FCR0::IOSR;
}

///
pub mod JDATAR6 {
    pub use super::JDATAR0::JDATA;
    pub use super::JDATAR0::JDATACH;
}

///
pub mod RDATAR6 {
    pub use super::RDATAR0::RDATA;
    pub use super::RDATAR0::RDATACH;
    pub use super::RDATAR0::RPEND;
}

///
pub mod AWHTR6 {
    pub use super::AWHTR0::AWHT;
    pub use super::AWHTR0::BKAWH;
}

///
pub mod AWLTR6 {
    pub use super::AWLTR0::AWLT;
    pub use super::AWLTR0::BKAWL;
}

///
pub mod AWSR6 {
    pub use super::AWSR0::AWHTF;
    pub use super::AWSR0::AWLTF;
}

///
pub mod AWCFR6 {
    pub use super::AWCFR0::CLRAWHTF;
    pub use super::AWCFR0::CLRAWLTF;
}

///
pub mod EXMAX6 {
    pub use super::EXMAX0::EXMAX;
    pub use super::EXMAX0::EXMAXCH;
}

///
pub mod EXMIN6 {
    pub use super::EXMIN0::EXMIN;
    pub use super::EXMIN0::EXMINCH;
}

///
pub mod FLTCNVTIMR6 {
    pub use super::FLTCNVTIMR0::CNVCNT;
}

///
pub mod CR17 {
    pub use super::CR10::AWFSEL;
    pub use super::CR10::DFEN;
    pub use super::CR10::FAST;
    pub use super::CR10::JDMAEN;
    pub use super::CR10::JEXTEN;
    pub use super::CR10::JEXTSEL;
    pub use super::CR10::JSCAN;
    pub use super::CR10::JSWSTART;
    pub use super::CR10::JSYNC;
    pub use super::CR10::RCH;
    pub use super::CR10::RCONT;
    pub use super::CR10::RDMAEN;
    pub use super::CR10::RSWSTART;
    pub use super::CR10::RSYNC;
}

///
pub mod CR27 {
    pub use super::CR20::AWDCH;
    pub use super::CR20::AWDIE;
    pub use super::CR20::CKABIE;
    pub use super::CR20::EXCH;
    pub use super::CR20::JEOCIE;
    pub use super::CR20::JOVRIE;
    pub use super::CR20::REOCIE;
    pub use super::CR20::ROVRIE;
    pub use super::CR20::SCDIE;
}

///
pub mod ISR7 {
    pub use super::ISR0::AWDF;
    pub use super::ISR0::CKABF;
    pub use super::ISR0::JCIP;
    pub use super::ISR0::JEOCF;
    pub use super::ISR0::JOVRF;
    pub use super::ISR0::RCIP;
    pub use super::ISR0::REOCF;
    pub use super::ISR0::ROVRF;
    pub use super::ISR0::SCDF;
}

///
pub mod ICR7 {
    pub use super::ICR0::CLRCKABF;
    pub use super::ICR0::CLRJOVRF;
    pub use super::ICR0::CLRROVRF;
    pub use super::ICR0::CLRSCDF;
}

///
pub mod JCHGR7 {
    pub use super::JCHGR0::JCHG;
}

///
pub mod FCR7 {
    pub use super::FCR0::FORD;
    pub use super::FCR0::FOSR;
    pub use super::FCR0::IOSR;
}

///
pub mod JDATAR7 {
    pub use super::JDATAR0::JDATA;
    pub use super::JDATAR0::JDATACH;
}

///
pub mod RDATAR7 {
    pub use super::RDATAR0::RDATA;
    pub use super::RDATAR0::RDATACH;
    pub use super::RDATAR0::RPEND;
}

///
pub mod AWHTR7 {
    pub use super::AWHTR0::AWHT;
    pub use super::AWHTR0::BKAWH;
}

///
pub mod AWLTR7 {
    pub use super::AWLTR0::AWLT;
    pub use super::AWLTR0::BKAWL;
}

///
pub mod AWSR7 {
    pub use super::AWSR0::AWHTF;
    pub use super::AWSR0::AWLTF;
}

///
pub mod AWCFR7 {
    pub use super::AWCFR0::CLRAWHTF;
    pub use super::AWCFR0::CLRAWLTF;
}

///
pub mod EXMAX7 {
    pub use super::EXMAX0::EXMAX;
    pub use super::EXMAX0::EXMAXCH;
}

///
pub mod EXMIN7 {
    pub use super::EXMIN0::EXMIN;
    pub use super::EXMIN0::EXMINCH;
}

///
pub mod FLTCNVTIMR7 {
    pub use super::FLTCNVTIMR0::CNVCNT;
}
#[repr(C)]
pub struct RegisterBlock {
    /// DFSDM channel 0 configuration register
    pub CFGR10: RWRegister<u32>,

    /// DFSDM channel 0 configuration register
    pub CFGR20: RWRegister<u32>,

    /// DFSDM channel 0 analog watchdog and short-circuit detector register
    pub AWSCDR0: RWRegister<u32>,

    /// DFSDM channel 0 watchdog filter data register
    pub WDATR0: RORegister<u32>,

    /// DFSDM channel 0 data input register
    pub DATINR0: RWRegister<u32>,

    ///
    pub DLYR0: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// DFSDM channel 0 configuration register
    pub CFGR11: RWRegister<u32>,

    /// DFSDM channel 0 configuration register
    pub CFGR21: RWRegister<u32>,

    /// DFSDM channel 0 analog watchdog and short-circuit detector register
    pub AWSCDR1: RWRegister<u32>,

    /// DFSDM channel 0 watchdog filter data register
    pub WDATR1: RORegister<u32>,

    /// DFSDM channel 0 data input register
    pub DATINR1: RWRegister<u32>,

    ///
    pub DLYR1: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// DFSDM channel 0 configuration register
    pub CFGR12: RWRegister<u32>,

    /// DFSDM channel 0 configuration register
    pub CFGR22: RWRegister<u32>,

    /// DFSDM channel 0 analog watchdog and short-circuit detector register
    pub AWSCDR2: RWRegister<u32>,

    /// DFSDM channel 0 watchdog filter data register
    pub WDATR2: RORegister<u32>,

    /// DFSDM channel 0 data input register
    pub DATINR2: RWRegister<u32>,

    ///
    pub DLYR2: RWRegister<u32>,

    _reserved3: [u32; 2],

    /// DFSDM channel 0 configuration register
    pub CFGR13: RWRegister<u32>,

    /// DFSDM channel 0 configuration register
    pub CFGR23: RWRegister<u32>,

    /// DFSDM channel 0 analog watchdog and short-circuit detector register
    pub AWSCDR3: RWRegister<u32>,

    /// DFSDM channel 0 watchdog filter data register
    pub WDATR3: RORegister<u32>,

    /// DFSDM channel 0 data input register
    pub DATINR3: RWRegister<u32>,

    ///
    pub DLYR3: RWRegister<u32>,

    _reserved4: [u32; 2],

    /// DFSDM channel 0 configuration register
    pub CFGR14: RWRegister<u32>,

    /// DFSDM channel 0 configuration register
    pub CFGR24: RWRegister<u32>,

    /// DFSDM channel 0 analog watchdog and short-circuit detector register
    pub AWSCDR4: RWRegister<u32>,

    /// DFSDM channel 0 watchdog filter data register
    pub WDATR4: RORegister<u32>,

    /// DFSDM channel 0 data input register
    pub DATINR4: RWRegister<u32>,

    ///
    pub DLYR4: RWRegister<u32>,

    _reserved5: [u32; 2],

    /// DFSDM channel 0 configuration register
    pub CFGR15: RWRegister<u32>,

    /// DFSDM channel 0 configuration register
    pub CFGR25: RWRegister<u32>,

    /// DFSDM channel 0 analog watchdog and short-circuit detector register
    pub AWSCDR5: RWRegister<u32>,

    /// DFSDM channel 0 watchdog filter data register
    pub WDATR5: RORegister<u32>,

    /// DFSDM channel 0 data input register
    pub DATINR5: RWRegister<u32>,

    ///
    pub DLYR5: RWRegister<u32>,

    _reserved6: [u32; 2],

    /// DFSDM channel 0 configuration register
    pub CFGR16: RWRegister<u32>,

    /// DFSDM channel 0 configuration register
    pub CFGR26: RWRegister<u32>,

    /// DFSDM channel 0 analog watchdog and short-circuit detector register
    pub AWSCDR6: RWRegister<u32>,

    /// DFSDM channel 0 watchdog filter data register
    pub WDATR6: RORegister<u32>,

    /// DFSDM channel 0 data input register
    pub DATINR6: RWRegister<u32>,

    ///
    pub DLYR6: RWRegister<u32>,

    _reserved7: [u32; 2],

    /// DFSDM channel 0 configuration register
    pub CFGR17: RWRegister<u32>,

    /// DFSDM channel 0 configuration register
    pub CFGR27: RWRegister<u32>,

    /// DFSDM channel 0 analog watchdog and short-circuit detector register
    pub AWSCDR7: RWRegister<u32>,

    /// DFSDM channel 0 watchdog filter data register
    pub WDATR7: RORegister<u32>,

    /// DFSDM channel 0 data input register
    pub DATINR7: RWRegister<u32>,

    ///
    pub DLYR7: RWRegister<u32>,

    _reserved8: [u32; 2],

    ///
    pub CR10: RWRegister<u32>,

    ///
    pub CR20: RWRegister<u32>,

    ///
    pub ISR0: RORegister<u32>,

    ///
    pub ICR0: RWRegister<u32>,

    ///
    pub JCHGR0: RWRegister<u32>,

    ///
    pub FCR0: RWRegister<u32>,

    ///
    pub JDATAR0: RORegister<u32>,

    ///
    pub RDATAR0: RORegister<u32>,

    ///
    pub AWHTR0: RWRegister<u32>,

    ///
    pub AWLTR0: RWRegister<u32>,

    ///
    pub AWSR0: RORegister<u32>,

    ///
    pub AWCFR0: RWRegister<u32>,

    ///
    pub EXMAX0: RORegister<u32>,

    ///
    pub EXMIN0: RWRegister<u32>,

    ///
    pub FLTCNVTIMR0: RORegister<u32>,

    _reserved9: [u32; 17],

    ///
    pub CR11: RWRegister<u32>,

    ///
    pub CR21: RWRegister<u32>,

    ///
    pub ISR1: RORegister<u32>,

    ///
    pub ICR1: RWRegister<u32>,

    ///
    pub JCHGR1: RWRegister<u32>,

    ///
    pub FCR1: RWRegister<u32>,

    ///
    pub JDATAR1: RORegister<u32>,

    ///
    pub RDATAR1: RORegister<u32>,

    ///
    pub AWHTR1: RWRegister<u32>,

    ///
    pub AWLTR1: RWRegister<u32>,

    ///
    pub AWSR1: RORegister<u32>,

    ///
    pub AWCFR1: RWRegister<u32>,

    ///
    pub EXMAX1: RORegister<u32>,

    ///
    pub EXMIN1: RWRegister<u32>,

    ///
    pub FLTCNVTIMR1: RORegister<u32>,

    _reserved10: [u32; 17],

    ///
    pub CR12: RWRegister<u32>,

    ///
    pub CR22: RWRegister<u32>,

    ///
    pub ISR2: RORegister<u32>,

    ///
    pub ICR2: RWRegister<u32>,

    ///
    pub JCHGR2: RWRegister<u32>,

    ///
    pub FCR2: RWRegister<u32>,

    ///
    pub JDATAR2: RORegister<u32>,

    ///
    pub RDATAR2: RORegister<u32>,

    ///
    pub AWHTR2: RWRegister<u32>,

    ///
    pub AWLTR2: RWRegister<u32>,

    ///
    pub AWSR2: RORegister<u32>,

    ///
    pub AWCFR2: RWRegister<u32>,

    ///
    pub EXMAX2: RORegister<u32>,

    ///
    pub EXMIN2: RWRegister<u32>,

    ///
    pub FLTCNVTIMR2: RORegister<u32>,

    _reserved11: [u32; 17],

    ///
    pub CR13: RWRegister<u32>,

    ///
    pub CR23: RWRegister<u32>,

    ///
    pub ISR3: RORegister<u32>,

    ///
    pub ICR3: RWRegister<u32>,

    ///
    pub JCHGR3: RWRegister<u32>,

    ///
    pub FCR3: RWRegister<u32>,

    ///
    pub JDATAR3: RORegister<u32>,

    ///
    pub RDATAR3: RORegister<u32>,

    ///
    pub AWHTR3: RWRegister<u32>,

    ///
    pub AWLTR3: RWRegister<u32>,

    ///
    pub AWSR3: RORegister<u32>,

    ///
    pub AWCFR3: RWRegister<u32>,

    ///
    pub EXMAX3: RORegister<u32>,

    ///
    pub EXMIN3: RWRegister<u32>,

    ///
    pub FLTCNVTIMR3: RORegister<u32>,

    _reserved12: [u32; 17],

    ///
    pub CR14: RWRegister<u32>,

    ///
    pub CR24: RWRegister<u32>,

    ///
    pub ISR4: RORegister<u32>,

    ///
    pub ICR4: RWRegister<u32>,

    ///
    pub JCHGR4: RWRegister<u32>,

    ///
    pub FCR4: RWRegister<u32>,

    ///
    pub JDATAR4: RORegister<u32>,

    ///
    pub RDATAR4: RORegister<u32>,

    ///
    pub AWHTR4: RWRegister<u32>,

    ///
    pub AWLTR4: RWRegister<u32>,

    ///
    pub AWSR4: RORegister<u32>,

    ///
    pub AWCFR4: RWRegister<u32>,

    ///
    pub EXMAX4: RORegister<u32>,

    ///
    pub EXMIN4: RWRegister<u32>,

    ///
    pub FLTCNVTIMR4: RORegister<u32>,

    _reserved13: [u32; 17],

    ///
    pub CR15: RWRegister<u32>,

    ///
    pub CR25: RWRegister<u32>,

    ///
    pub ISR5: RORegister<u32>,

    ///
    pub ICR5: RWRegister<u32>,

    ///
    pub JCHGR5: RWRegister<u32>,

    ///
    pub FCR5: RWRegister<u32>,

    ///
    pub JDATAR5: RORegister<u32>,

    ///
    pub RDATAR5: RORegister<u32>,

    ///
    pub AWHTR5: RWRegister<u32>,

    ///
    pub AWLTR5: RWRegister<u32>,

    ///
    pub AWSR5: RORegister<u32>,

    ///
    pub AWCFR5: RWRegister<u32>,

    ///
    pub EXMAX5: RORegister<u32>,

    ///
    pub EXMIN5: RWRegister<u32>,

    ///
    pub FLTCNVTIMR5: RORegister<u32>,

    _reserved14: [u32; 17],

    ///
    pub CR16: RWRegister<u32>,

    ///
    pub CR26: RWRegister<u32>,

    ///
    pub ISR6: RORegister<u32>,

    ///
    pub ICR6: RWRegister<u32>,

    ///
    pub JCHGR6: RWRegister<u32>,

    ///
    pub FCR6: RWRegister<u32>,

    ///
    pub JDATAR6: RORegister<u32>,

    ///
    pub RDATAR6: RORegister<u32>,

    ///
    pub AWHTR6: RWRegister<u32>,

    ///
    pub AWLTR6: RWRegister<u32>,

    ///
    pub AWSR6: RORegister<u32>,

    ///
    pub AWCFR6: RWRegister<u32>,

    ///
    pub EXMAX6: RORegister<u32>,

    ///
    pub EXMIN6: RWRegister<u32>,

    ///
    pub FLTCNVTIMR6: RORegister<u32>,

    _reserved15: [u32; 17],

    ///
    pub CR17: RWRegister<u32>,

    ///
    pub CR27: RWRegister<u32>,

    ///
    pub ISR7: RORegister<u32>,

    ///
    pub ICR7: RWRegister<u32>,

    ///
    pub JCHGR7: RWRegister<u32>,

    ///
    pub FCR7: RWRegister<u32>,

    ///
    pub JDATAR7: RORegister<u32>,

    ///
    pub RDATAR7: RORegister<u32>,

    ///
    pub AWHTR7: RWRegister<u32>,

    ///
    pub AWLTR7: RWRegister<u32>,

    ///
    pub AWSR7: RORegister<u32>,

    ///
    pub AWCFR7: RWRegister<u32>,

    ///
    pub EXMAX7: RORegister<u32>,

    ///
    pub EXMIN7: RWRegister<u32>,

    ///
    pub FLTCNVTIMR7: RORegister<u32>,
}
pub struct ResetValues {
    pub CFGR10: u32,
    pub CFGR20: u32,
    pub AWSCDR0: u32,
    pub WDATR0: u32,
    pub DATINR0: u32,
    pub DLYR0: u32,
    pub CFGR11: u32,
    pub CFGR21: u32,
    pub AWSCDR1: u32,
    pub WDATR1: u32,
    pub DATINR1: u32,
    pub DLYR1: u32,
    pub CFGR12: u32,
    pub CFGR22: u32,
    pub AWSCDR2: u32,
    pub WDATR2: u32,
    pub DATINR2: u32,
    pub DLYR2: u32,
    pub CFGR13: u32,
    pub CFGR23: u32,
    pub AWSCDR3: u32,
    pub WDATR3: u32,
    pub DATINR3: u32,
    pub DLYR3: u32,
    pub CFGR14: u32,
    pub CFGR24: u32,
    pub AWSCDR4: u32,
    pub WDATR4: u32,
    pub DATINR4: u32,
    pub DLYR4: u32,
    pub CFGR15: u32,
    pub CFGR25: u32,
    pub AWSCDR5: u32,
    pub WDATR5: u32,
    pub DATINR5: u32,
    pub DLYR5: u32,
    pub CFGR16: u32,
    pub CFGR26: u32,
    pub AWSCDR6: u32,
    pub WDATR6: u32,
    pub DATINR6: u32,
    pub DLYR6: u32,
    pub CFGR17: u32,
    pub CFGR27: u32,
    pub AWSCDR7: u32,
    pub WDATR7: u32,
    pub DATINR7: u32,
    pub DLYR7: u32,
    pub CR10: u32,
    pub CR20: u32,
    pub ISR0: u32,
    pub ICR0: u32,
    pub JCHGR0: u32,
    pub FCR0: u32,
    pub JDATAR0: u32,
    pub RDATAR0: u32,
    pub AWHTR0: u32,
    pub AWLTR0: u32,
    pub AWSR0: u32,
    pub AWCFR0: u32,
    pub EXMAX0: u32,
    pub EXMIN0: u32,
    pub FLTCNVTIMR0: u32,
    pub CR11: u32,
    pub CR21: u32,
    pub ISR1: u32,
    pub ICR1: u32,
    pub JCHGR1: u32,
    pub FCR1: u32,
    pub JDATAR1: u32,
    pub RDATAR1: u32,
    pub AWHTR1: u32,
    pub AWLTR1: u32,
    pub AWSR1: u32,
    pub AWCFR1: u32,
    pub EXMAX1: u32,
    pub EXMIN1: u32,
    pub FLTCNVTIMR1: u32,
    pub CR12: u32,
    pub CR22: u32,
    pub ISR2: u32,
    pub ICR2: u32,
    pub JCHGR2: u32,
    pub FCR2: u32,
    pub JDATAR2: u32,
    pub RDATAR2: u32,
    pub AWHTR2: u32,
    pub AWLTR2: u32,
    pub AWSR2: u32,
    pub AWCFR2: u32,
    pub EXMAX2: u32,
    pub EXMIN2: u32,
    pub FLTCNVTIMR2: u32,
    pub CR13: u32,
    pub CR23: u32,
    pub ISR3: u32,
    pub ICR3: u32,
    pub JCHGR3: u32,
    pub FCR3: u32,
    pub JDATAR3: u32,
    pub RDATAR3: u32,
    pub AWHTR3: u32,
    pub AWLTR3: u32,
    pub AWSR3: u32,
    pub AWCFR3: u32,
    pub EXMAX3: u32,
    pub EXMIN3: u32,
    pub FLTCNVTIMR3: u32,
    pub CR14: u32,
    pub CR24: u32,
    pub ISR4: u32,
    pub ICR4: u32,
    pub JCHGR4: u32,
    pub FCR4: u32,
    pub JDATAR4: u32,
    pub RDATAR4: u32,
    pub AWHTR4: u32,
    pub AWLTR4: u32,
    pub AWSR4: u32,
    pub AWCFR4: u32,
    pub EXMAX4: u32,
    pub EXMIN4: u32,
    pub FLTCNVTIMR4: u32,
    pub CR15: u32,
    pub CR25: u32,
    pub ISR5: u32,
    pub ICR5: u32,
    pub JCHGR5: u32,
    pub FCR5: u32,
    pub JDATAR5: u32,
    pub RDATAR5: u32,
    pub AWHTR5: u32,
    pub AWLTR5: u32,
    pub AWSR5: u32,
    pub AWCFR5: u32,
    pub EXMAX5: u32,
    pub EXMIN5: u32,
    pub FLTCNVTIMR5: u32,
    pub CR16: u32,
    pub CR26: u32,
    pub ISR6: u32,
    pub ICR6: u32,
    pub JCHGR6: u32,
    pub FCR6: u32,
    pub JDATAR6: u32,
    pub RDATAR6: u32,
    pub AWHTR6: u32,
    pub AWLTR6: u32,
    pub AWSR6: u32,
    pub AWCFR6: u32,
    pub EXMAX6: u32,
    pub EXMIN6: u32,
    pub FLTCNVTIMR6: u32,
    pub CR17: u32,
    pub CR27: u32,
    pub ISR7: u32,
    pub ICR7: u32,
    pub JCHGR7: u32,
    pub FCR7: u32,
    pub JDATAR7: u32,
    pub RDATAR7: u32,
    pub AWHTR7: u32,
    pub AWLTR7: u32,
    pub AWSR7: u32,
    pub AWCFR7: u32,
    pub EXMAX7: u32,
    pub EXMIN7: u32,
    pub FLTCNVTIMR7: u32,
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

/// Access functions for the DFSDM1 peripheral instance
pub mod DFSDM1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40017800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DFSDM1
    pub const reset: ResetValues = ResetValues {
        CFGR10: 0x00000000,
        CFGR20: 0x00000000,
        AWSCDR0: 0x00000000,
        WDATR0: 0x00000000,
        DATINR0: 0x00000000,
        DLYR0: 0x00000000,
        CFGR11: 0x00000000,
        CFGR21: 0x00000000,
        AWSCDR1: 0x00000000,
        WDATR1: 0x00000000,
        DATINR1: 0x00000000,
        DLYR1: 0x00000000,
        CFGR12: 0x00000000,
        CFGR22: 0x00000000,
        AWSCDR2: 0x00000000,
        WDATR2: 0x00000000,
        DATINR2: 0x00000000,
        DLYR2: 0x00000000,
        CFGR13: 0x00000000,
        CFGR23: 0x00000000,
        AWSCDR3: 0x00000000,
        WDATR3: 0x00000000,
        DATINR3: 0x00000000,
        DLYR3: 0x00000000,
        CFGR14: 0x00000000,
        CFGR24: 0x00000000,
        AWSCDR4: 0x00000000,
        WDATR4: 0x00000000,
        DATINR4: 0x00000000,
        DLYR4: 0x00000000,
        CFGR15: 0x00000000,
        CFGR25: 0x00000000,
        AWSCDR5: 0x00000000,
        WDATR5: 0x00000000,
        DATINR5: 0x00000000,
        DLYR5: 0x00000000,
        CFGR16: 0x00000000,
        CFGR26: 0x00000000,
        AWSCDR6: 0x00000000,
        WDATR6: 0x00000000,
        DATINR6: 0x00000000,
        DLYR6: 0x00000000,
        CFGR17: 0x00000000,
        CFGR27: 0x00000000,
        AWSCDR7: 0x00000000,
        WDATR7: 0x00000000,
        DATINR7: 0x00000000,
        DLYR7: 0x00000000,
        CR10: 0x00000000,
        CR20: 0x00000000,
        ISR0: 0x00FF0000,
        ICR0: 0x00000000,
        JCHGR0: 0x00000001,
        FCR0: 0x00000000,
        JDATAR0: 0x00000000,
        RDATAR0: 0x00000000,
        AWHTR0: 0x00000000,
        AWLTR0: 0x00000000,
        AWSR0: 0x00000000,
        AWCFR0: 0x00000000,
        EXMAX0: 0x80000000,
        EXMIN0: 0x7FFFFF00,
        FLTCNVTIMR0: 0x00000000,
        CR11: 0x00000000,
        CR21: 0x00000000,
        ISR1: 0x00FF0000,
        ICR1: 0x00000000,
        JCHGR1: 0x00000001,
        FCR1: 0x00000000,
        JDATAR1: 0x00000000,
        RDATAR1: 0x00000000,
        AWHTR1: 0x00000000,
        AWLTR1: 0x00000000,
        AWSR1: 0x00000000,
        AWCFR1: 0x00000000,
        EXMAX1: 0x80000000,
        EXMIN1: 0x7FFFFF00,
        FLTCNVTIMR1: 0x00000000,
        CR12: 0x00000000,
        CR22: 0x00000000,
        ISR2: 0x00FF0000,
        ICR2: 0x00000000,
        JCHGR2: 0x00000001,
        FCR2: 0x00000000,
        JDATAR2: 0x00000000,
        RDATAR2: 0x00000000,
        AWHTR2: 0x00000000,
        AWLTR2: 0x00000000,
        AWSR2: 0x00000000,
        AWCFR2: 0x00000000,
        EXMAX2: 0x80000000,
        EXMIN2: 0x7FFFFF00,
        FLTCNVTIMR2: 0x00000000,
        CR13: 0x00000000,
        CR23: 0x00000000,
        ISR3: 0x00FF0000,
        ICR3: 0x00000000,
        JCHGR3: 0x00000001,
        FCR3: 0x00000000,
        JDATAR3: 0x00000000,
        RDATAR3: 0x00000000,
        AWHTR3: 0x00000000,
        AWLTR3: 0x00000000,
        AWSR3: 0x00000000,
        AWCFR3: 0x00000000,
        EXMAX3: 0x80000000,
        EXMIN3: 0x7FFFFF00,
        FLTCNVTIMR3: 0x00000000,
        CR14: 0x00000000,
        CR24: 0x00000000,
        ISR4: 0x00FF0000,
        ICR4: 0x00000000,
        JCHGR4: 0x00000001,
        FCR4: 0x00000000,
        JDATAR4: 0x00000000,
        RDATAR4: 0x00000000,
        AWHTR4: 0x00000000,
        AWLTR4: 0x00000000,
        AWSR4: 0x00000000,
        AWCFR4: 0x00000000,
        EXMAX4: 0x80000000,
        EXMIN4: 0x7FFFFF00,
        FLTCNVTIMR4: 0x00000000,
        CR15: 0x00000000,
        CR25: 0x00000000,
        ISR5: 0x00FF0000,
        ICR5: 0x00000000,
        JCHGR5: 0x00000001,
        FCR5: 0x00000000,
        JDATAR5: 0x00000000,
        RDATAR5: 0x00000000,
        AWHTR5: 0x00000000,
        AWLTR5: 0x00000000,
        AWSR5: 0x00000000,
        AWCFR5: 0x00000000,
        EXMAX5: 0x80000000,
        EXMIN5: 0x7FFFFF00,
        FLTCNVTIMR5: 0x00000000,
        CR16: 0x00000000,
        CR26: 0x00000000,
        ISR6: 0x00FF0000,
        ICR6: 0x00000000,
        JCHGR6: 0x00000001,
        FCR6: 0x00000000,
        JDATAR6: 0x00000000,
        RDATAR6: 0x00000000,
        AWHTR6: 0x00000000,
        AWLTR6: 0x00000000,
        AWSR6: 0x00000000,
        AWCFR6: 0x00000000,
        EXMAX6: 0x80000000,
        EXMIN6: 0x7FFFFF00,
        FLTCNVTIMR6: 0x00000000,
        CR17: 0x00000000,
        CR27: 0x00000000,
        ISR7: 0x00FF0000,
        ICR7: 0x00000000,
        JCHGR7: 0x00000001,
        FCR7: 0x00000000,
        JDATAR7: 0x00000000,
        RDATAR7: 0x00000000,
        AWHTR7: 0x00000000,
        AWLTR7: 0x00000000,
        AWSR7: 0x00000000,
        AWCFR7: 0x00000000,
        EXMAX7: 0x80000000,
        EXMIN7: 0x7FFFFF00,
        FLTCNVTIMR7: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DFSDM1_TAKEN: bool = false;

    /// Safe access to DFSDM1
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DFSDM1_TAKEN {
                None
            } else {
                DFSDM1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DFSDM1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DFSDM1_TAKEN && inst.addr == INSTANCE.addr {
                DFSDM1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DFSDM1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DFSDM1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DFSDM1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DFSDM1: *const RegisterBlock = 0x40017800 as *const _;

/// Access functions for the DFSDM2 peripheral instance
pub mod DFSDM2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58006c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DFSDM2
    pub const reset: ResetValues = ResetValues {
        CFGR10: 0x00000000,
        CFGR20: 0x00000000,
        AWSCDR0: 0x00000000,
        WDATR0: 0x00000000,
        DATINR0: 0x00000000,
        DLYR0: 0x00000000,
        CFGR11: 0x00000000,
        CFGR21: 0x00000000,
        AWSCDR1: 0x00000000,
        WDATR1: 0x00000000,
        DATINR1: 0x00000000,
        DLYR1: 0x00000000,
        CFGR12: 0x00000000,
        CFGR22: 0x00000000,
        AWSCDR2: 0x00000000,
        WDATR2: 0x00000000,
        DATINR2: 0x00000000,
        DLYR2: 0x00000000,
        CFGR13: 0x00000000,
        CFGR23: 0x00000000,
        AWSCDR3: 0x00000000,
        WDATR3: 0x00000000,
        DATINR3: 0x00000000,
        DLYR3: 0x00000000,
        CFGR14: 0x00000000,
        CFGR24: 0x00000000,
        AWSCDR4: 0x00000000,
        WDATR4: 0x00000000,
        DATINR4: 0x00000000,
        DLYR4: 0x00000000,
        CFGR15: 0x00000000,
        CFGR25: 0x00000000,
        AWSCDR5: 0x00000000,
        WDATR5: 0x00000000,
        DATINR5: 0x00000000,
        DLYR5: 0x00000000,
        CFGR16: 0x00000000,
        CFGR26: 0x00000000,
        AWSCDR6: 0x00000000,
        WDATR6: 0x00000000,
        DATINR6: 0x00000000,
        DLYR6: 0x00000000,
        CFGR17: 0x00000000,
        CFGR27: 0x00000000,
        AWSCDR7: 0x00000000,
        WDATR7: 0x00000000,
        DATINR7: 0x00000000,
        DLYR7: 0x00000000,
        CR10: 0x00000000,
        CR20: 0x00000000,
        ISR0: 0x00FF0000,
        ICR0: 0x00000000,
        JCHGR0: 0x00000001,
        FCR0: 0x00000000,
        JDATAR0: 0x00000000,
        RDATAR0: 0x00000000,
        AWHTR0: 0x00000000,
        AWLTR0: 0x00000000,
        AWSR0: 0x00000000,
        AWCFR0: 0x00000000,
        EXMAX0: 0x80000000,
        EXMIN0: 0x7FFFFF00,
        FLTCNVTIMR0: 0x00000000,
        CR11: 0x00000000,
        CR21: 0x00000000,
        ISR1: 0x00FF0000,
        ICR1: 0x00000000,
        JCHGR1: 0x00000001,
        FCR1: 0x00000000,
        JDATAR1: 0x00000000,
        RDATAR1: 0x00000000,
        AWHTR1: 0x00000000,
        AWLTR1: 0x00000000,
        AWSR1: 0x00000000,
        AWCFR1: 0x00000000,
        EXMAX1: 0x80000000,
        EXMIN1: 0x7FFFFF00,
        FLTCNVTIMR1: 0x00000000,
        CR12: 0x00000000,
        CR22: 0x00000000,
        ISR2: 0x00FF0000,
        ICR2: 0x00000000,
        JCHGR2: 0x00000001,
        FCR2: 0x00000000,
        JDATAR2: 0x00000000,
        RDATAR2: 0x00000000,
        AWHTR2: 0x00000000,
        AWLTR2: 0x00000000,
        AWSR2: 0x00000000,
        AWCFR2: 0x00000000,
        EXMAX2: 0x80000000,
        EXMIN2: 0x7FFFFF00,
        FLTCNVTIMR2: 0x00000000,
        CR13: 0x00000000,
        CR23: 0x00000000,
        ISR3: 0x00FF0000,
        ICR3: 0x00000000,
        JCHGR3: 0x00000001,
        FCR3: 0x00000000,
        JDATAR3: 0x00000000,
        RDATAR3: 0x00000000,
        AWHTR3: 0x00000000,
        AWLTR3: 0x00000000,
        AWSR3: 0x00000000,
        AWCFR3: 0x00000000,
        EXMAX3: 0x80000000,
        EXMIN3: 0x7FFFFF00,
        FLTCNVTIMR3: 0x00000000,
        CR14: 0x00000000,
        CR24: 0x00000000,
        ISR4: 0x00FF0000,
        ICR4: 0x00000000,
        JCHGR4: 0x00000001,
        FCR4: 0x00000000,
        JDATAR4: 0x00000000,
        RDATAR4: 0x00000000,
        AWHTR4: 0x00000000,
        AWLTR4: 0x00000000,
        AWSR4: 0x00000000,
        AWCFR4: 0x00000000,
        EXMAX4: 0x80000000,
        EXMIN4: 0x7FFFFF00,
        FLTCNVTIMR4: 0x00000000,
        CR15: 0x00000000,
        CR25: 0x00000000,
        ISR5: 0x00FF0000,
        ICR5: 0x00000000,
        JCHGR5: 0x00000001,
        FCR5: 0x00000000,
        JDATAR5: 0x00000000,
        RDATAR5: 0x00000000,
        AWHTR5: 0x00000000,
        AWLTR5: 0x00000000,
        AWSR5: 0x00000000,
        AWCFR5: 0x00000000,
        EXMAX5: 0x80000000,
        EXMIN5: 0x7FFFFF00,
        FLTCNVTIMR5: 0x00000000,
        CR16: 0x00000000,
        CR26: 0x00000000,
        ISR6: 0x00FF0000,
        ICR6: 0x00000000,
        JCHGR6: 0x00000001,
        FCR6: 0x00000000,
        JDATAR6: 0x00000000,
        RDATAR6: 0x00000000,
        AWHTR6: 0x00000000,
        AWLTR6: 0x00000000,
        AWSR6: 0x00000000,
        AWCFR6: 0x00000000,
        EXMAX6: 0x80000000,
        EXMIN6: 0x7FFFFF00,
        FLTCNVTIMR6: 0x00000000,
        CR17: 0x00000000,
        CR27: 0x00000000,
        ISR7: 0x00FF0000,
        ICR7: 0x00000000,
        JCHGR7: 0x00000001,
        FCR7: 0x00000000,
        JDATAR7: 0x00000000,
        RDATAR7: 0x00000000,
        AWHTR7: 0x00000000,
        AWLTR7: 0x00000000,
        AWSR7: 0x00000000,
        AWCFR7: 0x00000000,
        EXMAX7: 0x80000000,
        EXMIN7: 0x7FFFFF00,
        FLTCNVTIMR7: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DFSDM2_TAKEN: bool = false;

    /// Safe access to DFSDM2
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DFSDM2_TAKEN {
                None
            } else {
                DFSDM2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DFSDM2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DFSDM2_TAKEN && inst.addr == INSTANCE.addr {
                DFSDM2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DFSDM2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DFSDM2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DFSDM2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DFSDM2: *const RegisterBlock = 0x58006c00 as *const _;
