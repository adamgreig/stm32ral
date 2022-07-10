#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! High Resolution Timer: TIMA

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Timerx Control Register
pub mod TIMACR {

    /// Update Gating
    pub mod UPDGAT {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Update occurs independently from the DMA burst transfer
            pub const Independent: u32 = 0b0000;

            /// 0b0001: Update occurs when the DMA burst transfer is completed
            pub const DMABurst: u32 = 0b0001;

            /// 0b0010: Update occurs on the update event following DMA burst transfer completion
            pub const DMABurst_Update: u32 = 0b0010;

            /// 0b0011: Update occurs on a rising edge of HRTIM update enable input 1
            pub const Input1: u32 = 0b0011;

            /// 0b0100: Update occurs on a rising edge of HRTIM update enable input 2
            pub const Input2: u32 = 0b0100;

            /// 0b0101: Update occurs on a rising edge of HRTIM update enable input 3
            pub const Input3: u32 = 0b0101;

            /// 0b0110: Update occurs on the update event following a rising edge of HRTIM update enable input 1
            pub const Input1_Update: u32 = 0b0110;

            /// 0b0111: Update occurs on the update event following a rising edge of HRTIM update enable input 2
            pub const Input2_Update: u32 = 0b0111;

            /// 0b1000: Update occurs on the update event following a rising edge of HRTIM update enable input 3
            pub const Input3_Update: u32 = 0b1000;
        }
    }

    /// Preload enable
    pub mod PREEN {
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

            /// 0b0: Preload disabled: the write access is directly done into the active register
            pub const Disabled: u32 = 0b0;

            /// 0b1: Preload enabled: the write access is done into the preload register
            pub const Enabled: u32 = 0b1;
        }
    }

    /// AC Synchronization
    pub mod DACSYNC {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (2 bits: 0b11 << 25)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No DAC trigger generated
            pub const Disabled: u32 = 0b00;

            /// 0b01: Trigger generated on DACSync1
            pub const DACSync1: u32 = 0b01;

            /// 0b10: Trigger generated on DACSync2
            pub const DACSync2: u32 = 0b10;

            /// 0b11: Trigger generated on DACSync3
            pub const DACSync3: u32 = 0b11;
        }
    }

    /// Master Timer update
    pub mod MSTU {
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

            /// 0b0: Update by master timer disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Update by master timer enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TEU
    pub mod TEU {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Update by timer x disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Update by timer x enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TDU
    pub mod TDU {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEU::RW;
    }

    /// TCU
    pub mod TCU {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEU::RW;
    }

    /// TBU
    pub mod TBU {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEU::RW;
    }

    /// Timerx reset update
    pub mod TxRSTU {
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

            /// 0b0: Update by timer x reset/roll-over disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Update by timer x reset/roll-over enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Timer x Repetition update
    pub mod TxREPU {
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

            /// 0b0: Update by timer x repetition disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Update by timer x repetition enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Delayed CMP4 mode
    pub mod DELCMP4 {
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

            /// 0b00: CMP4 register is always active (standard compare mode)
            pub const Standard: u32 = 0b00;

            /// 0b01: CMP4 is recomputed and is active following a capture 2 event
            pub const Capture2: u32 = 0b01;

            /// 0b10: CMP4 is recomputed and is active following a capture 2 event or a Compare 1 match
            pub const Capture2_Compare1: u32 = 0b10;

            /// 0b11: CMP4 is recomputed and is active following a capture event or a Compare 3 match
            pub const Capture_Compare3: u32 = 0b11;
        }
    }

    /// Delayed CMP2 mode
    pub mod DELCMP2 {
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

            /// 0b00: CMP2 register is always active (standard compare mode)
            pub const Standard: u32 = 0b00;

            /// 0b01: CMP2 is recomputed and is active following a capture 1 event
            pub const Capture1: u32 = 0b01;

            /// 0b10: CMP2 is recomputed and is active following a capture 1 event or a Compare 1 match
            pub const Capture1_Compare1: u32 = 0b10;

            /// 0b11: CMP2 is recomputed and is active following a capture 1 event or a Compare 3 match
            pub const Capture1_Compare3: u32 = 0b11;
        }
    }

    /// Synchronization Starts Timer x
    pub mod SYNCSTRTx {
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

            /// 0b0: Synchronization event has no effect on Timer x
            pub const Disabled: u32 = 0b0;

            /// 0b1: Synchronization event starts Timer x
            pub const Start: u32 = 0b1;
        }
    }

    /// Synchronization Resets Timer x
    pub mod SYNCRSTx {
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

            /// 0b0: Synchronization event has no effect on Timer x
            pub const Disabled: u32 = 0b0;

            /// 0b1: Synchronization event resets Timer x
            pub const Reset: u32 = 0b1;
        }
    }

    /// Push-Pull mode enable
    pub mod PSHPLL {
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

            /// 0b0: Push-pull mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Push-pull mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Half mode enable
    pub mod HALF {
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

            /// 0b0: Half mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Half mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Re-triggerable mode
    pub mod RETRIG {
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

            /// 0b0: The timer is not re-triggerable: a counter reset can be done only if the counter is stopped
            pub const Disabled: u32 = 0b0;

            /// 0b1: The timer is retriggerable: a counter reset is done whatever the counter state
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Continuous mode
    pub mod CONT {
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

            /// 0b0: The timer operates in single-shot mode and stops when it reaches the MPER value
            pub const SingleShot: u32 = 0b0;

            /// 0b1: The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value
            pub const Continuous: u32 = 0b1;
        }
    }

    /// HRTIM Timer x Clock prescaler
    pub mod CKPSCx {
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
}

/// Timerx Interrupt Status Register
pub mod TIMAISR {

    /// Output 2 Copy
    pub mod O2CPY {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Output is inactive
            pub const Inactive: u32 = 0b0;

            /// 0b1: Output is active
            pub const Active: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output 1 Copy
    pub mod O1CPY {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        pub use super::O2CPY::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output 2 State
    pub mod O2STAT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Output was inactive
            pub const Inactive: u32 = 0b0;

            /// 0b1: Output was active
            pub const Active: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output 1 State
    pub mod O1STAT {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        pub use super::O2STAT::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Idle Push Pull Status
    pub mod IPPSTAT {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Protection occurred when the output 1 was active and output 2 forced inactive
            pub const Output1Active: u32 = 0b0;

            /// 0b1: Protection occurred when the output 2 was active and output 1 forced inactive
            pub const Output2Active: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Current Push Pull Status
    pub mod CPPSTAT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Signal applied on output 1 and output 2 forced inactive
            pub const Output1Active: u32 = 0b0;

            /// 0b1: Signal applied on output 2 and output 1 forced inactive
            pub const Output2Active: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Delayed Protection Flag
    pub mod DLYPRT {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Not in delayed idle or balanced idle mode
            pub const Inactive: u32 = 0b0;

            /// 0b1: Delayed idle or balanced idle mode entry
            pub const Active: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reset Interrupt Flag
    pub mod RST {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No TIMx counter reset/roll-over interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: TIMx counter reset/roll-over interrupt occurred
            pub const Event: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output 2 Reset Interrupt Flag
    pub mod RSTx2 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No Tx output reset interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Tx output reset interrupt occurred
            pub const Event: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output 2 Set Interrupt Flag
    pub mod SETx2 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No Tx output set interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Tx output set interrupt occurred
            pub const Event: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output 1 Reset Interrupt Flag
    pub mod RSTx1 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        pub use super::RSTx2::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output 1 Set Interrupt Flag
    pub mod SETx1 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        pub use super::SETx2::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture2 Interrupt Flag
    pub mod CPT2 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No timer x capture reset interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Timer x capture reset interrupt occurred
            pub const Event: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture1 Interrupt Flag
    pub mod CPT1 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        pub use super::CPT2::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update Interrupt Flag
    pub mod UPD {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No timer update interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Timer update interrupt occurred
            pub const Event: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Repetition Interrupt Flag
    pub mod REP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No timer repetition interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Timer repetition interrupt occurred
            pub const Event: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Compare 4 Interrupt Flag
    pub mod CMP4 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No compare interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Compare interrupt occurred
            pub const Event: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Compare 3 Interrupt Flag
    pub mod CMP3 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::CMP4::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Compare 2 Interrupt Flag
    pub mod CMP2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::CMP4::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Compare 1 Interrupt Flag
    pub mod CMP1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        pub use super::CMP4::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timerx Interrupt Clear Register
pub mod TIMAICR {

    /// Delayed Protection Flag Clear
    pub mod DLYPRTC {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clears associated flag in ISR register
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reset Interrupt flag Clear
    pub mod RSTC {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DLYPRTC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output 2 Reset flag Clear
    pub mod RSTx2C {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DLYPRTC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output 2 Set flag Clear
    pub mod SET2xC {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DLYPRTC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output 1 Reset flag Clear
    pub mod RSTx1C {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DLYPRTC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output 1 Set flag Clear
    pub mod SET1xC {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DLYPRTC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture2 Interrupt flag Clear
    pub mod CPT2C {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DLYPRTC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture1 Interrupt flag Clear
    pub mod CPT1C {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DLYPRTC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update Interrupt flag Clear
    pub mod UPDC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DLYPRTC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Repetition Interrupt flag Clear
    pub mod REPC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DLYPRTC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Compare 4 Interrupt flag Clear
    pub mod CMP4C {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DLYPRTC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Compare 3 Interrupt flag Clear
    pub mod CMP3C {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DLYPRTC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Compare 2 Interrupt flag Clear
    pub mod CMP2C {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DLYPRTC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Compare 1 Interrupt flag Clear
    pub mod CMP1C {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::DLYPRTC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIMxDIER5
pub mod TIMADIER {

    /// DLYPRTDE
    pub mod DLYPRTDE {
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

            /// 0b0: Delayed protection DMA request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Delayed protection DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RSTDE
    pub mod RSTDE {
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

            /// 0b0: Timer x counter reset/roll-over DMA request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Timer x counter reset/roll-over DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RSTx2DE
    pub mod RSTx2DE {
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

            /// 0b0: Tx output reset DMA request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Tx output reset DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SETx2DE
    pub mod SETx2DE {
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

            /// 0b0: Tx output set DMA request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Tx output set DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RSTx1DE
    pub mod RSTx1DE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RSTx2DE::RW;
    }

    /// SET1xDE
    pub mod SETx1DE {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SETx2DE::RW;
    }

    /// CPT2DE
    pub mod CPT2DE {
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

            /// 0b0: Capture DMA request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Capture DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CPT1DE
    pub mod CPT1DE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CPT2DE::RW;
    }

    /// UPDDE
    pub mod UPDDE {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
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

    /// REPDE
    pub mod REPDE {
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

            /// 0b0: Repetition DMA request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Repetition DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CMP4DE
    pub mod CMP4DE {
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

            /// 0b0: Compare DMA request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Compare DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CMP3DE
    pub mod CMP3DE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CMP4DE::RW;
    }

    /// CMP2DE
    pub mod CMP2DE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CMP4DE::RW;
    }

    /// CMP1DE
    pub mod CMP1DE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CMP4DE::RW;
    }

    /// DLYPRTIE
    pub mod DLYPRTIE {
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

            /// 0b0: Delayed protection interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Delayed protection interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RSTIE
    pub mod RSTIE {
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

            /// 0b0: Timer x counter/reset roll-over interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Timer x counter/reset roll-over interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RSTx2IE
    pub mod RSTx2IE {
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

            /// 0b0: Tx output reset interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Tx output reset interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SETx2IE
    pub mod SETx2IE {
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

            /// 0b0: Tx output set interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Tx output set interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RSTx1IE
    pub mod RSTx1IE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RSTx2IE::RW;
    }

    /// SET1xIE
    pub mod SETx1IE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SETx2IE::RW;
    }

    /// CPT2IE
    pub mod CPT2IE {
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

            /// 0b0: Capture interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Capture interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CPT1IE
    pub mod CPT1IE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CPT2IE::RW;
    }

    /// UPDIE
    pub mod UPDIE {
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

            /// 0b0: Update interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Update interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// REPIE
    pub mod REPIE {
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

            /// 0b0: Repetition interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Repetition interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CMP4IE
    pub mod CMP4IE {
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

            /// 0b0: Compare interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Compare interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CMP3IE
    pub mod CMP3IE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CMP4IE::RW;
    }

    /// CMP2IE
    pub mod CMP2IE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CMP4IE::RW;
    }

    /// CMP1IE
    pub mod CMP1IE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CMP4IE::RW;
    }
}

/// Timerx Counter Register
pub mod CNTAR {

    /// Timerx Counter value
    pub mod CNTx {
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

/// Timerx Period Register
pub mod PERAR {

    /// Timerx Period value
    pub mod PERx {
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

/// Timerx Repetition Register
pub mod REPAR {

    /// Timerx Repetition counter value
    pub mod REPx {
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

/// Timerx Compare 1 Register
pub mod CMP1AR {

    /// Timerx Compare 1 value
    pub mod CMP1x {
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

/// Timerx Compare 1 Compound Register
pub mod CMP1CAR {

    /// Timerx Repetition value (aliased from HRTIM_REPx register)
    pub mod REPx {
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

    /// Timerx Compare 1 value
    pub mod CMP1x {
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

/// Timerx Compare 2 Register
pub mod CMP2AR {

    /// Timerx Compare 2 value
    pub mod CMP2x {
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

/// Timerx Compare 3 Register
pub mod CMP3AR {

    /// Timerx Compare 3 value
    pub mod CMP3x {
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

/// Timerx Compare 4 Register
pub mod CMP4AR {

    /// Timerx Compare 4 value
    pub mod CMP4x {
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

/// Timerx Capture 1 Register
pub mod CPT1AR {

    /// Timerx Capture 1 value
    pub mod CPT1x {
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

/// Timerx Capture 2 Register
pub mod CPT2AR {

    /// Timerx Capture 2 value
    pub mod CPT2x {
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

/// Timerx Deadtime Register
pub mod DTAR {

    /// Deadtime Falling Lock
    pub mod DTFLKx {
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

            /// 0b0: Deadtime falling value and sign is writable
            pub const Unlocked: u32 = 0b0;

            /// 0b1: Deadtime falling value and sign is read-only
            pub const Locked: u32 = 0b1;
        }
    }

    /// Deadtime Falling Sign Lock
    pub mod DTFSLKx {
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

            /// 0b0: Deadtime falling sign is writable
            pub const Unlocked: u32 = 0b0;

            /// 0b1: Deadtime falling sign is read-only
            pub const Locked: u32 = 0b1;
        }
    }

    /// Sign Deadtime Falling value
    pub mod SDTFx {
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

            /// 0b0: Positive deadtime on falling edge
            pub const Positive: u32 = 0b0;

            /// 0b1: Negative deadtime on falling edge
            pub const Negative: u32 = 0b1;
        }
    }

    /// Deadtime Falling value
    pub mod DTFx {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (9 bits: 0x1ff << 16)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Deadtime Rising Lock
    pub mod DTRLKx {
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

            /// 0b0: Deadtime rising value and sign is writable
            pub const Unlocked: u32 = 0b0;

            /// 0b1: Deadtime rising value and sign is read-only
            pub const Locked: u32 = 0b1;
        }
    }

    /// Deadtime Rising Sign Lock
    pub mod DTRSLKx {
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

            /// 0b0: Deadtime rising sign is writable
            pub const Unlocked: u32 = 0b0;

            /// 0b1: Deadtime rising sign is read-only
            pub const Locked: u32 = 0b1;
        }
    }

    /// Deadtime Prescaler
    pub mod DTPRSC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (3 bits: 0b111 << 10)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Sign Deadtime Rising value
    pub mod SDTRx {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Positive deadtime on rising edge
            pub const Positive: u32 = 0b0;

            /// 0b1: Negative deadtime on rising edge
            pub const Negative: u32 = 0b1;
        }
    }

    /// Deadtime Rising value
    pub mod DTRx {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timerx Output1 Set Register
pub mod SETA1R {

    /// Registers update (transfer preload to active)
    pub mod UPDATE {
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

            /// 0b0: Register update event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Register update event forces the output to its active state
            pub const SetActive: u32 = 0b1;
        }
    }

    /// External Event 10
    pub mod EXTEVNT10 {
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

            /// 0b0: External event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: External event forces the output to its active state
            pub const SetActive: u32 = 0b1;
        }
    }

    /// External Event 9
    pub mod EXTEVNT9 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 8
    pub mod EXTEVNT8 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 7
    pub mod EXTEVNT7 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 6
    pub mod EXTEVNT6 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 5
    pub mod EXTEVNT5 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 4
    pub mod EXTEVNT4 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 3
    pub mod EXTEVNT3 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 2
    pub mod EXTEVNT2 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 1
    pub mod EXTEVNT1 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// Timer Event 9
    pub mod TIMEVNT9 {
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

            /// 0b0: Timer event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer event forces the output to its active state
            pub const SetActive: u32 = 0b1;
        }
    }

    /// Timer Event 8
    pub mod TIMEVNT8 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEVNT9::RW;
    }

    /// Timer Event 7
    pub mod TIMEVNT7 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEVNT9::RW;
    }

    /// Timer Event 6
    pub mod TIMEVNT6 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEVNT9::RW;
    }

    /// Timer Event 5
    pub mod TIMEVNT5 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEVNT9::RW;
    }

    /// Timer Event 4
    pub mod TIMEVNT4 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEVNT9::RW;
    }

    /// Timer Event 3
    pub mod TIMEVNT3 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEVNT9::RW;
    }

    /// Timer Event 2
    pub mod TIMEVNT2 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEVNT9::RW;
    }

    /// Timer Event 1
    pub mod TIMEVNT1 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEVNT9::RW;
    }

    /// Master Compare 4
    pub mod MSTCMP4 {
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

            /// 0b0: Master timer compare event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Master timer compare event forces the output to its active state
            pub const SetActive: u32 = 0b1;
        }
    }

    /// Master Compare 3
    pub mod MSTCMP3 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MSTCMP4::RW;
    }

    /// Master Compare 2
    pub mod MSTCMP2 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MSTCMP4::RW;
    }

    /// Master Compare 1
    pub mod MSTCMP1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MSTCMP4::RW;
    }

    /// Master Period
    pub mod MSTPER {
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

            /// 0b0: Master timer counter roll-over/reset has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Master timer counter roll-over/reset forces the output to its active state
            pub const SetActive: u32 = 0b1;
        }
    }

    /// Timer A compare 4
    pub mod CMP4 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No compare interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Compare interrupt occurred
            pub const Event: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer A compare 3
    pub mod CMP3 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        pub use super::CMP4::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer A compare 2
    pub mod CMP2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::CMP4::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer A compare 1
    pub mod CMP1 {
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

            /// 0b0: Timer compare event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer compare event forces the output to its active state
            pub const SetActive: u32 = 0b1;
        }
    }

    /// Timer A Period
    pub mod PER {
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

            /// 0b0: Timer period event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer period event forces the output to its active state
            pub const SetActive: u32 = 0b1;
        }
    }

    /// Timer A resynchronizaton
    pub mod RESYNC {
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

            /// 0b0: Timer reset event coming solely from software or SYNC input event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer reset event coming solely from software or SYNC input event forces the output to its active state
            pub const SetActive: u32 = 0b1;
        }
    }

    /// Software Set trigger
    pub mod SST {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Force output to its active state
            pub const SetActive: u32 = 0b1;
        }
    }
}

/// Timerx Output1 Reset Register
pub mod RSTA1R {

    /// UPDATE
    pub mod UPDATE {
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

            /// 0b0: Register update event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Register update event forces the output to its inactive state
            pub const SetInactive: u32 = 0b1;
        }
    }

    /// EXTEVNT10
    pub mod EXTEVNT10 {
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

            /// 0b0: External event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: External event forces the output to its active state
            pub const SetActive: u32 = 0b1;
        }
    }

    /// EXTEVNT9
    pub mod EXTEVNT9 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// EXTEVNT8
    pub mod EXTEVNT8 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// EXTEVNT7
    pub mod EXTEVNT7 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// EXTEVNT6
    pub mod EXTEVNT6 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// EXTEVNT5
    pub mod EXTEVNT5 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// EXTEVNT4
    pub mod EXTEVNT4 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// EXTEVNT3
    pub mod EXTEVNT3 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// EXTEVNT2
    pub mod EXTEVNT2 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// EXTEVNT1
    pub mod EXTEVNT1 {
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

            /// 0b0: External event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: External event forces the output to its inactive state
            pub const SetInactive: u32 = 0b1;
        }
    }

    /// TIMEVNT9
    pub mod TIMEVNT9 {
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

            /// 0b0: Timer event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer event forces the output to its active state
            pub const SetActive: u32 = 0b1;
        }
    }

    /// TIMEVNT8
    pub mod TIMEVNT8 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEVNT9::RW;
    }

    /// TIMEVNT7
    pub mod TIMEVNT7 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEVNT9::RW;
    }

    /// TIMEVNT6
    pub mod TIMEVNT6 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEVNT9::RW;
    }

    /// TIMEVNT5
    pub mod TIMEVNT5 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEVNT9::RW;
    }

    /// TIMEVNT4
    pub mod TIMEVNT4 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEVNT9::RW;
    }

    /// TIMEVNT3
    pub mod TIMEVNT3 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEVNT9::RW;
    }

    /// TIMEVNT2
    pub mod TIMEVNT2 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEVNT9::RW;
    }

    /// TIMEVNT1
    pub mod TIMEVNT1 {
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

            /// 0b0: Timer event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer event forces the output to its inactive state
            pub const SetInactive: u32 = 0b1;
        }
    }

    /// MSTCMP4
    pub mod MSTCMP4 {
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

            /// 0b0: Master timer compare event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Master timer compare event forces the output to its active state
            pub const SetActive: u32 = 0b1;
        }
    }

    /// MSTCMP3
    pub mod MSTCMP3 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MSTCMP4::RW;
    }

    /// MSTCMP2
    pub mod MSTCMP2 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MSTCMP4::RW;
    }

    /// MSTCMP1
    pub mod MSTCMP1 {
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

            /// 0b0: Master timer compare event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Master timer compare event forces the output to its inactive state
            pub const SetInactive: u32 = 0b1;
        }
    }

    /// MSTPER
    pub mod MSTPER {
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

            /// 0b0: Master timer counter roll-over/reset has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Master timer counter roll-over/reset forces the output to its inactive state
            pub const SetInactive: u32 = 0b1;
        }
    }

    /// CMP4
    pub mod CMP4 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No compare interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Compare interrupt occurred
            pub const Event: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CMP3
    pub mod CMP3 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        pub use super::CMP4::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CMP2
    pub mod CMP2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::CMP4::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CMP1
    pub mod CMP1 {
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

            /// 0b0: Timer compare event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer compare event forces the output to its inactive state
            pub const SetInactive: u32 = 0b1;
        }
    }

    /// PER
    pub mod PER {
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

            /// 0b0: Timer period event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer period event forces the output to its inactive state
            pub const SetInactive: u32 = 0b1;
        }
    }

    /// RESYNC
    pub mod RESYNC {
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

            /// 0b0: Timer reset event coming solely from software or SYNC input event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer reset event coming solely from software or SYNC input event forces the output to its inactive state
            pub const SetInactive: u32 = 0b1;
        }
    }

    /// SRT
    pub mod SRT {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Force output to its inactive state
            pub const SetInactive: u32 = 0b1;
        }
    }
}

/// Timerx Output2 Set Register
pub mod SETA2R {
    pub use super::SETA1R::CMP1;
    pub use super::SETA1R::CMP2;
    pub use super::SETA1R::CMP3;
    pub use super::SETA1R::CMP4;
    pub use super::SETA1R::EXTEVNT1;
    pub use super::SETA1R::EXTEVNT10;
    pub use super::SETA1R::EXTEVNT2;
    pub use super::SETA1R::EXTEVNT3;
    pub use super::SETA1R::EXTEVNT4;
    pub use super::SETA1R::EXTEVNT5;
    pub use super::SETA1R::EXTEVNT6;
    pub use super::SETA1R::EXTEVNT7;
    pub use super::SETA1R::EXTEVNT8;
    pub use super::SETA1R::EXTEVNT9;
    pub use super::SETA1R::MSTCMP1;
    pub use super::SETA1R::MSTCMP2;
    pub use super::SETA1R::MSTCMP3;
    pub use super::SETA1R::MSTCMP4;
    pub use super::SETA1R::MSTPER;
    pub use super::SETA1R::PER;
    pub use super::SETA1R::RESYNC;
    pub use super::SETA1R::SST;
    pub use super::SETA1R::TIMEVNT1;
    pub use super::SETA1R::TIMEVNT2;
    pub use super::SETA1R::TIMEVNT3;
    pub use super::SETA1R::TIMEVNT4;
    pub use super::SETA1R::TIMEVNT5;
    pub use super::SETA1R::TIMEVNT6;
    pub use super::SETA1R::TIMEVNT7;
    pub use super::SETA1R::TIMEVNT8;
    pub use super::SETA1R::TIMEVNT9;
    pub use super::SETA1R::UPDATE;
}

/// Timerx Output2 Reset Register
pub mod RSTA2R {
    pub use super::RSTA1R::CMP1;
    pub use super::RSTA1R::CMP2;
    pub use super::RSTA1R::CMP3;
    pub use super::RSTA1R::CMP4;
    pub use super::RSTA1R::EXTEVNT1;
    pub use super::RSTA1R::EXTEVNT10;
    pub use super::RSTA1R::EXTEVNT2;
    pub use super::RSTA1R::EXTEVNT3;
    pub use super::RSTA1R::EXTEVNT4;
    pub use super::RSTA1R::EXTEVNT5;
    pub use super::RSTA1R::EXTEVNT6;
    pub use super::RSTA1R::EXTEVNT7;
    pub use super::RSTA1R::EXTEVNT8;
    pub use super::RSTA1R::EXTEVNT9;
    pub use super::RSTA1R::MSTCMP1;
    pub use super::RSTA1R::MSTCMP2;
    pub use super::RSTA1R::MSTCMP3;
    pub use super::RSTA1R::MSTCMP4;
    pub use super::RSTA1R::MSTPER;
    pub use super::RSTA1R::PER;
    pub use super::RSTA1R::RESYNC;
    pub use super::RSTA1R::SRT;
    pub use super::RSTA1R::TIMEVNT1;
    pub use super::RSTA1R::TIMEVNT2;
    pub use super::RSTA1R::TIMEVNT3;
    pub use super::RSTA1R::TIMEVNT4;
    pub use super::RSTA1R::TIMEVNT5;
    pub use super::RSTA1R::TIMEVNT6;
    pub use super::RSTA1R::TIMEVNT7;
    pub use super::RSTA1R::TIMEVNT8;
    pub use super::RSTA1R::TIMEVNT9;
    pub use super::RSTA1R::UPDATE;
}

/// Timerx External Event Filtering Register 1
pub mod EEFAR1 {

    /// External Event 5 filter
    pub mod EE5FLTR {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (4 bits: 0b1111 << 25)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No filtering
            pub const Disabled: u32 = 0b0000;

            /// 0b0001: Blanking from counter reset/roll-over to Compare 1
            pub const BlankResetToCompare1: u32 = 0b0001;

            /// 0b0010: Blanking from counter reset/roll-over to Compare 2
            pub const BlankResetToCompare2: u32 = 0b0010;

            /// 0b0011: Blanking from counter reset/roll-over to Compare 3
            pub const BlankResetToCompare3: u32 = 0b0011;

            /// 0b0100: Blanking from counter reset/roll-over to Compare 4
            pub const BlankResetToCompare4: u32 = 0b0100;

            /// 0b0101: Blanking from another timing unit: TIMFLTR1 source
            pub const BlankTIMFLTR1: u32 = 0b0101;

            /// 0b0110: Blanking from another timing unit: TIMFLTR2 source
            pub const BlankTIMFLTR2: u32 = 0b0110;

            /// 0b0111: Blanking from another timing unit: TIMFLTR3 source
            pub const BlankTIMFLTR3: u32 = 0b0111;

            /// 0b1000: Blanking from another timing unit: TIMFLTR4 source
            pub const BlankTIMFLTR4: u32 = 0b1000;

            /// 0b1001: Blanking from another timing unit: TIMFLTR5 source
            pub const BlankTIMFLTR5: u32 = 0b1001;

            /// 0b1010: Blanking from another timing unit: TIMFLTR6 source
            pub const BlankTIMFLTR6: u32 = 0b1010;

            /// 0b1011: Blanking from another timing unit: TIMFLTR7 source
            pub const BlankTIMFLTR7: u32 = 0b1011;

            /// 0b1100: Blanking from another timing unit: TIMFLTR8 source
            pub const BlankTIMFLTR8: u32 = 0b1100;

            /// 0b1101: Windowing from counter reset/roll-over to compare 2
            pub const WindowResetToCompare2: u32 = 0b1101;

            /// 0b1110: Windowing from counter reset/roll-over to compare 3
            pub const WindowResetToCompare3: u32 = 0b1110;

            /// 0b1111: Windowing from another timing unit: TIMWIN source
            pub const WindowTIMWIN: u32 = 0b1111;
        }
    }

    /// External Event 5 latch
    pub mod EE5LTCH {
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

            /// 0b0: Event is ignored if it happens during a blank, or passed through during a window
            pub const Disabled: u32 = 0b0;

            /// 0b1: Event is latched and delayed till the end of the blanking or windowing period
            pub const Enabled: u32 = 0b1;
        }
    }

    /// External Event 4 filter
    pub mod EE4FLTR {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (4 bits: 0b1111 << 19)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5FLTR::RW;
    }

    /// External Event 4 latch
    pub mod EE4LTCH {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5LTCH::RW;
    }

    /// External Event 3 filter
    pub mod EE3FLTR {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (4 bits: 0b1111 << 13)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5FLTR::RW;
    }

    /// External Event 3 latch
    pub mod EE3LTCH {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5LTCH::RW;
    }

    /// External Event 2 filter
    pub mod EE2FLTR {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (4 bits: 0b1111 << 7)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5FLTR::RW;
    }

    /// External Event 2 latch
    pub mod EE2LTCH {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5LTCH::RW;
    }

    /// External Event 1 filter
    pub mod EE1FLTR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (4 bits: 0b1111 << 1)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5FLTR::RW;
    }

    /// External Event 1 latch
    pub mod EE1LTCH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5LTCH::RW;
    }
}

/// Timerx External Event Filtering Register 2
pub mod EEFAR2 {

    /// External Event 10 filter
    pub mod EE10FLTR {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (4 bits: 0b1111 << 25)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No filtering
            pub const Disabled: u32 = 0b0000;

            /// 0b0001: Blanking from counter reset/roll-over to Compare 1
            pub const BlankResetToCompare1: u32 = 0b0001;

            /// 0b0010: Blanking from counter reset/roll-over to Compare 2
            pub const BlankResetToCompare2: u32 = 0b0010;

            /// 0b0011: Blanking from counter reset/roll-over to Compare 3
            pub const BlankResetToCompare3: u32 = 0b0011;

            /// 0b0100: Blanking from counter reset/roll-over to Compare 4
            pub const BlankResetToCompare4: u32 = 0b0100;

            /// 0b0101: Blanking from another timing unit: TIMFLTR1 source
            pub const BlankTIMFLTR1: u32 = 0b0101;

            /// 0b0110: Blanking from another timing unit: TIMFLTR2 source
            pub const BlankTIMFLTR2: u32 = 0b0110;

            /// 0b0111: Blanking from another timing unit: TIMFLTR3 source
            pub const BlankTIMFLTR3: u32 = 0b0111;

            /// 0b1000: Blanking from another timing unit: TIMFLTR4 source
            pub const BlankTIMFLTR4: u32 = 0b1000;

            /// 0b1001: Blanking from another timing unit: TIMFLTR5 source
            pub const BlankTIMFLTR5: u32 = 0b1001;

            /// 0b1010: Blanking from another timing unit: TIMFLTR6 source
            pub const BlankTIMFLTR6: u32 = 0b1010;

            /// 0b1011: Blanking from another timing unit: TIMFLTR7 source
            pub const BlankTIMFLTR7: u32 = 0b1011;

            /// 0b1100: Blanking from another timing unit: TIMFLTR8 source
            pub const BlankTIMFLTR8: u32 = 0b1100;

            /// 0b1101: Windowing from counter reset/roll-over to compare 2
            pub const WindowResetToCompare2: u32 = 0b1101;

            /// 0b1110: Windowing from counter reset/roll-over to compare 3
            pub const WindowResetToCompare3: u32 = 0b1110;

            /// 0b1111: Windowing from another timing unit: TIMWIN source
            pub const WindowTIMWIN: u32 = 0b1111;
        }
    }

    /// External Event 10 latch
    pub mod EE10LTCH {
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

            /// 0b0: Event is ignored if it happens during a blank, or passed through during a window
            pub const Disabled: u32 = 0b0;

            /// 0b1: Event is latched and delayed till the end of the blanking or windowing period
            pub const Enabled: u32 = 0b1;
        }
    }

    /// External Event 9 filter
    pub mod EE9FLTR {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (4 bits: 0b1111 << 19)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10FLTR::RW;
    }

    /// External Event 9 latch
    pub mod EE9LTCH {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10LTCH::RW;
    }

    /// External Event 8 filter
    pub mod EE8FLTR {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (4 bits: 0b1111 << 13)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10FLTR::RW;
    }

    /// External Event 8 latch
    pub mod EE8LTCH {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10LTCH::RW;
    }

    /// External Event 7 filter
    pub mod EE7FLTR {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (4 bits: 0b1111 << 7)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10FLTR::RW;
    }

    /// External Event 7 latch
    pub mod EE7LTCH {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10LTCH::RW;
    }

    /// External Event 6 filter
    pub mod EE6FLTR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (4 bits: 0b1111 << 1)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10FLTR::RW;
    }

    /// External Event 6 latch
    pub mod EE6LTCH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10LTCH::RW;
    }
}

