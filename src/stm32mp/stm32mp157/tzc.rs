#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TZC

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Provides information about TZC configuration.
pub mod TZC_BUILD_CONFIG {

    /// NO_OF_REGIONS
    pub mod NO_OF_REGIONS {
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

    /// ADDRESS_WIDTH
    pub mod ADDRESS_WIDTH {
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

    /// NO_OF_FILTERS
    pub mod NO_OF_FILTERS {
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

/// Controls interrupt and bus error response behavior when regions permission failures occur.
pub mod TZC_ACTION {

    /// REACTION_VALUE
    pub mod REACTION_VALUE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Provides control and status for the gate keeper in each filter unit implemented.
pub mod TZC_GATE_KEEPER {

    /// OPENREQ
    pub mod OPENREQ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OPENSTAT
    pub mod OPENSTAT {
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
}

/// Controls read and write access speculation.
pub mod TZC_SPECULATION_CTRL {

    /// READSPEC_DISABLE
    pub mod READSPEC_DISABLE {
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

    /// WRITESPEC_DISABLE
    pub mod WRITESPEC_DISABLE {
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

/// Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors.
pub mod TZC_INT_STATUS {

    /// STATUS
    pub mod STATUS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OVERRUN
    pub mod OVERRUN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OVERLAP
    pub mod OVERLAP {
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
}

/// Interrupt clear for each filter.
pub mod TZC_INT_CLEAR {

    /// CLEAR
    pub mod CLEAR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Status information about the first access that failed a region permission check in the associated filter (0 to 1).
pub mod TZC_FAIL_CONTROL0 {

    /// PRIVILEGE
    pub mod PRIVILEGE {
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

    /// NON_SECURE
    pub mod NON_SECURE {
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

    /// DIRECTION
    pub mod DIRECTION {
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

/// Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).
pub mod TZC_FAIL_ID0 {

    /// ID
    pub mod ID {
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
}

/// Status information about the first access that failed a region permission check in the associated filter (0 to 1).
pub mod TZC_FAIL_CONTROL1 {
    pub use super::TZC_FAIL_CONTROL0::DIRECTION;
    pub use super::TZC_FAIL_CONTROL0::NON_SECURE;
    pub use super::TZC_FAIL_CONTROL0::PRIVILEGE;
}

/// Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).
pub mod TZC_FAIL_ID1 {
    pub use super::TZC_FAIL_ID0::ID;
}

/// Region 0 attributes.
pub mod TZC_REGION_ATTRIBUTE0 {

    /// FILTER_EN
    pub mod FILTER_EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// S_RD_EN
    pub mod S_RD_EN {
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

    /// S_WR_EN
    pub mod S_WR_EN {
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

/// Region x attributes.
pub mod TZC_REGION_ATTRIBUTE1 {

    /// FILTER_EN
    pub mod FILTER_EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// S_RD_EN
    pub mod S_RD_EN {
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

    /// S_WR_EN
    pub mod S_WR_EN {
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

/// Region x attributes.
pub mod TZC_REGION_ATTRIBUTE2 {
    pub use super::TZC_REGION_ATTRIBUTE1::FILTER_EN;
    pub use super::TZC_REGION_ATTRIBUTE1::S_RD_EN;
    pub use super::TZC_REGION_ATTRIBUTE1::S_WR_EN;
}

/// Region x attributes.
pub mod TZC_REGION_ATTRIBUTE3 {
    pub use super::TZC_REGION_ATTRIBUTE1::FILTER_EN;
    pub use super::TZC_REGION_ATTRIBUTE1::S_RD_EN;
    pub use super::TZC_REGION_ATTRIBUTE1::S_WR_EN;
}

/// Region x attributes.
pub mod TZC_REGION_ATTRIBUTE4 {
    pub use super::TZC_REGION_ATTRIBUTE1::FILTER_EN;
    pub use super::TZC_REGION_ATTRIBUTE1::S_RD_EN;
    pub use super::TZC_REGION_ATTRIBUTE1::S_WR_EN;
}

/// Region x attributes.
pub mod TZC_REGION_ATTRIBUTE5 {
    pub use super::TZC_REGION_ATTRIBUTE1::FILTER_EN;
    pub use super::TZC_REGION_ATTRIBUTE1::S_RD_EN;
    pub use super::TZC_REGION_ATTRIBUTE1::S_WR_EN;
}

/// Region x attributes.
pub mod TZC_REGION_ATTRIBUTE6 {
    pub use super::TZC_REGION_ATTRIBUTE1::FILTER_EN;
    pub use super::TZC_REGION_ATTRIBUTE1::S_RD_EN;
    pub use super::TZC_REGION_ATTRIBUTE1::S_WR_EN;
}

/// Region x attributes.
pub mod TZC_REGION_ATTRIBUTE7 {
    pub use super::TZC_REGION_ATTRIBUTE1::FILTER_EN;
    pub use super::TZC_REGION_ATTRIBUTE1::S_RD_EN;
    pub use super::TZC_REGION_ATTRIBUTE1::S_WR_EN;
}

/// Region x attributes.
pub mod TZC_REGION_ATTRIBUTE8 {
    pub use super::TZC_REGION_ATTRIBUTE1::FILTER_EN;
    pub use super::TZC_REGION_ATTRIBUTE1::S_RD_EN;
    pub use super::TZC_REGION_ATTRIBUTE1::S_WR_EN;
}

/// Peripheral ID 4.
pub mod TZC_PID4 {

