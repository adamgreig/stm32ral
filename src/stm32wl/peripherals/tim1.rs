#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Advanced-control timers
//!
//! Used by: stm32wl5x_cm0p, stm32wl5x_cm4, stm32wle5

use crate::{RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register 1
pub mod CR1 {

    /// UIF status bit remapping
    pub mod UIFREMAP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No remapping. UIF status bit is not copied to TIMx_CNT register bit 31
            pub const Disabled: u32 = 0b0;

            /// 0b1: Remapping enabled. UIF status bit is copied to TIMx_CNT register bit 31
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Clock division
    pub mod CKD {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: t_DTS = t_CK_INT
            pub const Div1: u32 = 0b00;

            /// 0b01: t_DTS = 2 × t_CK_INT
            pub const Div2: u32 = 0b01;

            /// 0b10: t_DTS = 4 × t_CK_INT
            pub const Div4: u32 = 0b10;
        }
    }

    /// Auto-reload preload enable
    pub mod ARPE {
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

            /// 0b0: TIMx_APRR register is not buffered
            pub const Disabled: u32 = 0b0;

            /// 0b1: TIMx_APRR register is buffered
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Center-aligned mode selection
    pub mod CMS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: The counter counts up or down depending on the direction bit
            pub const EdgeAligned: u32 = 0b00;

            /// 0b01: The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down.
            pub const CenterAligned1: u32 = 0b01;

            /// 0b10: The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up.
            pub const CenterAligned2: u32 = 0b10;

            /// 0b11: The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down.
            pub const CenterAligned3: u32 = 0b11;
        }
    }

    /// Direction
    pub mod DIR {
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

            /// 0b0: Counter used as upcounter
            pub const Up: u32 = 0b0;

            /// 0b1: Counter used as downcounter
            pub const Down: u32 = 0b1;
        }
    }

    /// One pulse mode
    pub mod OPM {
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

            /// 0b0: Counter is not stopped at update event
            pub const Disabled: u32 = 0b0;

            /// 0b1: Counter stops counting at the next update event (clearing the CEN bit)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Update request source
    pub mod URS {
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

            /// 0b0: Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request
            pub const AnyEvent: u32 = 0b0;

            /// 0b1: Only counter overflow/underflow generates an update interrupt or DMA request
            pub const CounterOnly: u32 = 0b1;
        }
    }

    /// Update disable
    pub mod UDIS {
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

            /// 0b0: Update event enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: Update event disabled
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Counter enable
    pub mod CEN {
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

            /// 0b0: Counter disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Counter enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// control register 2
pub mod CR2 {

    /// Master mode selection 2
    pub mod MMS2 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on TRGO2 is delayed compared to the actual reset
            pub const Reset: u32 = 0b0000;

            /// 0b0001: Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register)
            pub const Enable: u32 = 0b0001;

            /// 0b0010: Update - the update event is selected as trigger output (TRGO2). For instance, a master timer can then be used as a prescaler for a slave timer
            pub const Update: u32 = 0b0010;

            /// 0b0011: Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (TRGO2)
            pub const ComparePulse: u32 = 0b0011;

            /// 0b0100: Compare - OC1REFC signal is used as trigger output (TRGO2)
            pub const CompareOC1: u32 = 0b0100;

            /// 0b0101: Compare - OC2REFC signal is used as trigger output (TRGO2)
            pub const CompareOC2: u32 = 0b0101;

            /// 0b0110: Compare - OC3REFC signal is used as trigger output (TRGO2)
            pub const CompareOC3: u32 = 0b0110;

            /// 0b0111: Compare - OC4REFC signal is used as trigger output (TRGO2)
            pub const CompareOC4: u32 = 0b0111;

            /// 0b1000: Compare - OC5REFC signal is used as trigger output (TRGO2)
            pub const CompareOC5: u32 = 0b1000;

            /// 0b1001: Compare - OC6REFC signal is used as trigger output (TRGO2)
            pub const CompareOC6: u32 = 0b1001;

            /// 0b1010: Compare Pulse - OC4REFC rising or falling edges generate pulses on TRGO2
            pub const PulseOC4: u32 = 0b1010;

            /// 0b1011: Compare Pulse - OC6REFC rising or falling edges generate pulses on TRGO2
            pub const PulseOC6: u32 = 0b1011;

            /// 0b1100: Compare Pulse - OC4REFC or OC6REFC rising edges generate pulses on TRGO2
            pub const RisingOC4_6: u32 = 0b1100;

            /// 0b1101: Compare Pulse - OC4REFC rising or OC6REFC falling edges generate pulses on TRGO2
            pub const RisingOC4_FallingOC6: u32 = 0b1101;

            /// 0b1110: Compare Pulse - OC5REFC or OC6REFC rising edges generate pulses on TRGO2
            pub const RisingOC5_6: u32 = 0b1110;

            /// 0b1111: Compare Pulse - OC5REFC rising or OC6REFC falling edges generate pulses on TRGO2
            pub const RisingOC5_FallingOC6: u32 = 0b1111;
        }
    }

    /// Output Idle state 6 (OC6 output)
    pub mod OIS6 {
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

            /// 0b0: OCx=0 (after a dead-time if OCx(N) is implemented) when MOE=0
            pub const Disabled: u32 = 0b0;

            /// 0b1: OCx=1 (after a dead-time if OCx(N) is implemented) when MOE=0
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Output Idle state 5 (OC5 output)
    pub mod OIS5 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OIS6::RW;
    }

    /// Output Idle state 4 (OC4 output)
    pub mod OIS4 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OIS6::RW;
    }

    /// Output Idle state 3 (OC3N output)
    pub mod OIS3N {
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

            /// 0b0: OCxN=0 after a dead-time when MOE=0
            pub const Disabled: u32 = 0b0;

            /// 0b1: OCxN=1 after a dead-time when MOE=0
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Output Idle state 3 (OC3 output)
    pub mod OIS3 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OIS6::RW;
    }

    /// Output Idle state 2 (OC2N output)
    pub mod OIS2N {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OIS3N::RW;
    }

    /// Output Idle state 2 (OC2 output)
    pub mod OIS2 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OIS6::RW;
    }

    /// Output Idle state 1 (OC1N output)
    pub mod OIS1N {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OIS3N::RW;
    }

    /// Output Idle state 1 (OC1 output)
    pub mod OIS1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OIS6::RW;
    }

    /// TI1 selection
    pub mod TI1S {
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

            /// 0b0: The TIMx_CH1 pin is connected to TI1 input
            pub const Normal: u32 = 0b0;

            /// 0b1: The TIMx_CH1, CH2, CH3 pins are connected to TI1 input
            pub const XOR: u32 = 0b1;
        }
    }

    /// Master mode selection
    pub mod MMS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: The UG bit from the TIMx_EGR register is used as trigger output
            pub const Reset: u32 = 0b000;

            /// 0b001: The counter enable signal, CNT_EN, is used as trigger output
            pub const Enable: u32 = 0b001;

            /// 0b010: The update event is selected as trigger output
            pub const Update: u32 = 0b010;

            /// 0b011: The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred
            pub const ComparePulse: u32 = 0b011;

            /// 0b100: OC1REF signal is used as trigger output
            pub const CompareOC1: u32 = 0b100;

            /// 0b101: OC2REF signal is used as trigger output
            pub const CompareOC2: u32 = 0b101;

            /// 0b110: OC3REF signal is used as trigger output
            pub const CompareOC3: u32 = 0b110;

            /// 0b111: OC4REF signal is used as trigger output
            pub const CompareOC4: u32 = 0b111;
        }
    }

    /// Capture/compare DMA selection
    pub mod CCDS {
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

            /// 0b0: CCx DMA request sent when CCx event occurs
            pub const OnCompare: u32 = 0b0;

            /// 0b1: CCx DMA request sent when update event occurs
            pub const OnUpdate: u32 = 0b1;
        }
    }

    /// Capture/compare control update selection
    pub mod CCUS {
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

            /// 0b0: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only
            pub const Bit: u32 = 0b0;

            /// 0b1: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI
            pub const BitOrEdge: u32 = 0b1;
        }
    }

    /// Capture/compare preloaded control
    pub mod CCPC {
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

            /// 0b0: CCxE, CCxNE and OCxM bits are not preloaded
            pub const NotPreloaded: u32 = 0b0;

            /// 0b1: CCxE, CCxNE and OCxM bits are preloaded
            pub const Preloaded: u32 = 0b1;
        }
    }
}

