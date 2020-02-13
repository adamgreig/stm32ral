#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Embedded Flash Memory
//!
//! Used by: stm32h747cm4, stm32h747cm7, stm32h757cm7

use crate::{RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// FLASH access control register
pub mod ACR {

    /// Flash signal delay
    pub mod WRHIGHFREQ {
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

    /// Read latency
    pub mod LATENCY {
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

/// FLASH option key register
pub mod OPTKEYR {

    /// FLASH option bytes control access unlock key
    pub mod OPTKEYR {
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

/// FLASH option control register
pub mod OPTCR {

    /// Bank swapping option configuration bit
    pub mod SWAP_BANK {
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

    /// Option byte change error interrupt enable bit
    pub mod OPTCHANGEERRIE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// mass erase request
    pub mod MER {
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

    /// Option byte start change option configuration bit
    pub mod OPTSTART {
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

    /// FLASH
    pub mod OPTLOCK {
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

/// FLASH option status register
pub mod OPTSR_CUR {

    /// Bank swapping option status bit
    pub mod SWAP_BANK_OPT {
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

    /// Option byte change error flag
    pub mod OPTCHANGEERR {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I
    pub mod IO_HSLV {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// D2 domain DStandby entry reset option status bit
    pub mod NRST_STBY_D2 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// D2 domain DStop entry reset option status bit
    pub mod NRST_STOP_D2 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Arm Cortex
    pub mod BOOT_CM7 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Arm Cortex
    pub mod BOOT_CM4 {
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

    /// Security enable option status bit
    pub mod SECURITY {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ST RAM size option status
    pub mod ST_RAM_SIZE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (2 bits: 0b11 << 19)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IWDG Standby mode freeze option status bit
    pub mod IWDG_FZ_SDBY {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IWDG Stop mode freeze option status bit
    pub mod IWDG_FZ_STOP {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Readout protection level option status byte
    pub mod RDP {
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

    /// D1 domain DStandby entry reset option status bit
    pub mod RST_STDY_D1 {
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

    /// D1 domain DStop entry reset option status bit
    pub mod NRST_STOP_D1 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IWDG2 control mode option status bit
    pub mod IWDG2_SW {
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

    /// IWDG control mode option status bit
    pub mod IWDG_SW {
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

    /// Brownout level option status bit
    pub mod BOR_LEV {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Option byte change ongoing flag
    pub mod OPT_BUSY {
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

/// FLASH option status register
pub mod OPTSR_PRG {

    /// Bank swapping option configuration bit
    pub mod SWAP_BANK_OPT {
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

    /// I
    pub mod IO_HSLV {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// D2 domain DStandby entry reset option configuration bit
    pub mod NRST_STBY_D2 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// D2 domain DStop entry reset option configuration bit
    pub mod NRST_STOP_D2 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Arm Cortex
    pub mod BOOT_CM7 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Arm Cortex
    pub mod BOOT_CM4 {
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

    /// Security enable option configuration bit
    pub mod SECURITY {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ST RAM size option configuration bits
    pub mod ST_RAM_SIZE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (2 bits: 0b11 << 19)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IWDG Standby mode freeze option configuration bit
    pub mod IWDG_FZ_SDBY {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IWDG Stop mode freeze option configuration bit
    pub mod IWDG_FZ_STOP {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Readout protection level option configuration bits
    pub mod RDP {
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

    /// D1 domain DStandby entry reset option configuration bit
    pub mod NRST_STDY_D1 {
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

    /// D1 domain DStop entry reset option configuration bit
    pub mod NRST_STOP_D1 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IWDG2 control mode option configuration bit
    pub mod IWDG2_SW {
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

    /// IWDG control mode option configuration bit
    pub mod IWDG_SW {
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

    /// Brownout level option configuration bit
    pub mod BOR_LEV {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// FLASH option clear control register
pub mod OPTCCR {

    /// OPTCHANGEERR reset bit
    pub mod CLR_OPTCHANGEERR {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// FLASH register boot address for Arm Cortex-M7 core
pub mod BOOT7_CURR {

    /// Arm Cortex-M7 boot address 1
    pub mod BOOT_CM7_ADD1 {
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

    /// Arm Cortex-M7 boot address 0
    pub mod BOOT_CM7_ADD0 {
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

/// FLASH register boot address for Arm Cortex-M7 core
pub mod BOOT7_PRGR {
    pub use super::BOOT7_CURR::BOOT_CM7_ADD0;
    pub use super::BOOT7_CURR::BOOT_CM7_ADD1;
}

/// FLASH register boot address for Arm Cortex-M4 core
pub mod BOOT4_CURR {

    /// Arm Cortex-M4 boot address 1
    pub mod BOOT_CM4_ADD1 {
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

    /// Arm Cortex-M4 boot address 0
    pub mod BOOT_CM4_ADD0 {
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

/// FLASH register boot address for Arm Cortex-M4 core
pub mod BOOT4_PRGR {
    pub use super::BOOT4_CURR::BOOT_CM4_ADD0;
    pub use super::BOOT4_CURR::BOOT_CM4_ADD1;
}

/// FLASH CRC data register
pub mod CRCDATAR {

    /// CRC result
    pub mod CRC_DATA {
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

/// FLASH key register for bank 1
pub mod KEYR1 {

    /// Bank access configuration unlock key
    pub mod KEYR {
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

/// FLASH control register for bank 1
pub mod CR1 {

    /// Bank 1 CRC read error interrupt enable bit
    pub mod CRCRDERRIE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 CRC end of calculation interrupt enable bit
    pub mod CRCENDIE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 ECC double detection error interrupt enable bit
    pub mod DBECCERRIE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 ECC single correction error interrupt enable bit
    pub mod SNECCERRIE {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 secure error interrupt enable bit
    pub mod RDSERRIE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 read protection error interrupt enable bit
    pub mod RDPERRIE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 write
    pub mod OPERRIE {
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

    /// Bank 1 inconsistency error interrupt enable bit
    pub mod INCERRIE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 strobe error interrupt enable bit
    pub mod STRBERRIE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 programming sequence error interrupt enable bit
    pub mod PGSERRIE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 write protection error interrupt enable bit
    pub mod WRPERRIE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 end-of-program interrupt control bit
    pub mod EOPIE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 CRC control bit
    pub mod CRC_EN {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 sector erase selection number
    pub mod SNB {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 erase start control bit
    pub mod START {
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

    /// Bank 1 write forcing control bit
    pub mod FW {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 program size
    pub mod PSIZE {
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

    /// Bank 1 erase request
    pub mod BER {
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

    /// Bank 1 sector erase request
    pub mod SER {
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

    /// Bank 1 internal buffer control bit
    pub mod PG {
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

    /// Bank 1 configuration lock bit
    pub mod LOCK {
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

/// FLASH status register for bank 1
pub mod SR1 {

    /// Bank 1 CRC read error flag
    pub mod CRCRDERR1 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 CRC end of calculation flag
    pub mod CRCEND1 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 ECC double detection error flag
    pub mod DBECCERR1 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 single correction error flag
    pub mod SNECCERR1 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 secure error flag
    pub mod RDSERR1 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 read protection error flag
    pub mod RDPERR1 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 write
    pub mod OPERR1 {
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

    /// Bank 1 inconsistency error flag
    pub mod INCERR1 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 strobe error flag
    pub mod STRBERR1 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 programming sequence error flag
    pub mod PGSERR1 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 write protection error flag
    pub mod WRPERR1 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 end-of-program flag
    pub mod EOP1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 CRC busy flag
    pub mod CRC_BUSY1 {
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

    /// Bank 1 wait queue flag
    pub mod QW1 {
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

    /// Bank 1 write buffer not empty flag
    pub mod WBNE1 {
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

    /// Bank 1 busy flag
    pub mod BSY1 {
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

/// FLASH clear control register for bank 1
pub mod CCR1 {

    /// Bank 1 CRCRDERR1 flag clear bit
    pub mod CLR_CRCRDERR1 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 CRCEND1 flag clear bit
    pub mod CLR_CRCEND1 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 DBECCERR1 flag clear bit
    pub mod CLR_DBECCERR1 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 SNECCERR1 flag clear bit
    pub mod CLR_SNECCERR1 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 RDSERR1 flag clear bit
    pub mod CLR_RDSERR1 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 RDPERR1 flag clear bit
    pub mod CLR_RDPERR1 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 OPERR1 flag clear bit
    pub mod CLR_OPERR1 {
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

    /// Bank 1 INCERR1 flag clear bit
    pub mod CLR_INCERR1 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 STRBERR1 flag clear bit
    pub mod CLR_STRBERR1 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 PGSERR1 flag clear bit
    pub mod CLR_PGSERR1 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 WRPERR1 flag clear bit
    pub mod CLR_WRPERR1 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 EOP1 flag clear bit
    pub mod CLR_EOP1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// FLASH protection address for bank 1
pub mod PRAR_CUR1 {

    /// Bank 1 PCROP protected erase enable option status bit
    pub mod DMEP1 {
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

    /// Bank 1 PCROP area end status bits
    pub mod PROT_AREA_END1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 PCROP area start status bits
    pub mod PROT_AREA_START1 {
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

/// FLASH protection address for bank 1
pub mod PRAR_PRG1 {
    pub use super::PRAR_CUR1::DMEP1;
    pub use super::PRAR_CUR1::PROT_AREA_END1;
    pub use super::PRAR_CUR1::PROT_AREA_START1;
}

/// FLASH secure address for bank 1
pub mod SCAR_CUR1 {

    /// Bank 1 secure access protected erase enable option status bit
    pub mod DMES1 {
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

    /// Bank 1 secure-only area end status bits
    pub mod SEC_AREA_END1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 secure-only area start status bits
    pub mod SEC_AREA_START1 {
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

/// FLASH secure address for bank 1
pub mod SCAR_PRG1 {
    pub use super::SCAR_CUR1::DMES1;
    pub use super::SCAR_CUR1::SEC_AREA_END1;
    pub use super::SCAR_CUR1::SEC_AREA_START1;
}

/// FLASH write sector protection for bank 1
pub mod WPSN_CURR1 {

    /// Bank 1 sector write protection option status byte
    pub mod WRPSn1 {
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

/// FLASH write sector protection for bank 1
pub mod WPSN_PRGR1 {
    pub use super::WPSN_CURR1::WRPSn1;
}

/// FLASH CRC control register for bank 1
pub mod CRCCR1 {

    /// Bank 1 CRC select bit
    pub mod ALL_BANK {
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

    /// Bank 1 CRC burst size
    pub mod CRC_BURST {
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

    /// Bank 1 CRC clear bit
    pub mod CLEAN_CRC {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 CRC start bit
    pub mod START_CRC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 CRC sector list clear bit
    pub mod CLEAN_SECT {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 CRC sector select bit
    pub mod ADD_SECT {
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

    /// Bank 1 CRC sector mode select bit
    pub mod CRC_BY_SECT {
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

    /// Bank 1 CRC sector number
    pub mod CRC_SECT {
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

/// FLASH CRC start address register for bank 1
pub mod CRCSADDR1 {

    /// CRC start address on bank 1
    pub mod CRC_START_ADDR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (18 bits: 0x3ffff << 2)
        pub const mask: u32 = 0x3ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// FLASH CRC end address register for bank 1
pub mod CRCEADDR1 {

    /// CRC end address on bank 1
    pub mod CRC_END_ADDR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (18 bits: 0x3ffff << 2)
        pub const mask: u32 = 0x3ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// FLASH ECC fail address for bank 1
pub mod FAR1 {

    /// Bank 1 ECC error address
    pub mod FAIL_ECC_ADDR1 {
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

/// FLASH key register for bank 1
pub mod KEYR2 {
    pub use super::KEYR1::KEYR;
}

/// FLASH control register for bank 1
pub mod CR2 {
    pub use super::CR1::BER;
    pub use super::CR1::CRCENDIE;
    pub use super::CR1::CRCRDERRIE;
    pub use super::CR1::CRC_EN;
    pub use super::CR1::DBECCERRIE;
    pub use super::CR1::EOPIE;
    pub use super::CR1::FW;
    pub use super::CR1::INCERRIE;
    pub use super::CR1::LOCK;
    pub use super::CR1::OPERRIE;
    pub use super::CR1::PG;
    pub use super::CR1::PGSERRIE;
    pub use super::CR1::PSIZE;
    pub use super::CR1::RDPERRIE;
    pub use super::CR1::RDSERRIE;
    pub use super::CR1::SER;
    pub use super::CR1::SNB;
    pub use super::CR1::SNECCERRIE;
    pub use super::CR1::START;
    pub use super::CR1::STRBERRIE;
    pub use super::CR1::WRPERRIE;
}

/// FLASH status register for bank 1
pub mod SR2 {
    pub use super::SR1::BSY1;
    pub use super::SR1::CRCEND1;
    pub use super::SR1::CRCRDERR1;
    pub use super::SR1::CRC_BUSY1;
    pub use super::SR1::DBECCERR1;
    pub use super::SR1::EOP1;
    pub use super::SR1::INCERR1;
    pub use super::SR1::OPERR1;
    pub use super::SR1::PGSERR1;
    pub use super::SR1::QW1;
    pub use super::SR1::RDPERR1;
    pub use super::SR1::RDSERR1;
    pub use super::SR1::SNECCERR1;
    pub use super::SR1::STRBERR1;
    pub use super::SR1::WBNE1;
    pub use super::SR1::WRPERR1;
}

/// FLASH clear control register for bank 1
pub mod CCR2 {
    pub use super::CCR1::CLR_CRCEND1;
    pub use super::CCR1::CLR_CRCRDERR1;
    pub use super::CCR1::CLR_DBECCERR1;
    pub use super::CCR1::CLR_EOP1;
    pub use super::CCR1::CLR_INCERR1;
    pub use super::CCR1::CLR_OPERR1;
    pub use super::CCR1::CLR_PGSERR1;
    pub use super::CCR1::CLR_RDPERR1;
    pub use super::CCR1::CLR_RDSERR1;
    pub use super::CCR1::CLR_SNECCERR1;
    pub use super::CCR1::CLR_STRBERR1;
    pub use super::CCR1::CLR_WRPERR1;
}

/// FLASH protection address for bank 1
pub mod PRAR_CUR2 {
    pub use super::PRAR_CUR1::DMEP1;
    pub use super::PRAR_CUR1::PROT_AREA_END1;
    pub use super::PRAR_CUR1::PROT_AREA_START1;
}

/// FLASH protection address for bank 1
pub mod PRAR_PRG2 {
    pub use super::PRAR_CUR1::DMEP1;
    pub use super::PRAR_CUR1::PROT_AREA_END1;
    pub use super::PRAR_CUR1::PROT_AREA_START1;
}

/// FLASH secure address for bank 1
pub mod SCAR_CUR2 {
    pub use super::SCAR_CUR1::DMES1;
    pub use super::SCAR_CUR1::SEC_AREA_END1;
    pub use super::SCAR_CUR1::SEC_AREA_START1;
}

/// FLASH secure address for bank 1
pub mod SCAR_PRG2 {
    pub use super::SCAR_CUR1::DMES1;
    pub use super::SCAR_CUR1::SEC_AREA_END1;
    pub use super::SCAR_CUR1::SEC_AREA_START1;
}

/// FLASH write sector protection for bank 1
pub mod WPSN_CURR2 {
    pub use super::WPSN_CURR1::WRPSn1;
}

/// FLASH write sector protection for bank 1
pub mod WPSN_PRGR2 {
    pub use super::WPSN_CURR1::WRPSn1;
}

/// FLASH CRC control register for bank 1
pub mod CRCCR2 {
    pub use super::CRCCR1::ADD_SECT;
    pub use super::CRCCR1::ALL_BANK;
    pub use super::CRCCR1::CLEAN_CRC;
    pub use super::CRCCR1::CLEAN_SECT;
    pub use super::CRCCR1::CRC_BURST;
    pub use super::CRCCR1::CRC_BY_SECT;
    pub use super::CRCCR1::CRC_SECT;
    pub use super::CRCCR1::START_CRC;
}

/// FLASH CRC start address register for bank 1
pub mod CRCSADDR2 {
    pub use super::CRCSADDR1::CRC_START_ADDR;
}

/// FLASH CRC end address register for bank 1
pub mod CRCEADDR2 {
    pub use super::CRCEADDR1::CRC_END_ADDR;
}

/// FLASH ECC fail address for bank 1
pub mod FAR2 {
    pub use super::FAR1::FAIL_ECC_ADDR1;
}
pub struct RegisterBlock {
    /// FLASH access control register
    pub ACR: RWRegister<u32>,

    /// FLASH key register for bank 1
    pub KEYR1: WORegister<u32>,

    /// FLASH option key register
    pub OPTKEYR: WORegister<u32>,

    /// FLASH control register for bank 1
    pub CR1: RWRegister<u32>,

    /// FLASH status register for bank 1
    pub SR1: RWRegister<u32>,

    /// FLASH clear control register for bank 1
    pub CCR1: RWRegister<u32>,

    /// FLASH option control register
    pub OPTCR: RWRegister<u32>,

    /// FLASH option status register
    pub OPTSR_CUR: RWRegister<u32>,

    /// FLASH option status register
    pub OPTSR_PRG: RWRegister<u32>,

    /// FLASH option clear control register
    pub OPTCCR: RWRegister<u32>,

    /// FLASH protection address for bank 1
    pub PRAR_CUR1: RWRegister<u32>,

    /// FLASH protection address for bank 1
    pub PRAR_PRG1: RWRegister<u32>,

    /// FLASH secure address for bank 1
    pub SCAR_CUR1: RWRegister<u32>,

    /// FLASH secure address for bank 1
    pub SCAR_PRG1: RWRegister<u32>,

    /// FLASH write sector protection for bank 1
    pub WPSN_CURR1: RWRegister<u32>,

    /// FLASH write sector protection for bank 1
    pub WPSN_PRGR1: RWRegister<u32>,

    /// FLASH register boot address for Arm Cortex-M7 core
    pub BOOT7_CURR: RWRegister<u32>,

    /// FLASH register boot address for Arm Cortex-M7 core
    pub BOOT7_PRGR: RWRegister<u32>,

    /// FLASH register boot address for Arm Cortex-M4 core
    pub BOOT4_CURR: RWRegister<u32>,

    /// FLASH register boot address for Arm Cortex-M4 core
    pub BOOT4_PRGR: RWRegister<u32>,

    /// FLASH CRC control register for bank 1
    pub CRCCR1: RWRegister<u32>,

    /// FLASH CRC start address register for bank 1
    pub CRCSADDR1: RWRegister<u32>,

    /// FLASH CRC end address register for bank 1
    pub CRCEADDR1: RWRegister<u32>,

    /// FLASH CRC data register
    pub CRCDATAR: RWRegister<u32>,

    /// FLASH ECC fail address for bank 1
    pub FAR1: RWRegister<u32>,

    _reserved1: [u32; 40],

    /// FLASH key register for bank 1
    pub KEYR2: WORegister<u32>,

    _reserved2: [u32; 1],

    /// FLASH control register for bank 1
    pub CR2: RWRegister<u32>,

    /// FLASH status register for bank 1
    pub SR2: RWRegister<u32>,

    /// FLASH clear control register for bank 1
    pub CCR2: RWRegister<u32>,

    _reserved3: [u32; 4],

    /// FLASH protection address for bank 1
    pub PRAR_CUR2: RWRegister<u32>,

    /// FLASH protection address for bank 1
    pub PRAR_PRG2: RWRegister<u32>,

    /// FLASH secure address for bank 1
    pub SCAR_CUR2: RWRegister<u32>,

    /// FLASH secure address for bank 1
    pub SCAR_PRG2: RWRegister<u32>,

    /// FLASH write sector protection for bank 1
    pub WPSN_CURR2: RWRegister<u32>,

    /// FLASH write sector protection for bank 1
    pub WPSN_PRGR2: RWRegister<u32>,

    _reserved4: [u32; 4],

    /// FLASH CRC control register for bank 1
    pub CRCCR2: RWRegister<u32>,

    /// FLASH CRC start address register for bank 1
    pub CRCSADDR2: RWRegister<u32>,

    /// FLASH CRC end address register for bank 1
    pub CRCEADDR2: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// FLASH ECC fail address for bank 1
    pub FAR2: RWRegister<u32>,
}
pub struct ResetValues {
    pub ACR: u32,
    pub KEYR1: u32,
    pub OPTKEYR: u32,
    pub CR1: u32,
    pub SR1: u32,
    pub CCR1: u32,
    pub OPTCR: u32,
    pub OPTSR_CUR: u32,
    pub OPTSR_PRG: u32,
    pub OPTCCR: u32,
    pub PRAR_CUR1: u32,
    pub PRAR_PRG1: u32,
    pub SCAR_CUR1: u32,
    pub SCAR_PRG1: u32,
    pub WPSN_CURR1: u32,
    pub WPSN_PRGR1: u32,
    pub BOOT7_CURR: u32,
    pub BOOT7_PRGR: u32,
    pub BOOT4_CURR: u32,
    pub BOOT4_PRGR: u32,
    pub CRCCR1: u32,
    pub CRCSADDR1: u32,
    pub CRCEADDR1: u32,
    pub CRCDATAR: u32,
    pub FAR1: u32,
    pub KEYR2: u32,
    pub CR2: u32,
    pub SR2: u32,
    pub CCR2: u32,
    pub PRAR_CUR2: u32,
    pub PRAR_PRG2: u32,
    pub SCAR_CUR2: u32,
    pub SCAR_PRG2: u32,
    pub WPSN_CURR2: u32,
    pub WPSN_PRGR2: u32,
    pub CRCCR2: u32,
    pub CRCSADDR2: u32,
    pub CRCEADDR2: u32,
    pub FAR2: u32,
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
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}