    /// PER_ID_4
    pub mod PER_ID_4 {
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

/// Peripheral ID 5.
pub mod TZC_PID5 {

    /// PER_ID_5
    pub mod PER_ID_5 {
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

/// Peripheral ID 6.
pub mod TZC_PID6 {

    /// PER_ID_6
    pub mod PER_ID_6 {
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

/// Peripheral ID 7.
pub mod TZC_PID7 {

    /// PER_ID_7
    pub mod PER_ID_7 {
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

/// Peripheral ID 0.
pub mod TZC_PID0 {

    /// PER_ID_0
    pub mod PER_ID_0 {
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

/// Peripheral ID 1.
pub mod TZC_PID1 {

    /// PER_ID_1
    pub mod PER_ID_1 {
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

/// Peripheral ID 2.
pub mod TZC_PID2 {

    /// PER_ID_2
    pub mod PER_ID_2 {
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

/// Peripheral ID 3.
pub mod TZC_PID3 {

    /// PER_ID_3
    pub mod PER_ID_3 {
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

/// Component ID 0.
pub mod TZC_CID0 {

    /// COMP_ID_0
    pub mod COMP_ID_0 {
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

/// Component ID 1.
pub mod TZC_CID1 {

    /// COMP_ID_1
    pub mod COMP_ID_1 {
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

/// Component ID 2.
pub mod TZC_CID2 {

    /// COMP_ID_2
    pub mod COMP_ID_2 {
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

/// Component ID 3.
pub mod TZC_CID3 {

    /// COMP_ID_3
    pub mod COMP_ID_3 {
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

/// Address low bits of the first failed access in the associated filter (0 to 1).
pub mod TZC_FAIL_ADDRESS_LOW0 {

    /// ADDR_STATUS_LOW
    pub mod ADDR_STATUS_LOW {
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

/// Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.
pub mod TZC_FAIL_ADDRESS_HIGH0 {}

/// Address low bits of the first failed access in the associated filter (0 to 1).
pub mod TZC_FAIL_ADDRESS_LOW1 {
    pub use super::TZC_FAIL_ADDRESS_LOW0::ADDR_STATUS_LOW;
}

/// Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.
pub mod TZC_FAIL_ADDRESS_HIGH1 {}

/// Base address high are not used with 32-bit address.
pub mod TZC_REGION_BASE_HIGH0 {}

/// Top address bits \[31:12\] for region 0.
pub mod TZC_REGION_TOP_LOW0 {

    /// TOP_ADDRESS_LOW
    pub mod TOP_ADDRESS_LOW {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (20 bits: 0xfffff << 12)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Top address high of region are not used with 32-bit address.
pub mod TZC_REGION_TOP_HIGH0 {}

/// Region non-secure access based on NSAID.
pub mod TZC_REGION_ID_ACCESS0 {

    /// NSAID_RD_EN
    pub mod NSAID_RD_EN {
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

    /// NSAID_WR_EN
    pub mod NSAID_WR_EN {
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

/// Base address low for regions 1 to 8.
pub mod TZC_REGION_BASE_LOW1 {

    /// BASE_ADDRESS_LOW
    pub mod BASE_ADDRESS_LOW {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (20 bits: 0xfffff << 12)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Base address high are not used with 32-bit address.
pub mod TZC_REGION_BASE_HIGH1 {}

/// Top address bits \[31:12\] for region x.
pub mod TZC_REGION_TOP_LOW1 {

