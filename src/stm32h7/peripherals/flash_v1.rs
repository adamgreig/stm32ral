#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Flash
//!
//! Used by: stm32h735, stm32h743, stm32h743v, stm32h753, stm32h753v

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Access control register
pub mod ACR {

    /// Read latency
    pub mod LATENCY {
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
}

/// Access control register
pub mod ACR_ {
    pub use super::ACR::LATENCY;
    pub use super::ACR::WRHIGHFREQ;
}

/// FLASH option key register
pub mod OPTKEYR {

    /// Unlock key option bytes
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

/// FLASH option key register
pub mod OPTKEYR_ {
    pub use super::OPTKEYR::OPTKEYR;
}

/// FLASH option control register
pub mod OPTCR {

    /// FLASH_OPTCR lock option configuration bit
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

    /// Flash mass erase enable bit
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

    /// Bank swapping configuration bit
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
}

/// FLASH option control register
pub mod OPTCR_ {
    pub use super::OPTCR::MER;
    pub use super::OPTCR::OPTCHANGEERRIE;
    pub use super::OPTCR::OPTLOCK;
    pub use super::OPTCR::OPTSTART;
    pub use super::OPTCR::SWAP_BANK;
}

/// FLASH option status register
pub mod OPTSR_CUR_ {

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

    /// IWDG1 control option status bit
    pub mod IWDG1_HW {
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

    /// D1 DStop entry reset option status bit
    pub mod nRST_STOP_D1 {
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

    /// D1 DStandby entry reset option status bit
    pub mod nRST_STBY_D1 {
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

    /// IWDG Stop mode freeze option status bit
    pub mod FZ_IWDG_STOP {
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

    /// IWDG Standby mode freeze option status bit
    pub mod FZ_IWDG_SDBY {
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

    /// DTCM RAM size option status
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

    /// User option bit 1
    pub mod RSS1 {
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

    /// Device personalization status bit
    pub mod PERSO_OK {
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

    /// I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)
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
}

/// FLASH option status register
pub mod OPTSR_CUR {
    pub use super::OPTSR_CUR_::nRST_STBY_D1;
    pub use super::OPTSR_CUR_::nRST_STOP_D1;
    pub use super::OPTSR_CUR_::BOR_LEV;
    pub use super::OPTSR_CUR_::FZ_IWDG_SDBY;
    pub use super::OPTSR_CUR_::FZ_IWDG_STOP;
    pub use super::OPTSR_CUR_::IO_HSLV;
    pub use super::OPTSR_CUR_::IWDG1_HW;
    pub use super::OPTSR_CUR_::OPTCHANGEERR;
    pub use super::OPTSR_CUR_::OPT_BUSY;
    pub use super::OPTSR_CUR_::PERSO_OK;
    pub use super::OPTSR_CUR_::RDP;
    pub use super::OPTSR_CUR_::RSS1;
    pub use super::OPTSR_CUR_::SECURITY;
    pub use super::OPTSR_CUR_::ST_RAM_SIZE;
    pub use super::OPTSR_CUR_::SWAP_BANK_OPT;
}

/// FLASH option status register
pub mod OPTSR_PRG {

    /// BOR reset level option configuration bits
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

    /// IWDG1 option configuration bit
    pub mod IWDG1_HW {
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

    /// Option byte erase after D1 DStop option configuration bit
    pub mod nRST_STOP_D1 {
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

    /// Option byte erase after D1 DStandby option configuration bit
    pub mod nRST_STBY_D1 {
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

    /// Readout protection level option configuration byte
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

    /// IWDG Stop mode freeze option configuration bit
    pub mod FZ_IWDG_STOP {
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

    /// IWDG Standby mode freeze option configuration bit
    pub mod FZ_IWDG_SDBY {
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

    /// DTCM size select option configuration bits
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

    /// Security option configuration bit
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

    /// User option configuration bit 1
    pub mod RSS1 {
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

    /// User option configuration bit 2
    pub mod RSS2 {
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

