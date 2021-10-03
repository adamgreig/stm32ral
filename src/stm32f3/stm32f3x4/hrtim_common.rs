#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! High Resolution Timer: Common functions

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Control Register 1
pub mod CR1 {

    /// ADC Trigger 4 Update Source
    pub mod AD4USRC {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (3 bits: 0b111 << 25)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: ADC trigger update from master timer
            pub const Master: u32 = 0b000;

            /// 0b001: ADC trigger update from timer A
            pub const TimerA: u32 = 0b001;

            /// 0b010: ADC trigger update from timer B
            pub const TimerB: u32 = 0b010;

            /// 0b011: ADC trigger update from timer C
            pub const TimerC: u32 = 0b011;

            /// 0b100: ADC trigger update from timer D
            pub const TimerD: u32 = 0b100;

            /// 0b101: ADC trigger update from timer E
            pub const TimerE: u32 = 0b101;
        }
    }

    /// ADC Trigger 3 Update Source
    pub mod AD3USRC {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (3 bits: 0b111 << 22)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD4USRC::RW;
    }

    /// ADC Trigger 2 Update Source
    pub mod AD2USRC {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (3 bits: 0b111 << 19)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD4USRC::RW;
    }

    /// ADC Trigger 1 Update Source
    pub mod AD1USRC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD4USRC::RW;
    }

    /// Timer E Update Disable
    pub mod TEUDIS {
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

            /// 0b0: Timer update enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: Timer update disabled
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Timer D Update Disable
    pub mod TDUDIS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEUDIS::RW;
    }

    /// Timer C Update Disable
    pub mod TCUDIS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEUDIS::RW;
    }

    /// Timer B Update Disable
    pub mod TBUDIS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEUDIS::RW;
    }

    /// Timer A Update Disable
    pub mod TAUDIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEUDIS::RW;
    }

    /// Master Update Disable
    pub mod MUDIS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEUDIS::RW;
    }
}

/// Control Register 2
pub mod CR2 {

    /// Timer E counter software reset
    pub mod TERST {
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

            /// 0b1: Reset timer
            pub const Reset: u32 = 0b1;
        }
    }

    /// Timer D counter software reset
    pub mod TDRST {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TERST::RW;
    }

    /// Timer C counter software reset
    pub mod TCRST {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TERST::RW;
    }

    /// Timer B counter software reset
    pub mod TBRST {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TERST::RW;
    }

    /// Timer A counter software reset
    pub mod TARST {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TERST::RW;
    }

    /// Master Counter software reset
    pub mod MRST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TERST::RW;
    }

    /// Timer E Software Update
    pub mod TESWU {
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

            /// 0b1: Force immediate update
            pub const Update: u32 = 0b1;
        }
    }

    /// Timer D Software Update
    pub mod TDSWU {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TESWU::RW;
    }

    /// Timer C Software Update
    pub mod TCSWU {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TESWU::RW;
    }

    /// Timer B Software Update
    pub mod TBSWU {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TESWU::RW;
    }

    /// Timer A Software update
    pub mod TASWU {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TESWU::RW;
    }

    /// Master Timer Software update
    pub mod MSWU {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TESWU::RW;
    }
}

/// Interrupt Status Register
pub mod ISR {

    /// Burst mode Period Interrupt Flag
    pub mod BMPER {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No burst mode period interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Burst mode period interrupt occured
            pub const Event: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear burst mode period interrupt
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DLL Ready Interrupt Flag
    pub mod DLLRDY {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No DLL calibration ready interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: DLL calibration ready interrupt occurred
            pub const Event: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear DLL calibration interrupt
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// System Fault Interrupt Flag
    pub mod SYSFLT {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No fault interrupt occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Fault interrupt occurred
            pub const Event: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear fault interrupt
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fault 5 Interrupt Flag
    pub mod FLT5 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::SYSFLT::R;
        pub use super::SYSFLT::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fault 4 Interrupt Flag
    pub mod FLT4 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::SYSFLT::R;
        pub use super::SYSFLT::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fault 3 Interrupt Flag
    pub mod FLT3 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::SYSFLT::R;
        pub use super::SYSFLT::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fault 2 Interrupt Flag
    pub mod FLT2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::SYSFLT::R;
        pub use super::SYSFLT::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fault 1 Interrupt Flag
    pub mod FLT1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        pub use super::SYSFLT::R;
        pub use super::SYSFLT::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Clear Register
pub mod ICR {

    /// Burst mode period flag Clear
    pub mod BMPERC {
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

            /// 0b1: Clears BMPER flag
            pub const Clear: u32 = 0b1;
        }
    }

    /// DLL Ready Interrupt flag Clear
    pub mod DLLRDYC {
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

            /// 0b1: Clears DLL ready flag
            pub const Clear: u32 = 0b1;
        }
    }

    /// System Fault Interrupt Flag Clear
    pub mod SYSFLTC {
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

            /// 0b1: Clears SYSFLT flag
            pub const Clear: u32 = 0b1;
        }
    }

    /// Fault 5 Interrupt Flag Clear
    pub mod FLT5C {
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

            /// 0b1: Clears FLTx flag
            pub const Clear: u32 = 0b1;
        }
    }

    /// Fault 4 Interrupt Flag Clear
    pub mod FLT4C {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT5C::RW;
    }

    /// Fault 3 Interrupt Flag Clear
    pub mod FLT3C {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT5C::RW;
    }

    /// Fault 2 Interrupt Flag Clear
    pub mod FLT2C {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT5C::RW;
    }

    /// Fault 1 Interrupt Flag Clear
    pub mod FLT1C {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT5C::RW;
    }
}

/// Interrupt Enable Register
pub mod IER {

    /// Burst mode period Interrupt Enable
    pub mod BMPERIE {
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

            /// 0b0: Burst mode period interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Burst mode period interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DLL Ready Interrupt Enable
    pub mod DLLRDYIE {
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

            /// 0b0: DLL ready interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DLL Ready interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// System Fault Interrupt Enable
    pub mod SYSFLTIE {
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

            /// 0b0: Fault interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Fault interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Fault 5 Interrupt Enable
    pub mod FLT5IE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSFLTIE::RW;
    }

    /// Fault 4 Interrupt Enable
    pub mod FLT4IE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSFLTIE::RW;
    }

    /// Fault 3 Interrupt Enable
    pub mod FLT3IE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSFLTIE::RW;
    }

    /// Fault 2 Interrupt Enable
    pub mod FLT2IE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSFLTIE::RW;
    }

    /// Fault 1 Interrupt Enable
    pub mod FLT1IE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYSFLTIE::RW;
    }
}

/// Output Enable Register
pub mod OENR {

    /// Timer E Output 2 Enable
    pub mod TE2OEN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Output disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Output enabled
            pub const Enabled: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Enable output
            pub const Enable: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer E Output 1 Enable
    pub mod TE1OEN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        pub use super::TE2OEN::R;
        pub use super::TE2OEN::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer D Output 2 Enable
    pub mod TD2OEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        pub use super::TE2OEN::R;
        pub use super::TE2OEN::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer D Output 1 Enable
    pub mod TD1OEN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        pub use super::TE2OEN::R;
        pub use super::TE2OEN::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer C Output 2 Enable
    pub mod TC2OEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        pub use super::TE2OEN::R;
        pub use super::TE2OEN::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer C Output 1 Enable
    pub mod TC1OEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        pub use super::TE2OEN::R;
        pub use super::TE2OEN::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer B Output 2 Enable
    pub mod TB2OEN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        pub use super::TE2OEN::R;
        pub use super::TE2OEN::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer B Output 1 Enable
    pub mod TB1OEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        pub use super::TE2OEN::R;
        pub use super::TE2OEN::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer A Output 2 Enable
    pub mod TA2OEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        pub use super::TE2OEN::R;
        pub use super::TE2OEN::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer A Output 1 Enable
    pub mod TA1OEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        pub use super::TE2OEN::R;
        pub use super::TE2OEN::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DISR
pub mod ODISR {