    /// TOP_ADDRESS_LOW
    pub mod TOP_ADDRESS_LOW {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (20 bits: 0xfffff << 12)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Top address high of region are not used with 32-bit address.
pub mod TZC_REGION_TOP_HIGH1 {}

/// Region non-secure access based on NSAID.
pub mod TZC_REGION_ID_ACCESS1 {
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_RD_EN;
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_WR_EN;
}

/// Base address low for regions 1 to 8.
pub mod TZC_REGION_BASE_LOW2 {
    pub use super::TZC_REGION_BASE_LOW1::BASE_ADDRESS_LOW;
}

/// Base address high are not used with 32-bit address.
pub mod TZC_REGION_BASE_HIGH2 {}

/// Top address bits \[31:12\] for region x.
pub mod TZC_REGION_TOP_LOW2 {
    pub use super::TZC_REGION_TOP_LOW1::TOP_ADDRESS_LOW;
}

/// Top address high of region are not used with 32-bit address.
pub mod TZC_REGION_TOP_HIGH2 {}

/// Region non-secure access based on NSAID.
pub mod TZC_REGION_ID_ACCESS2 {
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_RD_EN;
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_WR_EN;
}

/// Base address low for regions 1 to 8.
pub mod TZC_REGION_BASE_LOW3 {
    pub use super::TZC_REGION_BASE_LOW1::BASE_ADDRESS_LOW;
}

/// Base address high are not used with 32-bit address.
pub mod TZC_REGION_BASE_HIGH3 {}

/// Top address bits \[31:12\] for region x.
pub mod TZC_REGION_TOP_LOW3 {
    pub use super::TZC_REGION_TOP_LOW1::TOP_ADDRESS_LOW;
}

/// Top address high of region are not used with 32-bit address.
pub mod TZC_REGION_TOP_HIGH3 {}

/// Region non-secure access based on NSAID.
pub mod TZC_REGION_ID_ACCESS3 {
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_RD_EN;
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_WR_EN;
}

/// Base address low for regions 1 to 8.
pub mod TZC_REGION_BASE_LOW4 {
    pub use super::TZC_REGION_BASE_LOW1::BASE_ADDRESS_LOW;
}

/// Base address high are not used with 32-bit address.
pub mod TZC_REGION_BASE_HIGH4 {}

/// Top address bits \[31:12\] for region x.
pub mod TZC_REGION_TOP_LOW4 {
    pub use super::TZC_REGION_TOP_LOW1::TOP_ADDRESS_LOW;
}

/// Top address high of region are not used with 32-bit address.
pub mod TZC_REGION_TOP_HIGH4 {}

/// Region non-secure access based on NSAID.
pub mod TZC_REGION_ID_ACCESS4 {
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_RD_EN;
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_WR_EN;
}

/// Base address low for regions 1 to 8.
pub mod TZC_REGION_BASE_LOW5 {
    pub use super::TZC_REGION_BASE_LOW1::BASE_ADDRESS_LOW;
}

/// Base address high are not used with 32-bit address.
pub mod TZC_REGION_BASE_HIGH5 {}

/// Top address bits \[31:12\] for region x.
pub mod TZC_REGION_TOP_LOW5 {
    pub use super::TZC_REGION_TOP_LOW1::TOP_ADDRESS_LOW;
}

/// Top address high of region are not used with 32-bit address.
pub mod TZC_REGION_TOP_HIGH5 {}

/// Region non-secure access based on NSAID.
pub mod TZC_REGION_ID_ACCESS5 {
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_RD_EN;
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_WR_EN;
}

/// Base address low for regions 1 to 8.
pub mod TZC_REGION_BASE_LOW6 {
    pub use super::TZC_REGION_BASE_LOW1::BASE_ADDRESS_LOW;
}

/// Base address high are not used with 32-bit address.
pub mod TZC_REGION_BASE_HIGH6 {}

/// Top address bits \[31:12\] for region x.
pub mod TZC_REGION_TOP_LOW6 {
    pub use super::TZC_REGION_TOP_LOW1::TOP_ADDRESS_LOW;
}

/// Top address high of region are not used with 32-bit address.
pub mod TZC_REGION_TOP_HIGH6 {}

/// Region non-secure access based on NSAID.
pub mod TZC_REGION_ID_ACCESS6 {
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_RD_EN;
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_WR_EN;
}

/// Base address low for regions 1 to 8.
pub mod TZC_REGION_BASE_LOW7 {
    pub use super::TZC_REGION_BASE_LOW1::BASE_ADDRESS_LOW;
}

/// Base address high are not used with 32-bit address.
pub mod TZC_REGION_BASE_HIGH7 {}

/// Top address bits \[31:12\] for region x.
pub mod TZC_REGION_TOP_LOW7 {
    pub use super::TZC_REGION_TOP_LOW1::TOP_ADDRESS_LOW;
}

/// Top address high of region are not used with 32-bit address.
pub mod TZC_REGION_TOP_HIGH7 {}

/// Region non-secure access based on NSAID.
pub mod TZC_REGION_ID_ACCESS7 {
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_RD_EN;
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_WR_EN;
}

/// Base address low for regions 1 to 8.
pub mod TZC_REGION_BASE_LOW8 {
    pub use super::TZC_REGION_BASE_LOW1::BASE_ADDRESS_LOW;
}

/// Base address high are not used with 32-bit address.
pub mod TZC_REGION_BASE_HIGH8 {}

/// Top address bits \[31:12\] for region x.
pub mod TZC_REGION_TOP_LOW8 {
    pub use super::TZC_REGION_TOP_LOW1::TOP_ADDRESS_LOW;
}

/// Top address high of region are not used with 32-bit address.
pub mod TZC_REGION_TOP_HIGH8 {}

/// Region non-secure access based on NSAID.
pub mod TZC_REGION_ID_ACCESS8 {
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_RD_EN;
    pub use super::TZC_REGION_ID_ACCESS0::NSAID_WR_EN;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Provides information about TZC configuration.
    pub TZC_BUILD_CONFIG: RORegister<u32>,