    /// I/O high-speed at low-voltage (PRODUCT_BELOW_25V)
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
}

/// FLASH option status register
pub mod OPTSR_PRG_ {
    pub use super::OPTSR_PRG::nRST_STBY_D1;
    pub use super::OPTSR_PRG::nRST_STOP_D1;
    pub use super::OPTSR_PRG::BOR_LEV;
    pub use super::OPTSR_PRG::FZ_IWDG_SDBY;
    pub use super::OPTSR_PRG::FZ_IWDG_STOP;
    pub use super::OPTSR_PRG::IO_HSLV;
    pub use super::OPTSR_PRG::IWDG1_HW;
    pub use super::OPTSR_PRG::RDP;
    pub use super::OPTSR_PRG::RSS1;
    pub use super::OPTSR_PRG::RSS2;
    pub use super::OPTSR_PRG::SECURITY;
    pub use super::OPTSR_PRG::ST_RAM_SIZE;
    pub use super::OPTSR_PRG::SWAP_BANK_OPT;
}

/// FLASH option clear control register
pub mod OPTCCR_ {

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

/// FLASH option clear control register
pub mod OPTCCR {
    pub use super::OPTCCR_::CLR_OPTCHANGEERR;
}

/// FLASH register with boot address
pub mod BOOT_CURR {

    /// Boot address 0
    pub mod BOOT_ADD0 {
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

    /// Boot address 1
    pub mod BOOT_ADD1 {
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

/// FLASH register with boot address
pub mod BOOT_PRGR {

    /// Boot address 0
    pub mod BOOT_ADD0 {
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

    /// Boot address 1
    pub mod BOOT_ADD1 {
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

    /// Bank 1 access configuration unlock key
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

    /// Bank 1 program enable bit
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

    /// Bank 1 bank or sector erase start control bit
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

    /// Bank 1 write/erase error interrupt enable bit
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

    /// Bank 1 end of CRC calculation interrupt enable bit
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
}

/// FLASH status register for bank 1
pub mod SR1 {

    /// Bank 1 ongoing program flag
    pub mod BSY {
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

    /// Bank 1 write buffer not empty flag
    pub mod WBNE {
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

    /// Bank 1 wait queue flag
    pub mod QW {
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

    /// Bank 1 CRC busy flag
    pub mod CRC_BUSY {
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

    /// Bank 1 end-of-program flag
    pub mod EOP {
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

    /// Bank 1 write protection error flag
    pub mod WRPERR {
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

    /// Bank 1 programming sequence error flag
    pub mod PGSERR {
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

    /// Bank 1 strobe error flag
    pub mod STRBERR {
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

    /// Bank 1 inconsistency error flag
    pub mod INCERR {
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

    /// Bank 1 write/erase error flag
    pub mod OPERR {
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

    /// Bank 1 read protection error flag
    pub mod RDPERR {
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

    /// Bank 1 secure error flag
    pub mod RDSERR {
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

    /// Bank 1 ECC double detection error flag
    pub mod DBECCERR {
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

    /// Bank 1 CRC-complete flag
    pub mod CRCEND {
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
}

/// FLASH clear control register for bank 1
pub mod CCR1 {

    /// Bank 1 EOP1 flag clear bit
    pub mod CLR_EOP {
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

    /// Bank 1 WRPERR1 flag clear bit
    pub mod CLR_WRPERR {
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

    /// Bank 1 PGSERR1 flag clear bi
    pub mod CLR_PGSERR {
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

    /// Bank 1 STRBERR1 flag clear bit
    pub mod CLR_STRBERR {
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

    /// Bank 1 INCERR1 flag clear bit
    pub mod CLR_INCERR {
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

    /// Bank 1 OPERR1 flag clear bit
    pub mod CLR_OPERR {
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

    /// Bank 1 RDPERR1 flag clear bit
    pub mod CLR_RDPERR {
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

    /// Bank 1 RDSERR1 flag clear bit
    pub mod CLR_RDSERR {
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

    /// Bank 1 SNECCERR1 flag clear bit
    pub mod CLR_SNECCERR {
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

    /// Bank 1 DBECCERR1 flag clear bit
    pub mod CLR_DBECCERR {
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

    /// Bank 1 CRCEND1 flag clear bit
    pub mod CLR_CRCEND {
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
}

/// FLASH protection address for bank 1
pub mod PRAR_CUR1 {

    /// Bank 1 lowest PCROP protected address
    pub mod PROT_AREA_START {
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

    /// Bank 1 highest PCROP protected address
    pub mod PROT_AREA_END {
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

    /// Bank 1 PCROP protected erase enable option status bit
    pub mod DMEP {
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
}

/// FLASH protection address for bank 1
pub mod PRAR_PRG1 {

    /// Bank 1 lowest PCROP protected address configuration
    pub mod PROT_AREA_START {
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

