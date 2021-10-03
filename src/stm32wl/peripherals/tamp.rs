#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Tamper and backup registers
//!
//! Used by: stm32wl5x_cm0p, stm32wl5x_cm4, stm32wle5

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register 1
pub mod CR1 {

    /// TAMP1E
    pub mod TAMP1E {
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

            /// 0b0: Tamper detection on TAMP_INx is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Tamper detection on TAMP_IN3 is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TAMP2E
    pub mod TAMP2E {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP1E::RW;
    }

    /// TAMP2E
    pub mod TAMP3E {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP1E::RW;
    }

    /// ITAMP3E
    pub mod ITAMP3E {
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

            /// 0b0: Internal tamper x disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Internal tamper x enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ITAMP5E
    pub mod ITAMP5E {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ITAMP3E::RW;
    }

    /// ITAMP6E
    pub mod ITAMP6E {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ITAMP3E::RW;
    }

    /// ITAMP8E
    pub mod ITAMP8E {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ITAMP3E::RW;
    }
}

/// control register 2
pub mod CR2 {

    /// TAMP1NOER
    pub mod TAMP1NOER {
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

            /// 0b0: Tamper x event erases the backup registers
            pub const Erase: u32 = 0b0;

            /// 0b1: Tamper x event does not erase the backup registers
            pub const NotErase: u32 = 0b1;
        }
    }

    /// TAMP2NOER
    pub mod TAMP2NOER {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP1NOER::RW;
    }

    /// TAMP3NOER
    pub mod TAMP3NOER {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP1NOER::RW;
    }

    /// TAMP1MSK
    pub mod TAMP1MSK {
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

            /// 0b0: Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection
            pub const ResetBySoftware: u32 = 0b0;

            /// 0b1: Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased. The tamper x interrupt must not be enabled when TAMP3MSK is set
            pub const ResetByHardware: u32 = 0b1;
        }
    }

    /// TAMP2MSK
    pub mod TAMP2MSK {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP1MSK::RW;
    }

    /// TAMP3MSK
    pub mod TAMP3MSK {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP1MSK::RW;
    }

    /// Backup registerserase
    pub mod BKERASE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Reset backup registers
            pub const Reset: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TAMP1TRG
    pub mod TAMP1TRG {
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

            /// 0b0: If TAMPFLT != 00 Tamper x input staying low triggers a tamper detection event. If TAMPFLT = 00 Tamper x input rising edge and high level triggers a tamper detection event
            pub const FilteredLowOrUnfilteredHigh: u32 = 0b0;

            /// 0b1: If TAMPFLT != 00 Tamper x input staying high triggers a tamper detection event. If TAMPFLT = 00 Tamper x input falling edge and low level triggers a tamper detection event
            pub const FilteredHighOrUnfilteredLow: u32 = 0b1;
        }
    }

    /// TAMP2TRG
    pub mod TAMP2TRG {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP1TRG::RW;
    }

    /// TAMP3TRG
    pub mod TAMP3TRG {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP1TRG::RW;
    }
}

/// TAMP control register 3
pub mod CR3 {

    /// ITAMP3NOER
    pub mod ITAMP3NOER {
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

            /// 0b0: Internal tamper x event erases the backup registers
            pub const Erase: u32 = 0b0;

            /// 0b1: Internal tamper x event does not erase the backup registers
            pub const NotErase: u32 = 0b1;
        }
    }

    /// ITAMP5NOER
    pub mod ITAMP5NOER {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ITAMP3NOER::RW;
    }

    /// ITAMP6NOER
    pub mod ITAMP6NOER {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ITAMP3NOER::RW;
    }

    /// ITAMP8NOER
    pub mod ITAMP8NOER {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ITAMP3NOER::RW;
    }
}

/// TAMP filter control register
pub mod FLTCR {

    /// TAMPFREQ
    pub mod TAMPFREQ {
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

            /// 0b000: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
            pub const Hz_1: u32 = 0b000;

            /// 0b001: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
            pub const Hz_2: u32 = 0b001;