    /// Controls interrupt and bus error response behavior when regions permission failures occur.
    pub TZC_ACTION: RWRegister<u32>,

    /// Provides control and status for the gate keeper in each filter unit implemented.
    pub TZC_GATE_KEEPER: RWRegister<u32>,

    /// Controls read and write access speculation.
    pub TZC_SPECULATION_CTRL: RWRegister<u32>,

    /// Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors.
    pub TZC_INT_STATUS: RORegister<u32>,

    /// Interrupt clear for each filter.
    pub TZC_INT_CLEAR: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// Address low bits of the first failed access in the associated filter (0 to 1).
    pub TZC_FAIL_ADDRESS_LOW0: RORegister<u32>,

    /// Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.
    pub TZC_FAIL_ADDRESS_HIGH0: RORegister<u32>,

    /// Status information about the first access that failed a region permission check in the associated filter (0 to 1).
    pub TZC_FAIL_CONTROL0: RORegister<u32>,

    /// Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).
    pub TZC_FAIL_ID0: RORegister<u32>,

    /// Address low bits of the first failed access in the associated filter (0 to 1).
    pub TZC_FAIL_ADDRESS_LOW1: RORegister<u32>,

    /// Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.
    pub TZC_FAIL_ADDRESS_HIGH1: RORegister<u32>,

    /// Status information about the first access that failed a region permission check in the associated filter (0 to 1).
    pub TZC_FAIL_CONTROL1: RORegister<u32>,

    /// Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).
    pub TZC_FAIL_ID1: RORegister<u32>,

    _reserved2: [u32; 49],

    /// Base address high are not used with 32-bit address.
    pub TZC_REGION_BASE_HIGH0: RORegister<u32>,

    /// Top address bits \[31:12\] for region 0.
    pub TZC_REGION_TOP_LOW0: RORegister<u32>,

    /// Top address high of region are not used with 32-bit address.
    pub TZC_REGION_TOP_HIGH0: RORegister<u32>,

    /// Region 0 attributes.
    pub TZC_REGION_ATTRIBUTE0: RWRegister<u32>,

    /// Region non-secure access based on NSAID.
    pub TZC_REGION_ID_ACCESS0: RWRegister<u32>,

    _reserved3: [u32; 2],

    /// Base address low for regions 1 to 8.
    pub TZC_REGION_BASE_LOW1: RWRegister<u32>,

    /// Base address high are not used with 32-bit address.
    pub TZC_REGION_BASE_HIGH1: RORegister<u32>,

