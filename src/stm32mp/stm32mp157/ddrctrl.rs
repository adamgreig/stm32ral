#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DDRCTRL

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DDRCTRL master register 0
pub mod DDRCTRL_MSTR {

    /// DDR3
    pub mod DDR3 {
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

    /// LPDDR2
    pub mod LPDDR2 {
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

    /// LPDDR3
    pub mod LPDDR3 {
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

    /// BURSTCHOP
    pub mod BURSTCHOP {
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

    /// EN_2T_TIMING_MODE
    pub mod EN_2T_TIMING_MODE {
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

    /// DATA_BUS_WIDTH
    pub mod DATA_BUS_WIDTH {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DLL_OFF_MODE
    pub mod DLL_OFF_MODE {
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

    /// BURST_RDWR
    pub mod BURST_RDWR {
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
}

/// DDRCTRL operating mode status register
pub mod DDRCTRL_STAT {

    /// OPERATING_MODE
    pub mod OPERATING_MODE {
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

    /// SELFREF_TYPE
    pub mod SELFREF_TYPE {
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

    /// SELFREF_CAM_NOT_EMPTY
    pub mod SELFREF_CAM_NOT_EMPTY {
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
}

/// Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en
pub mod DDRCTRL_MRCTRL0 {

    /// MR_TYPE
    pub mod MR_TYPE {
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

    /// MR_RANK
    pub mod MR_RANK {
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

    /// MR_ADDR
    pub mod MR_ADDR {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MR_WR
    pub mod MR_WR {
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

/// DDRCTRL mode register read/write control register 1
pub mod DDRCTRL_MRCTRL1 {

    /// MR_DATA
    pub mod MR_DATA {
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

/// DDRCTRL mode register read/write status register
pub mod DDRCTRL_MRSTAT {

    /// MR_WR_BUSY
    pub mod MR_WR_BUSY {
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

/// DDRCTRL temperature derate enable register
pub mod DDRCTRL_DERATEEN {

    /// DERATE_ENABLE
    pub mod DERATE_ENABLE {
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

    /// DERATE_VALUE
    pub mod DERATE_VALUE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DERATE_BYTE
    pub mod DERATE_BYTE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL temperature derate interval register
pub mod DDRCTRL_DERATEINT {

    /// MR4_READ_INTERVAL
    pub mod MR4_READ_INTERVAL {
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

/// DDRCTRL low power control register
pub mod DDRCTRL_PWRCTL {

    /// SELFREF_EN
    pub mod SELFREF_EN {
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

    /// POWERDOWN_EN
    pub mod POWERDOWN_EN {
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

    /// DEEPPOWERDOWN_EN
    pub mod DEEPPOWERDOWN_EN {
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

    /// EN_DFI_DRAM_CLK_DISABLE
    pub mod EN_DFI_DRAM_CLK_DISABLE {
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

    /// SELFREF_SW
    pub mod SELFREF_SW {
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

    /// DIS_CAM_DRAIN_SELFREF
    pub mod DIS_CAM_DRAIN_SELFREF {
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
}

/// DDRCTRL low power timing register
pub mod DDRCTRL_PWRTMG {

    /// POWERDOWN_TO_X32
    pub mod POWERDOWN_TO_X32 {
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

    /// T_DPD_X4096
    pub mod T_DPD_X4096 {
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

    /// SELFREF_TO_X32
    pub mod SELFREF_TO_X32 {
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
}

/// DDRCTRL hardware low power control register
pub mod DDRCTRL_HWLPCTL {

    /// HW_LP_EN
    pub mod HW_LP_EN {
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

    /// HW_LP_EXIT_IDLE_EN
    pub mod HW_LP_EXIT_IDLE_EN {
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

    /// HW_LP_IDLE_X32
    pub mod HW_LP_IDLE_X32 {
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
}

/// DDRCTRL refresh control register 0
pub mod DDRCTRL_RFSHCTL0 {

    /// PER_BANK_REFRESH
    pub mod PER_BANK_REFRESH {
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

    /// REFRESH_BURST
    pub mod REFRESH_BURST {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (5 bits: 0b11111 << 4)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// REFRESH_TO_X32
    pub mod REFRESH_TO_X32 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (5 bits: 0b11111 << 12)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// REFRESH_MARGIN
    pub mod REFRESH_MARGIN {
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
}

/// DDRCTRL refresh control register 3
pub mod DDRCTRL_RFSHCTL3 {

    /// DIS_AUTO_REFRESH
    pub mod DIS_AUTO_REFRESH {
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

    /// REFRESH_UPDATE_LEVEL
    pub mod REFRESH_UPDATE_LEVEL {
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

/// DDRCTRL refresh timing register
pub mod DDRCTRL_RFSHTMG {

    /// T_RFC_MIN
    pub mod T_RFC_MIN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPDDR3_TREFBW_EN
    pub mod LPDDR3_TREFBW_EN {
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

    /// T_RFC_NOM_X1_X32
    pub mod T_RFC_NOM_X1_X32 {
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

    /// T_RFC_NOM_X1_SEL
    pub mod T_RFC_NOM_X1_SEL {
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

/// DDRCTRL CRC parity control register 0
pub mod DDRCTRL_CRCPARCTL0 {

    /// DFI_ALERT_ERR_INT_EN
    pub mod DFI_ALERT_ERR_INT_EN {
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

    /// DFI_ALERT_ERR_INT_CLR
    pub mod DFI_ALERT_ERR_INT_CLR {
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

    /// DFI_ALERT_ERR_CNT_CLR
    pub mod DFI_ALERT_ERR_CNT_CLR {
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
}

/// DDRCTRL CRC parity status register
pub mod DDRCTRL_CRCPARSTAT {

    /// DFI_ALERT_ERR_CNT
    pub mod DFI_ALERT_ERR_CNT {
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

    /// DFI_ALERT_ERR_INT
    pub mod DFI_ALERT_ERR_INT {
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

/// DDRCTRL SDRAM initialization register 0
pub mod DDRCTRL_INIT0 {

    /// PRE_CKE_X1024
    pub mod PRE_CKE_X1024 {
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

    /// POST_CKE_X1024
    pub mod POST_CKE_X1024 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (10 bits: 0x3ff << 16)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SKIP_DRAM_INIT
    pub mod SKIP_DRAM_INIT {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL SDRAM initialization register 1
pub mod DDRCTRL_INIT1 {

    /// PRE_OCD_X32
    pub mod PRE_OCD_X32 {
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

    /// DRAM_RSTN_X1024
    pub mod DRAM_RSTN_X1024 {
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
}

/// DDRCTRL SDRAM initialization register 2
pub mod DDRCTRL_INIT2 {

    /// MIN_STABLE_CLOCK_X1
    pub mod MIN_STABLE_CLOCK_X1 {
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

    /// IDLE_AFTER_RESET_X32
    pub mod IDLE_AFTER_RESET_X32 {
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
}

/// DDRCTRL SDRAM initialization register 3
pub mod DDRCTRL_INIT3 {

    /// EMR
    pub mod EMR {
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