    /// TE2ODIS
    pub mod TE2ODIS {
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

            /// 0b1: Disable output
            pub const Disable: u32 = 0b1;
        }
    }

    /// TE1ODIS
    pub mod TE1ODIS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODIS::RW;
    }

    /// TD2ODIS
    pub mod TD2ODIS {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODIS::RW;
    }

    /// TD1ODIS
    pub mod TD1ODIS {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODIS::RW;
    }

    /// TC2ODIS
    pub mod TC2ODIS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODIS::RW;
    }

    /// TC1ODIS
    pub mod TC1ODIS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODIS::RW;
    }

    /// TB2ODIS
    pub mod TB2ODIS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODIS::RW;
    }

    /// TB1ODIS
    pub mod TB1ODIS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODIS::RW;
    }

    /// TA2ODIS
    pub mod TA2ODIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODIS::RW;
    }

    /// TA1ODIS
    pub mod TA1ODIS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODIS::RW;
    }
}

/// Output Disable Status Register
pub mod ODSR {

    /// Timer E Output 2 disable status
    pub mod TE2ODS {
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

            /// 0b0: Output disabled in idle state
            pub const Idle: u32 = 0b0;

            /// 0b1: Output disabled in fault state
            pub const Fault: u32 = 0b1;
        }
    }

    /// Timer E Output 1 disable status
    pub mod TE1ODS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODS::RW;
    }

    /// Timer D Output 2 disable status
    pub mod TD2ODS {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODS::RW;
    }

    /// Timer D Output 1 disable status
    pub mod TD1ODS {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODS::RW;
    }

    /// Timer C Output 2 disable status
    pub mod TC2ODS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODS::RW;
    }

    /// Timer C Output 1 disable status
    pub mod TC1ODS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODS::RW;
    }

    /// Timer B Output 2 disable status
    pub mod TB2ODS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODS::RW;
    }

    /// Timer B Output 1 disable status
    pub mod TB1ODS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODS::RW;
    }

    /// Timer A Output 2 disable status
    pub mod TA2ODS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODS::RW;
    }

    /// Timer A Output 1 disable status
    pub mod TA1ODS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TE2ODS::RW;
    }
}

/// Burst Mode Control Register
pub mod BMCR {

    /// Burst Mode Status
    pub mod BMSTAT {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Normal operation
            pub const Normal: u32 = 0b0;

            /// 0b1: Burst operation ongoing
            pub const Burst: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: Terminate burst mode
            pub const Cancel: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer E Burst Mode
    pub mod TEBM {
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

            /// 0b0: Counter clock is maintained and timer operates normally
            pub const Normal: u32 = 0b0;

            /// 0b1: Counter clock is stopped and counter is reset
            pub const Stopped: u32 = 0b1;
        }
    }

    /// Timer D Burst Mode
    pub mod TDBM {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEBM::RW;
    }

    /// Timer C Burst Mode
    pub mod TCBM {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEBM::RW;
    }

    /// Timer B Burst Mode
    pub mod TBBM {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEBM::RW;
    }

    /// Timer A Burst Mode
    pub mod TABM {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEBM::RW;
    }

    /// Master Timer Burst Mode
    pub mod MTBM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEBM::RW;
    }

    /// Burst Mode Preload Enable
    pub mod BMPREN {
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

            /// 0b0: Preload disabled: the write access is directly done into active registers
            pub const Disabled: u32 = 0b0;

            /// 0b1: Preload enabled: the write access is done into preload registers
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Burst Mode Prescaler
    pub mod BMPRSC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (4 bits: 0b1111 << 6)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Clock not divided
            pub const Div1: u32 = 0b0000;

            /// 0b0001: Division by 2
            pub const Div2: u32 = 0b0001;

            /// 0b0010: Division by 4
            pub const Div4: u32 = 0b0010;

            /// 0b0011: Division by 8
            pub const Div8: u32 = 0b0011;

            /// 0b0100: Division by 16
            pub const Div16: u32 = 0b0100;

            /// 0b0101: Division by 32
            pub const Div32: u32 = 0b0101;

            /// 0b0110: Division by 64
            pub const Div64: u32 = 0b0110;

            /// 0b0111: Division by 128
            pub const Div128: u32 = 0b0111;

            /// 0b1000: Division by 256
            pub const Div256: u32 = 0b1000;

            /// 0b1001: Division by 512
            pub const Div512: u32 = 0b1001;

            /// 0b1010: Division by 1024
            pub const Div1024: u32 = 0b1010;

            /// 0b1011: Division by 2048
            pub const Div2048: u32 = 0b1011;

            /// 0b1100: Division by 4096
            pub const Div4096: u32 = 0b1100;

            /// 0b1101: Division by 8192
            pub const Div8192: u32 = 0b1101;

            /// 0b1110: Division by 16384
            pub const Div16384: u32 = 0b1110;

            /// 0b1111: Division by 32768
            pub const Div32768: u32 = 0b1111;
        }
    }

    /// Burst Mode Clock source
    pub mod BMCLK {
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

            /// 0b0000: Master timer reset/roll-over
            pub const Master: u32 = 0b0000;

            /// 0b0001: Timer A counter reset/roll-over
            pub const TimerA: u32 = 0b0001;

            /// 0b0010: Timer B counter reset/roll-over
            pub const TimerB: u32 = 0b0010;

            /// 0b0011: Timer C counter reset/roll-over
            pub const TimerC: u32 = 0b0011;

            /// 0b0100: Timer D counter reset/roll-over
            pub const TimerD: u32 = 0b0100;

            /// 0b0101: Timer E counter reset/roll-over
            pub const TimerE: u32 = 0b0101;

            /// 0b0110: On-chip Event 1 (BMClk\[1\]), acting as a burst mode counter clock
            pub const Event1: u32 = 0b0110;

            /// 0b0111: On-chip Event 2 (BMClk\[2\]), acting as a burst mode counter clock
            pub const Event2: u32 = 0b0111;

            /// 0b1000: On-chip Event 3 (BMClk\[3\]), acting as a burst mode counter clock
            pub const Event3: u32 = 0b1000;

            /// 0b1001: On-chip Event 4 (BMClk\[4\]), acting as a burst mode counter clock
            pub const Event4: u32 = 0b1001;

            /// 0b1010: Prescaled f_HRTIM clock (as per BMPRSC\[3:0\] setting
            pub const Clock: u32 = 0b1010;
        }
    }

    /// Burst Mode operating mode
    pub mod BMOM {
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

            /// 0b0: Single-shot mode
            pub const SingleShot: u32 = 0b0;

            /// 0b1: Continuous operation
            pub const Continuous: u32 = 0b1;
        }
    }

    /// Burst Mode enable
    pub mod BME {
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

            /// 0b0: Burst mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Burst mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// BMTRGR
pub mod BMTRGR {

    /// OCHPEV
    pub mod OCHPEV {
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

            /// 0b0: Rising edge on an on-chip event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Rising edge on an on-chip event triggers a burst mode entry
            pub const Trigger: u32 = 0b1;
        }
    }

    /// EEV8
    pub mod EEV8 {
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

            /// 0b0: External event X has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: External event X triggers a burst mode entry
            pub const Trigger: u32 = 0b1;
        }
    }

    /// EEV7
    pub mod EEV7 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EEV8::RW;
    }

    /// TDEEV8
    pub mod TDEEV8 {
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

            /// 0b0: Timer X period following external event Y has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer X period following external event Y triggers a burst mode entry
            pub const Trigger: u32 = 0b1;
        }
    }

    /// TAEEV7
    pub mod TAEEV7 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TDEEV8::RW;
    }

    /// TECMP2
    pub mod TECMP2 {
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

            /// 0b0: Timer X compare Y has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer X compare Y triggers capture Z
            pub const TriggerCapture: u32 = 0b1;
        }
    }

    /// TECMP1
    pub mod TECMP1 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECMP2::RW;
    }

    /// TEREP
    pub mod TEREP {
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

            /// 0b0: Timer X repetition event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer X repetition event triggers a burst mode entry
            pub const Trigger: u32 = 0b1;
        }
    }

    /// TERST
    pub mod TERST {
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

            /// 0b0: Timer X reset/roll-over event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer X reset/roll-over event triggers a burst mode entry
            pub const Trigger: u32 = 0b1;
        }
    }

    /// TDCMP2
    pub mod TDCMP2 {
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

    /// TDCMP1
    pub mod TDCMP1 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECMP2::RW;
    }

    /// TDREP
    pub mod TDREP {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEREP::RW;
    }

    /// TDRST
    pub mod TDRST {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TERST::RW;
    }

    /// TCCMP2
    pub mod TCCMP2 {
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

    /// TCCMP1
    pub mod TCCMP1 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECMP2::RW;
    }

    /// TCREP
    pub mod TCREP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEREP::RW;
    }

    /// TCRST
    pub mod TCRST {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TERST::RW;
    }

    /// TBCMP2
    pub mod TBCMP2 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECMP2::RW;
    }

    /// TBCMP1
    pub mod TBCMP1 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECMP2::RW;
    }

    /// TBREP
    pub mod TBREP {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEREP::RW;
    }

    /// TBRST
    pub mod TBRST {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TERST::RW;
    }

    /// TACMP2
    pub mod TACMP2 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TECMP2::RW;
    }

    /// TACMP1
    pub mod TACMP1 {
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

            /// 0b0: Timer X compare Y event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Timer X compare Y event triggers a burst mode entry
            pub const Trigger: u32 = 0b1;
        }
    }

    /// TAREP
    pub mod TAREP {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEREP::RW;
    }

    /// TARST
    pub mod TARST {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TERST::RW;
    }

    /// MSTCMP4
    pub mod MSTCMP4 {
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

            /// 0b0: Master timer compare event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Master timer compare event forces the output to its active state
            pub const SetActive: u32 = 0b1;
        }
    }

    /// MSTCMP3
    pub mod MSTCMP3 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MSTCMP4::RW;
    }

    /// MSTCMP2
    pub mod MSTCMP2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MSTCMP4::RW;
    }

    /// MSTCMP1
    pub mod MSTCMP1 {
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

            /// 0b0: Master timer compare X event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Master timer compare X event triggers a burst mode entry
            pub const Trigger: u32 = 0b1;
        }
    }

    /// MSTREP
    pub mod MSTREP {
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

            /// 0b0: Master timer repetition event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Master timer repetition event triggers a burst mode entry
            pub const Trigger: u32 = 0b1;
        }
    }

    /// MSTRST
    pub mod MSTRST {
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

            /// 0b0: Master timer reset/roll-over event has no effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Master timer reset/roll-over event triggers a burst mode entry
            pub const Trigger: u32 = 0b1;
        }
    }

    /// SW
    pub mod SW {
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

            /// 0b1: Trigger immediate burst mode operation
            pub const Trigger: u32 = 0b1;
        }
    }
}