/// slave mode control register
pub mod SMCR {

    /// Trigger selection
    pub mod TS3_4 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Slave mode selection
    pub mod SMS_3 {
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

            /// 0b0: Slave mode disabled (see SMS\[0:2\])
            pub const Disabled: u32 = 0b0;

            /// 0b1: SMS\[0:2\] must be 0b000 (DisabledOrCombined). Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter
            pub const CombinedResetTrigger: u32 = 0b1;
        }
    }

    /// External trigger polarity
    pub mod ETP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ETR is noninverted, active at high level or rising edge
            pub const NotInverted: u32 = 0b0;

            /// 0b1: ETR is inverted, active at low level or falling edge
            pub const Inverted: u32 = 0b1;
        }
    }

    /// External clock enable
    pub mod ECE {
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

            /// 0b0: External clock mode 2 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal
            pub const Enabled: u32 = 0b1;
        }
    }

    /// External trigger prescaler
    pub mod ETPS {
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

            /// 0b00: Prescaler OFF
            pub const Div1: u32 = 0b00;

            /// 0b01: ETRP frequency divided by 2
            pub const Div2: u32 = 0b01;

            /// 0b10: ETRP frequency divided by 4
            pub const Div4: u32 = 0b10;

            /// 0b11: ETRP frequency divided by 8
            pub const Div8: u32 = 0b11;
        }
    }

    /// External trigger filter
    pub mod ETF {
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

            /// 0b0000: No filter, sampling is done at fDTS
            pub const NoFilter: u32 = 0b0000;

            /// 0b0001: fSAMPLING=fCK_INT, N=2
            pub const FCK_INT_N2: u32 = 0b0001;

            /// 0b0010: fSAMPLING=fCK_INT, N=4
            pub const FCK_INT_N4: u32 = 0b0010;

            /// 0b0011: fSAMPLING=fCK_INT, N=8
            pub const FCK_INT_N8: u32 = 0b0011;

            /// 0b0100: fSAMPLING=fDTS/2, N=6
            pub const FDTS_Div2_N6: u32 = 0b0100;

            /// 0b0101: fSAMPLING=fDTS/2, N=8
            pub const FDTS_Div2_N8: u32 = 0b0101;

            /// 0b0110: fSAMPLING=fDTS/4, N=6
            pub const FDTS_Div4_N6: u32 = 0b0110;

            /// 0b0111: fSAMPLING=fDTS/4, N=8
            pub const FDTS_Div4_N8: u32 = 0b0111;

            /// 0b1000: fSAMPLING=fDTS/8, N=6
            pub const FDTS_Div8_N6: u32 = 0b1000;

            /// 0b1001: fSAMPLING=fDTS/8, N=8
            pub const FDTS_Div8_N8: u32 = 0b1001;

            /// 0b1010: fSAMPLING=fDTS/16, N=5
            pub const FDTS_Div16_N5: u32 = 0b1010;

            /// 0b1011: fSAMPLING=fDTS/16, N=6
            pub const FDTS_Div16_N6: u32 = 0b1011;

            /// 0b1100: fSAMPLING=fDTS/16, N=8
            pub const FDTS_Div16_N8: u32 = 0b1100;

            /// 0b1101: fSAMPLING=fDTS/32, N=5
            pub const FDTS_Div32_N5: u32 = 0b1101;

            /// 0b1110: fSAMPLING=fDTS/32, N=6
            pub const FDTS_Div32_N6: u32 = 0b1110;

            /// 0b1111: fSAMPLING=fDTS/32, N=8
            pub const FDTS_Div32_N8: u32 = 0b1111;
        }
    }

    /// Master/slave mode
    pub mod MSM {
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

            /// 0b0: No action
            pub const NoSync: u32 = 0b0;

            /// 0b1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event
            pub const Sync: u32 = 0b1;
        }
    }

    /// Trigger selection
    pub mod TS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Internal Trigger 0 (ITR0)
            pub const ITR0: u32 = 0b000;

            /// 0b001: Internal Trigger 1 (ITR1)
            pub const ITR1: u32 = 0b001;

            /// 0b010: Internal Trigger 2 (ITR2)
            pub const ITR2: u32 = 0b010;

            /// 0b100: TI1 Edge Detector (TI1F_ED)
            pub const TI1F_ED: u32 = 0b100;

            /// 0b101: Filtered Timer Input 1 (TI1FP1)
            pub const TI1FP1: u32 = 0b101;

            /// 0b110: Filtered Timer Input 2 (TI2FP2)
            pub const TI2FP2: u32 = 0b110;

            /// 0b111: External Trigger input (ETRF)
            pub const ETRF: u32 = 0b111;
        }
    }

    /// OCREF clear selection
    pub mod OCCS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Slave mode selection
    pub mod SMS {
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

            /// 0b000: Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock. If SMS\[3\]=1 then Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter
            pub const DisabledOrCombined: u32 = 0b000;

            /// 0b001: Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level
            pub const EncoderMode1: u32 = 0b001;

            /// 0b010: Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level
            pub const EncoderMode2: u32 = 0b010;

            /// 0b011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input
            pub const EncoderMode3: u32 = 0b011;

            /// 0b100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers
            pub const ResetMode: u32 = 0b100;

            /// 0b101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled
            pub const GatedMode: u32 = 0b101;

            /// 0b110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled
            pub const TriggerMode: u32 = 0b110;

            /// 0b111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter
            pub const ExtClockMode: u32 = 0b111;
        }
    }
}