    /// MR
    pub mod MR {
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

/// DDRCTRL SDRAM initialization register 4
pub mod DDRCTRL_INIT4 {

    /// EMR3
    pub mod EMR3 {
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

    /// EMR2
    pub mod EMR2 {
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

/// DDRCTRL SDRAM initialization register 5
pub mod DDRCTRL_INIT5 {

    /// MAX_AUTO_INIT_X1024
    pub mod MAX_AUTO_INIT_X1024 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DEV_ZQINIT_X32
    pub mod DEV_ZQINIT_X32 {
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
}

/// DDRCTRL DIMM control register
pub mod DDRCTRL_DIMMCTL {

    /// DIMM_STAGGER_CS_EN
    pub mod DIMM_STAGGER_CS_EN {
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

    /// DIMM_ADDR_MIRR_EN
    pub mod DIMM_ADDR_MIRR_EN {
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

/// DDRCTRL SDRAM timing register 0
pub mod DDRCTRL_DRAMTMG0 {

    /// T_RAS_MIN
    pub mod T_RAS_MIN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// T_RAS_MAX
    pub mod T_RAS_MAX {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// T_FAW
    pub mod T_FAW {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WR2PRE
    pub mod WR2PRE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (7 bits: 0x7f << 24)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL SDRAM timing register 1
pub mod DDRCTRL_DRAMTMG1 {

    /// T_RC
    pub mod T_RC {
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

    /// RD2PRE
    pub mod RD2PRE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// T_XP
    pub mod T_XP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL SDRAM timing register 2
pub mod DDRCTRL_DRAMTMG2 {

    /// WR2RD
    pub mod WR2RD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RD2WR
    pub mod RD2WR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// READ_LATENCY
    pub mod READ_LATENCY {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WRITE_LATENCY
    pub mod WRITE_LATENCY {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (6 bits: 0x3f << 24)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL SDRAM timing register 3
pub mod DDRCTRL_DRAMTMG3 {

    /// T_MOD
    pub mod T_MOD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// T_MRD
    pub mod T_MRD {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (6 bits: 0x3f << 12)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// T_MRW
    pub mod T_MRW {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (10 bits: 0x3ff << 20)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL SDRAM timing register 4
pub mod DDRCTRL_DRAMTMG4 {

    /// T_RP
    pub mod T_RP {
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

    /// T_RRD
    pub mod T_RRD {
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

    /// T_CCD
    pub mod T_CCD {
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

    /// T_RCD
    pub mod T_RCD {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL SDRAM timing register 5
pub mod DDRCTRL_DRAMTMG5 {

    /// T_CKE
    pub mod T_CKE {
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

    /// T_CKESR
    pub mod T_CKESR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// T_CKSRE
    pub mod T_CKSRE {
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

    /// T_CKSRX
    pub mod T_CKSRX {
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
}

/// DDRCTRL SDRAM timing register 6
pub mod DDRCTRL_DRAMTMG6 {

    /// T_CKCSX
    pub mod T_CKCSX {
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

    /// T_CKDPDX
    pub mod T_CKDPDX {
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

    /// T_CKDPDE
    pub mod T_CKDPDE {
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
}

/// DDRCTRL SDRAM timing register 7
pub mod DDRCTRL_DRAMTMG7 {

    /// T_CKPDX
    pub mod T_CKPDX {
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

    /// T_CKPDE
    pub mod T_CKPDE {
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
}

/// DDRCTRL SDRAM timing register 8
pub mod DDRCTRL_DRAMTMG8 {

    /// T_XS_X32
    pub mod T_XS_X32 {
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

    /// T_XS_DLL_X32
    pub mod T_XS_DLL_X32 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL SDRAM timing register 14
pub mod DDRCTRL_DRAMTMG14 {

    /// T_XSR
    pub mod T_XSR {
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

/// DDRCTRL SDRAM timing register 15
pub mod DDRCTRL_DRAMTMG15 {

    /// T_STAB_X32
    pub mod T_STAB_X32 {
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

    /// EN_DFI_LP_T_STAB
    pub mod EN_DFI_LP_T_STAB {
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

/// DDRCTRL ZQ control register 0
pub mod DDRCTRL_ZQCTL0 {

    /// T_ZQ_SHORT_NOP
    pub mod T_ZQ_SHORT_NOP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// T_ZQ_LONG_NOP
    pub mod T_ZQ_LONG_NOP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ZQ_RESISTOR_SHARED
    pub mod ZQ_RESISTOR_SHARED {
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

    /// DIS_SRX_ZQCL
    pub mod DIS_SRX_ZQCL {
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

    /// DIS_AUTO_ZQ
    pub mod DIS_AUTO_ZQ {
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

/// DDRCTRL ZQ control register 1
pub mod DDRCTRL_ZQCTL1 {

    /// T_ZQ_SHORT_INTERVAL_X1024
    pub mod T_ZQ_SHORT_INTERVAL_X1024 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (20 bits: 0xfffff << 0)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// T_ZQ_RESET_NOP
    pub mod T_ZQ_RESET_NOP {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (10 bits: 0x3ff << 20)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL ZQ control register 2
pub mod DDRCTRL_ZQCTL2 {

    /// ZQ_RESET
    pub mod ZQ_RESET {
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

/// DDRCTRL ZQ status register
pub mod DDRCTRL_ZQSTAT {

    /// ZQ_RESET_BUSY
    pub mod ZQ_RESET_BUSY {
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

/// DDRCTRL DFI timing register 0
pub mod DDRCTRL_DFITMG0 {

    /// DFI_TPHY_WRLAT
    pub mod DFI_TPHY_WRLAT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DFI_TPHY_WRDATA
    pub mod DFI_TPHY_WRDATA {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DFI_T_RDDATA_EN
    pub mod DFI_T_RDDATA_EN {
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

    /// DFI_T_CTRL_DELAY
    pub mod DFI_T_CTRL_DELAY {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL DFI timing register 1
pub mod DDRCTRL_DFITMG1 {

    /// DFI_T_DRAM_CLK_ENABLE
    pub mod DFI_T_DRAM_CLK_ENABLE {
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

    /// DFI_T_DRAM_CLK_DISABLE
    pub mod DFI_T_DRAM_CLK_DISABLE {
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

    /// DFI_T_WRDATA_DELAY
    pub mod DFI_T_WRDATA_DELAY {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL low power configuration register 0
pub mod DDRCTRL_DFILPCFG0 {

    /// DFI_LP_EN_PD
    pub mod DFI_LP_EN_PD {
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

    /// DFI_LP_WAKEUP_PD
    pub mod DFI_LP_WAKEUP_PD {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DFI_LP_EN_SR
    pub mod DFI_LP_EN_SR {
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

    /// DFI_LP_WAKEUP_SR
    pub mod DFI_LP_WAKEUP_SR {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DFI_LP_EN_DPD
    pub mod DFI_LP_EN_DPD {
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

    /// DFI_LP_WAKEUP_DPD
    pub mod DFI_LP_WAKEUP_DPD {
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

