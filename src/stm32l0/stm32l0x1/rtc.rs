#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Real-time clock

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister, WORegister};

/// RTC time register
pub mod TR {

    /// AM/PM notation
    pub mod PM {
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

            /// 0b0: AM or 24-hour format
            pub const AM: u32 = 0b0;

            /// 0b1: PM
            pub const PM: u32 = 0b1;
        }
    }

    /// Hour tens in BCD format
    pub mod HT {
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

    /// Hour units in BCD format
    pub mod HU {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Minute tens in BCD format
    pub mod MNT {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Minute units in BCD format
    pub mod MNU {
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

    /// Second tens in BCD format
    pub mod ST {
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

    /// Second units in BCD format
    pub mod SU {
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

/// RTC date register
pub mod DR {

    /// Year tens in BCD format
    pub mod YT {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Year units in BCD format
    pub mod YU {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Week day units
    pub mod WDU {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Month tens in BCD format
    pub mod MT {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Month units in BCD format
    pub mod MU {
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

    /// Date tens in BCD format
    pub mod DT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Date units in BCD format
    pub mod DU {
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

/// RTC control register
pub mod CR {

    /// Calibration output enable
    pub mod COE {
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

            /// 0b0: Calibration output disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Calibration output enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Output selection
    pub mod OSEL {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (2 bits: 0b11 << 21)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Output disabled
            pub const Disabled: u32 = 0b00;

            /// 0b01: Alarm A output enabled
            pub const AlarmA: u32 = 0b01;

            /// 0b10: Alarm B output enabled
            pub const AlarmB: u32 = 0b10;

            /// 0b11: Wakeup output enabled
            pub const Wakeup: u32 = 0b11;
        }
    }

    /// Output polarity
    pub mod POL {
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

            /// 0b0: The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
            pub const High: u32 = 0b0;

            /// 0b1: The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
            pub const Low: u32 = 0b1;
        }
    }

    /// Calibration output selection
    pub mod COSEL {
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

            /// 0b0: Calibration output is 512 Hz (with default prescaler setting)
            pub const CalFreq_512Hz: u32 = 0b0;

            /// 0b1: Calibration output is 1 Hz (with default prescaler setting)
            pub const CalFreq_1Hz: u32 = 0b1;
        }
    }

    /// Backup
    pub mod BKP {
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

            /// 0b0: Daylight Saving Time change has not been performed
            pub const DST_Not_Changed: u32 = 0b0;

            /// 0b1: Daylight Saving Time change has been performed
            pub const DST_Changed: u32 = 0b1;
        }
    }

    /// Subtract 1 hour (winter time change)
    pub mod SUB1H {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode
            pub const Sub1: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Add 1 hour (summer time change)
    pub mod ADD1H {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Adds 1 hour to the current time. This can be used for summer time change outside initialization mode
            pub const Add1: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Time-stamp interrupt enable
    pub mod TSIE {
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

            /// 0b0: Time-stamp Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Time-stamp Interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Wakeup timer interrupt enable
    pub mod WUTIE {
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

            /// 0b0: Wakeup timer interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Wakeup timer interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Alarm B interrupt enable
    pub mod ALRBIE {
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

            /// 0b0: Alarm B Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Alarm B Interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Alarm A interrupt enable
    pub mod ALRAIE {
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

            /// 0b0: Alarm A interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Alarm A interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// timestamp enable
    pub mod TSE {
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

            /// 0b0: Timestamp disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Timestamp enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Wakeup timer enable
    pub mod WUTE {
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

            /// 0b0: Wakeup timer disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Wakeup timer enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Alarm B enable
    pub mod ALRBE {
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

            /// 0b0: Alarm B disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Alarm B enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Alarm A enable
    pub mod ALRAE {
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

            /// 0b0: Alarm A disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Alarm A enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Hour format
    pub mod FMT {
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

            /// 0b0: 24 hour/day format
            pub const Twenty_Four_Hour: u32 = 0b0;

            /// 0b1: AM/PM hour format
            pub const AM_PM: u32 = 0b1;
        }
    }

    /// Bypass the shadow registers
    pub mod BYPSHAD {
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

            /// 0b0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles
            pub const ShadowReg: u32 = 0b0;

            /// 0b1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters
            pub const BypassShadowReg: u32 = 0b1;
        }
    }

    /// RTC_REFIN reference clock detection enable (50 or 60 Hz)
    pub mod REFCKON {
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

            /// 0b0: RTC_REFIN detection disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: RTC_REFIN detection enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Time-stamp event active edge
    pub mod TSEDGE {
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

            /// 0b0: RTC_TS input rising edge generates a time-stamp event
            pub const RisingEdge: u32 = 0b0;

            /// 0b1: RTC_TS input falling edge generates a time-stamp event
            pub const FallingEdge: u32 = 0b1;
        }
    }

    /// Wakeup clock selection
    pub mod WUCKSEL {
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

            /// 0b000: RTC/16 clock is selected
            pub const Div16: u32 = 0b000;

            /// 0b001: RTC/8 clock is selected
            pub const Div8: u32 = 0b001;

            /// 0b010: RTC/4 clock is selected
            pub const Div4: u32 = 0b010;

            /// 0b011: RTC/2 clock is selected
            pub const Div2: u32 = 0b011;

            /// 0b100: ck_spre (usually 1 Hz) clock is selected
            pub const ClockSpare: u32 = 0b100;

            /// 0b110: ck_spre (usually 1 Hz) clock is selected and 216 is added to the WUT counter value
            pub const ClockSpareWithOffset: u32 = 0b110;
        }
    }
}

/// RTC initialization and status register
pub mod ISR {

    /// RTC_TAMP2 detection flag
    pub mod TAMP2F {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: This flag is set by hardware when a tamper detection event is detected on the RTC_TAMPx input
            pub const Tampered: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: Flag cleared by software writing 0
            pub const Clear: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RTC_TAMP1 detection flag
    pub mod TAMP1F {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        pub use super::TAMP2F::R;
        pub use super::TAMP2F::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Time-stamp overflow flag
    pub mod TSOVF {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: This flag is set by hardware when a time-stamp event occurs while TSF is already set
            pub const Overflow: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: This flag is cleared by software by writing 0
            pub const Clear: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Time-stamp flag
    pub mod TSF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: This flag is set by hardware when a time-stamp event occurs
            pub const TimestampEvent: u32 = 0b1;
        }
        pub use super::TSOVF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup timer flag
    pub mod WUTF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: This flag is set by hardware when the wakeup auto-reload counter reaches 0
            pub const Zero: u32 = 0b1;
        }
        pub use super::TSOVF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Alarm B flag
    pub mod ALRBF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMBR)
            pub const Match: u32 = 0b1;
        }
        pub use super::TSOVF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Alarm A flag
    pub mod ALRAF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)
            pub const Match: u32 = 0b1;
        }
        pub use super::TSOVF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Initialization mode
    pub mod INIT {
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

            /// 0b0: Free running mode
            pub const FreeRunningMode: u32 = 0b0;

            /// 0b1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
            pub const InitMode: u32 = 0b1;
        }
    }

    /// Initialization flag
    pub mod INITF {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Calendar registers update is not allowed
            pub const NotAllowed: u32 = 0b0;

            /// 0b1: Calendar registers update is allowed
            pub const Allowed: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Registers synchronization flag
    pub mod RSF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Calendar shadow registers not yet synchronized
            pub const NotSynced: u32 = 0b0;

            /// 0b1: Calendar shadow registers synchronized
            pub const Synced: u32 = 0b1;
        }
        pub use super::TSOVF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Initialization status flag
    pub mod INITS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Calendar has not been initialized
            pub const NotInitalized: u32 = 0b0;

            /// 0b1: Calendar has been initialized
            pub const Initalized: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Shift operation pending
    pub mod SHPF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No shift operation is pending
            pub const NoShiftPending: u32 = 0b0;

            /// 0b1: A shift operation is pending
            pub const ShiftPending: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup timer write flag
    pub mod WUTWF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Wakeup timer configuration update not allowed
            pub const UpdateNotAllowed: u32 = 0b0;

            /// 0b1: Wakeup timer configuration update allowed
            pub const UpdateAllowed: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Alarm B write flag
    pub mod ALRBWF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Alarm update not allowed
            pub const UpdateNotAllowed: u32 = 0b0;

            /// 0b1: Alarm update allowed
            pub const UpdateAllowed: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Alarm A write flag
    pub mod ALRAWF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        pub use super::ALRBWF::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Recalibration pending flag
    pub mod RECALPF {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0
            pub const Pending: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RTC_TAMP3 detection flag
    pub mod TAMP3F {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        pub use super::TAMP2F::R;
        pub use super::TAMP2F::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RTC prescaler register
pub mod PRER {

    /// Asynchronous prescaler factor
    pub mod PREDIV_A {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (7 bits: 0x7f << 16)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Synchronous prescaler factor
    pub mod PREDIV_S {
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

/// RTC wakeup timer register
pub mod WUTR {

    /// Wakeup auto-reload value bits
    pub mod WUT {
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

/// RTC alarm A register
pub mod ALRMAR {

    /// Alarm A date mask
    pub mod MSK4 {
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

            /// 0b0: Alarm set if the date/day match
            pub const Mask: u32 = 0b0;

            /// 0b1: Date/day don’t care in Alarm comparison
            pub const NotMask: u32 = 0b1;
        }
    }

    /// Week day selection
    pub mod WDSEL {
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

            /// 0b0: DU\[3:0\] represents the date units
            pub const DateUnits: u32 = 0b0;

            /// 0b1: DU\[3:0\] represents the week day. DT\[1:0\] is don’t care.
            pub const WeekDay: u32 = 0b1;
        }
    }

    /// Date tens in BCD format.
    pub mod DT {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Date units or day in BCD format.
    pub mod DU {
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

    /// Alarm A hours mask
    pub mod MSK3 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MSK4::RW;
    }

    /// AM/PM notation
    pub mod PM {
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

            /// 0b0: AM or 24-hour format
            pub const AM: u32 = 0b0;

            /// 0b1: PM
            pub const PM: u32 = 0b1;
        }
    }

    /// Hour tens in BCD format.
    pub mod HT {
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

    /// Hour units in BCD format.
    pub mod HU {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Alarm A minutes mask
    pub mod MSK2 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MSK4::RW;
    }

    /// Minute tens in BCD format.
    pub mod MNT {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Minute units in BCD format.
    pub mod MNU {
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

    /// Alarm A seconds mask
    pub mod MSK1 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MSK4::RW;
    }

    /// Second tens in BCD format.
    pub mod ST {
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

    /// Second units in BCD format.
    pub mod SU {
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

/// RTC alarm B register
pub mod ALRMBR {
    pub use super::ALRMAR::DT;
    pub use super::ALRMAR::DU;
    pub use super::ALRMAR::HT;
    pub use super::ALRMAR::HU;
    pub use super::ALRMAR::MNT;
    pub use super::ALRMAR::MNU;
    pub use super::ALRMAR::MSK1;
    pub use super::ALRMAR::MSK2;
    pub use super::ALRMAR::MSK3;
    pub use super::ALRMAR::MSK4;
    pub use super::ALRMAR::PM;
    pub use super::ALRMAR::ST;
    pub use super::ALRMAR::SU;
    pub use super::ALRMAR::WDSEL;
}

/// write protection register
pub mod WPR {

    /// Write protection key
    pub mod KEY {
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

/// RTC sub second register
pub mod SSR {

    /// Sub second value
    pub mod SS {
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

/// RTC shift control register
pub mod SHIFTR {

    /// Add one second
    pub mod ADD1S {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Add one second to the clock/calendar
            pub const Add1: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Subtract a fraction of a second
    pub mod SUBFS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (15 bits: 0x7fff << 0)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RTC timestamp time register
pub mod TSTR {

    /// AM/PM notation
    pub mod PM {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Hour tens in BCD format.
    pub mod HT {
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

    /// Hour units in BCD format.
    pub mod HU {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Minute tens in BCD format.
    pub mod MNT {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Minute units in BCD format.
    pub mod MNU {
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

    /// Second tens in BCD format.
    pub mod ST {
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

    /// Second units in BCD format.
    pub mod SU {
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

/// RTC timestamp date register
pub mod TSDR {

    /// Week day units
    pub mod WDU {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Month tens in BCD format
    pub mod MT {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Month units in BCD format
    pub mod MU {
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

    /// Date tens in BCD format
    pub mod DT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Date units in BCD format
    pub mod DU {
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

/// RTC time-stamp sub second register
pub mod TSSSR {
    pub use super::SSR::SS;
}

/// RTC calibration register
pub mod CALR {

    /// Increase frequency of RTC by 488.5 ppm
    pub mod CALP {
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

            /// 0b0: No RTCCLK pulses are added
            pub const NoChange: u32 = 0b0;

            /// 0b1: One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)
            pub const IncreaseFreq: u32 = 0b1;
        }
    }

    /// Use an 8-second calibration cycle period
    pub mod CALW8 {
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

            /// 0b1: When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected
            pub const Eight_Second: u32 = 0b1;
        }
    }

    /// Use a 16-second calibration cycle period
    pub mod CALW16 {
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

            /// 0b1: When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1
            pub const Sixteen_Second: u32 = 0b1;
        }
    }

    /// Calibration minus
    pub mod CALM {
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

/// RTC tamper configuration register
pub mod TAMPCR {

    /// Tamper 2 mask flag
    pub mod TAMP2MF {
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

            /// 0b0: Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection
            pub const NotMasked: u32 = 0b0;

            /// 0b1: Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased.
            pub const Masked: u32 = 0b1;
        }
    }

    /// Tamper 2 no erase
    pub mod TAMP2NOERASE {
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

            /// 0b0: Tamper x event erases the backup registers
            pub const Erase: u32 = 0b0;

            /// 0b1: Tamper x event does not erase the backup registers
            pub const NoErase: u32 = 0b1;
        }
    }

    /// Tamper 2 interrupt enable
    pub mod TAMP2IE {
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

            /// 0b0: Tamper x interrupt is disabled if TAMPIE = 0
            pub const Disabled: u32 = 0b0;

            /// 0b1: Tamper x interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Tamper 1 mask flag
    pub mod TAMP1MF {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP2MF::RW;
    }

    /// Tamper 1 no erase
    pub mod TAMP1NOERASE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP2NOERASE::RW;
    }

    /// Tamper 1 interrupt enable
    pub mod TAMP1IE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP2IE::RW;
    }

    /// RTC_TAMPx pull-up disable
    pub mod TAMPPUDIS {
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

            /// 0b0: Precharge RTC_TAMPx pins before sampling (enable internal pull-up)
            pub const Enabled: u32 = 0b0;

            /// 0b1: Disable precharge of RTC_TAMPx pins
            pub const Disabled: u32 = 0b1;
        }
    }

    /// RTC_TAMPx precharge duration
    pub mod TAMPPRCH {
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

            /// 0b00: 1 RTCCLK cycle
            pub const RTCCLK_1_cycle: u32 = 0b00;

            /// 0b01: 2 RTCCLK cycles
            pub const RTCCLK_2_cycle: u32 = 0b01;

            /// 0b10: 4 RTCCLK cycles
            pub const RTCCLK_4_cycle: u32 = 0b10;

            /// 0b11: 8 RTCCLK cycles
            pub const RTCCLK_8_cycle: u32 = 0b11;
        }
    }

    /// RTC_TAMPx filter count
    pub mod TAMPFLT {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)
            pub const Immediate: u32 = 0b00;

            /// 0b01: Tamper event is activated after 2 consecutive samples at the active level
            pub const Two_Samples: u32 = 0b01;

            /// 0b10: Tamper event is activated after 4 consecutive samples at the active level
            pub const Four_Samples: u32 = 0b10;

            /// 0b11: Tamper event is activated after 8 consecutive samples at the active level
            pub const Eight_Samples: u32 = 0b11;
        }
    }

    /// Tamper sampling frequency
    pub mod TAMPFREQ {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
            pub const Div32768: u32 = 0b000;

            /// 0b001: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
            pub const Div16384: u32 = 0b001;

            /// 0b010: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
            pub const Div8192: u32 = 0b010;

            /// 0b011: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
            pub const Div4096: u32 = 0b011;

            /// 0b100: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
            pub const Div2048: u32 = 0b100;

            /// 0b101: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
            pub const Div1024: u32 = 0b101;

            /// 0b110: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
            pub const Div512: u32 = 0b110;

            /// 0b111: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
            pub const Div256: u32 = 0b111;
        }
    }

    /// Activate timestamp on tamper detection event
    pub mod TAMPTS {
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

            /// 0b0: Tamper detection event does not cause a timestamp to be saved
            pub const NoSave: u32 = 0b0;

            /// 0b1: Save timestamp on tamper detection event
            pub const Save: u32 = 0b1;
        }
    }

    /// Active level for RTC_TAMP2 input
    pub mod TAMP2TRG {
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

            /// 0b0: If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT ≠ 00: RTC_TAMPx input staying low triggers a tamper detection event.
            pub const RisingEdge: u32 = 0b0;

            /// 0b1: If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT ≠ 00: RTC_TAMPx input falling edge triggers a tamper detection event
            pub const FallingEdge: u32 = 0b1;
        }
    }

    /// RTC_TAMP2 input detection enable
    pub mod TAMP2E {
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

            /// 0b0: RTC_TAMPx input detection disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: RTC_TAMPx input detection enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Tamper interrupt enable
    pub mod TAMPIE {
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

            /// 0b0: Tamper interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Tamper interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Active level for RTC_TAMP1 input
    pub mod TAMP1TRG {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP2TRG::RW;
    }

    /// RTC_TAMP1 input detection enable
    pub mod TAMP1E {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP2E::RW;
    }

    /// Tamper 3 mask flag
    pub mod TAMP3MF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP2MF::RW;
    }

    /// Tamper 3 no erase
    pub mod TAMP3NOERASE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP2NOERASE::RW;
    }

    /// Tamper 3 interrupt enable
    pub mod TAMP3IE {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP2IE::RW;
    }

    /// Active level for RTC_TAMP3 input
    pub mod TAMP3TRG {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP2TRG::RW;
    }

    /// RTC_TAMP3 detection enable
    pub mod TAMP3E {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP2E::RW;
    }
}

/// RTC alarm A sub second register
pub mod ALRMASSR {

    /// Mask the most-significant bits starting at this bit
    pub mod MASKSS {
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

    /// Sub seconds value
    pub mod SS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (15 bits: 0x7fff << 0)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RTC alarm B sub second register
pub mod ALRMBSSR {
    pub use super::ALRMASSR::MASKSS;
    pub use super::ALRMASSR::SS;
}

/// option register
pub mod OR {

    /// RTC_ALARM on PC13 output type
    pub mod RTC_OUT_RMP {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RTC_ALARM on PC13 output type
    pub mod RTC_ALARM_TYPE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RTC backup registers
pub mod BKP0R {

    /// BKP
    pub mod BKP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RTC backup registers
pub mod BKP1R {
    pub use super::BKP0R::BKP;
}

/// RTC backup registers
pub mod BKP2R {
    pub use super::BKP0R::BKP;
}

/// RTC backup registers
pub mod BKP3R {
    pub use super::BKP0R::BKP;
}

/// RTC backup registers
pub mod BKP4R {
    pub use super::BKP0R::BKP;
}
pub struct RegisterBlock {
    /// RTC time register
    pub TR: RWRegister<u32>,

    /// RTC date register
    pub DR: RWRegister<u32>,

    /// RTC control register
    pub CR: RWRegister<u32>,

    /// RTC initialization and status register
    pub ISR: RWRegister<u32>,

    /// RTC prescaler register
    pub PRER: RWRegister<u32>,

    /// RTC wakeup timer register
    pub WUTR: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// RTC alarm A register
    pub ALRMAR: RWRegister<u32>,

    /// RTC alarm B register
    pub ALRMBR: RWRegister<u32>,

    /// write protection register
    pub WPR: WORegister<u32>,

    /// RTC sub second register
    pub SSR: RORegister<u32>,

    /// RTC shift control register
    pub SHIFTR: WORegister<u32>,

    /// RTC timestamp time register
    pub TSTR: RORegister<u32>,

    /// RTC timestamp date register
    pub TSDR: RORegister<u32>,

    /// RTC time-stamp sub second register
    pub TSSSR: RORegister<u32>,

    /// RTC calibration register
    pub CALR: RWRegister<u32>,

    /// RTC tamper configuration register
    pub TAMPCR: RWRegister<u32>,

    /// RTC alarm A sub second register
    pub ALRMASSR: RWRegister<u32>,

    /// RTC alarm B sub second register
    pub ALRMBSSR: RWRegister<u32>,

    /// option register
    pub OR: RWRegister<u32>,

    /// RTC backup registers
    pub BKP0R: RWRegister<u32>,

    /// RTC backup registers
    pub BKP1R: RWRegister<u32>,

    /// RTC backup registers
    pub BKP2R: RWRegister<u32>,

    /// RTC backup registers
    pub BKP3R: RWRegister<u32>,

    /// RTC backup registers
    pub BKP4R: RWRegister<u32>,
}
pub struct ResetValues {
    pub TR: u32,
    pub DR: u32,
    pub CR: u32,
    pub ISR: u32,
    pub PRER: u32,
    pub WUTR: u32,
    pub ALRMAR: u32,
    pub ALRMBR: u32,
    pub WPR: u32,
    pub SSR: u32,
    pub SHIFTR: u32,
    pub TSTR: u32,
    pub TSDR: u32,
    pub TSSSR: u32,
    pub CALR: u32,
    pub TAMPCR: u32,
    pub ALRMASSR: u32,
    pub ALRMBSSR: u32,
    pub OR: u32,
    pub BKP0R: u32,
    pub BKP1R: u32,
    pub BKP2R: u32,
    pub BKP3R: u32,
    pub BKP4R: u32,
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

/// Access functions for the RTC peripheral instance
pub mod RTC {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40002800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RTC
    pub const reset: ResetValues = ResetValues {
        TR: 0x00000000,
        DR: 0x00000000,
        CR: 0x00000000,
        ISR: 0x00000000,
        PRER: 0x00000000,
        WUTR: 0x00000000,
        ALRMAR: 0x00000000,
        ALRMBR: 0x00000000,
        WPR: 0x00000000,
        SSR: 0x00000000,
        SHIFTR: 0x00000000,
        TSTR: 0x00000000,
        TSDR: 0x00000000,
        TSSSR: 0x00000000,
        CALR: 0x00000000,
        TAMPCR: 0x00000000,
        ALRMASSR: 0x00000000,
        ALRMBSSR: 0x00000000,
        OR: 0x00000000,
        BKP0R: 0x00000000,
        BKP1R: 0x00000000,
        BKP2R: 0x00000000,
        BKP3R: 0x00000000,
        BKP4R: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RTC_TAKEN: bool = false;

    /// Safe access to RTC
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
            if RTC_TAKEN {
                None
            } else {
                RTC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RTC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RTC_TAKEN && inst.addr == INSTANCE.addr {
                RTC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to RTC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RTC: *const RegisterBlock = 0x40002800 as *const _;
