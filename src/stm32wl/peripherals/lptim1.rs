#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Low-power timer
//!
//! Used by: stm32wl5x_cm0p, stm32wl5x_cm4, stm32wle5

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// interrupt and status register
pub mod ISR {

    /// Repetition register update Ok
    pub mod REPOK {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: Repetition register update OK
            pub const Set: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPTIM update event occurred
    pub mod UE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: LPTIM update event occurred
            pub const Set: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Counter direction change up to down
    pub mod DOWN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: Counter direction change up to down
            pub const Set: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Counter direction change down to up
    pub mod UP {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: Counter direction change down to up
            pub const Set: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Autoreload register update OK
    pub mod ARROK {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: Autoreload register update OK
            pub const Set: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Compare register update OK
    pub mod CMPOK {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: Compare register update OK
            pub const Set: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External trigger edge event
    pub mod EXTTRIG {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: External trigger edge event
            pub const Set: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Autoreload match
    pub mod ARRM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: Autoreload match
            pub const Set: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Compare match
    pub mod CMPM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: Compare match
            pub const Set: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// interrupt clear register
pub mod ICR {

    /// Repetition register update OK clear flag
    pub mod REPOKCF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear REPOK flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update event clear flag
    pub mod UECF {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear update event flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Direction change to down Clear Flag
    pub mod DOWNCF {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Direction change to down Clear Flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Direction change to UP Clear Flag
    pub mod UPCF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Direction change to up Clear Flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Autoreload register update OK Clear Flag
    pub mod ARROKCF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Autoreload register update OK Clear Flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Compare register update OK Clear Flag
    pub mod CMPOKCF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Compare register update OK Clear Flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External trigger valid edge Clear Flag
    pub mod EXTTRIGCF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: External trigger valid edge Clear Flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Autoreload match Clear Flag
    pub mod ARRMCF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Autoreload match Clear Flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// compare match Clear Flag
    pub mod CMPMCF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Compare match Clear Flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// interrupt enable register
pub mod IER {

    /// Repetition register update OK interrupt Enable
    pub mod REPOKIE {
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

            /// 0b0: Repetition register update OK interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Repetition register update OK interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Update event interrupt enable
    pub mod UEIE {
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

            /// 0b0: Update event interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Update event interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Direction change to down Interrupt Enable
    pub mod DOWNIE {
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

            /// 0b0: DOWN interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DOWN interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Direction change to UP Interrupt Enable
    pub mod UPIE {
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

            /// 0b0: UP interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: UP interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Autoreload register update OK Interrupt Enable
    pub mod ARROKIE {
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

            /// 0b0: ARROK interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: ARROK interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Compare register update OK Interrupt Enable
    pub mod CMPOKIE {
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

            /// 0b0: CMPOK interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CMPOK interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// External trigger valid edge Interrupt Enable
    pub mod EXTTRIGIE {
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

            /// 0b0: EXTTRIG interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: EXTTRIG interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Autoreload match Interrupt Enable
    pub mod ARRMIE {
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

            /// 0b0: ARRM interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: ARRM interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Compare match Interrupt Enable
    pub mod CMPMIE {
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

            /// 0b0: CMPM interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CMPM interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// configuration register
pub mod CFGR {

    /// ENC
    pub mod ENC {
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

            /// 0b0: Encoder mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Encoder mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// COUNTMODE
    pub mod COUNTMODE {
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

            /// 0b0: The counter is incremented following each internal clock pulse
            pub const Internal: u32 = 0b0;

            /// 0b1: The counter is incremented following each valid clock pulse on the LPTIM external Input1
            pub const External: u32 = 0b1;
        }
    }

    /// PRELOAD
    pub mod PRELOAD {
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

            /// 0b0: Registers are updated after each APB bus write access
            pub const Immediate: u32 = 0b0;

            /// 0b1: Registers are updated at the end of the current LPTIM period
            pub const EndOfPeriod: u32 = 0b1;
        }
    }

    /// WAVPOL
    pub mod WAVPOL {
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

            /// 0b0: The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers
            pub const Positive: u32 = 0b0;

            /// 0b1: The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers
            pub const Negative: u32 = 0b1;
        }
    }

    /// WAVE
    pub mod WAVE {
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

            /// 0b0: Deactivate Set-once mode, PWM / One Pulse waveform (depending on OPMODE bit)
            pub const Inactive: u32 = 0b0;

            /// 0b1: Activate the Set-once mode
            pub const Active: u32 = 0b1;
        }
    }

    /// TIMOUT
    pub mod TIMOUT {
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

            /// 0b0: A trigger event arriving when the timer is already started will be ignored
            pub const Disabled: u32 = 0b0;

            /// 0b1: A trigger event arriving when the timer is already started will reset and restart the counter
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TRIGEN
    pub mod TRIGEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (2 bits: 0b11 << 17)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Software trigger (counting start is initiated by software)
            pub const SW: u32 = 0b00;

            /// 0b01: Rising edge is the active edge
            pub const RisingEdge: u32 = 0b01;

            /// 0b10: Falling edge is the active edge
            pub const FallingEdge: u32 = 0b10;

            /// 0b11: Both edges are active edges
            pub const BothEdges: u32 = 0b11;
        }
    }

    /// TRIGSEL
    pub mod TRIGSEL {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: lptim_ext_trig0
            pub const Trig0: u32 = 0b000;

            /// 0b001: lptim_ext_trig1
            pub const Trig1: u32 = 0b001;

            /// 0b010: lptim_ext_trig2
            pub const Trig2: u32 = 0b010;