/// DMA/interrupt enable register
pub mod DIER {

    /// Trigger DMA request enable
    pub mod TDE {
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

            /// 0b0: Trigger DMA request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Trigger DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// COM DMA request enable
    pub mod COMDE {
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

            /// 0b0: COM DMA request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: COM DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Capture/Compare 4 DMA request enable
    pub mod CC4DE {
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

            /// 0b0: CCx DMA request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CCx DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Capture/Compare 3 DMA request enable
    pub mod CC3DE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4DE::RW;
    }

    /// Capture/Compare 2 DMA request enable
    pub mod CC2DE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4DE::RW;
    }

    /// Capture/Compare 1 DMA request enable
    pub mod CC1DE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4DE::RW;
    }

    /// Update DMA request enable
    pub mod UDE {
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

            /// 0b0: Update DMA request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Update DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Break interrupt enable
    pub mod BIE {
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

            /// 0b0: Break interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Break interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Trigger interrupt enable
    pub mod TIE {
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

            /// 0b0: Trigger interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Trigger interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// COM interrupt enable
    pub mod COMIE {
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

            /// 0b0: COM interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: COM interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Capture/Compare 4 interrupt enable
    pub mod CC4IE {
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

            /// 0b0: CCx interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CCx interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Capture/Compare 3 interrupt enable
    pub mod CC3IE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4IE::RW;
    }

    /// Capture/Compare 2 interrupt enable
    pub mod CC2IE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4IE::RW;
    }

    /// Capture/Compare 1 interrupt enable
    pub mod CC1IE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4IE::RW;
    }

    /// Update interrupt enable
    pub mod UIE {
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

            /// 0b0: Update interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Update interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// status register
pub mod SR {

    /// Compare 6 interrupt flag
    pub mod CC6IF {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No campture/compare has been detected
            pub const NoMatch: u32 = 0b0;

            /// 0b1: If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register
            pub const Match: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: Clear flag
            pub const Clear: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Compare 5 interrupt flag
    pub mod CC5IF {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        pub use super::CC6IF::R;
        pub use super::CC6IF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// System Break interrupt flag
    pub mod SBIF {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No break event occurred
            pub const NoTrigger: u32 = 0b0;

            /// 0b1: An active level has been detected on the system break input. An interrupt is generated if BIE=1 in the TIMx_DIER register
            pub const Trigger: u32 = 0b1;
        }
        pub use super::CC6IF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 4 overcapture flag
    pub mod CC4OF {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No overcapture has been detected
            pub const NoOvercapture: u32 = 0b0;

            /// 0b1: The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set
            pub const Overcapture: u32 = 0b1;
        }
        pub use super::CC6IF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 3 overcapture flag
    pub mod CC3OF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No overcapture has been detected
            pub const NoOvercapture: u32 = 0b0;

            /// 0b1: The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set
            pub const Overcapture: u32 = 0b1;
        }
        pub use super::CC6IF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 2 overcapture flag
    pub mod CC2OF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No overcapture has been detected
            pub const NoOvercapture: u32 = 0b0;

            /// 0b1: The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set
            pub const Overcapture: u32 = 0b1;
        }
        pub use super::CC6IF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 1 overcapture flag
    pub mod CC1OF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No overcapture has been detected
            pub const NoOvercapture: u32 = 0b0;

            /// 0b1: The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set
            pub const Overcapture: u32 = 0b1;
        }
        pub use super::CC6IF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Break 2 interrupt flag
    pub mod B2IF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No break event occurred
            pub const NoTrigger: u32 = 0b0;

            /// 0b1: An active level has been detected on the break 2 input. An interrupt is generated if BIE=1 in the TIMx_DIER register
            pub const Trigger: u32 = 0b1;
        }
        pub use super::CC6IF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Break interrupt flag
    pub mod BIF {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No break event occurred
            pub const NoTrigger: u32 = 0b0;

            /// 0b1: An active level has been detected on the break input. An interrupt is generated if BIE=1 in the TIMx_DIER register
            pub const Trigger: u32 = 0b1;
        }
        pub use super::CC6IF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Trigger interrupt flag
    pub mod TIF {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No trigger event occurred
            pub const NoTrigger: u32 = 0b0;

            /// 0b1: Trigger interrupt pending
            pub const Trigger: u32 = 0b1;
        }
        pub use super::CC6IF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// COM interrupt flag
    pub mod COMIF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 4 interrupt flag
    pub mod CC4IF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::CC6IF::R;
        pub use super::CC6IF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 3 interrupt flag
    pub mod CC3IF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::CC6IF::R;
        pub use super::CC6IF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 2 interrupt flag
    pub mod CC2IF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::CC6IF::R;
        pub use super::CC6IF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 1 interrupt flag
    pub mod CC1IF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::CC6IF::R;
        pub use super::CC6IF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update interrupt flag
    pub mod UIF {
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