/// BMCMPR
pub mod BMCMPR {

    /// BMCMP
    pub mod BMCMP {
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

/// Burst Mode Period Register
pub mod BMPER {

    /// Burst mode Period
    pub mod BMPER {
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

/// Timer External Event Control Register 1
pub mod EECR1 {

    /// External Event 5 Fast mode
    pub mod EE5FAST {
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

            /// 0b0: External event is re-synchronised by the HRTIM logic before acting on outputs
            pub const Resynchronized: u32 = 0b0;

            /// 0b1: External event is acting asynchronously on outputs (low-latency mode)
            pub const Asynchronous: u32 = 0b1;
        }
    }

    /// External Event 5 Sensitivity
    pub mod EE5SNS {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (2 bits: 0b11 << 27)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: On active level defined by EExPOL bit
            pub const Active: u32 = 0b00;

            /// 0b01: Rising edge
            pub const Rising: u32 = 0b01;

            /// 0b10: Falling edge
            pub const Falling: u32 = 0b10;

            /// 0b11: Both edges
            pub const Both: u32 = 0b11;
        }
    }

    /// External Event 5 Polarity
    pub mod EE5POL {
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

            /// 0b0: External event is active high
            pub const ActiveHigh: u32 = 0b0;

            /// 0b1: External event is active low
            pub const ActiveLow: u32 = 0b1;
        }
    }

    /// External Event 5 Source
    pub mod EE5SRC {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Source 1
            pub const Src1: u32 = 0b00;

            /// 0b01: Source 2
            pub const Src2: u32 = 0b01;

            /// 0b10: Source 3
            pub const Src3: u32 = 0b10;

            /// 0b11: Source 4
            pub const Src4: u32 = 0b11;
        }
    }

    /// External Event 4 Fast mode
    pub mod EE4FAST {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5FAST::RW;
    }

    /// External Event 4 Sensitivity
    pub mod EE4SNS {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (2 bits: 0b11 << 21)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5SNS::RW;
    }

    /// External Event 4 Polarity
    pub mod EE4POL {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5POL::RW;
    }

    /// External Event 4 Source
    pub mod EE4SRC {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5SRC::RW;
    }

    /// External Event 3 Fast mode
    pub mod EE3FAST {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5FAST::RW;
    }

    /// External Event 3 Sensitivity
    pub mod EE3SNS {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (2 bits: 0b11 << 15)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5SNS::RW;
    }

    /// External Event 3 Polarity
    pub mod EE3POL {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5POL::RW;
    }

    /// External Event 3 Source
    pub mod EE3SRC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5SRC::RW;
    }

    /// External Event 2 Fast mode
    pub mod EE2FAST {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5FAST::RW;
    }

    /// External Event 2 Sensitivity
    pub mod EE2SNS {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5SNS::RW;
    }

    /// External Event 2 Polarity
    pub mod EE2POL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5POL::RW;
    }

    /// External Event 2 Source
    pub mod EE2SRC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5SRC::RW;
    }

    /// External Event 1 Fast mode
    pub mod EE1FAST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5FAST::RW;
    }

    /// External Event 1 Sensitivity
    pub mod EE1SNS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5SNS::RW;
    }

    /// External Event 1 Polarity
    pub mod EE1POL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5POL::RW;
    }

    /// External Event 1 Source
    pub mod EE1SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE5SRC::RW;
    }
}

/// Timer External Event Control Register 2
pub mod EECR2 {

    /// External Event 10 Sensitivity
    pub mod EE10SNS {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (2 bits: 0b11 << 27)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: On active level defined by EExPOL bit
            pub const Active: u32 = 0b00;

            /// 0b01: Rising edge
            pub const Rising: u32 = 0b01;

            /// 0b10: Falling edge
            pub const Falling: u32 = 0b10;