    /// DFI_TLP_RESP
    pub mod DFI_TLP_RESP {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL DFI update register 0
pub mod DDRCTRL_DFIUPD0 {

    /// DFI_T_CTRLUP_MIN
    pub mod DFI_T_CTRLUP_MIN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DFI_T_CTRLUP_MAX
    pub mod DFI_T_CTRLUP_MAX {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (10 bits: 0x3ff << 16)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CTRLUPD_PRE_SRX
    pub mod CTRLUPD_PRE_SRX {
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

    /// DIS_AUTO_CTRLUPD_SRX
    pub mod DIS_AUTO_CTRLUPD_SRX {
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

    /// DIS_AUTO_CTRLUPD
    pub mod DIS_AUTO_CTRLUPD {
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

/// DDRCTRL DFI update register 1
pub mod DDRCTRL_DFIUPD1 {

    /// DFI_T_CTRLUPD_INTERVAL_MAX_X1024
    pub mod DFI_T_CTRLUPD_INTERVAL_MAX_X1024 {
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

    /// DFI_T_CTRLUPD_INTERVAL_MIN_X1024
    pub mod DFI_T_CTRLUPD_INTERVAL_MIN_X1024 {
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
}

/// DDRCTRL DFI update register 2
pub mod DDRCTRL_DFIUPD2 {

    /// DFI_PHYUPD_EN
    pub mod DFI_PHYUPD_EN {
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

/// DDRCTRL DFI miscellaneous control register
pub mod DDRCTRL_DFIMISC {

    /// DFI_INIT_COMPLETE_EN
    pub mod DFI_INIT_COMPLETE_EN {
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

    /// CTL_IDLE_EN
    pub mod CTL_IDLE_EN {
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

    /// DFI_INIT_START
    pub mod DFI_INIT_START {
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

    /// DFI_FREQUENCY
    pub mod DFI_FREQUENCY {
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
}

/// DDRCTRL DFI status register
pub mod DDRCTRL_DFISTAT {

    /// DFI_INIT_COMPLETE
    pub mod DFI_INIT_COMPLETE {
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

    /// DFI_LP_ACK
    pub mod DFI_LP_ACK {
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

/// DDRCTRL DFI PHY master register
pub mod DDRCTRL_DFIPHYMSTR {

    /// DFI_PHYMSTR_EN
    pub mod DFI_PHYMSTR_EN {
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

/// DDRCTRL address map register 1
pub mod DDRCTRL_ADDRMAP1 {

    /// ADDRMAP_BANK_B0
    pub mod ADDRMAP_BANK_B0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADDRMAP_BANK_B1
    pub mod ADDRMAP_BANK_B1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADDRMAP_BANK_B2
    pub mod ADDRMAP_BANK_B2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL address map register 2
pub mod DDRCTRL_ADDRMAP2 {

    /// ADDRMAP_COL_B2
    pub mod ADDRMAP_COL_B2 {
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

    /// ADDRMAP_COL_B3
    pub mod ADDRMAP_COL_B3 {
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

    /// ADDRMAP_COL_B4
    pub mod ADDRMAP_COL_B4 {
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

    /// ADDRMAP_COL_B5
    pub mod ADDRMAP_COL_B5 {
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
}

/// DDRCTRL address map register 3
pub mod DDRCTRL_ADDRMAP3 {

    /// ADDRMAP_COL_B6
    pub mod ADDRMAP_COL_B6 {
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

    /// ADDRMAP_COL_B7
    pub mod ADDRMAP_COL_B7 {
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

    /// ADDRMAP_COL_B8
    pub mod ADDRMAP_COL_B8 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADDRMAP_COL_B9
    pub mod ADDRMAP_COL_B9 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL address map register 4
pub mod DDRCTRL_ADDRMAP4 {

    /// ADDRMAP_COL_B10
    pub mod ADDRMAP_COL_B10 {
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

    /// ADDRMAP_COL_B11
    pub mod ADDRMAP_COL_B11 {
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
}

/// DDRCTRL address map register 5
pub mod DDRCTRL_ADDRMAP5 {

    /// ADDRMAP_ROW_B0
    pub mod ADDRMAP_ROW_B0 {
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

    /// ADDRMAP_ROW_B1
    pub mod ADDRMAP_ROW_B1 {
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

    /// ADDRMAP_ROW_B2_10
    pub mod ADDRMAP_ROW_B2_10 {
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

    /// ADDRMAP_ROW_B11
    pub mod ADDRMAP_ROW_B11 {
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
}

/// DDRCTRL address register 6
pub mod DDRCTRL_ADDRMAP6 {

    /// ADDRMAP_ROW_B12
    pub mod ADDRMAP_ROW_B12 {
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

    /// ADDRMAP_ROW_B13
    pub mod ADDRMAP_ROW_B13 {
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

    /// ADDRMAP_ROW_B14
    pub mod ADDRMAP_ROW_B14 {
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

    /// ADDRMAP_ROW_B15
    pub mod ADDRMAP_ROW_B15 {
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

    /// LPDDR3_6GB_12GB
    pub mod LPDDR3_6GB_12GB {
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

/// DDRCTRL address map register 9
pub mod DDRCTRL_ADDRMAP9 {

    /// ADDRMAP_ROW_B2
    pub mod ADDRMAP_ROW_B2 {
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

    /// ADDRMAP_ROW_B3
    pub mod ADDRMAP_ROW_B3 {
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

    /// ADDRMAP_ROW_B4
    pub mod ADDRMAP_ROW_B4 {
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

    /// ADDRMAP_ROW_B5
    pub mod ADDRMAP_ROW_B5 {
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
}

/// DDRCTRL address map register 10
pub mod DDRCTRL_ADDRMAP10 {

    /// ADDRMAP_ROW_B6
    pub mod ADDRMAP_ROW_B6 {
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

    /// ADDRMAP_ROW_B7
    pub mod ADDRMAP_ROW_B7 {
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

    /// ADDRMAP_ROW_B8
    pub mod ADDRMAP_ROW_B8 {
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

    /// ADDRMAP_ROW_B9
    pub mod ADDRMAP_ROW_B9 {
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
}

/// DDRCTRL address map register 11
pub mod DDRCTRL_ADDRMAP11 {

    /// ADDRMAP_ROW_B10
    pub mod ADDRMAP_ROW_B10 {
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

/// DDRCTRL ODT configuration register
pub mod DDRCTRL_ODTCFG {

    /// RD_ODT_DELAY
    pub mod RD_ODT_DELAY {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (5 bits: 0b11111 << 2)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RD_ODT_HOLD
    pub mod RD_ODT_HOLD {
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

    /// WR_ODT_DELAY
    pub mod WR_ODT_DELAY {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WR_ODT_HOLD
    pub mod WR_ODT_HOLD {
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
}

/// DDRCTRL ODT/Rank map register
pub mod DDRCTRL_ODTMAP {

    /// RANK0_WR_ODT
    pub mod RANK0_WR_ODT {
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

    /// RANK0_RD_ODT
    pub mod RANK0_RD_ODT {
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
}

/// DDRCTRL scheduler control register
pub mod DDRCTRL_SCHED {