/// TimerA Reset Register
pub mod RSTAR {

    /// Timer E Compare 4
    pub mod TIMECMP4 {
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

            /// 0b0: Timer Y compare Z event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer X counter is reset upon timer Y compare Z event
            pub const ResetCounter: u32 = 0b1;
        }
    }

    /// Timer E Compare 2
    pub mod TIMECMP2 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMECMP4::RW;
    }

    /// Timer E Compare 1
    pub mod TIMECMP1 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMECMP4::RW;
    }

    /// Timer D Compare 4
    pub mod TIMDCMP4 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMECMP4::RW;
    }

    /// Timer D Compare 2
    pub mod TIMDCMP2 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMECMP4::RW;
    }

    /// Timer D Compare 1
    pub mod TIMDCMP1 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMECMP4::RW;
    }

    /// Timer C Compare 4
    pub mod TIMCCMP4 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMECMP4::RW;
    }

    /// Timer C Compare 2
    pub mod TIMCCMP2 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMECMP4::RW;
    }

    /// Timer C Compare 1
    pub mod TIMCCMP1 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMECMP4::RW;
    }

    /// Timer B Compare 4
    pub mod TIMBCMP4 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMECMP4::RW;
    }

    /// Timer B Compare 2
    pub mod TIMBCMP2 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMECMP4::RW;
    }

    /// Timer B Compare 1
    pub mod TIMBCMP1 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMECMP4::RW;
    }

    /// External Event 10
    pub mod EXTEVNT10 {
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

            /// 0b0: External event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: External event forces the output to its active state
            pub const SetActive: u32 = 0b1;
        }
    }

    /// External Event 9
    pub mod EXTEVNT9 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 8
    pub mod EXTEVNT8 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 7
    pub mod EXTEVNT7 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 6
    pub mod EXTEVNT6 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 5
    pub mod EXTEVNT5 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 4
    pub mod EXTEVNT4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 3
    pub mod EXTEVNT3 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 2
    pub mod EXTEVNT2 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXTEVNT10::RW;
    }

    /// External Event 1
    pub mod EXTEVNT1 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External event Z has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer X counter is reset upon external event Z
            pub const ResetCounter: u32 = 0b1;
        }
    }

    /// Master compare 4
    pub mod MSTCMP4 {
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

            /// 0b0: Master timer compare event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Master timer compare event forces the output to its active state
            pub const SetActive: u32 = 0b1;
        }
    }

    /// Master compare 3
    pub mod MSTCMP3 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MSTCMP4::RW;
    }

    /// Master compare 2
    pub mod MSTCMP2 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MSTCMP4::RW;
    }

    /// Master compare 1
    pub mod MSTCMP1 {
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

            /// 0b0: Master timer compare Z event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer X counter is reset upon master timer compare Z event
            pub const ResetCounter: u32 = 0b1;
        }
    }

    /// Master timer Period
    pub mod MSTPER {
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

            /// 0b0: Master timer period event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer X counter is reset upon master timer period event
            pub const ResetCounter: u32 = 0b1;
        }
    }

    /// Timer A compare 4 reset
    pub mod CMP4 {
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

            /// 0b0: Timer X compare Z event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer X counter is reset upon timer X compare Z event
            pub const ResetCounter: u32 = 0b1;
        }
    }

    /// Timer A compare 2 reset
    pub mod CMP2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CMP4::RW;
    }

    /// Timer A Update reset
    pub mod UPDT {
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

            /// 0b0: Update event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer X counter is reset upon update event
            pub const ResetCounter: u32 = 0b1;
        }
    }
}