            /// 0b010: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
            pub const Hz_4: u32 = 0b010;

            /// 0b011: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
            pub const Hz_8: u32 = 0b011;

            /// 0b100: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
            pub const Hz_16: u32 = 0b100;

            /// 0b101: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
            pub const Hz_32: u32 = 0b101;

            /// 0b110: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
            pub const Hz_64: u32 = 0b110;

            /// 0b111: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
            pub const Hz_128: u32 = 0b111;
        }
    }

    /// TAMPFLT
    pub mod TAMPFLT {
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

            /// 0b00: Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)"
            pub const NoFilter: u32 = 0b00;

            /// 0b01: Tamper event is activated after 2 consecutive samples at the active level"
            pub const Filter2: u32 = 0b01;

            /// 0b10: Tamper event is activated after 4 consecutive samples at the active level"
            pub const Filter4: u32 = 0b10;

            /// 0b11: Tamper event is activated after 8 consecutive samples at the active level"
            pub const Filter8: u32 = 0b11;
        }
    }

    /// TAMPPRCH
    pub mod TAMPPRCH {
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

            /// 0b00: 1 RTCCLK cycle
            pub const Cycles1: u32 = 0b00;

            /// 0b01: 2 RTCCLK cycles
            pub const Cycles2: u32 = 0b01;

            /// 0b10: 4 RTCCLK cycles
            pub const Cycles4: u32 = 0b10;

            /// 0b11: 8 RTCCLK cycles
            pub const Cycles8: u32 = 0b11;
        }
    }

    /// TAMPPUDIS
    pub mod TAMPPUDIS {
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

            /// 0b0: Precharge TAMP_INx pins before sampling (enable internal pull-up)
            pub const Enabled: u32 = 0b0;

            /// 0b1: Disable precharge of TAMP_INx pins
            pub const Disabled: u32 = 0b1;
        }
    }
}

/// TAMP interrupt enable register
pub mod IER {

    /// TAMP1IE
    pub mod TAMP1IE {
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

            /// 0b0: Tamper x interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Tampoer x interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TAMP2IE
    pub mod TAMP2IE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP1IE::RW;
    }

    /// TAMP3IE
    pub mod TAMP3IE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP1IE::RW;
    }

    /// ITAMP3IE
    pub mod ITAMP3IE {
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

            /// 0b0: Internal tamper x interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Internal tamper x interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ITAMP5IE
    pub mod ITAMP5IE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ITAMP3IE::RW;
    }

    /// ITAMP6IE
    pub mod ITAMP6IE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ITAMP3IE::RW;
    }

    /// ITAMP8IE
    pub mod ITAMP8IE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ITAMP3IE::RW;
    }
}

/// TAMP status register
pub mod SR {

    /// TAMP1F
    pub mod TAMP1F {
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

            /// 0b0: No tamper detected
            pub const Idle: u32 = 0b0;

            /// 0b1: Tamper detected
            pub const Tamper: u32 = 0b1;
        }
    }

    /// TAMP2F
    pub mod TAMP2F {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP1F::RW;
    }

    /// TAMP3F
    pub mod TAMP3F {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP1F::RW;
    }

    /// ITAMP3F
    pub mod ITAMP3F {
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

            /// 0b0: No tamper detected
            pub const Idle: u32 = 0b0;

            /// 0b1: Internal tamper detected
            pub const Tamper: u32 = 0b1;
        }
    }

    /// ITAMP5F
    pub mod ITAMP5F {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ITAMP3F::RW;
    }

    /// ITAMP6F
    pub mod ITAMP6F {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ITAMP3F::RW;
    }

    /// ITAMP8F
    pub mod ITAMP8F {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ITAMP3F::RW;
    }
}

/// TAMP masked interrupt status register
pub mod MISR {

    /// TAMP1MF:
    pub mod TAMP1MF {
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

            /// 0b0: No tamper detected - Masked
            pub const Idle: u32 = 0b0;