    /// FORCE_LOW_PRI_N
    pub mod FORCE_LOW_PRI_N {
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

    /// PREFER_WRITE
    pub mod PREFER_WRITE {
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

    /// PAGECLOSE
    pub mod PAGECLOSE {
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

    /// LPR_NUM_ENTRIES
    pub mod LPR_NUM_ENTRIES {
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

    /// GO2CRITICAL_HYSTERESIS
    pub mod GO2CRITICAL_HYSTERESIS {
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

    /// RDWR_IDLE_GAP
    pub mod RDWR_IDLE_GAP {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (7 bits: 0x7f << 24)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL scheduler control register 1
pub mod DDRCTRL_SCHED1 {

    /// PAGECLOSE_TIMER
    pub mod PAGECLOSE_TIMER {
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

/// DDRCTRL high priority read CAM register 1
pub mod DDRCTRL_PERFHPR1 {

    /// HPR_MAX_STARVE
    pub mod HPR_MAX_STARVE {
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

    /// HPR_XACT_RUN_LENGTH
    pub mod HPR_XACT_RUN_LENGTH {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL low priority read CAM register 1
pub mod DDRCTRL_PERFLPR1 {

    /// LPR_MAX_STARVE
    pub mod LPR_MAX_STARVE {
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

    /// LPR_XACT_RUN_LENGTH
    pub mod LPR_XACT_RUN_LENGTH {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL write CAM register 1
pub mod DDRCTRL_PERFWR1 {

    /// W_MAX_STARVE
    pub mod W_MAX_STARVE {
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

    /// W_XACT_RUN_LENGTH
    pub mod W_XACT_RUN_LENGTH {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL debug register 0
pub mod DDRCTRL_DBG0 {

    /// DIS_WC
    pub mod DIS_WC {
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

    /// DIS_COLLISION_PAGE_OPT
    pub mod DIS_COLLISION_PAGE_OPT {
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
}

/// DDRCTRL debug register 1
pub mod DDRCTRL_DBG1 {

    /// DIS_DQ
    pub mod DIS_DQ {
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

    /// DIS_HIF
    pub mod DIS_HIF {
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

/// DDRCTRL CAM debug register
pub mod DDRCTRL_DBGCAM {

    /// DBG_HPR_Q_DEPTH
    pub mod DBG_HPR_Q_DEPTH {
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

    /// DBG_LPR_Q_DEPTH
    pub mod DBG_LPR_Q_DEPTH {
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

    /// DBG_W_Q_DEPTH
    pub mod DBG_W_Q_DEPTH {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DBG_STALL
    pub mod DBG_STALL {
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

    /// DBG_RD_Q_EMPTY
    pub mod DBG_RD_Q_EMPTY {
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

    /// DBG_WR_Q_EMPTY
    pub mod DBG_WR_Q_EMPTY {
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

    /// RD_DATA_PIPELINE_EMPTY
    pub mod RD_DATA_PIPELINE_EMPTY {
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

    /// WR_DATA_PIPELINE_EMPTY
    pub mod WR_DATA_PIPELINE_EMPTY {
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
}

/// DDRCTRL command debug register
pub mod DDRCTRL_DBGCMD {

    /// RANK0_REFRESH
    pub mod RANK0_REFRESH {
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

    /// ZQ_CALIB_SHORT
    pub mod ZQ_CALIB_SHORT {
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

    /// CTRLUPD
    pub mod CTRLUPD {
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
}

/// DDRCTRL status debug register
pub mod DDRCTRL_DBGSTAT {

    /// RANK0_REFRESH_BUSY
    pub mod RANK0_REFRESH_BUSY {
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

    /// ZQ_CALIB_SHORT_BUSY
    pub mod ZQ_CALIB_SHORT_BUSY {
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

    /// CTRLUPD_BUSY
    pub mod CTRLUPD_BUSY {
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
}

/// DDRCTRL software register programming control enable
pub mod DDRCTRL_SWCTL {

    /// SW_DONE
    pub mod SW_DONE {
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

/// DDRCTRL software register programming control status
pub mod DDRCTRL_SWSTAT {

    /// SW_DONE_ACK
    pub mod SW_DONE_ACK {
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

/// AXI Poison configuration register common for all AXI ports.
pub mod DDRCTRL_POISONCFG {

    /// WR_POISON_SLVERR_EN
    pub mod WR_POISON_SLVERR_EN {
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

    /// WR_POISON_INTR_EN
    pub mod WR_POISON_INTR_EN {
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

    /// WR_POISON_INTR_CLR
    pub mod WR_POISON_INTR_CLR {
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

    /// RD_POISON_SLVERR_EN
    pub mod RD_POISON_SLVERR_EN {
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

    /// RD_POISON_INTR_EN
    pub mod RD_POISON_INTR_EN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RD_POISON_INTR_CLR
    pub mod RD_POISON_INTR_CLR {
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
}

/// DDRCTRL AXI Poison status register
pub mod DDRCTRL_POISONSTAT {

    /// WR_POISON_INTR_0
    pub mod WR_POISON_INTR_0 {
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

    /// WR_POISON_INTR_1
    pub mod WR_POISON_INTR_1 {
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

    /// RD_POISON_INTR_0
    pub mod RD_POISON_INTR_0 {
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

    /// RD_POISON_INTR_1
    pub mod RD_POISON_INTR_1 {
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
}

/// DDRCTRL port status register
pub mod DDRCTRL_PSTAT {

    /// RD_PORT_BUSY_0
    pub mod RD_PORT_BUSY_0 {
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

    /// RD_PORT_BUSY_1
    pub mod RD_PORT_BUSY_1 {
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

    /// WR_PORT_BUSY_0
    pub mod WR_PORT_BUSY_0 {
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

    /// WR_PORT_BUSY_1
    pub mod WR_PORT_BUSY_1 {
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
}

/// DDRCTRL port common configuration register
pub mod DDRCTRL_PCCFG {

    /// GO2CRITICAL_EN
    pub mod GO2CRITICAL_EN {
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

    /// PAGEMATCH_LIMIT
    pub mod PAGEMATCH_LIMIT {
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

    /// BL_EXP_MODE
    pub mod BL_EXP_MODE {
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
}

/// DDRCTRL port 0 configuration read register
pub mod DDRCTRL_PCFGR_0 {

    /// RD_PORT_PRIORITY
    pub mod RD_PORT_PRIORITY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RD_PORT_AGING_EN
    pub mod RD_PORT_AGING_EN {
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

    /// RD_PORT_URGENT_EN
    pub mod RD_PORT_URGENT_EN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RD_PORT_PAGEMATCH_EN
    pub mod RD_PORT_PAGEMATCH_EN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RDWR_ORDERED_EN
    pub mod RDWR_ORDERED_EN {
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

/// DDRCTRL port 0 configuration write register
pub mod DDRCTRL_PCFGW_0 {

    /// WR_PORT_PRIORITY
    pub mod WR_PORT_PRIORITY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WR_PORT_AGING_EN
    pub mod WR_PORT_AGING_EN {
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