            /// 0b011: lptim_ext_trig3
            pub const Trig3: u32 = 0b011;

            /// 0b100: lptim_ext_trig4
            pub const Trig4: u32 = 0b100;

            /// 0b101: lptim_ext_trig5
            pub const Trig5: u32 = 0b101;

            /// 0b110: lptim_ext_trig6
            pub const Trig6: u32 = 0b110;

            /// 0b111: lptim_ext_trig7
            pub const Trig7: u32 = 0b111;
        }
    }

    /// PRESC
    pub mod PRESC {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: /1
            pub const Div1: u32 = 0b000;

            /// 0b001: /2
            pub const Div2: u32 = 0b001;

            /// 0b010: /4
            pub const Div4: u32 = 0b010;

            /// 0b011: /8
            pub const Div8: u32 = 0b011;

            /// 0b100: /16
            pub const Div16: u32 = 0b100;

            /// 0b101: /32
            pub const Div32: u32 = 0b101;

            /// 0b110: /64
            pub const Div64: u32 = 0b110;

            /// 0b111: /128
            pub const Div128: u32 = 0b111;
        }
    }

    /// TRGFLT
    pub mod TRGFLT {
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

            /// 0b00: Any trigger active level change is considered as a valid trigger
            pub const Immediate: u32 = 0b00;

            /// 0b01: Trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger
            pub const Clocks2: u32 = 0b01;

            /// 0b10: Trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger
            pub const Clocks4: u32 = 0b10;

            /// 0b11: Trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger
            pub const Clocks8: u32 = 0b11;
        }
    }

    /// CKFLT
    pub mod CKFLT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Any external clock signal level change is considered as a valid transition
            pub const Immediate: u32 = 0b00;

            /// 0b01: External clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition
            pub const Clocks2: u32 = 0b01;

            /// 0b10: External clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition
            pub const Clocks4: u32 = 0b10;

            /// 0b11: External clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition
            pub const Clocks8: u32 = 0b11;
        }
    }

    /// CKPOL
    pub mod CKPOL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: The rising edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 1 is active.
            pub const RisingEdge: u32 = 0b00;

            /// 0b01: The falling edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 2 is active.
            pub const FallingEdge: u32 = 0b01;

            /// 0b10: Both edges are active edge. If LPTIM is in encoder mode: Encoder sub-mode 3 is active.
            pub const BothEdges: u32 = 0b10;
        }
    }

    /// CKSEL
    pub mod CKSEL {
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

            /// 0b0: LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)
            pub const Internal: u32 = 0b0;

            /// 0b1: LPTIM is clocked by an external clock source through the LPTIM external Input1
            pub const External: u32 = 0b1;
        }
    }
}

/// control register
pub mod CR {

    /// RSTARE
    pub mod RSTARE {
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

            /// 0b0: CNT Register reads do not trigger reset
            pub const Disabled: u32 = 0b0;

            /// 0b1: CNT Register reads trigger reset of LPTIM
            pub const Enabled: u32 = 0b1;
        }
    }

    /// COUNTRST
    pub mod COUNTRST {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Triggering of reset is possible
            pub const Idle: u32 = 0b0;

            /// 0b1: Reset in progress, do not write 1 to this field
            pub const Busy: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Trigger synchronous reset of CNT (3 LPTimer core clock cycles)
            pub const Reset: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CNTSTRT
    pub mod CNTSTRT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Timer start in Continuous mode
            pub const Start: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SNGSTRT
    pub mod SNGSTRT {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: LPTIM start in Single mode
            pub const Start: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENABLE
    pub mod ENABLE {
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

            /// 0b0: LPTIM is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: LPTIM is enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// compare register
pub mod CMP {

    /// CMP
    pub mod CMP {
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

/// autoreload register
pub mod ARR {

    /// Auto reload value
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

/// counter register
pub mod CNT {

    /// Counter value
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

/// option register
pub mod OR {

    /// Option register bit 1
    pub mod OR_1 {
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

            /// 0b0: LPTIM1 input 2 is connected to I/O
            pub const IO: u32 = 0b0;

            /// 0b1: LPTIM1 input 2 is connected to COMP2_OUT
            pub const COMP2_OUT: u32 = 0b1;
        }
    }

    /// Option register bit 0
    pub mod OR_0 {
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

            /// 0b0: LPTIM1 input 1 is connected to I/O
            pub const IO: u32 = 0b0;

            /// 0b1: LPTIM1 input 1 is connected to COMP1_OUT
            pub const COMP1_OUT: u32 = 0b1;
        }
    }
}

/// repetition register
pub mod RCR {

    /// Repetition register value
    pub mod REP {
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
    /// interrupt and status register
    pub ISR: RORegister<u32>,

    /// interrupt clear register
    pub ICR: WORegister<u32>,

    /// interrupt enable register
    pub IER: RWRegister<u32>,

    /// configuration register
    pub CFGR: RWRegister<u32>,

    /// control register
    pub CR: RWRegister<u32>,

    /// compare register
    pub CMP: RWRegister<u32>,

    /// autoreload register
    pub ARR: RWRegister<u32>,

    /// counter register
    pub CNT: RORegister<u32>,

    /// option register
    pub OR: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// repetition register
    pub RCR: RWRegister<u32>,
}
pub struct ResetValues {
    pub ISR: u32,
    pub ICR: u32,
    pub IER: u32,
    pub CFGR: u32,
    pub CR: u32,
    pub CMP: u32,
    pub ARR: u32,
    pub CNT: u32,
    pub OR: u32,
    pub RCR: u32,
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