    /// Top address bits \[31:12\] for region x.
    pub TZC_REGION_TOP_LOW1: RWRegister<u32>,

    /// Top address high of region are not used with 32-bit address.
    pub TZC_REGION_TOP_HIGH1: RORegister<u32>,

    /// Region x attributes.
    pub TZC_REGION_ATTRIBUTE1: RWRegister<u32>,

    /// Region non-secure access based on NSAID.
    pub TZC_REGION_ID_ACCESS1: RWRegister<u32>,

    _reserved4: [u32; 2],

    /// Base address low for regions 1 to 8.
    pub TZC_REGION_BASE_LOW2: RWRegister<u32>,

    /// Base address high are not used with 32-bit address.
    pub TZC_REGION_BASE_HIGH2: RORegister<u32>,

    /// Top address bits \[31:12\] for region x.
    pub TZC_REGION_TOP_LOW2: RWRegister<u32>,

    /// Top address high of region are not used with 32-bit address.
    pub TZC_REGION_TOP_HIGH2: RORegister<u32>,

    /// Region x attributes.
    pub TZC_REGION_ATTRIBUTE2: RWRegister<u32>,

    /// Region non-secure access based on NSAID.
    pub TZC_REGION_ID_ACCESS2: RWRegister<u32>,

    _reserved5: [u32; 2],

    /// Base address low for regions 1 to 8.
    pub TZC_REGION_BASE_LOW3: RWRegister<u32>,

    /// Base address high are not used with 32-bit address.
    pub TZC_REGION_BASE_HIGH3: RORegister<u32>,

    /// Top address bits \[31:12\] for region x.
    pub TZC_REGION_TOP_LOW3: RWRegister<u32>,

    /// Top address high of region are not used with 32-bit address.
    pub TZC_REGION_TOP_HIGH3: RORegister<u32>,

    /// Region x attributes.
    pub TZC_REGION_ATTRIBUTE3: RWRegister<u32>,

    /// Region non-secure access based on NSAID.
    pub TZC_REGION_ID_ACCESS3: RWRegister<u32>,

    _reserved6: [u32; 2],

    /// Base address low for regions 1 to 8.
    pub TZC_REGION_BASE_LOW4: RWRegister<u32>,

    /// Base address high are not used with 32-bit address.
    pub TZC_REGION_BASE_HIGH4: RORegister<u32>,

    /// Top address bits \[31:12\] for region x.
    pub TZC_REGION_TOP_LOW4: RWRegister<u32>,

    /// Top address high of region are not used with 32-bit address.
    pub TZC_REGION_TOP_HIGH4: RORegister<u32>,

    /// Region x attributes.
    pub TZC_REGION_ATTRIBUTE4: RWRegister<u32>,

    /// Region non-secure access based on NSAID.
    pub TZC_REGION_ID_ACCESS4: RWRegister<u32>,

    _reserved7: [u32; 2],

    /// Base address low for regions 1 to 8.
    pub TZC_REGION_BASE_LOW5: RWRegister<u32>,

    /// Base address high are not used with 32-bit address.
    pub TZC_REGION_BASE_HIGH5: RORegister<u32>,

    /// Top address bits \[31:12\] for region x.
    pub TZC_REGION_TOP_LOW5: RWRegister<u32>,

    /// Top address high of region are not used with 32-bit address.
    pub TZC_REGION_TOP_HIGH5: RORegister<u32>,

    /// Region x attributes.
    pub TZC_REGION_ATTRIBUTE5: RWRegister<u32>,

    /// Region non-secure access based on NSAID.
    pub TZC_REGION_ID_ACCESS5: RWRegister<u32>,

    _reserved8: [u32; 2],

    /// Base address low for regions 1 to 8.
    pub TZC_REGION_BASE_LOW6: RWRegister<u32>,

    /// Base address high are not used with 32-bit address.
    pub TZC_REGION_BASE_HIGH6: RORegister<u32>,

    /// Top address bits \[31:12\] for region x.
    pub TZC_REGION_TOP_LOW6: RWRegister<u32>,

    /// Top address high of region are not used with 32-bit address.
    pub TZC_REGION_TOP_HIGH6: RORegister<u32>,

    /// Region x attributes.
    pub TZC_REGION_ATTRIBUTE6: RWRegister<u32>,