    /// WR_PORT_URGENT_EN
    pub mod WR_PORT_URGENT_EN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WR_PORT_PAGEMATCH_EN
    pub mod WR_PORT_PAGEMATCH_EN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL port 0 control register
pub mod DDRCTRL_PCTRL_0 {

    /// PORT_EN
    pub mod PORT_EN {
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

/// DDRCTRL port 0 read Q0S configuration register 0
pub mod DDRCTRL_PCFGQOS0_0 {

    /// RQOS_MAP_LEVEL1
    pub mod RQOS_MAP_LEVEL1 {
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

    /// RQOS_MAP_LEVEL2
    pub mod RQOS_MAP_LEVEL2 {
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

    /// RQOS_MAP_REGION0
    pub mod RQOS_MAP_REGION0 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RQOS_MAP_REGION1
    pub mod RQOS_MAP_REGION1 {
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

    /// RQOS_MAP_REGION2
    pub mod RQOS_MAP_REGION2 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL port 0 read Q0S configuration register 1
pub mod DDRCTRL_PCFGQOS1_0 {

    /// RQOS_MAP_TIMEOUTB
    pub mod RQOS_MAP_TIMEOUTB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RQOS_MAP_TIMEOUTR
    pub mod RQOS_MAP_TIMEOUTR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL port 0 write Q0S configuration register 0
pub mod DDRCTRL_PCFGWQOS0_0 {

    /// WQOS_MAP_LEVEL1
    pub mod WQOS_MAP_LEVEL1 {
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

    /// WQOS_MAP_LEVEL2
    pub mod WQOS_MAP_LEVEL2 {
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

    /// WQOS_MAP_REGION0
    pub mod WQOS_MAP_REGION0 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WQOS_MAP_REGION1
    pub mod WQOS_MAP_REGION1 {
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

    /// WQOS_MAP_REGION2
    pub mod WQOS_MAP_REGION2 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL port 0 write Q0S configuration register 1
pub mod DDRCTRL_PCFGWQOS1_0 {

    /// WQOS_MAP_TIMEOUT1
    pub mod WQOS_MAP_TIMEOUT1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WQOS_MAP_TIMEOUT2
    pub mod WQOS_MAP_TIMEOUT2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRCTRL port 1 configuration read register
pub mod DDRCTRL_PCFGR_1 {
    pub use super::DDRCTRL_PCFGR_0::RDWR_ORDERED_EN;
    pub use super::DDRCTRL_PCFGR_0::RD_PORT_AGING_EN;
    pub use super::DDRCTRL_PCFGR_0::RD_PORT_PAGEMATCH_EN;
    pub use super::DDRCTRL_PCFGR_0::RD_PORT_PRIORITY;
    pub use super::DDRCTRL_PCFGR_0::RD_PORT_URGENT_EN;
}

/// DDRCTRL port 1 configuration write register
pub mod DDRCTRL_PCFGW_1 {
    pub use super::DDRCTRL_PCFGW_0::WR_PORT_AGING_EN;
    pub use super::DDRCTRL_PCFGW_0::WR_PORT_PAGEMATCH_EN;
    pub use super::DDRCTRL_PCFGW_0::WR_PORT_PRIORITY;
    pub use super::DDRCTRL_PCFGW_0::WR_PORT_URGENT_EN;
}

/// DDRCTRL port 1 control register
pub mod DDRCTRL_PCTRL_1 {
    pub use super::DDRCTRL_PCTRL_0::PORT_EN;
}

/// DDRCTRL port 1 read Q0S configuration register 0
pub mod DDRCTRL_PCFGQOS0_1 {
    pub use super::DDRCTRL_PCFGQOS0_0::RQOS_MAP_LEVEL1;
    pub use super::DDRCTRL_PCFGQOS0_0::RQOS_MAP_LEVEL2;
    pub use super::DDRCTRL_PCFGQOS0_0::RQOS_MAP_REGION0;
    pub use super::DDRCTRL_PCFGQOS0_0::RQOS_MAP_REGION1;
    pub use super::DDRCTRL_PCFGQOS0_0::RQOS_MAP_REGION2;
}

/// DDRCTRL port 1 read Q0S configuration register 1
pub mod DDRCTRL_PCFGQOS1_1 {
    pub use super::DDRCTRL_PCFGQOS1_0::RQOS_MAP_TIMEOUTB;
    pub use super::DDRCTRL_PCFGQOS1_0::RQOS_MAP_TIMEOUTR;
}

/// DDRCTRL port 1 write Q0S configuration register 0
pub mod DDRCTRL_PCFGWQOS0_1 {
    pub use super::DDRCTRL_PCFGWQOS0_0::WQOS_MAP_LEVEL1;
    pub use super::DDRCTRL_PCFGWQOS0_0::WQOS_MAP_LEVEL2;
    pub use super::DDRCTRL_PCFGWQOS0_0::WQOS_MAP_REGION0;
    pub use super::DDRCTRL_PCFGWQOS0_0::WQOS_MAP_REGION1;
    pub use super::DDRCTRL_PCFGWQOS0_0::WQOS_MAP_REGION2;
}

/// DDRCTRL port 1 write Q0S configuration register 1
pub mod DDRCTRL_PCFGWQOS1_1 {
    pub use super::DDRCTRL_PCFGWQOS1_0::WQOS_MAP_TIMEOUT1;
    pub use super::DDRCTRL_PCFGWQOS1_0::WQOS_MAP_TIMEOUT2;
}
#[repr(C)]
pub struct RegisterBlock {
    /// DDRCTRL master register 0
    pub DDRCTRL_MSTR: RWRegister<u32>,

    /// DDRCTRL operating mode status register
    pub DDRCTRL_STAT: RORegister<u32>,

    _reserved1: [u32; 2],

    /// Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en
    pub DDRCTRL_MRCTRL0: RWRegister<u32>,

    /// DDRCTRL mode register read/write control register 1
    pub DDRCTRL_MRCTRL1: RWRegister<u32>,

    /// DDRCTRL mode register read/write status register
    pub DDRCTRL_MRSTAT: RORegister<u32>,

    _reserved2: [u32; 1],

    /// DDRCTRL temperature derate enable register
    pub DDRCTRL_DERATEEN: RWRegister<u32>,

    /// DDRCTRL temperature derate interval register
    pub DDRCTRL_DERATEINT: RWRegister<u32>,

    _reserved3: [u32; 2],

    /// DDRCTRL low power control register
    pub DDRCTRL_PWRCTL: RWRegister<u32>,

    /// DDRCTRL low power timing register
    pub DDRCTRL_PWRTMG: RWRegister<u32>,

    /// DDRCTRL hardware low power control register
    pub DDRCTRL_HWLPCTL: RWRegister<u32>,

    _reserved4: [u32; 5],

    /// DDRCTRL refresh control register 0
    pub DDRCTRL_RFSHCTL0: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// DDRCTRL refresh control register 3
    pub DDRCTRL_RFSHCTL3: RWRegister<u32>,

    /// DDRCTRL refresh timing register
    pub DDRCTRL_RFSHTMG: RWRegister<u32>,