            /// 0b0: No update occurred
            pub const Clear: u32 = 0b0;

            /// 0b1: Update interrupt pending.
            pub const UpdatePending: u32 = 0b1;
        }
    }
}

/// event generation register
pub mod EGR {

    /// Break 2 generation
    pub mod B2G {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled
            pub const Trigger: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Break generation
    pub mod BG {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled
            pub const Trigger: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Trigger generation
    pub mod TG {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled
            pub const Trigger: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare control update generation
    pub mod COMG {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated
            pub const Trigger: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 4 generation
    pub mod CC4G {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register
            pub const Trigger: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 3 generation
    pub mod CC3G {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CC4G::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 2 generation
    pub mod CC2G {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CC4G::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 1 generation
    pub mod CC1G {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CC4G::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update generation
    pub mod UG {
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

            /// 0b1: Re-initializes the timer counter and generates an update of the registers.
            pub const Update: u32 = 0b1;
        }
    }
}

/// CCMR1_Output and CCMR1_Input
/// CCMR1_Output: capture/compare mode register 1 (output mode)
/// CCMR1_Input: capture/compare mode register 1 (input mode)
pub mod CCMR1 {

    /// Output Compare 2 mode - bit 3
    pub mod OC2M_3 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal output compare mode (modes 0-7)
            pub const Normal: u32 = 0b0;

            /// 0b1: Extended output compare mode (modes 7-15)
            pub const Extended: u32 = 0b1;
        }
    }

    /// Output Compare 1 mode - bit 3
    pub mod OC1M_3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC2M_3::RW;
    }

    /// Output compare 2 clear enable
    pub mod OC2CE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OCxRef is not affected by the ocref_clr_int signal
            pub const Disabled: u32 = 0b0;

            /// 0b1: OCxRef is cleared as soon as a High level is detected on ocref_clr_int signal
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Output compare 2 mode
    pub mod OC2M {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
            pub const Frozen: u32 = 0b000;

            /// 0b001: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
            pub const ActiveOnMatch: u32 = 0b001;

            /// 0b010: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
            pub const InactiveOnMatch: u32 = 0b010;

            /// 0b011: OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
            pub const Toggle: u32 = 0b011;

            /// 0b100: OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
            pub const ForceInactive: u32 = 0b100;

            /// 0b101: OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
            pub const ForceActive: u32 = 0b101;

            /// 0b110: In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
            pub const PwmMode1: u32 = 0b110;

            /// 0b111: Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
            pub const PwmMode2: u32 = 0b111;
        }
    }

    /// Output compare 2 preload enable
    pub mod OC2PE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
            pub const Disabled: u32 = 0b0;

            /// 0b1: Preload register on CCRx enabled. Preload value is loaded into active register on each update event
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Output compare 2 fast enable
    pub mod OC2FE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Fast output disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Fast output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Capture/Compare 2 selection
    pub mod CC2S {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: CCx channel is configured as output
            pub const Output: u32 = 0b00;

            /// 0b01: CCx channel is configured as input, ICx is mapped on TI1
            pub const TI1: u32 = 0b01;

            /// 0b10: CCx channel is configured as input, ICx is mapped on TI2
            pub const TI2: u32 = 0b10;

            /// 0b11: CCx channel is configured as input, ICx is mapped on TRC
            pub const TRC: u32 = 0b11;
        }
    }

    /// Output compare 1 clear enable
    pub mod OC1CE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC2CE::RW;
    }

    /// Output compare 1 mode
    pub mod OC1M {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC2M::RW;
    }

    /// Output compare 1 preload enable
    pub mod OC1PE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC2PE::RW;
    }

    /// Output compare 1 fast enable
    pub mod OC1FE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC2FE::RW;
    }

    /// Capture/Compare 1 selection
    pub mod CC1S {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC2S::RW;
    }

    /// Input capture 2 filter
    pub mod IC2F {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No filter, sampling is done at fDTS
            pub const NoFilter: u32 = 0b0000;

            /// 0b0001: fSAMPLING=fCK_INT, N=2
            pub const FCK_INT_N2: u32 = 0b0001;

            /// 0b0010: fSAMPLING=fCK_INT, N=4
            pub const FCK_INT_N4: u32 = 0b0010;

            /// 0b0011: fSAMPLING=fCK_INT, N=8
            pub const FCK_INT_N8: u32 = 0b0011;

            /// 0b0100: fSAMPLING=fDTS/2, N=6
            pub const FDTS_Div2_N6: u32 = 0b0100;

            /// 0b0101: fSAMPLING=fDTS/2, N=8
            pub const FDTS_Div2_N8: u32 = 0b0101;

            /// 0b0110: fSAMPLING=fDTS/4, N=6
            pub const FDTS_Div4_N6: u32 = 0b0110;

            /// 0b0111: fSAMPLING=fDTS/4, N=8
            pub const FDTS_Div4_N8: u32 = 0b0111;

            /// 0b1000: fSAMPLING=fDTS/8, N=6
            pub const FDTS_Div8_N6: u32 = 0b1000;

            /// 0b1001: fSAMPLING=fDTS/8, N=8
            pub const FDTS_Div8_N8: u32 = 0b1001;

            /// 0b1010: fSAMPLING=fDTS/16, N=5
            pub const FDTS_Div16_N5: u32 = 0b1010;

            /// 0b1011: fSAMPLING=fDTS/16, N=6
            pub const FDTS_Div16_N6: u32 = 0b1011;

            /// 0b1100: fSAMPLING=fDTS/16, N=8
            pub const FDTS_Div16_N8: u32 = 0b1100;

            /// 0b1101: fSAMPLING=fDTS/32, N=5
            pub const FDTS_Div32_N5: u32 = 0b1101;

            /// 0b1110: fSAMPLING=fDTS/32, N=6
            pub const FDTS_Div32_N6: u32 = 0b1110;