    /// Region non-secure access based on NSAID.
    pub TZC_REGION_ID_ACCESS6: RWRegister<u32>,

    _reserved9: [u32; 4],

    /// Top address bits \[31:12\] for region x.
    pub TZC_REGION_TOP_LOW7: RWRegister<u32>,

    _reserved10: [u32; 1],

    /// Region x attributes.
    pub TZC_REGION_ATTRIBUTE7: RWRegister<u32>,

    _reserved11: [u32; 3],

    /// Base address low for regions 1 to 8.
    pub TZC_REGION_BASE_LOW8: RWRegister<u32>,

    /// Base address high are not used with 32-bit address.
    pub TZC_REGION_BASE_HIGH8: RORegister<u32>,

    _reserved12: [u32; 2],

    /// Region x attributes.
    pub TZC_REGION_ATTRIBUTE8: RWRegister<u32>,

    _reserved13: [u32; 51],

    /// Base address low for regions 1 to 8.
    pub TZC_REGION_BASE_LOW7: RWRegister<u32>,

    /// Base address high are not used with 32-bit address.
    pub TZC_REGION_BASE_HIGH7: RORegister<u32>,

    _reserved14: [u32; 1],

    /// Top address high of region are not used with 32-bit address.
    pub TZC_REGION_TOP_HIGH7: RORegister<u32>,

    _reserved15: [u32; 1],

    /// Region non-secure access based on NSAID.
    pub TZC_REGION_ID_ACCESS7: RWRegister<u32>,

    _reserved16: [u32; 4],

    /// Top address bits \[31:12\] for region x.
    pub TZC_REGION_TOP_LOW8: RWRegister<u32>,

    /// Top address high of region are not used with 32-bit address.
    pub TZC_REGION_TOP_HIGH8: RORegister<u32>,

    _reserved17: [u32; 1],

    /// Region non-secure access based on NSAID.
    pub TZC_REGION_ID_ACCESS8: RWRegister<u32>,

    _reserved18: [u32; 814],

    /// Peripheral ID 4.
    pub TZC_PID4: RORegister<u32>,

    /// Peripheral ID 5.
    pub TZC_PID5: RORegister<u32>,

    /// Peripheral ID 6.
    pub TZC_PID6: RORegister<u32>,

    /// Peripheral ID 7.
    pub TZC_PID7: RORegister<u32>,

    /// Peripheral ID 0.
    pub TZC_PID0: RORegister<u32>,

    /// Peripheral ID 1.
    pub TZC_PID1: RORegister<u32>,

    /// Peripheral ID 2.
    pub TZC_PID2: RORegister<u32>,

    /// Peripheral ID 3.
    pub TZC_PID3: RORegister<u32>,

    /// Component ID 0.
    pub TZC_CID0: RORegister<u32>,

    /// Component ID 1.
    pub TZC_CID1: RORegister<u32>,

    /// Component ID 2.
    pub TZC_CID2: RORegister<u32>,