    _reserved6: [u32; 22],

    /// DDRCTRL CRC parity control register 0
    pub DDRCTRL_CRCPARCTL0: RWRegister<u32>,

    _reserved7: [u32; 2],

    /// DDRCTRL CRC parity status register
    pub DDRCTRL_CRCPARSTAT: RORegister<u32>,

    /// DDRCTRL SDRAM initialization register 0
    pub DDRCTRL_INIT0: RWRegister<u32>,

    /// DDRCTRL SDRAM initialization register 1
    pub DDRCTRL_INIT1: RWRegister<u32>,

    /// DDRCTRL SDRAM initialization register 2
    pub DDRCTRL_INIT2: RWRegister<u32>,

    /// DDRCTRL SDRAM initialization register 3
    pub DDRCTRL_INIT3: RWRegister<u32>,

    /// DDRCTRL SDRAM initialization register 4
    pub DDRCTRL_INIT4: RWRegister<u32>,

    /// DDRCTRL SDRAM initialization register 5
    pub DDRCTRL_INIT5: RWRegister<u32>,

    _reserved8: [u32; 2],

    /// DDRCTRL DIMM control register
    pub DDRCTRL_DIMMCTL: RWRegister<u32>,

    _reserved9: [u32; 3],

    /// DDRCTRL SDRAM timing register 0
    pub DDRCTRL_DRAMTMG0: RWRegister<u32>,

    /// DDRCTRL SDRAM timing register 1
    pub DDRCTRL_DRAMTMG1: RWRegister<u32>,

    /// DDRCTRL SDRAM timing register 2
    pub DDRCTRL_DRAMTMG2: RWRegister<u32>,

    /// DDRCTRL SDRAM timing register 3
    pub DDRCTRL_DRAMTMG3: RWRegister<u32>,

    /// DDRCTRL SDRAM timing register 4
    pub DDRCTRL_DRAMTMG4: RWRegister<u32>,

    /// DDRCTRL SDRAM timing register 5
    pub DDRCTRL_DRAMTMG5: RWRegister<u32>,

    /// DDRCTRL SDRAM timing register 6
    pub DDRCTRL_DRAMTMG6: RWRegister<u32>,

    /// DDRCTRL SDRAM timing register 7
    pub DDRCTRL_DRAMTMG7: RWRegister<u32>,

    /// DDRCTRL SDRAM timing register 8
    pub DDRCTRL_DRAMTMG8: RWRegister<u32>,

    _reserved10: [u32; 5],

    /// DDRCTRL SDRAM timing register 14
    pub DDRCTRL_DRAMTMG14: RWRegister<u32>,

    /// DDRCTRL SDRAM timing register 15
    pub DDRCTRL_DRAMTMG15: RWRegister<u32>,

    _reserved11: [u32; 16],

    /// DDRCTRL ZQ control register 0
    pub DDRCTRL_ZQCTL0: RWRegister<u32>,

    /// DDRCTRL ZQ control register 1
    pub DDRCTRL_ZQCTL1: RWRegister<u32>,

    /// DDRCTRL ZQ control register 2
    pub DDRCTRL_ZQCTL2: RWRegister<u32>,

    /// DDRCTRL ZQ status register
    pub DDRCTRL_ZQSTAT: RORegister<u32>,

    /// DDRCTRL DFI timing register 0
    pub DDRCTRL_DFITMG0: RWRegister<u32>,

    /// DDRCTRL DFI timing register 1
    pub DDRCTRL_DFITMG1: RWRegister<u32>,

    /// DDRCTRL low power configuration register 0
    pub DDRCTRL_DFILPCFG0: RWRegister<u32>,

    _reserved12: [u32; 1],

    /// DDRCTRL DFI update register 0
    pub DDRCTRL_DFIUPD0: RWRegister<u32>,

    /// DDRCTRL DFI update register 1
    pub DDRCTRL_DFIUPD1: RWRegister<u32>,

    /// DDRCTRL DFI update register 2
    pub DDRCTRL_DFIUPD2: RWRegister<u32>,

    _reserved13: [u32; 1],

    /// DDRCTRL DFI miscellaneous control register
    pub DDRCTRL_DFIMISC: RWRegister<u32>,

    _reserved14: [u32; 2],

    /// DDRCTRL DFI status register
    pub DDRCTRL_DFISTAT: RORegister<u32>,

    _reserved15: [u32; 1],

    /// DDRCTRL DFI PHY master register
    pub DDRCTRL_DFIPHYMSTR: RWRegister<u32>,

    _reserved16: [u32; 15],

    /// DDRCTRL address map register 1
    pub DDRCTRL_ADDRMAP1: RWRegister<u32>,

    /// DDRCTRL address map register 2
    pub DDRCTRL_ADDRMAP2: RWRegister<u32>,

    /// DDRCTRL address map register 3
    pub DDRCTRL_ADDRMAP3: RWRegister<u32>,

    /// DDRCTRL address map register 4
    pub DDRCTRL_ADDRMAP4: RWRegister<u32>,

    /// DDRCTRL address map register 5
    pub DDRCTRL_ADDRMAP5: RWRegister<u32>,

    /// DDRCTRL address register 6
    pub DDRCTRL_ADDRMAP6: RWRegister<u32>,

    _reserved17: [u32; 2],

    /// DDRCTRL address map register 9
    pub DDRCTRL_ADDRMAP9: RWRegister<u32>,

    /// DDRCTRL address map register 10
    pub DDRCTRL_ADDRMAP10: RWRegister<u32>,

    /// DDRCTRL address map register 11
    pub DDRCTRL_ADDRMAP11: RWRegister<u32>,

    _reserved18: [u32; 4],

    /// DDRCTRL ODT configuration register
    pub DDRCTRL_ODTCFG: RWRegister<u32>,

    /// DDRCTRL ODT/Rank map register
    pub DDRCTRL_ODTMAP: RWRegister<u32>,

    _reserved19: [u32; 2],

    /// DDRCTRL scheduler control register
    pub DDRCTRL_SCHED: RWRegister<u32>,

    /// DDRCTRL scheduler control register 1
    pub DDRCTRL_SCHED1: RWRegister<u32>,

    _reserved20: [u32; 1],

    /// DDRCTRL high priority read CAM register 1
    pub DDRCTRL_PERFHPR1: RWRegister<u32>,

    _reserved21: [u32; 1],

    /// DDRCTRL low priority read CAM register 1
    pub DDRCTRL_PERFLPR1: RWRegister<u32>,

    _reserved22: [u32; 1],

    /// DDRCTRL write CAM register 1
    pub DDRCTRL_PERFWR1: RWRegister<u32>,

    _reserved23: [u32; 36],

    /// DDRCTRL debug register 0
    pub DDRCTRL_DBG0: RWRegister<u32>,

    /// DDRCTRL debug register 1
    pub DDRCTRL_DBG1: RWRegister<u32>,

    /// DDRCTRL CAM debug register
    pub DDRCTRL_DBGCAM: RORegister<u32>,

