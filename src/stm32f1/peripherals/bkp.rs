#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Backup registers
//!
//! Used by: stm32f100, stm32f101, stm32f102, stm32f103, stm32f107

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Backup data register (BKP_DR)
pub mod DR1 {

    /// Backup data
    pub mod D {
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

/// Backup data register (BKP_DR)
pub mod DR2 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR3 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR4 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR5 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR6 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR7 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR8 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR9 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR10 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR11 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR12 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR13 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR14 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR15 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR16 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR17 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR18 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR19 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR20 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR21 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR22 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR23 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR24 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR25 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR26 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR27 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR28 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR29 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR30 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR31 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR32 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR33 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR34 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR35 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR36 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR37 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR38 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR39 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR40 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR41 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR42 {
    pub use super::DR1::D;
}

/// RTC clock calibration register (BKP_RTCCR)
pub mod RTCCR {

    /// Calibration value
    pub mod CAL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Calibration Clock Output
    pub mod CCO {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Alarm or second output enable
    pub mod ASOE {
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

            /// 0b0: Disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Setting this bit outputs either the RTC Alarm pulse signal or the Second pulse signal on the TAMPER pin depending on the ASOS bit
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Alarm or second output selection
    pub mod ASOS {
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

            /// 0b0: RTC Alarm pulse output selected
            pub const Alarm: u32 = 0b0;

            /// 0b1: RTC Second pulse output selected
            pub const Second: u32 = 0b1;
        }
    }
}

/// Backup control register (BKP_CR)
pub mod CR {

    /// Tamper pin enable
    pub mod TPE {
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

            /// 0b0: The TAMPER pin is free for general purpose I/O
            pub const General: u32 = 0b0;

            /// 0b1: Tamper alternate I/O function is activated
            pub const Alternate: u32 = 0b1;
        }
    }

    /// Tamper pin active level
    pub mod TPAL {
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

            /// 0b0: A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)
            pub const High: u32 = 0b0;

            /// 0b1: A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)
            pub const Low: u32 = 0b1;
        }
    }
}

/// BKP_CSR control/status register (BKP_CSR)
pub mod CSR {

    /// Clear Tamper event
    pub mod CTE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Reset the TEF Tamper event flag (and the Tamper detector)
            pub const Reset: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear Tamper Interrupt
    pub mod CTI {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the Tamper interrupt and the TIF Tamper interrupt flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Tamper Pin interrupt enable
    pub mod TPIE {
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

            /// 0b1: Tamper interrupt enabled (the TPE bit must also be set in the BKP_CR register
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Tamper Event Flag
    pub mod TEF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Tamper Interrupt Flag
    pub mod TIF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
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
    /// Backup data register (BKP_DR)
    pub DR1: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR2: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR3: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR4: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR5: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR6: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR7: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR8: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR9: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR10: RWRegister<u32>,

    /// RTC clock calibration register (BKP_RTCCR)
    pub RTCCR: RWRegister<u32>,

    /// Backup control register (BKP_CR)
    pub CR: RWRegister<u32>,

    /// BKP_CSR control/status register (BKP_CSR)
    pub CSR: RWRegister<u32>,

    _reserved1: [u8; 8],

    /// Backup data register (BKP_DR)
    pub BKP_DR11: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR12: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR13: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR14: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR15: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR16: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR17: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR18: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR19: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR20: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR21: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR22: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR23: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR24: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR25: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR26: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR27: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR28: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR29: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR30: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR31: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR32: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR33: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR34: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR35: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR36: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR37: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR38: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR39: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR40: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR41: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR42: RWRegister<u32>,
}
pub struct ResetValues {
    pub DR1: u32,
    pub DR2: u32,
    pub DR3: u32,
    pub DR4: u32,
    pub DR5: u32,
    pub DR6: u32,
    pub DR7: u32,
    pub DR8: u32,
    pub DR9: u32,
    pub DR10: u32,
    pub RTCCR: u32,
    pub CR: u32,
    pub CSR: u32,
    pub BKP_DR11: u32,
    pub BKP_DR12: u32,
    pub BKP_DR13: u32,
    pub BKP_DR14: u32,
    pub BKP_DR15: u32,
    pub BKP_DR16: u32,
    pub BKP_DR17: u32,
    pub BKP_DR18: u32,
    pub BKP_DR19: u32,
    pub BKP_DR20: u32,
    pub BKP_DR21: u32,
    pub BKP_DR22: u32,
    pub BKP_DR23: u32,
    pub BKP_DR24: u32,
    pub BKP_DR25: u32,
    pub BKP_DR26: u32,
    pub BKP_DR27: u32,
    pub BKP_DR28: u32,
    pub BKP_DR29: u32,
    pub BKP_DR30: u32,
    pub BKP_DR31: u32,
    pub BKP_DR32: u32,
    pub BKP_DR33: u32,
    pub BKP_DR34: u32,
    pub BKP_DR35: u32,
    pub BKP_DR36: u32,
    pub BKP_DR37: u32,
    pub BKP_DR38: u32,
    pub BKP_DR39: u32,
    pub BKP_DR40: u32,
    pub BKP_DR41: u32,
    pub BKP_DR42: u32,
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