    /// Component ID 3.
    pub TZC_CID3: RORegister<u32>,
}
pub struct ResetValues {
    pub TZC_BUILD_CONFIG: u32,
    pub TZC_ACTION: u32,
    pub TZC_GATE_KEEPER: u32,
    pub TZC_SPECULATION_CTRL: u32,
    pub TZC_INT_STATUS: u32,
    pub TZC_INT_CLEAR: u32,
    pub TZC_FAIL_ADDRESS_LOW0: u32,
    pub TZC_FAIL_ADDRESS_HIGH0: u32,
    pub TZC_FAIL_CONTROL0: u32,
    pub TZC_FAIL_ID0: u32,
    pub TZC_FAIL_ADDRESS_LOW1: u32,
    pub TZC_FAIL_ADDRESS_HIGH1: u32,
    pub TZC_FAIL_CONTROL1: u32,
    pub TZC_FAIL_ID1: u32,
    pub TZC_REGION_BASE_HIGH0: u32,
    pub TZC_REGION_TOP_LOW0: u32,
    pub TZC_REGION_TOP_HIGH0: u32,
    pub TZC_REGION_ATTRIBUTE0: u32,
    pub TZC_REGION_ID_ACCESS0: u32,
    pub TZC_REGION_BASE_LOW1: u32,
    pub TZC_REGION_BASE_HIGH1: u32,
    pub TZC_REGION_TOP_LOW1: u32,
    pub TZC_REGION_TOP_HIGH1: u32,
    pub TZC_REGION_ATTRIBUTE1: u32,
    pub TZC_REGION_ID_ACCESS1: u32,
    pub TZC_REGION_BASE_LOW2: u32,
    pub TZC_REGION_BASE_HIGH2: u32,
    pub TZC_REGION_TOP_LOW2: u32,
    pub TZC_REGION_TOP_HIGH2: u32,
    pub TZC_REGION_ATTRIBUTE2: u32,
    pub TZC_REGION_ID_ACCESS2: u32,
    pub TZC_REGION_BASE_LOW3: u32,
    pub TZC_REGION_BASE_HIGH3: u32,
    pub TZC_REGION_TOP_LOW3: u32,
    pub TZC_REGION_TOP_HIGH3: u32,
    pub TZC_REGION_ATTRIBUTE3: u32,
    pub TZC_REGION_ID_ACCESS3: u32,
    pub TZC_REGION_BASE_LOW4: u32,
    pub TZC_REGION_BASE_HIGH4: u32,
    pub TZC_REGION_TOP_LOW4: u32,
    pub TZC_REGION_TOP_HIGH4: u32,
    pub TZC_REGION_ATTRIBUTE4: u32,
    pub TZC_REGION_ID_ACCESS4: u32,
    pub TZC_REGION_BASE_LOW5: u32,
    pub TZC_REGION_BASE_HIGH5: u32,
    pub TZC_REGION_TOP_LOW5: u32,
    pub TZC_REGION_TOP_HIGH5: u32,
    pub TZC_REGION_ATTRIBUTE5: u32,
    pub TZC_REGION_ID_ACCESS5: u32,
    pub TZC_REGION_BASE_LOW6: u32,
    pub TZC_REGION_BASE_HIGH6: u32,
    pub TZC_REGION_TOP_LOW6: u32,
    pub TZC_REGION_TOP_HIGH6: u32,
    pub TZC_REGION_ATTRIBUTE6: u32,
    pub TZC_REGION_ID_ACCESS6: u32,
    pub TZC_REGION_TOP_LOW7: u32,
    pub TZC_REGION_ATTRIBUTE7: u32,
    pub TZC_REGION_BASE_LOW8: u32,
    pub TZC_REGION_BASE_HIGH8: u32,
    pub TZC_REGION_ATTRIBUTE8: u32,
    pub TZC_REGION_BASE_LOW7: u32,
    pub TZC_REGION_BASE_HIGH7: u32,
    pub TZC_REGION_TOP_HIGH7: u32,
    pub TZC_REGION_ID_ACCESS7: u32,
    pub TZC_REGION_TOP_LOW8: u32,
    pub TZC_REGION_TOP_HIGH8: u32,
    pub TZC_REGION_ID_ACCESS8: u32,
    pub TZC_PID4: u32,
    pub TZC_PID5: u32,
    pub TZC_PID6: u32,
    pub TZC_PID7: u32,
    pub TZC_PID0: u32,
    pub TZC_PID1: u32,
    pub TZC_PID2: u32,
    pub TZC_PID3: u32,
    pub TZC_CID0: u32,
    pub TZC_CID1: u32,
    pub TZC_CID2: u32,
    pub TZC_CID3: u32,
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

/// Access functions for the TZC peripheral instance
pub mod TZC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5c006000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TZC
    pub const reset: ResetValues = ResetValues {
        TZC_BUILD_CONFIG: 0x01001F08,
        TZC_ACTION: 0x00000000,
        TZC_GATE_KEEPER: 0x00000000,
        TZC_SPECULATION_CTRL: 0x00000000,
        TZC_INT_STATUS: 0x00000000,
        TZC_INT_CLEAR: 0x00000000,
        TZC_FAIL_CONTROL0: 0x00000000,
        TZC_FAIL_ID0: 0x00000000,
        TZC_FAIL_CONTROL1: 0x00000000,
        TZC_FAIL_ID1: 0x00000000,
        TZC_REGION_ATTRIBUTE0: 0x00000003,
        TZC_REGION_ATTRIBUTE1: 0x00000000,
        TZC_REGION_ATTRIBUTE2: 0x00000000,
        TZC_REGION_ATTRIBUTE3: 0x00000000,
        TZC_REGION_ATTRIBUTE4: 0x00000000,
        TZC_REGION_ATTRIBUTE5: 0x00000000,
        TZC_REGION_ATTRIBUTE6: 0x00000000,
        TZC_REGION_ATTRIBUTE7: 0x00000000,
        TZC_REGION_ATTRIBUTE8: 0x00000000,
        TZC_PID4: 0x00000004,
        TZC_PID5: 0x00000000,
        TZC_PID6: 0x00000000,
        TZC_PID7: 0x00000000,
        TZC_PID0: 0x00000060,
        TZC_PID1: 0x000000B4,
        TZC_PID2: 0x0000002B,
        TZC_PID3: 0x00000000,
        TZC_CID0: 0x0000000D,
        TZC_CID1: 0x000000F0,
        TZC_CID2: 0x00000005,
        TZC_CID3: 0x000000B1,
        TZC_FAIL_ADDRESS_LOW0: 0x00000000,
        TZC_FAIL_ADDRESS_HIGH0: 0x00000000,
        TZC_FAIL_ADDRESS_LOW1: 0x00000000,
        TZC_FAIL_ADDRESS_HIGH1: 0x00000000,
        TZC_REGION_BASE_HIGH0: 0x00000000,
        TZC_REGION_TOP_LOW0: 0xFFFFFFFF,
        TZC_REGION_TOP_HIGH0: 0x00000000,
        TZC_REGION_ID_ACCESS0: 0x00000000,
        TZC_REGION_BASE_LOW1: 0x00000000,
        TZC_REGION_BASE_HIGH1: 0x00000000,
        TZC_REGION_TOP_LOW1: 0x00000FFF,
        TZC_REGION_TOP_HIGH1: 0x00000000,
        TZC_REGION_ID_ACCESS1: 0x00000000,
        TZC_REGION_BASE_LOW2: 0x00000000,
        TZC_REGION_BASE_HIGH2: 0x00000000,
        TZC_REGION_TOP_LOW2: 0x00000FFF,
        TZC_REGION_TOP_HIGH2: 0x00000000,
        TZC_REGION_ID_ACCESS2: 0x00000000,
        TZC_REGION_BASE_LOW3: 0x00000000,
        TZC_REGION_BASE_HIGH3: 0x00000000,
        TZC_REGION_TOP_LOW3: 0x00000FFF,
        TZC_REGION_TOP_HIGH3: 0x00000000,
        TZC_REGION_ID_ACCESS3: 0x00000000,
        TZC_REGION_BASE_LOW4: 0x00000000,
        TZC_REGION_BASE_HIGH4: 0x00000000,
        TZC_REGION_TOP_LOW4: 0x00000FFF,
        TZC_REGION_TOP_HIGH4: 0x00000000,
        TZC_REGION_ID_ACCESS4: 0x00000000,
        TZC_REGION_BASE_LOW5: 0x00000000,
        TZC_REGION_BASE_HIGH5: 0x00000000,
        TZC_REGION_TOP_LOW5: 0x00000FFF,
        TZC_REGION_TOP_HIGH5: 0x00000000,
        TZC_REGION_ID_ACCESS5: 0x00000000,
        TZC_REGION_BASE_LOW6: 0x00000000,
        TZC_REGION_BASE_HIGH6: 0x00000000,
        TZC_REGION_TOP_LOW6: 0x00000FFF,
        TZC_REGION_TOP_HIGH6: 0x00000000,
        TZC_REGION_ID_ACCESS6: 0x00000000,
        TZC_REGION_BASE_LOW7: 0x00000000,
        TZC_REGION_BASE_HIGH7: 0x00000000,
        TZC_REGION_TOP_LOW7: 0x00000FFF,
        TZC_REGION_TOP_HIGH7: 0x00000000,
        TZC_REGION_ID_ACCESS7: 0x00000000,
        TZC_REGION_BASE_LOW8: 0x00000000,
        TZC_REGION_BASE_HIGH8: 0x00000000,
        TZC_REGION_TOP_LOW8: 0x00000FFF,
        TZC_REGION_TOP_HIGH8: 0x00000000,
        TZC_REGION_ID_ACCESS8: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TZC_TAKEN: bool = false;

    /// Safe access to TZC
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
            if TZC_TAKEN {
                None
            } else {
                TZC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TZC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TZC_TAKEN && inst.addr == INSTANCE.addr {
                TZC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TZC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TZC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TZC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TZC: *const RegisterBlock = 0x5c006000 as *const _;