            /// 0b11: Both edges
            pub const Both: u32 = 0b11;
        }
    }

    /// External Event 10 Polarity
    pub mod EE10POL {
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

            /// 0b0: External event is active high
            pub const ActiveHigh: u32 = 0b0;

            /// 0b1: External event is active low
            pub const ActiveLow: u32 = 0b1;
        }
    }

    /// External Event 10 Source
    pub mod EE10SRC {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Source 1
            pub const Src1: u32 = 0b00;

            /// 0b01: Source 2
            pub const Src2: u32 = 0b01;

            /// 0b10: Source 3
            pub const Src3: u32 = 0b10;

            /// 0b11: Source 4
            pub const Src4: u32 = 0b11;
        }
    }

    /// External Event 9 Sensitivity
    pub mod EE9SNS {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (2 bits: 0b11 << 21)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10SNS::RW;
    }

    /// External Event 9 Polarity
    pub mod EE9POL {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10POL::RW;
    }

    /// External Event 9 Source
    pub mod EE9SRC {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10SRC::RW;
    }

    /// External Event 8 Sensitivity
    pub mod EE8SNS {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (2 bits: 0b11 << 15)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10SNS::RW;
    }

    /// External Event 8 Polarity
    pub mod EE8POL {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10POL::RW;
    }

    /// External Event 8 Source
    pub mod EE8SRC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10SRC::RW;
    }

    /// External Event 7 Sensitivity
    pub mod EE7SNS {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10SNS::RW;
    }

    /// External Event 7 Polarity
    pub mod EE7POL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10POL::RW;
    }

    /// External Event 7 Source
    pub mod EE7SRC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10SRC::RW;
    }

    /// External Event 6 Sensitivity
    pub mod EE6SNS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10SNS::RW;
    }

    /// External Event 6 Polarity
    pub mod EE6POL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10POL::RW;
    }

    /// External Event 6 Source
    pub mod EE6SRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10SRC::RW;
    }
}

/// Timer External Event Control Register 3
pub mod EECR3 {

    /// EEVSD
    pub mod EEVSD {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: f_EEVS=f_HRTIM
            pub const Div1: u32 = 0b00;

            /// 0b01: f_EEVS=f_HRTIM/2
            pub const Div2: u32 = 0b01;

            /// 0b10: f_EEVS=f_HRTIM/4
            pub const Div4: u32 = 0b10;

            /// 0b11: f_EEVS=f_HRTIM/8
            pub const Div8: u32 = 0b11;
        }
    }

    /// EE10F
    pub mod EE10F {
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

            /// 0b0000: Filter disabled
            pub const Disabled: u32 = 0b0000;

            /// 0b0001: f_SAMPLING=f_HRTIM, N=2
            pub const Div1_N2: u32 = 0b0001;

            /// 0b0010: f_SAMPLING=f_HRTIM, N=4
            pub const Div1_N4: u32 = 0b0010;

            /// 0b0011: f_SAMPLING=f_HRTIM, N=8
            pub const Div1_N8: u32 = 0b0011;

            /// 0b0100: f_SAMPLING=f_HRTIM/2, N=6
            pub const Div2_N6: u32 = 0b0100;

            /// 0b0101: f_SAMPLING=f_HRTIM/2, N=8
            pub const Div2_N8: u32 = 0b0101;

            /// 0b0110: f_SAMPLING=f_HRTIM/4, N=6
            pub const Div4_N6: u32 = 0b0110;

            /// 0b0111: f_SAMPLING=f_HRTIM/4, N=8
            pub const Div4_N8: u32 = 0b0111;

            /// 0b1000: f_SAMPLING=f_HRTIM/8, N=6
            pub const Div8_N6: u32 = 0b1000;

            /// 0b1001: f_SAMPLING=f_HRTIM/8, N=8
            pub const Div8_N8: u32 = 0b1001;

            /// 0b1010: f_SAMPLING=f_HRTIM/16, N=5
            pub const Div16_N5: u32 = 0b1010;

            /// 0b1011: f_SAMPLING=f_HRTIM/16, N=6
            pub const Div16_N6: u32 = 0b1011;

            /// 0b1100: f_SAMPLING=f_HRTIM/16, N=8
            pub const Div16_N8: u32 = 0b1100;

            /// 0b1101: f_SAMPLING=f_HRTIM/32, N=5
            pub const Div32_N5: u32 = 0b1101;

            /// 0b1110: f_SAMPLING=f_HRTIM/32, N=6
            pub const Div32_N6: u32 = 0b1110;

            /// 0b1111: f_SAMPLING=f_HRTIM/32, N=8
            pub const Div32_N8: u32 = 0b1111;
        }
    }

    /// EE9F
    pub mod EE9F {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (4 bits: 0b1111 << 18)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10F::RW;
    }

    /// EE8F
    pub mod EE8F {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10F::RW;
    }

    /// EE7F
    pub mod EE7F {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (4 bits: 0b1111 << 6)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10F::RW;
    }

    /// EE6F
    pub mod EE6F {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EE10F::RW;
    }
}

/// ADC Trigger 1 Register
pub mod ADC1R {

    /// ADC trigger 1 on Timer E Period
    pub mod AD1TEPER {
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

            /// 0b0: No generation of ADC trigger on timer period event
            pub const Disabled: u32 = 0b0;

            /// 0b1: Generation of ADC trigger on timer period event
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADC trigger 1 on Timer E compare 4
    pub mod AD1TEC4 {
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

            /// 0b0: No generation of ADC trigger on timer compare event
            pub const Disabled: u32 = 0b0;

            /// 0b1: Generation of ADC trigger on timer compare event
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADC trigger 1 on Timer E compare 3
    pub mod AD1TEC3 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEC4::RW;
    }

    /// ADC trigger 1 on Timer E compare 2
    pub mod AD1TEC2 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEC4::RW;
    }

    /// ADC trigger 1 on Timer D Period
    pub mod AD1TDPER {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEPER::RW;
    }

    /// ADC trigger 1 on Timer D compare 4
    pub mod AD1TDC4 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEC4::RW;
    }

    /// ADC trigger 1 on Timer D compare 3
    pub mod AD1TDC3 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEC4::RW;
    }

    /// ADC trigger 1 on Timer D compare 2
    pub mod AD1TDC2 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEC4::RW;
    }

    /// ADC trigger 1 on Timer C Period
    pub mod AD1TCPER {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEPER::RW;
    }

    /// ADC trigger 1 on Timer C compare 4
    pub mod AD1TCC4 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEC4::RW;
    }

    /// ADC trigger 1 on Timer C compare 3
    pub mod AD1TCC3 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEC4::RW;
    }

    /// ADC trigger 1 on Timer C compare 2
    pub mod AD1TCC2 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEC4::RW;
    }

    /// ADC trigger 1 on Timer B Reset
    pub mod AD1TBRST {
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

            /// 0b0: No generation of ADC trigger on timer reset and roll-over
            pub const Disabled: u32 = 0b0;

            /// 0b1: Generation of ADC trigger on timer reset and roll-over
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADC trigger 1 on Timer B Period
    pub mod AD1TBPER {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEPER::RW;
    }

    /// ADC trigger 1 on Timer B compare 4
    pub mod AD1TBC4 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEC4::RW;
    }

    /// ADC trigger 1 on Timer B compare 3
    pub mod AD1TBC3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEC4::RW;
    }

    /// ADC trigger 1 on Timer B compare 2
    pub mod AD1TBC2 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEC4::RW;
    }

    /// ADC trigger 1 on Timer A Reset
    pub mod AD1TARST {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TBRST::RW;
    }

    /// ADC trigger 1 on Timer A Period
    pub mod AD1TAPER {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEPER::RW;
    }

    /// ADC trigger 1 on Timer A compare 4
    pub mod AD1TAC4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEC4::RW;
    }

    /// ADC trigger 1 on Timer A compare 3
    pub mod AD1TAC3 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEC4::RW;
    }

    /// ADC trigger 1 on Timer A compare 2
    pub mod AD1TAC2 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEC4::RW;
    }

    /// ADC trigger 1 on External Event 5
    pub mod AD1EEV5 {
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

            /// 0b0: No generation of ADC trigger on external event
            pub const Disabled: u32 = 0b0;

            /// 0b1: Generation of ADC trigger on external event
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADC trigger 1 on External Event 4
    pub mod AD1EEV4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1EEV5::RW;
    }

    /// ADC trigger 1 on External Event 3
    pub mod AD1EEV3 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1EEV5::RW;
    }

    /// ADC trigger 1 on External Event 2
    pub mod AD1EEV2 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1EEV5::RW;
    }

    /// ADC trigger 1 on External Event 1
    pub mod AD1EEV1 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1EEV5::RW;
    }

    /// ADC trigger 1 on Master Period
    pub mod AD1MPER {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1TEPER::RW;
    }

    /// ADC trigger 1 on Master Compare 4
    pub mod AD1MC4 {
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

            /// 0b0: No generation of ADC trigger on master compare event
            pub const Disabled: u32 = 0b0;

            /// 0b1: Generation of ADC trigger on master compare event
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADC trigger 1 on Master Compare 3
    pub mod AD1MC3 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1MC4::RW;
    }

    /// ADC trigger 1 on Master Compare 2
    pub mod AD1MC2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1MC4::RW;
    }

    /// ADC trigger 1 on Master Compare 1
    pub mod AD1MC1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD1MC4::RW;
    }
}