            /// 0b1111: fSAMPLING=fDTS/32, N=8
            pub const FDTS_Div32_N8: u32 = 0b1111;
        }
    }

    /// Input capture 2 prescaler
    pub mod IC2PSC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: CCx channel is configured as output
            pub const Output: u32 = 0b00;

            /// 0b01: Capture is done once every 2 events
            pub const Capture2: u32 = 0b01;

            /// 0b10: Capture is done once every 4 events
            pub const Capture4: u32 = 0b10;

            /// 0b11: Capture is done once every 8 events
            pub const Capture8: u32 = 0b11;
        }
    }

    /// Input capture 1 filter
    pub mod IC1F {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IC2F::RW;
    }

    /// Input capture 1 prescaler
    pub mod IC1PSC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IC2PSC::RW;
    }
}

/// CCMR2_Output and CCMR2_Input
/// CCMR2_Output: capture/compare mode register 2 (output mode)
/// CCMR2_Input: capture/compare mode register 2 (input mode)
pub mod CCMR2 {

    /// Output Compare 4 mode - bit 3
    pub mod OC4M_3 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal output compare mode (modes 0-7)
            pub const Normal: u32 = 0b0;

            /// 0b1: Extended output compare mode (modes 7-15)
            pub const Extended: u32 = 0b1;
        }
    }

    /// Output Compare 3 mode - bit 3
    pub mod OC3M_3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC4M_3::RW;
    }

    /// Output compare 4 clear enable
    pub mod OC4CE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OCxRef is not affected by the ocref_clr_int signal
            pub const Disabled: u32 = 0b0;

            /// 0b1: OCxRef is cleared as soon as a High level is detected on ocref_clr_int signal
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Output compare 4 mode
    pub mod OC4M {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
            pub const Frozen: u32 = 0b000;

            /// 0b001: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
            pub const ActiveOnMatch: u32 = 0b001;

            /// 0b010: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
            pub const InactiveOnMatch: u32 = 0b010;

            /// 0b011: OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
            pub const Toggle: u32 = 0b011;

            /// 0b100: OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
            pub const ForceInactive: u32 = 0b100;

            /// 0b101: OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
            pub const ForceActive: u32 = 0b101;

            /// 0b110: In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
            pub const PwmMode1: u32 = 0b110;

            /// 0b111: Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
            pub const PwmMode2: u32 = 0b111;
        }
    }

    /// Output compare 4 preload enable
    pub mod OC4PE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
            pub const Disabled: u32 = 0b0;

            /// 0b1: Preload register on CCRx enabled. Preload value is loaded into active register on each update event
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Output compare 4 fast enable
    pub mod OC4FE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Fast output disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Fast output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Capture/Compare 4 selection
    pub mod CC4S {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: CCx channel is configured as output
            pub const Output: u32 = 0b00;

            /// 0b01: CCx channel is configured as input, ICx is mapped on TI1
            pub const TI1: u32 = 0b01;

            /// 0b10: CCx channel is configured as input, ICx is mapped on TI2
            pub const TI2: u32 = 0b10;

            /// 0b11: CCx channel is configured as input, ICx is mapped on TRC
            pub const TRC: u32 = 0b11;
        }
    }

    /// Output compare 3 clear enable
    pub mod OC3CE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC4CE::RW;
    }

    /// Output compare 3 mode
    pub mod OC3M {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC4M::RW;
    }

    /// Output compare 3 preload enable
    pub mod OC3PE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC4PE::RW;
    }

    /// Output compare 3 fast enable
    pub mod OC3FE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC4FE::RW;
    }

    /// Capture/Compare 3 selection
    pub mod CC3S {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4S::RW;
    }

    /// Input capture 4 filter
    pub mod IC4F {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No filter, sampling is done at fDTS
            pub const NoFilter: u32 = 0b0000;

            /// 0b0001: fSAMPLING=fCK_INT, N=2
            pub const FCK_INT_N2: u32 = 0b0001;

            /// 0b0010: fSAMPLING=fCK_INT, N=4
            pub const FCK_INT_N4: u32 = 0b0010;

            /// 0b0011: fSAMPLING=fCK_INT, N=8
            pub const FCK_INT_N8: u32 = 0b0011;

            /// 0b0100: fSAMPLING=fDTS/2, N=6
            pub const FDTS_Div2_N6: u32 = 0b0100;

            /// 0b0101: fSAMPLING=fDTS/2, N=8
            pub const FDTS_Div2_N8: u32 = 0b0101;

            /// 0b0110: fSAMPLING=fDTS/4, N=6
            pub const FDTS_Div4_N6: u32 = 0b0110;

            /// 0b0111: fSAMPLING=fDTS/4, N=8
            pub const FDTS_Div4_N8: u32 = 0b0111;

            /// 0b1000: fSAMPLING=fDTS/8, N=6
            pub const FDTS_Div8_N6: u32 = 0b1000;

            /// 0b1001: fSAMPLING=fDTS/8, N=8
            pub const FDTS_Div8_N8: u32 = 0b1001;

            /// 0b1010: fSAMPLING=fDTS/16, N=5
            pub const FDTS_Div16_N5: u32 = 0b1010;

            /// 0b1011: fSAMPLING=fDTS/16, N=6
            pub const FDTS_Div16_N6: u32 = 0b1011;

            /// 0b1100: fSAMPLING=fDTS/16, N=8
            pub const FDTS_Div16_N8: u32 = 0b1100;

            /// 0b1101: fSAMPLING=fDTS/32, N=5
            pub const FDTS_Div32_N5: u32 = 0b1101;

            /// 0b1110: fSAMPLING=fDTS/32, N=6
            pub const FDTS_Div32_N6: u32 = 0b1110;

            /// 0b1111: fSAMPLING=fDTS/32, N=8
            pub const FDTS_Div32_N8: u32 = 0b1111;
        }
    }

    /// Input capture 4 prescaler
    pub mod IC4PSC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: CCx channel is configured as output
            pub const Output: u32 = 0b00;

            /// 0b01: Capture is done once every 2 events
            pub const Capture2: u32 = 0b01;

            /// 0b10: Capture is done once every 4 events
            pub const Capture4: u32 = 0b10;

            /// 0b11: Capture is done once every 8 events
            pub const Capture8: u32 = 0b11;
        }
    }

    /// Input capture 3 filter
    pub mod IC3F {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IC4F::RW;
    }

    /// Input capture 3 prescaler
    pub mod IC3PSC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IC4PSC::RW;
    }
}