/// Timerx Chopper Register
pub mod CHPAR {

    /// STRTPW
    pub mod STRTPW {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (4 bits: 0b1111 << 7)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timerx chopper duty cycle value
    pub mod CARDTY {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timerx carrier frequency value
    pub mod CARFRQ {
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
}

/// Timerx Capture 2 Control Register
pub mod CPT1ACR {

    /// Timer E Compare 2
    pub mod TECMP2 {
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

            /// 0b0: Timer X compare Y has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer X compare Y triggers capture Z
            pub const TriggerCapture: u32 = 0b1;
        }
    }

    /// Timer E Compare 1
    pub mod TECMP1 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECMP2::RW;
    }

    /// Timer E output 1 Reset
    pub mod TE1RST {
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

            /// 0b0: Timer X output Y active to inactive transition has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer X output Y active to inactive transition triggers capture Z
            pub const TriggerCapture: u32 = 0b1;
        }
    }

    /// Timer E output 1 Set
    pub mod TE1SET {
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

            /// 0b0: Timer X output Y inactive to active transition has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer X output Y inactive to active transition triggers capture Z
            pub const TriggerCapture: u32 = 0b1;
        }
    }

    /// Timer D Compare 2
    pub mod TDCMP2 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECMP2::RW;
    }

    /// Timer D Compare 1
    pub mod TDCMP1 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECMP2::RW;
    }

    /// Timer D output 1 Reset
    pub mod TD1RST {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE1RST::RW;
    }

    /// Timer D output 1 Set
    pub mod TD1SET {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE1SET::RW;
    }

    /// Timer C Compare 2
    pub mod TCCMP2 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECMP2::RW;
    }

    /// Timer C Compare 1
    pub mod TCCMP1 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECMP2::RW;
    }

    /// Timer C output 1 Reset
    pub mod TC1RST {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE1RST::RW;
    }

    /// Timer C output 1 Set
    pub mod TC1SET {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE1SET::RW;
    }

    /// Timer B Compare 2
    pub mod TBCMP2 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECMP2::RW;
    }

    /// Timer B Compare 1
    pub mod TBCMP1 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECMP2::RW;
    }

    /// Timer B output 1 Reset
    pub mod TB1RST {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE1RST::RW;
    }

    /// Timer B output 1 Set
    pub mod TB1SET {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE1SET::RW;
    }

    /// External Event 10 Capture
    pub mod EXEV10CPT {
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

            /// 0b0: External event Y has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: External event Y triggers capture Z
            pub const TriggerCapture: u32 = 0b1;
        }
    }

    /// External Event 9 Capture
    pub mod EXEV9CPT {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXEV10CPT::RW;
    }

    /// External Event 8 Capture
    pub mod EXEV8CPT {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXEV10CPT::RW;
    }

    /// External Event 7 Capture
    pub mod EXEV7CPT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXEV10CPT::RW;
    }

    /// External Event 6 Capture
    pub mod EXEV6CPT {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXEV10CPT::RW;
    }

    /// External Event 5 Capture
    pub mod EXEV5CPT {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXEV10CPT::RW;
    }

    /// External Event 4 Capture
    pub mod EXEV4CPT {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXEV10CPT::RW;
    }

    /// External Event 3 Capture
    pub mod EXEV3CPT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXEV10CPT::RW;
    }

    /// External Event 2 Capture
    pub mod EXEV2CPT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXEV10CPT::RW;
    }

    /// External Event 1 Capture
    pub mod EXEV1CPT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EXEV10CPT::RW;
    }

    /// Update Capture
    pub mod UPDCPT {
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

            /// 0b0: Update event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Update event triggers capture Z
            pub const TriggerCapture: u32 = 0b1;
        }
    }

    /// Software Capture
    pub mod SWCPT {
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

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Force capture Z
            pub const TriggerCapture: u32 = 0b1;
        }
    }
}