/// ADC Trigger 2 Register
pub mod ADC2R {

    /// ADC trigger 2 on Timer E Reset
    pub mod AD2TERST {
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

            /// 0b0: No generation of ADC trigger on timer reset and roll-over
            pub const Disabled: u32 = 0b0;

            /// 0b1: Generation of ADC trigger on timer reset and roll-over
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADC trigger 2 on Timer E compare 4
    pub mod AD2TEC4 {
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

            /// 0b0: No generation of ADC trigger on timer compare event
            pub const Disabled: u32 = 0b0;

            /// 0b1: Generation of ADC trigger on timer compare event
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADC trigger 2 on Timer E compare 3
    pub mod AD2TEC3 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TEC4::RW;
    }

    /// ADC trigger 2 on Timer E compare 2
    pub mod AD2TEC2 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TEC4::RW;
    }

    /// ADC trigger 2 on Timer D Reset
    pub mod AD2TDRST {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TERST::RW;
    }

    /// ADC trigger 2 on Timer D Period
    pub mod AD2TDPER {
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

            /// 0b0: No generation of ADC trigger on timer period event
            pub const Disabled: u32 = 0b0;

            /// 0b1: Generation of ADC trigger on timer period event
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADC trigger 2 on Timer D compare 4
    pub mod AD2TDC4 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TEC4::RW;
    }

    /// ADC trigger 2 on Timer D compare 3
    pub mod AD2TDC3 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TEC4::RW;
    }

    /// ADC trigger 2 on Timer D compare 2
    pub mod AD2TDC2 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TEC4::RW;
    }

    /// ADC trigger 2 on Timer C Reset
    pub mod AD2TCRST {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TERST::RW;
    }

    /// ADC trigger 2 on Timer C Period
    pub mod AD2TCPER {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TDPER::RW;
    }

    /// ADC trigger 2 on Timer C compare 4
    pub mod AD2TCC4 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TEC4::RW;
    }

    /// ADC trigger 2 on Timer C compare 3
    pub mod AD2TCC3 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TEC4::RW;
    }

    /// ADC trigger 2 on Timer C compare 2
    pub mod AD2TCC2 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TEC4::RW;
    }

    /// ADC trigger 2 on Timer B Period
    pub mod AD2TBPER {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TDPER::RW;
    }

    /// ADC trigger 2 on Timer B compare 4
    pub mod AD2TBC4 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TEC4::RW;
    }

    /// ADC trigger 2 on Timer B compare 3
    pub mod AD2TBC3 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TEC4::RW;
    }

    /// ADC trigger 2 on Timer B compare 2
    pub mod AD2TBC2 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TEC4::RW;
    }

    /// ADC trigger 2 on Timer A Period
    pub mod AD2TAPER {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TDPER::RW;
    }

    /// ADC trigger 2 on Timer A compare 4
    pub mod AD2TAC4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TEC4::RW;
    }

    /// ADC trigger 2 on Timer A compare 3
    pub mod AD2TAC3 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TEC4::RW;
    }

    /// ADC trigger 2 on Timer A compare 2
    pub mod AD2TAC2 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TEC4::RW;
    }

    /// ADC trigger 2 on External Event 10
    pub mod AD2EEV10 {
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

            /// 0b0: No generation of ADC trigger on external event
            pub const Disabled: u32 = 0b0;

            /// 0b1: Generation of ADC trigger on external event
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADC trigger 2 on External Event 9
    pub mod AD2EEV9 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2EEV10::RW;
    }

    /// ADC trigger 2 on External Event 8
    pub mod AD2EEV8 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2EEV10::RW;
    }

    /// ADC trigger 2 on External Event 7
    pub mod AD2EEV7 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2EEV10::RW;
    }

    /// ADC trigger 2 on External Event 6
    pub mod AD2EEV6 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2EEV10::RW;
    }

    /// ADC trigger 2 on Master Period
    pub mod AD2MPER {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2TDPER::RW;
    }

    /// ADC trigger 2 on Master Compare 4
    pub mod AD2MC4 {
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

            /// 0b0: No generation of ADC trigger on master compare event
            pub const Disabled: u32 = 0b0;

            /// 0b1: Generation of ADC trigger on master compare event
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADC trigger 2 on Master Compare 3
    pub mod AD2MC3 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2MC4::RW;
    }

    /// ADC trigger 2 on Master Compare 2
    pub mod AD2MC2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2MC4::RW;
    }

    /// ADC trigger 2 on Master Compare 1
    pub mod AD2MC1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AD2MC4::RW;
    }
}

/// ADC Trigger 3 Register
pub mod ADC3R {
    pub use super::ADC1R::AD1EEV1;
    pub use super::ADC1R::AD1EEV2;
    pub use super::ADC1R::AD1EEV3;
    pub use super::ADC1R::AD1EEV4;
    pub use super::ADC1R::AD1EEV5;
    pub use super::ADC1R::AD1MC1;
    pub use super::ADC1R::AD1MC2;
    pub use super::ADC1R::AD1MC3;
    pub use super::ADC1R::AD1MC4;
    pub use super::ADC1R::AD1MPER;
    pub use super::ADC1R::AD1TAC2;
    pub use super::ADC1R::AD1TAC3;
    pub use super::ADC1R::AD1TAC4;
    pub use super::ADC1R::AD1TAPER;
    pub use super::ADC1R::AD1TARST;
    pub use super::ADC1R::AD1TBC2;
    pub use super::ADC1R::AD1TBC3;
    pub use super::ADC1R::AD1TBC4;
    pub use super::ADC1R::AD1TBPER;
    pub use super::ADC1R::AD1TBRST;
    pub use super::ADC1R::AD1TCC2;
    pub use super::ADC1R::AD1TCC3;
    pub use super::ADC1R::AD1TCC4;
    pub use super::ADC1R::AD1TCPER;
    pub use super::ADC1R::AD1TDC2;
    pub use super::ADC1R::AD1TDC3;
    pub use super::ADC1R::AD1TDC4;
    pub use super::ADC1R::AD1TDPER;
    pub use super::ADC1R::AD1TEC2;
    pub use super::ADC1R::AD1TEC3;
    pub use super::ADC1R::AD1TEC4;
    pub use super::ADC1R::AD1TEPER;
}