/// capture/compare enable register
pub mod CCER {

    /// CC6P
    pub mod CC6P {
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

            /// 0b0: Noninverted/rising edge
            pub const RisingEdge: u32 = 0b0;

            /// 0b1: Inverted/falling edge
            pub const FallingEdge: u32 = 0b1;
        }
    }

    /// CC6E
    pub mod CC6E {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Capture disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Capture enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CC5P
    pub mod CC5P {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC6P::RW;
    }

    /// CC5E
    pub mod CC5E {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC6E::RW;
    }

    /// CC4P
    pub mod CC4P {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC6P::RW;
    }

    /// CC4E
    pub mod CC4E {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC6E::RW;
    }

    /// CC3NP
    pub mod CC3NP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OCxN active high
            pub const ActiveHigh: u32 = 0b0;

            /// 0b1: OCxN active low
            pub const ActiveLow: u32 = 0b1;
        }
    }

    /// CC3NE
    pub mod CC3NE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Complementary output disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Complementary output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CC3P
    pub mod CC3P {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC6P::RW;
    }

    /// CC3E
    pub mod CC3E {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC6E::RW;
    }

    /// CC2NP
    pub mod CC2NP {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC3NP::RW;
    }

    /// CC2NE
    pub mod CC2NE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC3NE::RW;
    }

    /// CC2P
    pub mod CC2P {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC6P::RW;
    }

    /// CC2E
    pub mod CC2E {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC6E::RW;
    }

    /// CC1NP
    pub mod CC1NP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC3NP::RW;
    }

    /// CC1NE
    pub mod CC1NE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC3NE::RW;
    }

    /// CC1P
    pub mod CC1P {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC6P::RW;
    }

    /// CC1E
    pub mod CC1E {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC6E::RW;
    }
}

/// counter
pub mod CNT {

    /// UIF copy
    pub mod UIFCPY {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CNT
    pub mod CNT {
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

/// prescaler
pub mod PSC {

    /// Prescaler value
    pub mod PSC {
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

/// auto-reload register
pub mod ARR {

    /// Auto-reload value
    pub mod ARR {
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

/// repetition counter register
pub mod RCR {

    /// Repetition counter value
    pub mod REP {
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

/// capture/compare register 1
pub mod CCR1 {

    /// Capture/Compare 1 value
    pub mod CCR1 {
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

/// capture/compare register 2
pub mod CCR2 {

    /// Capture/Compare 2 value
    pub mod CCR2 {
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

/// capture/compare register 3
pub mod CCR3 {

    /// Capture/Compare value
    pub mod CCR3 {
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

/// capture/compare register 4
pub mod CCR4 {

    /// Capture/Compare value
    pub mod CCR4 {
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

/// break and dead-time register
pub mod BDTR {

    /// Break2 bidirectional
    pub mod BK2BID {
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

            /// 0b0: Break input BRK2 in input mode
            pub const Input: u32 = 0b0;

            /// 0b1: Break input BRK2 in bidirectional mode
            pub const Bidirectional: u32 = 0b1;
        }
    }

    /// BKBID
    pub mod BKBID {
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

            /// 0b0: Break input BRK in input mode
            pub const Input: u32 = 0b0;

            /// 0b1: Break input BRK in bidirectional mode
            pub const Bidirectional: u32 = 0b1;
        }
    }

    /// Break2 Disarm
    pub mod BK2DSRM {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Break input BRK2 is armed
            pub const Armed: u32 = 0b0;

            /// 0b1: Break input BRK2 is disarmed
            pub const Disarmed: u32 = 0b1;
        }
    }

    /// BKDSRM
    pub mod BKDSRM {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Break input BRK is armed
            pub const Armed: u32 = 0b0;

            /// 0b1: Break input BRK is disarmed
            pub const Disarmed: u32 = 0b1;
        }
    }

    /// Break 2 polarity
    pub mod BK2P {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Break input BRK2 is active low
            pub const Low: u32 = 0b0;

            /// 0b1: Break input BRK2 is active high
            pub const High: u32 = 0b1;
        }
    }

    /// Break 2 enable
    pub mod BK2E {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Break function disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Break function enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Break 2 filter
    pub mod BK2F {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No filter, sampling is done at fDTS
            pub const NoFilter: u32 = 0b0000;

            /// 0b0001: fSAMPLING=fCK_INT, N=2
            pub const FCK_INT_N2: u32 = 0b0001;

            /// 0b0010: fSAMPLING=fCK_INT, N=4
            pub const FCK_INT_N4: u32 = 0b0010;

            /// 0b0011: fSAMPLING=fCK_INT, N=8
            pub const FCK_INT_N8: u32 = 0b0011;

            /// 0b0100: fSAMPLING=fDTS/2, N=6
            pub const FDTS_Div2_N6: u32 = 0b0100;

            /// 0b0101: fSAMPLING=fDTS/2, N=8
            pub const FDTS_Div2_N8: u32 = 0b0101;

            /// 0b0110: fSAMPLING=fDTS/4, N=6
            pub const FDTS_Div4_N6: u32 = 0b0110;

            /// 0b0111: fSAMPLING=fDTS/4, N=8
            pub const FDTS_Div4_N8: u32 = 0b0111;

            /// 0b1000: fSAMPLING=fDTS/8, N=6
            pub const FDTS_Div8_N6: u32 = 0b1000;

            /// 0b1001: fSAMPLING=fDTS/8, N=8
            pub const FDTS_Div8_N8: u32 = 0b1001;

            /// 0b1010: fSAMPLING=fDTS/16, N=5
            pub const FDTS_Div16_N5: u32 = 0b1010;

            /// 0b1011: fSAMPLING=fDTS/16, N=6
            pub const FDTS_Div16_N6: u32 = 0b1011;

            /// 0b1100: fSAMPLING=fDTS/16, N=8
            pub const FDTS_Div16_N8: u32 = 0b1100;