/// CPT2xCR
pub mod CPT2ACR {
    pub use super::CPT1ACR::EXEV10CPT;
    pub use super::CPT1ACR::EXEV1CPT;
    pub use super::CPT1ACR::EXEV2CPT;
    pub use super::CPT1ACR::EXEV3CPT;
    pub use super::CPT1ACR::EXEV4CPT;
    pub use super::CPT1ACR::EXEV5CPT;
    pub use super::CPT1ACR::EXEV6CPT;
    pub use super::CPT1ACR::EXEV7CPT;
    pub use super::CPT1ACR::EXEV8CPT;
    pub use super::CPT1ACR::EXEV9CPT;
    pub use super::CPT1ACR::SWCPT;
    pub use super::CPT1ACR::TB1RST;
    pub use super::CPT1ACR::TB1SET;
    pub use super::CPT1ACR::TBCMP1;
    pub use super::CPT1ACR::TBCMP2;
    pub use super::CPT1ACR::TC1RST;
    pub use super::CPT1ACR::TC1SET;
    pub use super::CPT1ACR::TCCMP1;
    pub use super::CPT1ACR::TCCMP2;
    pub use super::CPT1ACR::TD1RST;
    pub use super::CPT1ACR::TD1SET;
    pub use super::CPT1ACR::TDCMP1;
    pub use super::CPT1ACR::TDCMP2;
    pub use super::CPT1ACR::TE1RST;
    pub use super::CPT1ACR::TE1SET;
    pub use super::CPT1ACR::TECMP1;
    pub use super::CPT1ACR::TECMP2;
    pub use super::CPT1ACR::UPDCPT;
}