/// ADC Trigger 4 Register
pub mod ADC4R {
    pub use super::ADC2R::AD2EEV10;
    pub use super::ADC2R::AD2EEV6;
    pub use super::ADC2R::AD2EEV7;
    pub use super::ADC2R::AD2EEV8;
    pub use super::ADC2R::AD2EEV9;
    pub use super::ADC2R::AD2MC1;
    pub use super::ADC2R::AD2MC2;
    pub use super::ADC2R::AD2MC3;
    pub use super::ADC2R::AD2MC4;
    pub use super::ADC2R::AD2MPER;
    pub use super::ADC2R::AD2TAC2;
    pub use super::ADC2R::AD2TAC3;
    pub use super::ADC2R::AD2TAC4;
    pub use super::ADC2R::AD2TAPER;
    pub use super::ADC2R::AD2TBC2;
    pub use super::ADC2R::AD2TBC3;
    pub use super::ADC2R::AD2TBC4;
    pub use super::ADC2R::AD2TBPER;
    pub use super::ADC2R::AD2TCC2;
    pub use super::ADC2R::AD2TCC3;
    pub use super::ADC2R::AD2TCC4;
    pub use super::ADC2R::AD2TCPER;
    pub use super::ADC2R::AD2TCRST;
    pub use super::ADC2R::AD2TDC2;
    pub use super::ADC2R::AD2TDC3;
    pub use super::ADC2R::AD2TDC4;
    pub use super::ADC2R::AD2TDPER;
    pub use super::ADC2R::AD2TDRST;
    pub use super::ADC2R::AD2TEC2;
    pub use super::ADC2R::AD2TEC3;
    pub use super::ADC2R::AD2TEC4;
    pub use super::ADC2R::AD2TERST;
}

/// DLL Control Register
pub mod DLLCR {

    /// DLL Calibration rate
    pub mod CALRTE {
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

            /// 0b00: 1048576*t_HRTIM (7.3ms)
            pub const Millis7_3: u32 = 0b00;

            /// 0b01: 131072*t_HRTIM (910µs)
            pub const Micros910: u32 = 0b01;

            /// 0b10: 16384*t_HRTIM (114µs)
            pub const Micros114: u32 = 0b10;

            /// 0b11: 2048*t_HRTIM (14µs)
            pub const Micros14: u32 = 0b11;
        }
    }

    /// DLL Calibration Enable
    pub mod CALEN {
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

            /// 0b0: Periodic calibration disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Calibration is performed periodically, as per CALRTE setting
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DLL Calibration Start
    pub mod CAL {
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

            /// 0b1: Calibration start
            pub const Start: u32 = 0b1;
        }
    }
}

/// HRTIM Fault Input Register 1
pub mod FLTINR1 {

    /// FLT4LCK
    pub mod FLT4LCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Fault bits are read/write
            pub const Unlocked: u32 = 0b0;

            /// 0b1: Fault bits are read-only
            pub const Locked: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Lock corresponding fault bits
            pub const Lock: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FLT4F
    pub mod FLT4F {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (4 bits: 0b1111 << 27)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No filter, FLTx acts asynchronously
            pub const Disabled: u32 = 0b0000;

            /// 0b0001: f_SAMPLING=f_HRTIM, N=2
            pub const Div1_N2: u32 = 0b0001;

            /// 0b0010: f_SAMPLING=f_HRTIM, N=4
            pub const Div1_N4: u32 = 0b0010;

            /// 0b0011: f_SAMPLING=f_HRTIM, N=8
            pub const Div1_N8: u32 = 0b0011;

            /// 0b0100: f_SAMPLING=f_HRTIM/2, N=6
            pub const Div2_N6: u32 = 0b0100;

            /// 0b0101: f_SAMPLING=f_HRTIM/2, N=8
            pub const Div2_N8: u32 = 0b0101;

            /// 0b0110: f_SAMPLING=f_HRTIM/4, N=6
            pub const Div4_N6: u32 = 0b0110;

            /// 0b0111: f_SAMPLING=f_HRTIM/4, N=8
            pub const Div4_N8: u32 = 0b0111;

            /// 0b1000: f_SAMPLING=f_HRTIM/8, N=6
            pub const Div8_N6: u32 = 0b1000;

            /// 0b1001: f_SAMPLING=f_HRTIM/8, N=8
            pub const Div8_N8: u32 = 0b1001;

            /// 0b1010: f_SAMPLING=f_HRTIM/16, N=5
            pub const Div16_N5: u32 = 0b1010;

            /// 0b1011: f_SAMPLING=f_HRTIM/16, N=6
            pub const Div16_N6: u32 = 0b1011;

            /// 0b1100: f_SAMPLING=f_HRTIM/16, N=8
            pub const Div16_N8: u32 = 0b1100;

            /// 0b1101: f_SAMPLING=f_HRTIM/32, N=5
            pub const Div32_N5: u32 = 0b1101;

            /// 0b1110: f_SAMPLING=f_HRTIM/32, N=6
            pub const Div32_N6: u32 = 0b1110;