            /// 0b1101: fSAMPLING=fDTS/32, N=5
            pub const FDTS_Div32_N5: u32 = 0b1101;

            /// 0b1110: fSAMPLING=fDTS/32, N=6
            pub const FDTS_Div32_N6: u32 = 0b1110;

            /// 0b1111: fSAMPLING=fDTS/32, N=8
            pub const FDTS_Div32_N8: u32 = 0b1111;
        }
    }

    /// Break filter
    pub mod BKF {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BK2F::RW;
    }

    /// Main output enable
    pub mod MOE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: In response to a break 2 event OC and OCN outputs are disabled - In response to a break event or if MOE is written to 0 OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit
            pub const Disabled: u32 = 0b0;

            /// 0b1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Automatic output enable
    pub mod AOE {
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

            /// 0b0: MOE can be set only by software
            pub const Disabled: u32 = 0b0;

            /// 0b1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Break polarity
    pub mod BKP {
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

            /// 0b0: Break input BRK is active low
            pub const ActiveLow: u32 = 0b0;

            /// 0b1: Break input BRK is active high
            pub const ActiveHigh: u32 = 0b1;
        }
    }

    /// Break enable
    pub mod BKE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BK2E::RW;
    }

    /// Off-state selection for Run mode
    pub mod OSSR {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OC/OCN outputs are disabled when inactive
            pub const Disabled: u32 = 0b0;

            /// 0b1: OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Off-state selection for Idle mode
    pub mod OSSI {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OC/OCN outputs are disabled when inactive
            pub const Disabled: u32 = 0b0;

            /// 0b1: OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Lock configuration
    pub mod LOCK {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No write protection
            pub const Off: u32 = 0b00;

            /// 0b01: Level 1 write protection
            pub const Level1: u32 = 0b01;

            /// 0b10: Level 2 write protection
            pub const Level2: u32 = 0b10;

            /// 0b11: Level 3 write protection
            pub const Level3: u32 = 0b11;
        }
    }

    /// Dead-time generator setup
    pub mod DTG {
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

/// DMA control register
pub mod DCR {

    /// DMA burst length
    pub mod DBL {
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

    /// DMA base address
    pub mod DBA {
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

/// DMA address for full transfer
pub mod DMAR {

    /// DMA register for burst accesses
    pub mod DMAB {
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

/// option register 1
pub mod OR1 {

    /// Input Capture 1 remap
    pub mod TI1_RMP {
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

            /// 0b0: TIM1 input capture 1 is connected to I/O
            pub const IO: u32 = 0b0;

            /// 0b1: TIM1 input capture 1 is connected to COMP1 output
            pub const COMP1: u32 = 0b1;
        }
    }

    /// TIM1_ETR_ADC1 remapping capability
    pub mod TIM1_ETR_ADC1_RMP {
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

            /// 0b00: TIM1_ETR is not connected to ADC AWDx (must be selected when the ETR comes from the ETR input pin)
            pub const Select: u32 = 0b00;

            /// 0b01: TIM1_ETR is connected to ADC AWD1
            pub const ADC_AWD1: u32 = 0b01;

            /// 0b10: TIM1_ETR is connected to ADC AWD2
            pub const ADC_AWD2: u32 = 0b10;

            /// 0b11: TIM1_ETR is connected to ADC AWD3
            pub const ADC_AWD3: u32 = 0b11;
        }
    }
}

/// capture/compare mode register 3
pub mod CCMR3_Output {

    /// OC6M
    pub mod OC6M_3 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal output compare mode (modes 0-7)
            pub const Normal: u32 = 0b0;

            /// 0b1: Extended output compare mode (modes 7-15)
            pub const Extended: u32 = 0b1;
        }
    }

    /// OC5M
    pub mod OC5M_3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC6M_3::RW;
    }

    /// OC6CE
    pub mod OC6CE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OCxRef is not affected by the ocref_clr_int signal
            pub const Disabled: u32 = 0b0;

            /// 0b1: OCxRef is cleared as soon as a High level is detected on ocref_clr_int signal
            pub const Enabled: u32 = 0b1;
        }
    }

    /// OC6M
    pub mod OC6M {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
            pub const Frozen: u32 = 0b000;

            /// 0b001: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
            pub const ActiveOnMatch: u32 = 0b001;

            /// 0b010: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
            pub const InactiveOnMatch: u32 = 0b010;

            /// 0b011: OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
            pub const Toggle: u32 = 0b011;

            /// 0b100: OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
            pub const ForceInactive: u32 = 0b100;

            /// 0b101: OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
            pub const ForceActive: u32 = 0b101;

            /// 0b110: In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
            pub const PwmMode1: u32 = 0b110;

            /// 0b111: Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
            pub const PwmMode2: u32 = 0b111;
        }
    }

    /// OC6PE
    pub mod OC6PE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
            pub const Disabled: u32 = 0b0;

            /// 0b1: Preload register on CCRx enabled. Preload value is loaded into active register on each update event
            pub const Enabled: u32 = 0b1;
        }
    }

    /// OC6FE
    pub mod OC6FE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Fast output disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Fast output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// OC5CE
    pub mod OC5CE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC6CE::RW;
    }

    /// OC5M
    pub mod OC5M {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC6M::RW;
    }

    /// OC5PE
    pub mod OC5PE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC6PE::RW;
    }

    /// OC5FE
    pub mod OC5FE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC6FE::RW;
    }
}

/// capture/compare register 5
pub mod CCR5 {

    /// Group Channel 5 and Channel 3
    pub mod GC5C3 {
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

            /// 0b0: No effect of OC5REF on OC3REFC
            pub const Disabled: u32 = 0b0;

            /// 0b1: OC3REFC is the logical AND of OC3REFC and OC5REF
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Group Channel 5 and Channel 2
    pub mod GC5C2 {
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

            /// 0b0: No effect of OC5REF on OC2REFC
            pub const Disabled: u32 = 0b0;

            /// 0b1: OC2REFC is the logical AND of OC2REFC and OC5REF
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Group Channel 5 and Channel 1
    pub mod GC5C1 {
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

            /// 0b0: No effect of OC5REF on OC1REFC
            pub const Disabled: u32 = 0b0;

