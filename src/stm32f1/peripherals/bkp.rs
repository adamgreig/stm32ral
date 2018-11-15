#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Backup registers
//!
//! Used by: stm32f100, stm32f101, stm32f102, stm32f103, stm32f107

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use RWRegister;

/// Backup data register (BKP_DR)
pub mod DR1 {

    /// Backup data
    pub mod D1 {
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

    /// Backup data
    pub mod D2 {
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
pub mod DR3 {

    /// Backup data
    pub mod D3 {
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
pub mod DR4 {

    /// Backup data
    pub mod D4 {
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
pub mod DR5 {

    /// Backup data
    pub mod D5 {
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
pub mod DR6 {

    /// Backup data
    pub mod D6 {
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
pub mod DR7 {

    /// Backup data
    pub mod D7 {
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
pub mod DR8 {

    /// Backup data
    pub mod D8 {
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
pub mod DR9 {

    /// Backup data
    pub mod D9 {
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
pub mod DR10 {

    /// Backup data
    pub mod D10 {
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
pub mod DR11 {

    /// Backup data
    pub mod DR11 {
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
pub mod DR12 {

    /// Backup data
    pub mod DR12 {
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
pub mod DR13 {

    /// Backup data
    pub mod DR13 {
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
pub mod DR14 {

    /// Backup data
    pub mod D14 {
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
pub mod DR15 {

    /// Backup data
    pub mod D15 {
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
pub mod DR16 {

    /// Backup data
    pub mod D16 {
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
pub mod DR17 {

    /// Backup data
    pub mod D17 {
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
pub mod DR18 {

    /// Backup data
    pub mod D18 {
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
pub mod DR19 {

    /// Backup data
    pub mod D19 {
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
pub mod DR20 {

    /// Backup data
    pub mod D20 {
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
pub mod DR21 {

    /// Backup data
    pub mod D21 {
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
pub mod DR22 {

    /// Backup data
    pub mod D22 {
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
pub mod DR23 {

    /// Backup data
    pub mod D23 {
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
pub mod DR24 {

    /// Backup data
    pub mod D24 {
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
pub mod DR25 {

    /// Backup data
    pub mod D25 {
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
pub mod DR26 {

    /// Backup data
    pub mod D26 {
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
pub mod DR27 {

    /// Backup data
    pub mod D27 {
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
pub mod DR28 {

    /// Backup data
    pub mod D28 {
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
pub mod DR29 {

    /// Backup data
    pub mod D29 {
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
pub mod DR30 {

    /// Backup data
    pub mod D30 {
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
pub mod DR31 {

    /// Backup data
    pub mod D31 {
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
pub mod DR32 {

    /// Backup data
    pub mod D32 {
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
pub mod DR33 {

    /// Backup data
    pub mod D33 {
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
pub mod DR34 {

    /// Backup data
    pub mod D34 {
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
pub mod DR35 {

    /// Backup data
    pub mod D35 {
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
pub mod DR36 {

    /// Backup data
    pub mod D36 {
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
pub mod DR37 {

    /// Backup data
    pub mod D37 {
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
pub mod DR38 {

    /// Backup data
    pub mod D38 {
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
pub mod DR39 {

    /// Backup data
    pub mod D39 {
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
pub mod DR40 {

    /// Backup data
    pub mod D40 {
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
pub mod DR41 {

    /// Backup data
    pub mod D41 {
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
pub mod DR42 {

    /// Backup data
    pub mod D42 {
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
        /// Read-write values (empty)
        pub mod RW {}
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
        /// Read-write values (empty)
        pub mod RW {}
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
        /// Read-write values (empty)
        pub mod RW {}
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
        /// Read-write values (empty)
        pub mod RW {}
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
        /// Write-only values (empty)
        pub mod W {}
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
        /// Write-only values (empty)
        pub mod W {}
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
        /// Read-write values (empty)
        pub mod RW {}
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

    _reserved1: [u32; 2],

    /// Backup data register (BKP_DR)
    pub DR11: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR12: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR13: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR14: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR15: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR16: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR17: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR18: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR19: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR20: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR21: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR22: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR23: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR24: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR25: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR26: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR27: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR28: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR29: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR30: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR31: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR32: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR33: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR34: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR35: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR36: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR37: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR38: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR39: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR40: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR41: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR42: RWRegister<u32>,
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
    pub DR11: u32,
    pub DR12: u32,
    pub DR13: u32,
    pub DR14: u32,
    pub DR15: u32,
    pub DR16: u32,
    pub DR17: u32,
    pub DR18: u32,
    pub DR19: u32,
    pub DR20: u32,
    pub DR21: u32,
    pub DR22: u32,
    pub DR23: u32,
    pub DR24: u32,
    pub DR25: u32,
    pub DR26: u32,
    pub DR27: u32,
    pub DR28: u32,
    pub DR29: u32,
    pub DR30: u32,
    pub DR31: u32,
    pub DR32: u32,
    pub DR33: u32,
    pub DR34: u32,
    pub DR35: u32,
    pub DR36: u32,
    pub DR37: u32,
    pub DR38: u32,
    pub DR39: u32,
    pub DR40: u32,
    pub DR41: u32,
    pub DR42: u32,
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