            /// 0b1111: f_SAMPLING=f_HRTIM/32, N=8
            pub const Div32_N8: u32 = 0b1111;
        }
    }

    /// FLT4SRC
    pub mod FLT4SRC {
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

            /// 0b0: Fault input is FLTx input pin
            pub const Input: u32 = 0b0;

            /// 0b1: Fault input is FLTn_Int signal
            pub const Internal: u32 = 0b1;
        }
    }

    /// FLT4P
    pub mod FLT4P {
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

            /// 0b0: Fault input is active low
            pub const ActiveLow: u32 = 0b0;

            /// 0b1: Fault input is active high
            pub const ActiveHigh: u32 = 0b1;
        }
    }

    /// FLT4E
    pub mod FLT4E {
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

            /// 0b0: Fault input disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Fault input enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// FLT3LCK
    pub mod FLT3LCK {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        pub use super::FLT4LCK::R;
        pub use super::FLT4LCK::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FLT3F
    pub mod FLT3F {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (4 bits: 0b1111 << 19)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT4F::RW;
    }

    /// FLT3SRC
    pub mod FLT3SRC {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT4SRC::RW;
    }

    /// FLT3P
    pub mod FLT3P {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT4P::RW;
    }

    /// FLT3E
    pub mod FLT3E {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT4E::RW;
    }

    /// FLT2LCK
    pub mod FLT2LCK {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        pub use super::FLT4LCK::R;
        pub use super::FLT4LCK::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FLT2F
    pub mod FLT2F {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (4 bits: 0b1111 << 11)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT4F::RW;
    }

    /// FLT2SRC
    pub mod FLT2SRC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT4SRC::RW;
    }

    /// FLT2P
    pub mod FLT2P {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT4P::RW;
    }

    /// FLT2E
    pub mod FLT2E {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT4E::RW;
    }

    /// FLT1LCK
    pub mod FLT1LCK {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        pub use super::FLT4LCK::R;
        pub use super::FLT4LCK::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FLT1F
    pub mod FLT1F {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (4 bits: 0b1111 << 3)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT4F::RW;
    }

    /// FLT1SRC
    pub mod FLT1SRC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT4SRC::RW;
    }

    /// FLT1P
    pub mod FLT1P {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT4P::RW;
    }

    /// FLT1E
    pub mod FLT1E {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLT4E::RW;
    }
}

/// HRTIM Fault Input Register 2
pub mod FLTINR2 {

    /// FLTSD
    pub mod FLTSD {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: f_FLTS=f_HRTIM
            pub const Div1: u32 = 0b00;

            /// 0b01: f_FLTS=f_HRTIM/2
            pub const Div2: u32 = 0b01;

            /// 0b10: f_FLTS=f_HRTIM/4
            pub const Div4: u32 = 0b10;

            /// 0b11: f_FLTS=f_HRTIM/8
            pub const Div8: u32 = 0b11;
        }
    }

    /// FLT5LCK
    pub mod FLT5LCK {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Fault bits are read/write
            pub const Unlocked: u32 = 0b0;

            /// 0b1: Fault bits are read-only
            pub const Locked: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Lock corresponding fault bits
            pub const Lock: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FLT5F
    pub mod FLT5F {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (4 bits: 0b1111 << 3)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No filter, FLTx acts asynchronously
            pub const Disabled: u32 = 0b0000;

            /// 0b0001: f_SAMPLING=f_HRTIM, N=2
            pub const Div1_N2: u32 = 0b0001;

            /// 0b0010: f_SAMPLING=f_HRTIM, N=4
            pub const Div1_N4: u32 = 0b0010;

            /// 0b0011: f_SAMPLING=f_HRTIM, N=8
            pub const Div1_N8: u32 = 0b0011;

            /// 0b0100: f_SAMPLING=f_HRTIM/2, N=6
            pub const Div2_N6: u32 = 0b0100;

            /// 0b0101: f_SAMPLING=f_HRTIM/2, N=8
            pub const Div2_N8: u32 = 0b0101;

            /// 0b0110: f_SAMPLING=f_HRTIM/4, N=6
            pub const Div4_N6: u32 = 0b0110;

            /// 0b0111: f_SAMPLING=f_HRTIM/4, N=8
            pub const Div4_N8: u32 = 0b0111;

            /// 0b1000: f_SAMPLING=f_HRTIM/8, N=6
            pub const Div8_N6: u32 = 0b1000;

            /// 0b1001: f_SAMPLING=f_HRTIM/8, N=8
            pub const Div8_N8: u32 = 0b1001;

            /// 0b1010: f_SAMPLING=f_HRTIM/16, N=5
            pub const Div16_N5: u32 = 0b1010;

            /// 0b1011: f_SAMPLING=f_HRTIM/16, N=6
            pub const Div16_N6: u32 = 0b1011;

            /// 0b1100: f_SAMPLING=f_HRTIM/16, N=8
            pub const Div16_N8: u32 = 0b1100;

            /// 0b1101: f_SAMPLING=f_HRTIM/32, N=5
            pub const Div32_N5: u32 = 0b1101;

            /// 0b1110: f_SAMPLING=f_HRTIM/32, N=6
            pub const Div32_N6: u32 = 0b1110;

            /// 0b1111: f_SAMPLING=f_HRTIM/32, N=8
            pub const Div32_N8: u32 = 0b1111;
        }
    }

    /// FLT5SRC
    pub mod FLT5SRC {
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

            /// 0b0: Fault input is FLTx input pin
            pub const Input: u32 = 0b0;

            /// 0b1: Fault input is FLTn_Int signal
            pub const Internal: u32 = 0b1;
        }
    }

    /// FLT5P
    pub mod FLT5P {
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

            /// 0b0: Fault input is active low
            pub const ActiveLow: u32 = 0b0;

            /// 0b1: Fault input is active high
            pub const ActiveHigh: u32 = 0b1;
        }
    }

    /// FLT5E
    pub mod FLT5E {
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

            /// 0b0: Fault input disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Fault input enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// BDMUPDR
pub mod BDMUPR {

    /// MCMP4
    pub mod MCMP4 {
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

            /// 0b0: Register not updated by burst DMA access
            pub const NotUpdated: u32 = 0b0;

            /// 0b1: Register updated by burst DMA access
            pub const Updated: u32 = 0b1;
        }
    }

    /// MCMP3
    pub mod MCMP3 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MCMP4::RW;
    }

    /// MCMP2
    pub mod MCMP2 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MCMP4::RW;
    }

    /// MCMP1
    pub mod MCMP1 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MCMP4::RW;
    }

    /// MREP
    pub mod MREP {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MCMP4::RW;
    }

    /// MPER
    pub mod MPER {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MCMP4::RW;
    }

    /// MCNT
    pub mod MCNT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MCMP4::RW;
    }

    /// MDIER
    pub mod MDIER {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MCMP4::RW;
    }

    /// MICR
    pub mod MICR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MCMP4::RW;
    }

    /// MCR
    pub mod MCR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MCMP4::RW;
    }
}

/// Burst DMA Timerx update Register
pub mod BDTAUPR {

    /// HRTIM_FLTxR register update enable
    pub mod TIMxFLTR {
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

            /// 0b0: Register not updated by burst DMA access
            pub const NotUpdated: u32 = 0b0;