            /// 0b1: OC1REFC is the logical AND of OC1REFC and OC5REF
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Capture/Compare 5 value
    pub mod CCR5 {
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

/// capture/compare register 6
pub mod CCR6 {

    /// Capture/Compare 6 value
    pub mod CCR6 {
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

/// alternate function option register 1
pub mod AF1 {

    /// ETR source selection
    pub mod ETRSEL {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (4 bits: 0b1111 << 14)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: ETR legacy mode
            pub const Legacy: u32 = 0b0000;

            /// 0b0001: COMP1 output
            pub const COMP1: u32 = 0b0001;

            /// 0b0010: COMP2 output
            pub const COMP2: u32 = 0b0010;
        }
    }

    /// BRK COMP2 input polarity
    pub mod BKCMP2P {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input polarity not inverted
            pub const NotInverted: u32 = 0b0;

            /// 0b1: Input polarity inverted
            pub const Inverted: u32 = 0b1;
        }
    }

    /// BRK COMP1 input polarity
    pub mod BKCMP1P {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BKCMP2P::RW;
    }

    /// BRK BKIN input polarity
    pub mod BKINP {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BKCMP2P::RW;
    }

    /// BRK COMP2 enable
    pub mod BKCMP2E {
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

            /// 0b0: COMP2 input disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: COMP2 input enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// BRK COMP1 enable
    pub mod BKCMP1E {
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

            /// 0b0: COMP1 input disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: COMP1 input enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// BRK BKIN input enable
    pub mod BKINE {
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

            /// 0b0: BKIN input disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: BKIN input enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Alternate function register 2
pub mod AF2 {

    /// BRK2 COMP2 input polarity
    pub mod BK2CMP2P {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input polarity not inverted
            pub const NotInverted: u32 = 0b0;

            /// 0b1: Input polarity inverted
            pub const Inverted: u32 = 0b1;
        }
    }

    /// BRK2 COMP1 input polarity
    pub mod BK2CMP1P {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BK2CMP2P::RW;
    }

    /// BRK2 BKIN2 input polarity
    pub mod BK2INP {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::BK2CMP2P::RW;
    }

    /// BRK2 COMP2 enable
    pub mod BK2CMP2E {
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

            /// 0b0: COMP2 input disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: COMP2 input enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// BRK2 COMP1 enable
    pub mod BK2CMP1E {
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

            /// 0b0: COMP1 input disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: COMP1 input enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// BRK2 BKIN input enable
    pub mod BK2INE {
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

            /// 0b0: BKIN input disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: BKIN input enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// timer input selection register
pub mod TISEL {

    /// selects TI4\[0\] to TI4\[15\] input
    pub mod TI4SEL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: TIM1_CHx input selected
            pub const Selected: u32 = 0b0000;
        }
    }

    /// selects TI3\[0\] to TI3\[15\] input
    pub mod TI3SEL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TI4SEL::RW;
    }

    /// selects TI2\[0\] to TI2\[15\] input
    pub mod TI2SEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TI4SEL::RW;
    }

    /// selects TI1\[0\] to TI1\[15\] input
    pub mod TI1SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TI4SEL::RW;
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// control register 1
    pub CR1: RWRegister<u32>,

    /// control register 2
    pub CR2: RWRegister<u32>,

    /// slave mode control register
    pub SMCR: RWRegister<u32>,

    /// DMA/interrupt enable register
    pub DIER: RWRegister<u32>,

    /// status register
    pub SR: RWRegister<u32>,

    /// event generation register
    pub EGR: WORegister<u32>,

    /// CCMR1_Output and CCMR1_Input
    /// CCMR1_Output: capture/compare mode register 1 (output mode)
    /// CCMR1_Input: capture/compare mode register 1 (input mode)
    pub CCMR1: RWRegister<u32>,

    /// CCMR2_Output and CCMR2_Input
    /// CCMR2_Output: capture/compare mode register 2 (output mode)
    /// CCMR2_Input: capture/compare mode register 2 (input mode)
    pub CCMR2: RWRegister<u32>,

    /// capture/compare enable register
    pub CCER: RWRegister<u32>,

    /// counter
    pub CNT: RWRegister<u32>,

    /// prescaler
    pub PSC: RWRegister<u32>,

    /// auto-reload register
    pub ARR: RWRegister<u32>,

    /// repetition counter register
    pub RCR: RWRegister<u32>,

    /// capture/compare register 1
    pub CCR1: RWRegister<u32>,

    /// capture/compare register 2
    pub CCR2: RWRegister<u32>,

    /// capture/compare register 3
    pub CCR3: RWRegister<u32>,

    /// capture/compare register 4
    pub CCR4: RWRegister<u32>,

    /// break and dead-time register
    pub BDTR: RWRegister<u32>,

    /// DMA control register
    pub DCR: RWRegister<u32>,

    /// DMA address for full transfer
    pub DMAR: RWRegister<u32>,

    /// option register 1
    pub OR1: RWRegister<u32>,

    /// capture/compare mode register 3
    pub CCMR3_Output: RWRegister<u32>,

    /// capture/compare register 5
    pub CCR5: RWRegister<u32>,

    /// capture/compare register 6
    pub CCR6: RWRegister<u32>,

    /// alternate function option register 1
    pub AF1: RWRegister<u32>,

    /// Alternate function register 2
    pub AF2: RWRegister<u32>,

    /// timer input selection register
    pub TISEL: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1: u32,
    pub CR2: u32,
    pub SMCR: u32,
    pub DIER: u32,
    pub SR: u32,
    pub EGR: u32,
    pub CCMR1: u32,
    pub CCMR2: u32,
    pub CCER: u32,
    pub CNT: u32,
    pub PSC: u32,
    pub ARR: u32,
    pub RCR: u32,
    pub CCR1: u32,
    pub CCR2: u32,
    pub CCR3: u32,
    pub CCR4: u32,
    pub BDTR: u32,
    pub DCR: u32,
    pub DMAR: u32,
    pub OR1: u32,
    pub CCMR3_Output: u32,
    pub CCR5: u32,
    pub CCR6: u32,
    pub AF1: u32,
    pub AF2: u32,
    pub TISEL: u32,
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