/// Timerx Output Register
pub mod OUTAR {

    /// Output 2 Deadtime upon burst mode Idle entry
    pub mod DIDL2 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The programmed idle state is applied immediately to the output
            pub const Disabled: u32 = 0b0;

            /// 0b1: Deadtime (inactive level) is inserted on output before entering the idle mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Output 2 Chopper enable
    pub mod CHP2 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Output signal not altered
            pub const Disabled: u32 = 0b0;

            /// 0b1: Output signal is chopped by a carrier signal
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Output 2 Fault state
    pub mod FAULT2 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No action: the output is not affected by the fault input and stays in run mode
            pub const Disabled: u32 = 0b00;

            /// 0b01: Output goes to active state after a fault event
            pub const SetActive: u32 = 0b01;

            /// 0b10: Output goes to inactive state after a fault event
            pub const SetInactive: u32 = 0b10;

            /// 0b11: Output goes to high-z state after a fault event
            pub const SetHighZ: u32 = 0b11;
        }
    }

    /// Output 2 Idle State
    pub mod IDLES2 {
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

            /// 0b0: Output idle state is inactive
            pub const Inactive: u32 = 0b0;

            /// 0b1: Output idle state is active
            pub const Active: u32 = 0b1;
        }
    }

    /// Output 2 Idle mode
    pub mod IDLEM2 {
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

            /// 0b0: No action: the output is not affected by the burst mode operation
            pub const NoEffect: u32 = 0b0;

            /// 0b1: The output is in idle state when requested by the burst mode controller
            pub const SetIdle: u32 = 0b1;
        }
    }

    /// Output 2 polarity
    pub mod POL2 {
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

            /// 0b0: Positive polarity (output active high)
            pub const ActiveHigh: u32 = 0b0;

            /// 0b1: Negative polarity (output active low)
            pub const ActiveLow: u32 = 0b1;
        }
    }

    /// Delayed Protection
    pub mod DLYPRT {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (3 bits: 0b111 << 10)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Output 1 delayed idle on external event 6
            pub const Output1_EE6: u32 = 0b000;

            /// 0b001: Output 2 delayed idle on external event 6
            pub const Output2_EE6: u32 = 0b001;

            /// 0b010: Output 1 and 2 delayed idle on external event 6
            pub const Output1_2_EE6: u32 = 0b010;

            /// 0b011: Balanced idle on external event 6
            pub const Balanced_EE6: u32 = 0b011;

            /// 0b100: Output 1 delayed idle on external event 7
            pub const Output1_EE7: u32 = 0b100;

            /// 0b101: Output 2 delayed idle on external event 7
            pub const Output2_EE7: u32 = 0b101;

            /// 0b110: Output 1 and 2 delayed idle on external event 7
            pub const Output1_2_EE7: u32 = 0b110;

            /// 0b111: Balanced idle on external event 7
            pub const Balanced_EE7: u32 = 0b111;
        }
    }

    /// Delayed Protection Enable
    pub mod DLYPRTEN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No action
            pub const Disabled: u32 = 0b0;

            /// 0b1: Delayed protection is enabled, as per DLYPRT bits
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Deadtime enable
    pub mod DTEN {
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

            /// 0b0: Output 1 and 2 signals are independent
            pub const Disabled: u32 = 0b0;

            /// 0b1: Deadtime is inserted between output 1 and output 2
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Output 1 Deadtime upon burst mode Idle entry
    pub mod DIDL1 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DIDL2::RW;
    }

    /// Output 1 Chopper enable
    pub mod CHP1 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHP2::RW;
    }

    /// Output 1 Fault state
    pub mod FAULT1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FAULT2::RW;
    }

    /// Output 1 Idle State
    pub mod IDLES1 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDLES2::RW;
    }

    /// Output 1 Idle mode
    pub mod IDLEM1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDLEM2::RW;
    }

    /// Output 1 polarity
    pub mod POL1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::POL2::RW;
    }
}