    /// Bank 1 highest PCROP protected address configuration
    pub mod PROT_AREA_END {
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

    /// Bank 1 PCROP protected erase enable option configuration bit
    pub mod DMEP {
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
}

/// FLASH secure address for bank 1
pub mod SCAR_CUR1 {

    /// Bank 1 lowest secure protected address
    pub mod SEC_AREA_START {
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

    /// Bank 1 highest secure protected address
    pub mod SEC_AREA_END {
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

    /// Bank 1 secure protected erase enable option status bit
    pub mod DMES {
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
}

/// FLASH secure address for bank 1
pub mod SCAR_PRG1 {
    pub use super::SCAR_CUR1::DMES;
    pub use super::SCAR_CUR1::SEC_AREA_END;
    pub use super::SCAR_CUR1::SEC_AREA_START;
}

/// FLASH write sector protection for bank 1
pub mod WPSN_CURR1 {

    /// Bank 1 sector write protection option status byte
    pub mod WRPSn {
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

    /// Bank 1 sector write protection configuration byte
    pub mod WRPSn {
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

/// FLASH CRC control register for bank 1
pub mod CRCCR1 {

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

    /// Bank 1 CRC select bit
    pub mod ALL_BANK {
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
}

/// FLASH CRC start address register for bank 1
pub mod CRCSADDR1 {

    /// CRC start address on bank 1
    pub mod CRC_START_ADDR {
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

/// FLASH CRC end address register for bank 1
pub mod CRCEADDR1 {

    /// CRC end address on bank 1
    pub mod CRC_END_ADDR {
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

/// FLASH ECC fail address for bank 1
pub mod FAR1 {

    /// Bank 1 ECC error address
    pub mod FAIL_ECC_ADDR {
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
    pub use super::SR1::BSY;
    pub use super::SR1::CRCEND;
    pub use super::SR1::CRC_BUSY;
    pub use super::SR1::DBECCERR;
    pub use super::SR1::EOP;
    pub use super::SR1::INCERR;
    pub use super::SR1::OPERR;
    pub use super::SR1::PGSERR;
    pub use super::SR1::QW;
    pub use super::SR1::RDPERR;
    pub use super::SR1::RDSERR;
    pub use super::SR1::SNECCERR1;
    pub use super::SR1::STRBERR;
    pub use super::SR1::WBNE;
    pub use super::SR1::WRPERR;
}

/// FLASH clear control register for bank 1
pub mod CCR2 {
    pub use super::CCR1::CLR_CRCEND;
    pub use super::CCR1::CLR_DBECCERR;
    pub use super::CCR1::CLR_EOP;
    pub use super::CCR1::CLR_INCERR;
    pub use super::CCR1::CLR_OPERR;
    pub use super::CCR1::CLR_PGSERR;
    pub use super::CCR1::CLR_RDPERR;
    pub use super::CCR1::CLR_RDSERR;
    pub use super::CCR1::CLR_SNECCERR;
    pub use super::CCR1::CLR_STRBERR;
    pub use super::CCR1::CLR_WRPERR;
}

/// FLASH protection address for bank 1
pub mod PRAR_CUR2 {
    pub use super::PRAR_CUR1::DMEP;
    pub use super::PRAR_CUR1::PROT_AREA_END;
    pub use super::PRAR_CUR1::PROT_AREA_START;
}

/// FLASH protection address for bank 1
pub mod PRAR_PRG2 {
    pub use super::PRAR_PRG1::DMEP;
    pub use super::PRAR_PRG1::PROT_AREA_END;
    pub use super::PRAR_PRG1::PROT_AREA_START;
}

/// FLASH secure address for bank 1
pub mod SCAR_CUR2 {
    pub use super::SCAR_CUR1::DMES;
    pub use super::SCAR_CUR1::SEC_AREA_END;
    pub use super::SCAR_CUR1::SEC_AREA_START;
}

/// FLASH secure address for bank 1
pub mod SCAR_PRG2 {
    pub use super::SCAR_CUR1::DMES;
    pub use super::SCAR_CUR1::SEC_AREA_END;
    pub use super::SCAR_CUR1::SEC_AREA_START;
}

/// FLASH write sector protection for bank 1
pub mod WPSN_CURR2 {
    pub use super::WPSN_CURR1::WRPSn;
}

/// FLASH write sector protection for bank 1
pub mod WPSN_PRGR2 {
    pub use super::WPSN_PRGR1::WRPSn;
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
    pub use super::FAR1::FAIL_ECC_ADDR;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Access control register
    pub ACR: RWRegister<u32>,