            /// 0b1: Register updated by burst DMA access
            pub const Updated: u32 = 0b1;
        }
    }

    /// HRTIM_OUTxR register update enable
    pub mod TIMxOUTR {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_CHPxR register update enable
    pub mod TIMxCHPR {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_RSTxR register update enable
    pub mod TIMxRSTR {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_EEFxR2 register update enable
    pub mod TIMxEEFR2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_EEFxR1 register update enable
    pub mod TIMxEEFR1 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_RST2xR register update enable
    pub mod TIMxRST2R {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_SET2xR register update enable
    pub mod TIMxSET2R {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_RST1xR register update enable
    pub mod TIMxRST1R {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_SET1xR register update enable
    pub mod TIMxSET1R {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_DTxR register update enable
    pub mod TIMx_DTxR {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_CMP4xR register update enable
    pub mod TIMxCMP4 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_CMP3xR register update enable
    pub mod TIMxCMP3 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_CMP2xR register update enable
    pub mod TIMxCMP2 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_CMP1xR register update enable
    pub mod TIMxCMP1 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_REPxR register update enable
    pub mod TIMxREP {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_PERxR register update enable
    pub mod TIMxPER {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_CNTxR register update enable
    pub mod TIMxCNT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_TIMxDIER register update enable
    pub mod TIMxDIER {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_TIMxICR register update enable
    pub mod TIMxICR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }

    /// HRTIM_TIMxCR register update enable
    pub mod TIMxCR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMxFLTR::RW;
    }
}

/// Burst DMA Timerx update Register
pub mod BDTBUPR {
    pub use super::BDTAUPR::TIMxCHPR;
    pub use super::BDTAUPR::TIMxCMP1;
    pub use super::BDTAUPR::TIMxCMP2;
    pub use super::BDTAUPR::TIMxCMP3;
    pub use super::BDTAUPR::TIMxCMP4;
    pub use super::BDTAUPR::TIMxCNT;
    pub use super::BDTAUPR::TIMxCR;
    pub use super::BDTAUPR::TIMxDIER;
    pub use super::BDTAUPR::TIMxEEFR1;
    pub use super::BDTAUPR::TIMxEEFR2;
    pub use super::BDTAUPR::TIMxFLTR;
    pub use super::BDTAUPR::TIMxICR;
    pub use super::BDTAUPR::TIMxOUTR;
    pub use super::BDTAUPR::TIMxPER;
    pub use super::BDTAUPR::TIMxREP;
    pub use super::BDTAUPR::TIMxRST1R;
    pub use super::BDTAUPR::TIMxRST2R;
    pub use super::BDTAUPR::TIMxRSTR;
    pub use super::BDTAUPR::TIMxSET1R;
    pub use super::BDTAUPR::TIMxSET2R;
    pub use super::BDTAUPR::TIMx_DTxR;
}

/// Burst DMA Timerx update Register
pub mod BDTCUPR {
    pub use super::BDTAUPR::TIMxCHPR;
    pub use super::BDTAUPR::TIMxCMP1;
    pub use super::BDTAUPR::TIMxCMP2;
    pub use super::BDTAUPR::TIMxCMP3;
    pub use super::BDTAUPR::TIMxCMP4;
    pub use super::BDTAUPR::TIMxCNT;
    pub use super::BDTAUPR::TIMxCR;
    pub use super::BDTAUPR::TIMxDIER;
    pub use super::BDTAUPR::TIMxEEFR1;
    pub use super::BDTAUPR::TIMxEEFR2;
    pub use super::BDTAUPR::TIMxFLTR;
    pub use super::BDTAUPR::TIMxICR;
    pub use super::BDTAUPR::TIMxOUTR;
    pub use super::BDTAUPR::TIMxPER;
    pub use super::BDTAUPR::TIMxREP;
    pub use super::BDTAUPR::TIMxRST1R;
    pub use super::BDTAUPR::TIMxRST2R;
    pub use super::BDTAUPR::TIMxRSTR;
    pub use super::BDTAUPR::TIMxSET1R;
    pub use super::BDTAUPR::TIMxSET2R;
    pub use super::BDTAUPR::TIMx_DTxR;
}

/// Burst DMA Timerx update Register
pub mod BDTDUPR {
    pub use super::BDTAUPR::TIMxCHPR;
    pub use super::BDTAUPR::TIMxCMP1;
    pub use super::BDTAUPR::TIMxCMP2;
    pub use super::BDTAUPR::TIMxCMP3;
    pub use super::BDTAUPR::TIMxCMP4;
    pub use super::BDTAUPR::TIMxCNT;
    pub use super::BDTAUPR::TIMxCR;
    pub use super::BDTAUPR::TIMxDIER;
    pub use super::BDTAUPR::TIMxEEFR1;
    pub use super::BDTAUPR::TIMxEEFR2;
    pub use super::BDTAUPR::TIMxFLTR;
    pub use super::BDTAUPR::TIMxICR;
    pub use super::BDTAUPR::TIMxOUTR;
    pub use super::BDTAUPR::TIMxPER;
    pub use super::BDTAUPR::TIMxREP;
    pub use super::BDTAUPR::TIMxRST1R;
    pub use super::BDTAUPR::TIMxRST2R;
    pub use super::BDTAUPR::TIMxRSTR;
    pub use super::BDTAUPR::TIMxSET1R;
    pub use super::BDTAUPR::TIMxSET2R;
    pub use super::BDTAUPR::TIMx_DTxR;
}

/// Burst DMA Timerx update Register
pub mod BDTEUPR {
    pub use super::BDTAUPR::TIMxCHPR;
    pub use super::BDTAUPR::TIMxCMP1;
    pub use super::BDTAUPR::TIMxCMP2;
    pub use super::BDTAUPR::TIMxCMP3;
    pub use super::BDTAUPR::TIMxCMP4;
    pub use super::BDTAUPR::TIMxCNT;
    pub use super::BDTAUPR::TIMxCR;
    pub use super::BDTAUPR::TIMxDIER;
    pub use super::BDTAUPR::TIMxEEFR1;
    pub use super::BDTAUPR::TIMxEEFR2;
    pub use super::BDTAUPR::TIMxFLTR;
    pub use super::BDTAUPR::TIMxICR;
    pub use super::BDTAUPR::TIMxOUTR;
    pub use super::BDTAUPR::TIMxPER;
    pub use super::BDTAUPR::TIMxREP;
    pub use super::BDTAUPR::TIMxRST1R;
    pub use super::BDTAUPR::TIMxRST2R;
    pub use super::BDTAUPR::TIMxRSTR;
    pub use super::BDTAUPR::TIMxSET1R;
    pub use super::BDTAUPR::TIMxSET2R;
    pub use super::BDTAUPR::TIMx_DTxR;
}

/// Burst DMA Data Register
pub mod BDMADR {

    /// Burst DMA Data register
    pub mod BDMADR {
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
#[repr(C)]
pub struct RegisterBlock {
    /// Control Register 1
    pub CR1: RWRegister<u32>,

    /// Control Register 2
    pub CR2: RWRegister<u32>,

    /// Interrupt Status Register
    pub ISR: RWRegister<u32>,

    /// Interrupt Clear Register
    pub ICR: WORegister<u32>,

    /// Interrupt Enable Register
    pub IER: RWRegister<u32>,

    /// Output Enable Register
    pub OENR: WORegister<u32>,

    /// DISR
    pub ODISR: RWRegister<u32>,

    /// Output Disable Status Register
    pub ODSR: RORegister<u32>,

    /// Burst Mode Control Register
    pub BMCR: RWRegister<u32>,

    /// BMTRGR
    pub BMTRGR: RWRegister<u32>,

    /// BMCMPR
    pub BMCMPR: RWRegister<u32>,

    /// Burst Mode Period Register
    pub BMPER: RWRegister<u32>,

    /// Timer External Event Control Register 1
    pub EECR1: RWRegister<u32>,

    /// Timer External Event Control Register 2
    pub EECR2: RWRegister<u32>,

    /// Timer External Event Control Register 3
    pub EECR3: RWRegister<u32>,

    /// ADC Trigger 1 Register
    pub ADC1R: RWRegister<u32>,

    /// ADC Trigger 2 Register
    pub ADC2R: RWRegister<u32>,

    /// ADC Trigger 3 Register
    pub ADC3R: RWRegister<u32>,

    /// ADC Trigger 4 Register
    pub ADC4R: RWRegister<u32>,

    /// DLL Control Register
    pub DLLCR: RWRegister<u32>,

    /// HRTIM Fault Input Register 1
    pub FLTINR1: RWRegister<u32>,

    /// HRTIM Fault Input Register 2
    pub FLTINR2: RWRegister<u32>,

    /// BDMUPDR
    pub BDMUPR: RWRegister<u32>,

    /// Burst DMA Timerx update Register
    pub BDTAUPR: RWRegister<u32>,

    /// Burst DMA Timerx update Register
    pub BDTBUPR: RWRegister<u32>,

    /// Burst DMA Timerx update Register
    pub BDTCUPR: RWRegister<u32>,

    /// Burst DMA Timerx update Register
    pub BDTDUPR: RWRegister<u32>,

    /// Burst DMA Timerx update Register
    pub BDTEUPR: RWRegister<u32>,

    /// Burst DMA Data Register
    pub BDMADR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1: u32,
    pub CR2: u32,
    pub ISR: u32,
    pub ICR: u32,
    pub IER: u32,
    pub OENR: u32,
    pub ODISR: u32,
    pub ODSR: u32,
    pub BMCR: u32,
    pub BMTRGR: u32,
    pub BMCMPR: u32,
    pub BMPER: u32,
    pub EECR1: u32,
    pub EECR2: u32,
    pub EECR3: u32,
    pub ADC1R: u32,
    pub ADC2R: u32,
    pub ADC3R: u32,
    pub ADC4R: u32,
    pub DLLCR: u32,
    pub FLTINR1: u32,
    pub FLTINR2: u32,
    pub BDMUPR: u32,
    pub BDTAUPR: u32,
    pub BDTBUPR: u32,
    pub BDTCUPR: u32,
    pub BDTDUPR: u32,
    pub BDTEUPR: u32,
    pub BDMADR: u32,
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

/// Access functions for the HRTIM_Common peripheral instance
pub mod HRTIM_Common {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40017780,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HRTIM_Common
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        ISR: 0x00000000,
        ICR: 0x00000000,
        IER: 0x00000000,
        OENR: 0x00000000,
        ODISR: 0x00000000,
        ODSR: 0x00000000,
        BMCR: 0x00000000,
        BMTRGR: 0x00000000,
        BMCMPR: 0x00000000,
        BMPER: 0x00000000,
        EECR1: 0x00000000,
        EECR2: 0x00000000,
        EECR3: 0x00000000,
        ADC1R: 0x00000000,
        ADC2R: 0x00000000,
        ADC3R: 0x00000000,
        ADC4R: 0x00000000,
        DLLCR: 0x00000000,
        FLTINR1: 0x00000000,
        FLTINR2: 0x00000000,
        BDMUPR: 0x00000000,
        BDTAUPR: 0x00000000,
        BDTBUPR: 0x00000000,
        BDTCUPR: 0x00000000,
        BDTDUPR: 0x00000000,
        BDTEUPR: 0x00000000,
        BDMADR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HRTIM_Common_TAKEN: bool = false;

    /// Safe access to HRTIM_Common
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
            if HRTIM_Common_TAKEN {
                None
            } else {
                HRTIM_Common_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HRTIM_Common
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HRTIM_Common_TAKEN && inst.addr == INSTANCE.addr {
                HRTIM_Common_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HRTIM_Common
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HRTIM_Common_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HRTIM_Common
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HRTIM_Common: *const RegisterBlock = 0x40017780 as *const _;