    /// DDRCTRL command debug register
    pub DDRCTRL_DBGCMD: RWRegister<u32>,

    /// DDRCTRL status debug register
    pub DDRCTRL_DBGSTAT: RORegister<u32>,

    _reserved24: [u32; 3],

    /// DDRCTRL software register programming control enable
    pub DDRCTRL_SWCTL: RWRegister<u32>,

    /// DDRCTRL software register programming control status
    pub DDRCTRL_SWSTAT: RORegister<u32>,

    _reserved25: [u32; 17],

    /// AXI Poison configuration register common for all AXI ports.
    pub DDRCTRL_POISONCFG: RWRegister<u32>,

    /// DDRCTRL AXI Poison status register
    pub DDRCTRL_POISONSTAT: RORegister<u32>,

    _reserved26: [u32; 34],

    /// DDRCTRL port status register
    pub DDRCTRL_PSTAT: RORegister<u32>,

    /// DDRCTRL port common configuration register
    pub DDRCTRL_PCCFG: RWRegister<u32>,

    /// DDRCTRL port 0 configuration read register
    pub DDRCTRL_PCFGR_0: RWRegister<u32>,

    /// DDRCTRL port 0 configuration write register
    pub DDRCTRL_PCFGW_0: RWRegister<u32>,

    _reserved27: [u32; 33],

    /// DDRCTRL port 0 control register
    pub DDRCTRL_PCTRL_0: RWRegister<u32>,

    /// DDRCTRL port 0 read Q0S configuration register 0
    pub DDRCTRL_PCFGQOS0_0: RWRegister<u32>,

    /// DDRCTRL port 0 read Q0S configuration register 1
    pub DDRCTRL_PCFGQOS1_0: RWRegister<u32>,

    /// DDRCTRL port 0 write Q0S configuration register 0
    pub DDRCTRL_PCFGWQOS0_0: RWRegister<u32>,

    /// DDRCTRL port 0 write Q0S configuration register 1
    pub DDRCTRL_PCFGWQOS1_0: RWRegister<u32>,

    _reserved28: [u32; 4],

    /// DDRCTRL port 1 configuration read register
    pub DDRCTRL_PCFGR_1: RWRegister<u32>,

    /// DDRCTRL port 1 configuration write register
    pub DDRCTRL_PCFGW_1: RWRegister<u32>,

    _reserved29: [u32; 33],

    /// DDRCTRL port 1 control register
    pub DDRCTRL_PCTRL_1: RWRegister<u32>,

    /// DDRCTRL port 1 read Q0S configuration register 0
    pub DDRCTRL_PCFGQOS0_1: RWRegister<u32>,

    /// DDRCTRL port 1 read Q0S configuration register 1
    pub DDRCTRL_PCFGQOS1_1: RWRegister<u32>,

    /// DDRCTRL port 1 write Q0S configuration register 0
    pub DDRCTRL_PCFGWQOS0_1: RWRegister<u32>,