            /// 0b1: Tamper detected - Masked
            pub const Tamper: u32 = 0b1;
        }
    }

    /// TAMP2MF
    pub mod TAMP2MF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP1MF::RW;
    }

    /// TAMP3MF
    pub mod TAMP3MF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TAMP1MF::RW;
    }

    /// ITAMP3MF
    pub mod ITAMP3MF {
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

            /// 0b0: No tamper detected - Masked
            pub const Idle: u32 = 0b0;

            /// 0b1: Internal tamper detected - Masked
            pub const Tamper: u32 = 0b1;
        }
    }

    /// ITAMP5MF
    pub mod ITAMP5MF {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ITAMP3MF::RW;
    }

    /// ITAMP6MF
    pub mod ITAMP6MF {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ITAMP3MF::RW;
    }

    /// ITAMP8MF
    pub mod ITAMP8MF {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ITAMP3MF::RW;
    }
}

/// TAMP status clear register
pub mod SCR {

    /// CTAMP1F
    pub mod CTAMP1F {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear tamper flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CTAMP2F
    pub mod CTAMP2F {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTAMP1F::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CTAMP3F
    pub mod CTAMP3F {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTAMP1F::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CITAMP3F
    pub mod CITAMP3F {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTAMP1F::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CITAMP5F
    pub mod CITAMP5F {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTAMP1F::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CITAMP6F
    pub mod CITAMP6F {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTAMP1F::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CITAMP8F
    pub mod CITAMP8F {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTAMP1F::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// monotonic counter register
pub mod COUNTR {

    /// COUNT
    pub mod COUNT {
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

/// TAMP backup register
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

/// TAMP backup register
pub mod BKP1R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP2R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP3R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP4R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP5R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP6R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP7R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP8R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP9R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP10R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP11R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP12R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP13R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP14R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP15R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP16R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP17R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP18R {
    pub use super::BKP0R::BKP;
}

/// TAMP backup register
pub mod BKP19R {
    pub use super::BKP0R::BKP;
}
#[repr(C)]
pub struct RegisterBlock {
    /// control register 1
    pub CR1: RWRegister<u32>,

    /// control register 2
    pub CR2: RWRegister<u32>,

    /// TAMP control register 3
    pub CR3: RWRegister<u32>,

    /// TAMP filter control register
    pub FLTCR: RWRegister<u32>,

    _reserved1: [u32; 7],

    /// TAMP interrupt enable register
    pub IER: RWRegister<u32>,

    /// TAMP status register
    pub SR: RORegister<u32>,

    /// TAMP masked interrupt status register
    pub MISR: RORegister<u32>,

    _reserved2: [u32; 1],

    /// TAMP status clear register
    pub SCR: WORegister<u32>,

    /// monotonic counter register
    pub COUNTR: RORegister<u32>,

    _reserved3: [u32; 47],

    /// TAMP backup register
    pub BKP0R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP1R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP2R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP3R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP4R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP5R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP6R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP7R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP8R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP9R: RWRegister<u32>,

    _reserved4: [u32; 6],

    /// TAMP backup register
    pub BKP10R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP11R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP12R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP13R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP14R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP15R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP16R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP17R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP18R: RWRegister<u32>,

    /// TAMP backup register
    pub BKP19R: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1: u32,
    pub CR2: u32,
    pub CR3: u32,
    pub FLTCR: u32,
    pub IER: u32,
    pub SR: u32,
    pub MISR: u32,
    pub SCR: u32,
    pub COUNTR: u32,
    pub BKP0R: u32,
    pub BKP1R: u32,
    pub BKP2R: u32,
    pub BKP3R: u32,
    pub BKP4R: u32,
    pub BKP5R: u32,
    pub BKP6R: u32,
    pub BKP7R: u32,
    pub BKP8R: u32,
    pub BKP9R: u32,
    pub BKP10R: u32,
    pub BKP11R: u32,
    pub BKP12R: u32,
    pub BKP13R: u32,
    pub BKP14R: u32,
    pub BKP15R: u32,
    pub BKP16R: u32,
    pub BKP17R: u32,
    pub BKP18R: u32,
    pub BKP19R: u32,
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