/// Timerx Fault Register
pub mod FLTAR {

    /// Fault sources Lock
    pub mod FLTLCK {
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

            /// 0b0: FLT1EN..FLT5EN bits are read/write
            pub const Unlocked: u32 = 0b0;

            /// 0b1: FLT1EN..FLT5EN bits are read only
            pub const Locked: u32 = 0b1;
        }
    }

    /// Fault 5 enable
    pub mod FLT5EN {
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

            /// 0b0: Fault input ignored
            pub const Ignored: u32 = 0b0;

            /// 0b1: Fault input is active and can disable HRTIM outputs
            pub const Active: u32 = 0b1;
        }
    }

    /// Fault 4 enable
    pub mod FLT4EN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT5EN::RW;
    }

    /// Fault 3 enable
    pub mod FLT3EN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT5EN::RW;
    }

    /// Fault 2 enable
    pub mod FLT2EN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT5EN::RW;
    }

    /// Fault 1 enable
    pub mod FLT1EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT5EN::RW;
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Timerx Control Register
    pub TIMACR: RWRegister<u32>,

    /// Timerx Interrupt Status Register
    pub TIMAISR: RORegister<u32>,

    /// Timerx Interrupt Clear Register
    pub TIMAICR: WORegister<u32>,

    /// TIMxDIER5
    pub TIMADIER: RWRegister<u32>,

    /// Timerx Counter Register
    pub CNTAR: RWRegister<u32>,

    /// Timerx Period Register
    pub PERAR: RWRegister<u32>,

    /// Timerx Repetition Register
    pub REPAR: RWRegister<u32>,

    /// Timerx Compare 1 Register
    pub CMP1AR: RWRegister<u32>,

    /// Timerx Compare 1 Compound Register
    pub CMP1CAR: RWRegister<u32>,

    /// Timerx Compare 2 Register
    pub CMP2AR: RWRegister<u32>,

    /// Timerx Compare 3 Register
    pub CMP3AR: RWRegister<u32>,

    /// Timerx Compare 4 Register
    pub CMP4AR: RWRegister<u32>,

    /// Timerx Capture 1 Register
    pub CPT1AR: RORegister<u32>,

    /// Timerx Capture 2 Register
    pub CPT2AR: RORegister<u32>,

    /// Timerx Deadtime Register
    pub DTAR: RWRegister<u32>,

    /// Timerx Output1 Set Register
    pub SETA1R: RWRegister<u32>,

    /// Timerx Output1 Reset Register
    pub RSTA1R: RWRegister<u32>,

    /// Timerx Output2 Set Register
    pub SETA2R: RWRegister<u32>,

    /// Timerx Output2 Reset Register
    pub RSTA2R: RWRegister<u32>,

    /// Timerx External Event Filtering Register 1
    pub EEFAR1: RWRegister<u32>,

    /// Timerx External Event Filtering Register 2
    pub EEFAR2: RWRegister<u32>,

    /// TimerA Reset Register
    pub RSTAR: RWRegister<u32>,

    /// Timerx Chopper Register
    pub CHPAR: RWRegister<u32>,

    /// Timerx Capture 2 Control Register
    pub CPT1ACR: RWRegister<u32>,

    /// CPT2xCR
    pub CPT2ACR: RWRegister<u32>,

    /// Timerx Output Register
    pub OUTAR: RWRegister<u32>,

    /// Timerx Fault Register
    pub FLTAR: RWRegister<u32>,
}
pub struct ResetValues {
    pub TIMACR: u32,
    pub TIMAISR: u32,
    pub TIMAICR: u32,
    pub TIMADIER: u32,
    pub CNTAR: u32,
    pub PERAR: u32,
    pub REPAR: u32,
    pub CMP1AR: u32,
    pub CMP1CAR: u32,
    pub CMP2AR: u32,
    pub CMP3AR: u32,
    pub CMP4AR: u32,
    pub CPT1AR: u32,
    pub CPT2AR: u32,
    pub DTAR: u32,
    pub SETA1R: u32,
    pub RSTA1R: u32,
    pub SETA2R: u32,
    pub RSTA2R: u32,
    pub EEFAR1: u32,
    pub EEFAR2: u32,
    pub RSTAR: u32,
    pub CHPAR: u32,
    pub CPT1ACR: u32,
    pub CPT2ACR: u32,
    pub OUTAR: u32,
    pub FLTAR: u32,
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

/// Access functions for the HRTIM_TIMA peripheral instance
pub mod HRTIM_TIMA {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40017480,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HRTIM_TIMA
    pub const reset: ResetValues = ResetValues {
        TIMACR: 0x00000000,
        TIMAISR: 0x00000000,
        TIMAICR: 0x00000000,
        TIMADIER: 0x00000000,
        CNTAR: 0x00000000,
        PERAR: 0x0000FFFF,
        REPAR: 0x00000000,
        CMP1AR: 0x00000000,
        CMP1CAR: 0x00000000,
        CMP2AR: 0x00000000,
        CMP3AR: 0x00000000,
        CMP4AR: 0x00000000,
        CPT1AR: 0x00000000,
        CPT2AR: 0x00000000,
        DTAR: 0x00000000,
        SETA1R: 0x00000000,
        RSTA1R: 0x00000000,
        SETA2R: 0x00000000,
        RSTA2R: 0x00000000,
        EEFAR1: 0x00000000,
        EEFAR2: 0x00000000,
        RSTAR: 0x00000000,
        CHPAR: 0x00000000,
        CPT1ACR: 0x00000000,
        CPT2ACR: 0x00000000,
        OUTAR: 0x00000000,
        FLTAR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HRTIM_TIMA_TAKEN: bool = false;

    /// Safe access to HRTIM_TIMA
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
            if HRTIM_TIMA_TAKEN {
                None
            } else {
                HRTIM_TIMA_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HRTIM_TIMA
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HRTIM_TIMA_TAKEN && inst.addr == INSTANCE.addr {
                HRTIM_TIMA_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HRTIM_TIMA
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HRTIM_TIMA_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HRTIM_TIMA
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HRTIM_TIMA: *const RegisterBlock = 0x40017480 as *const _;