    /// DDRCTRL port 1 write Q0S configuration register 1
    pub DDRCTRL_PCFGWQOS1_1: RWRegister<u32>,
}
pub struct ResetValues {
    pub DDRCTRL_MSTR: u32,
    pub DDRCTRL_STAT: u32,
    pub DDRCTRL_MRCTRL0: u32,
    pub DDRCTRL_MRCTRL1: u32,
    pub DDRCTRL_MRSTAT: u32,
    pub DDRCTRL_DERATEEN: u32,
    pub DDRCTRL_DERATEINT: u32,
    pub DDRCTRL_PWRCTL: u32,
    pub DDRCTRL_PWRTMG: u32,
    pub DDRCTRL_HWLPCTL: u32,
    pub DDRCTRL_RFSHCTL0: u32,
    pub DDRCTRL_RFSHCTL3: u32,
    pub DDRCTRL_RFSHTMG: u32,
    pub DDRCTRL_CRCPARCTL0: u32,
    pub DDRCTRL_CRCPARSTAT: u32,
    pub DDRCTRL_INIT0: u32,
    pub DDRCTRL_INIT1: u32,
    pub DDRCTRL_INIT2: u32,
    pub DDRCTRL_INIT3: u32,
    pub DDRCTRL_INIT4: u32,
    pub DDRCTRL_INIT5: u32,
    pub DDRCTRL_DIMMCTL: u32,
    pub DDRCTRL_DRAMTMG0: u32,
    pub DDRCTRL_DRAMTMG1: u32,
    pub DDRCTRL_DRAMTMG2: u32,
    pub DDRCTRL_DRAMTMG3: u32,
    pub DDRCTRL_DRAMTMG4: u32,
    pub DDRCTRL_DRAMTMG5: u32,
    pub DDRCTRL_DRAMTMG6: u32,
    pub DDRCTRL_DRAMTMG7: u32,
    pub DDRCTRL_DRAMTMG8: u32,
    pub DDRCTRL_DRAMTMG14: u32,
    pub DDRCTRL_DRAMTMG15: u32,
    pub DDRCTRL_ZQCTL0: u32,
    pub DDRCTRL_ZQCTL1: u32,
    pub DDRCTRL_ZQCTL2: u32,
    pub DDRCTRL_ZQSTAT: u32,
    pub DDRCTRL_DFITMG0: u32,
    pub DDRCTRL_DFITMG1: u32,
    pub DDRCTRL_DFILPCFG0: u32,
    pub DDRCTRL_DFIUPD0: u32,
    pub DDRCTRL_DFIUPD1: u32,
    pub DDRCTRL_DFIUPD2: u32,
    pub DDRCTRL_DFIMISC: u32,
    pub DDRCTRL_DFISTAT: u32,
    pub DDRCTRL_DFIPHYMSTR: u32,
    pub DDRCTRL_ADDRMAP1: u32,
    pub DDRCTRL_ADDRMAP2: u32,
    pub DDRCTRL_ADDRMAP3: u32,
    pub DDRCTRL_ADDRMAP4: u32,
    pub DDRCTRL_ADDRMAP5: u32,
    pub DDRCTRL_ADDRMAP6: u32,
    pub DDRCTRL_ADDRMAP9: u32,
    pub DDRCTRL_ADDRMAP10: u32,
    pub DDRCTRL_ADDRMAP11: u32,
    pub DDRCTRL_ODTCFG: u32,
    pub DDRCTRL_ODTMAP: u32,
    pub DDRCTRL_SCHED: u32,
    pub DDRCTRL_SCHED1: u32,
    pub DDRCTRL_PERFHPR1: u32,
    pub DDRCTRL_PERFLPR1: u32,
    pub DDRCTRL_PERFWR1: u32,
    pub DDRCTRL_DBG0: u32,
    pub DDRCTRL_DBG1: u32,
    pub DDRCTRL_DBGCAM: u32,
    pub DDRCTRL_DBGCMD: u32,
    pub DDRCTRL_DBGSTAT: u32,
    pub DDRCTRL_SWCTL: u32,
    pub DDRCTRL_SWSTAT: u32,
    pub DDRCTRL_POISONCFG: u32,
    pub DDRCTRL_POISONSTAT: u32,
    pub DDRCTRL_PSTAT: u32,
    pub DDRCTRL_PCCFG: u32,
    pub DDRCTRL_PCFGR_0: u32,
    pub DDRCTRL_PCFGW_0: u32,
    pub DDRCTRL_PCTRL_0: u32,
    pub DDRCTRL_PCFGQOS0_0: u32,
    pub DDRCTRL_PCFGQOS1_0: u32,
    pub DDRCTRL_PCFGWQOS0_0: u32,
    pub DDRCTRL_PCFGWQOS1_0: u32,
    pub DDRCTRL_PCFGR_1: u32,
    pub DDRCTRL_PCFGW_1: u32,
    pub DDRCTRL_PCTRL_1: u32,
    pub DDRCTRL_PCFGQOS0_1: u32,
    pub DDRCTRL_PCFGQOS1_1: u32,
    pub DDRCTRL_PCFGWQOS0_1: u32,
    pub DDRCTRL_PCFGWQOS1_1: u32,
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

/// Access functions for the DDRCTRL peripheral instance
pub mod DDRCTRL {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5a003000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DDRCTRL
    pub const reset: ResetValues = ResetValues {
        DDRCTRL_MSTR: 0x00040001,
        DDRCTRL_STAT: 0x00000000,
        DDRCTRL_MRCTRL0: 0x00000010,
        DDRCTRL_MRCTRL1: 0x00000000,
        DDRCTRL_MRSTAT: 0x00000000,
        DDRCTRL_DERATEEN: 0x00000000,
        DDRCTRL_DERATEINT: 0x00800000,
        DDRCTRL_PWRCTL: 0x00000000,
        DDRCTRL_PWRTMG: 0x00402010,
        DDRCTRL_HWLPCTL: 0x00000003,
        DDRCTRL_RFSHCTL0: 0x00210000,
        DDRCTRL_RFSHCTL3: 0x00000000,
        DDRCTRL_RFSHTMG: 0x0062008C,
        DDRCTRL_CRCPARCTL0: 0x00000000,
        DDRCTRL_CRCPARSTAT: 0x00000000,
        DDRCTRL_INIT0: 0x0002004E,
        DDRCTRL_INIT1: 0x00000000,
        DDRCTRL_INIT2: 0x00000D05,
        DDRCTRL_INIT3: 0x00000510,
        DDRCTRL_INIT4: 0x00000000,
        DDRCTRL_INIT5: 0x00100004,
        DDRCTRL_DIMMCTL: 0x00000000,
        DDRCTRL_DRAMTMG0: 0x0F101B0F,
        DDRCTRL_DRAMTMG1: 0x00080414,
        DDRCTRL_DRAMTMG2: 0x0305060D,
        DDRCTRL_DRAMTMG3: 0x0050400C,
        DDRCTRL_DRAMTMG4: 0x05040405,
        DDRCTRL_DRAMTMG5: 0x05050403,
        DDRCTRL_DRAMTMG6: 0x02020005,
        DDRCTRL_DRAMTMG7: 0x00000202,
        DDRCTRL_DRAMTMG8: 0x00004405,
        DDRCTRL_DRAMTMG14: 0x000000A0,
        DDRCTRL_DRAMTMG15: 0x00000000,
        DDRCTRL_ZQCTL0: 0x02000040,
        DDRCTRL_ZQCTL1: 0x02000100,
        DDRCTRL_ZQCTL2: 0x00000000,
        DDRCTRL_ZQSTAT: 0x00000000,
        DDRCTRL_DFITMG0: 0x07020002,
        DDRCTRL_DFITMG1: 0x00000404,
        DDRCTRL_DFILPCFG0: 0x07000000,
        DDRCTRL_DFIUPD0: 0x00400003,
        DDRCTRL_DFIUPD1: 0x00010001,
        DDRCTRL_DFIUPD2: 0x80000000,
        DDRCTRL_DFIMISC: 0x00000001,
        DDRCTRL_DFISTAT: 0x00000000,
        DDRCTRL_DFIPHYMSTR: 0x00000001,
        DDRCTRL_ADDRMAP1: 0x00000000,
        DDRCTRL_ADDRMAP2: 0x00000000,
        DDRCTRL_ADDRMAP3: 0x00000000,
        DDRCTRL_ADDRMAP4: 0x00000000,
        DDRCTRL_ADDRMAP5: 0x00000000,
        DDRCTRL_ADDRMAP6: 0x00000000,
        DDRCTRL_ADDRMAP9: 0x00000000,
        DDRCTRL_ADDRMAP10: 0x00000000,
        DDRCTRL_ADDRMAP11: 0x00000000,
        DDRCTRL_ODTCFG: 0x04000400,
        DDRCTRL_ODTMAP: 0x00000011,
        DDRCTRL_SCHED: 0x00000805,
        DDRCTRL_SCHED1: 0x00000000,
        DDRCTRL_PERFHPR1: 0x0F000001,
        DDRCTRL_PERFLPR1: 0x0F00007F,
        DDRCTRL_PERFWR1: 0x0F00007F,
        DDRCTRL_DBG0: 0x00000000,
        DDRCTRL_DBG1: 0x00000000,
        DDRCTRL_DBGCAM: 0x00000000,
        DDRCTRL_DBGCMD: 0x00000000,
        DDRCTRL_DBGSTAT: 0x00000000,
        DDRCTRL_SWCTL: 0x00000001,
        DDRCTRL_SWSTAT: 0x00000001,
        DDRCTRL_POISONCFG: 0x00110011,
        DDRCTRL_POISONSTAT: 0x00000000,
        DDRCTRL_PSTAT: 0x00000000,
        DDRCTRL_PCCFG: 0x00000000,
        DDRCTRL_PCFGR_0: 0x00004000,
        DDRCTRL_PCFGW_0: 0x00004000,
        DDRCTRL_PCTRL_0: 0x00000000,
        DDRCTRL_PCFGQOS0_0: 0x02000E00,
        DDRCTRL_PCFGQOS1_0: 0x00000000,
        DDRCTRL_PCFGWQOS0_0: 0x00000E00,
        DDRCTRL_PCFGWQOS1_0: 0x00000000,
        DDRCTRL_PCFGR_1: 0x00004000,
        DDRCTRL_PCFGW_1: 0x00004000,
        DDRCTRL_PCTRL_1: 0x00000000,
        DDRCTRL_PCFGQOS0_1: 0x02000E00,
        DDRCTRL_PCFGQOS1_1: 0x00000000,
        DDRCTRL_PCFGWQOS0_1: 0x00000E00,
        DDRCTRL_PCFGWQOS1_1: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DDRCTRL_TAKEN: bool = false;

    /// Safe access to DDRCTRL
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
            if DDRCTRL_TAKEN {
                None
            } else {
                DDRCTRL_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DDRCTRL
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DDRCTRL_TAKEN && inst.addr == INSTANCE.addr {
                DDRCTRL_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DDRCTRL
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DDRCTRL_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DDRCTRL
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DDRCTRL: *const RegisterBlock = 0x5a003000 as *const _;