    /// FLASH key register for bank 1
    pub KEYR1: WORegister<u32>,

    /// FLASH option key register
    pub OPTKEYR: RWRegister<u32>,

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
    pub OPTCCR: WORegister<u32>,

    /// FLASH protection address for bank 1
    pub PRAR_CUR1: RORegister<u32>,

    /// FLASH protection address for bank 1
    pub PRAR_PRG1: RWRegister<u32>,

    /// FLASH secure address for bank 1
    pub SCAR_CUR1: RWRegister<u32>,

    /// FLASH secure address for bank 1
    pub SCAR_PRG1: RWRegister<u32>,

    /// FLASH write sector protection for bank 1
    pub WPSN_CURR1: RORegister<u32>,

    /// FLASH write sector protection for bank 1
    pub WPSN_PRGR1: RWRegister<u32>,

    /// FLASH register with boot address
    pub BOOT_CURR: RORegister<u32>,

    /// FLASH register with boot address
    pub BOOT_PRGR: RWRegister<u32>,

    _reserved1: [u8; 8],

    /// FLASH CRC control register for bank 1
    pub CRCCR1: RWRegister<u32>,

    /// FLASH CRC start address register for bank 1
    pub CRCSADDR1: RWRegister<u32>,

    /// FLASH CRC end address register for bank 1
    pub CRCEADDR1: RWRegister<u32>,

    /// FLASH CRC data register
    pub CRCDATAR: RWRegister<u32>,

    /// FLASH ECC fail address for bank 1
    pub FAR1: RORegister<u32>,

    _reserved2: [u8; 156],

    /// Access control register
    pub ACR_: RWRegister<u32>,

    /// FLASH key register for bank 1
    pub KEYR2: WORegister<u32>,

    /// FLASH option key register
    pub OPTKEYR_: RWRegister<u32>,

    /// FLASH control register for bank 1
    pub CR2: RWRegister<u32>,

    /// FLASH status register for bank 1
    pub SR2: RWRegister<u32>,

    /// FLASH clear control register for bank 1
    pub CCR2: RWRegister<u32>,

    /// FLASH option control register
    pub OPTCR_: RWRegister<u32>,

    /// FLASH option status register
    pub OPTSR_CUR_: RWRegister<u32>,

    /// FLASH option status register
    pub OPTSR_PRG_: RWRegister<u32>,

    /// FLASH option clear control register
    pub OPTCCR_: WORegister<u32>,

    /// FLASH protection address for bank 1
    pub PRAR_CUR2: RORegister<u32>,

    /// FLASH protection address for bank 1
    pub PRAR_PRG2: RWRegister<u32>,

    /// FLASH secure address for bank 1
    pub SCAR_CUR2: RWRegister<u32>,

    /// FLASH secure address for bank 1
    pub SCAR_PRG2: RWRegister<u32>,

    /// FLASH write sector protection for bank 1
    pub WPSN_CURR2: RORegister<u32>,

    /// FLASH write sector protection for bank 1
    pub WPSN_PRGR2: RWRegister<u32>,

    _reserved3: [u8; 16],

    /// FLASH CRC control register for bank 1
    pub CRCCR2: RWRegister<u32>,

    /// FLASH CRC start address register for bank 1
    pub CRCSADDR2: RWRegister<u32>,

    /// FLASH CRC end address register for bank 1
    pub CRCEADDR2: RWRegister<u32>,

    _reserved4: [u8; 4],

    /// FLASH ECC fail address for bank 1
    pub FAR2: RORegister<u32>,
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
    pub BOOT_CURR: u32,
    pub BOOT_PRGR: u32,
    pub CRCCR1: u32,
    pub CRCSADDR1: u32,
    pub CRCEADDR1: u32,
    pub CRCDATAR: u32,
    pub FAR1: u32,
    pub ACR_: u32,
    pub KEYR2: u32,
    pub OPTKEYR_: u32,
    pub CR2: u32,
    pub SR2: u32,
    pub CCR2: u32,
    pub OPTCR_: u32,
    pub OPTSR_CUR_: u32,
    pub OPTSR_PRG_: u32,
    pub OPTCCR_: u32,
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
#[cfg(feature = "rtic")]
unsafe impl Send for Instance {}
