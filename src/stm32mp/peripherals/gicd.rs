#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GICD
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// GICD control register
pub mod GICD_CTLR {

    /// ENABLEGRP0
    pub mod ENABLEGRP0 {
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

    /// ENABLEGRP1
    pub mod ENABLEGRP1 {
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

/// GICD interrupt controller type register
pub mod GICD_TYPER {

    /// ITLINESNUMBER
    pub mod ITLINESNUMBER {
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

    /// CPUNUMBER
    pub mod CPUNUMBER {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (3 bits: 0b111 << 5)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SECURITYEXTN
    pub mod SECURITYEXTN {
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

    /// LSPI
    pub mod LSPI {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GICD implementer identification register
pub mod GICD_IIDR {

    /// IMPLEMENTER
    pub mod IMPLEMENTER {
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

    /// VARIANT
    pub mod VARIANT {
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

    /// REVISION
    pub mod REVISION {
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

    /// PRODUCTID
    pub mod PRODUCTID {
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

/// For interrupts ID
pub mod GICD_IGROUPR0 {

    /// IGROUPR0
    pub mod IGROUPR0 {
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

/// For interrupts ID
pub mod GICD_IGROUPR1 {

    /// IGROUPR1
    pub mod IGROUPR1 {
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

/// For interrupts ID
pub mod GICD_IGROUPR2 {

    /// IGROUPR2
    pub mod IGROUPR2 {
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

/// For interrupts ID = x*32 to ID = x*32+31
pub mod GICD_IGROUPR3 {

    /// IGROUPR3
    pub mod IGROUPR3 {
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

/// For interrupts ID = x*32 to ID = x*32+31
pub mod GICD_IGROUPR4 {

    /// IGROUPR4
    pub mod IGROUPR4 {
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

/// For interrupts ID
pub mod GICD_IGROUPR5 {

    /// IGROUPR5
    pub mod IGROUPR5 {
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

/// For interrupts ID
pub mod GICD_IGROUPR6 {

    /// IGROUPR6
    pub mod IGROUPR6 {
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

/// For interrupts ID
pub mod GICD_IGROUPR7 {

    /// IGROUPR7
    pub mod IGROUPR7 {
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

/// For interrupts ID
pub mod GICD_IGROUPR8 {

    /// IGROUPR8
    pub mod IGROUPR8 {
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

/// For interrupts ID = 0 to ID = 31
pub mod GICD_ISENABLER0 {

    /// ISENABLER0
    pub mod ISENABLER0 {
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

/// For interrupts ID
pub mod GICD_ISENABLER1 {

    /// ISENABLER1
    pub mod ISENABLER1 {
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

/// For interrupts ID
pub mod GICD_ISENABLER2 {

    /// ISENABLER2
    pub mod ISENABLER2 {
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

/// For interrupts ID
pub mod GICD_ISENABLER3 {

    /// ISENABLER3
    pub mod ISENABLER3 {
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

/// For interrupts ID
pub mod GICD_ISENABLER4 {

    /// ISENABLER4
    pub mod ISENABLER4 {
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

/// For interrupts ID
pub mod GICD_ISENABLER5 {

    /// ISENABLER5
    pub mod ISENABLER5 {
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

/// For interrupts ID
pub mod GICD_ISENABLER6 {

    /// ISENABLER6
    pub mod ISENABLER6 {
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

/// For interrupts ID
pub mod GICD_ISENABLER7 {

    /// ISENABLER7
    pub mod ISENABLER7 {
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

/// For interrupts ID
pub mod GICD_ISENABLER8 {

    /// ISENABLER8
    pub mod ISENABLER8 {
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

/// For interrupts ID = 0 to ID = 31
pub mod GICD_ICENABLER0 {

    /// ICENABLER0
    pub mod ICENABLER0 {
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

/// For interrupts ID
pub mod GICD_ICENABLER1 {

    /// ICENABLER1
    pub mod ICENABLER1 {
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

/// For interrupts ID
pub mod GICD_ICENABLER2 {

    /// ICENABLER2
    pub mod ICENABLER2 {
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

/// For interrupts ID
pub mod GICD_ICENABLER3 {

    /// ICENABLER3
    pub mod ICENABLER3 {
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

/// For interrupts ID
pub mod GICD_ICENABLER4 {

    /// ICENABLER4
    pub mod ICENABLER4 {
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

/// For interrupts ID
pub mod GICD_ICENABLER5 {

    /// ICENABLER5
    pub mod ICENABLER5 {
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

/// For interrupts ID
pub mod GICD_ICENABLER6 {

    /// ICENABLER6
    pub mod ICENABLER6 {
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

/// For interrupts ID
pub mod GICD_ICENABLER7 {

    /// ICENABLER7
    pub mod ICENABLER7 {
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

/// For interrupts ID
pub mod GICD_ICENABLER8 {

    /// ICENABLER8
    pub mod ICENABLER8 {
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

/// For interrupts ID
pub mod GICD_ISPENDR0 {

    /// ISPENDR0
    pub mod ISPENDR0 {
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

/// For interrupts ID
pub mod GICD_ISPENDR1 {

    /// ISPENDR1
    pub mod ISPENDR1 {
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

/// For interrupts ID
pub mod GICD_ISPENDR2 {

    /// ISPENDR2
    pub mod ISPENDR2 {
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

/// For interrupts ID
pub mod GICD_ISPENDR3 {

    /// ISPENDR3
    pub mod ISPENDR3 {
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

/// For interrupts ID
pub mod GICD_ISPENDR4 {

    /// ISPENDR4
    pub mod ISPENDR4 {
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

/// For interrupts ID
pub mod GICD_ISPENDR5 {

    /// ISPENDR5
    pub mod ISPENDR5 {
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

/// For interrupts ID
pub mod GICD_ISPENDR6 {

    /// ISPENDR6
    pub mod ISPENDR6 {
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

/// For interrupts ID
pub mod GICD_ISPENDR7 {

    /// ISPENDR7
    pub mod ISPENDR7 {
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

/// For interrupts ID
pub mod GICD_ISPENDR8 {

    /// ISPENDR8
    pub mod ISPENDR8 {
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

/// For interrupts ID
pub mod GICD_ICPENDR0 {

    /// ICPENDR0
    pub mod ICPENDR0 {
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

/// For interrupts ID
pub mod GICD_ICPENDR1 {

    /// ICPENDR1
    pub mod ICPENDR1 {
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

/// For interrupts ID
pub mod GICD_ICPENDR2 {

    /// ICPENDR2
    pub mod ICPENDR2 {
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

/// For interrupts ID
pub mod GICD_ICPENDR3 {

    /// ICPENDR3
    pub mod ICPENDR3 {
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

/// For interrupts ID
pub mod GICD_ICPENDR4 {

    /// ICPENDR4
    pub mod ICPENDR4 {
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

/// For interrupts ID
pub mod GICD_ICPENDR5 {

    /// ICPENDR5
    pub mod ICPENDR5 {
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

/// For interrupts ID
pub mod GICD_ICPENDR6 {

    /// ICPENDR6
    pub mod ICPENDR6 {
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

/// For interrupts ID
pub mod GICD_ICPENDR7 {

    /// ICPENDR7
    pub mod ICPENDR7 {
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

/// For interrupts ID
pub mod GICD_ICPENDR8 {

    /// ICPENDR8
    pub mod ICPENDR8 {
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

/// For interrupts ID
pub mod GICD_ISACTIVER0 {

    /// ISACTIVER0
    pub mod ISACTIVER0 {
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

/// For interrupts ID
pub mod GICD_ISACTIVER1 {

    /// ISACTIVER1
    pub mod ISACTIVER1 {
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

/// For interrupts ID
pub mod GICD_ISACTIVER2 {

    /// ISACTIVER2
    pub mod ISACTIVER2 {
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

/// For interrupts ID
pub mod GICD_ISACTIVER3 {

    /// ISACTIVER3
    pub mod ISACTIVER3 {
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

/// For interrupts ID
pub mod GICD_ISACTIVER4 {

    /// ISACTIVER4
    pub mod ISACTIVER4 {
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

/// For interrupts ID
pub mod GICD_ISACTIVER5 {

    /// ISACTIVER5
    pub mod ISACTIVER5 {
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

/// For interrupts ID
pub mod GICD_ISACTIVER6 {

    /// ISACTIVER6
    pub mod ISACTIVER6 {
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

/// For interrupts ID
pub mod GICD_ISACTIVER7 {

    /// ISACTIVER7
    pub mod ISACTIVER7 {
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

/// For interrupts ID
pub mod GICD_ISACTIVER8 {

    /// ISACTIVER8
    pub mod ISACTIVER8 {
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

/// For interrupts ID
pub mod GICD_ICACTIVER0 {

    /// ICACTIVER0
    pub mod ICACTIVER0 {
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

/// For interrupts ID
pub mod GICD_ICACTIVER1 {

    /// ICACTIVER1
    pub mod ICACTIVER1 {
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

/// For interrupts ID
pub mod GICD_ICACTIVER2 {

    /// ICACTIVER2
    pub mod ICACTIVER2 {
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

/// For interrupts ID
pub mod GICD_ICACTIVER3 {

    /// ICACTIVER3
    pub mod ICACTIVER3 {
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

/// For interrupts ID
pub mod GICD_ICACTIVER4 {

    /// ICACTIVER4
    pub mod ICACTIVER4 {
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

/// For interrupts ID
pub mod GICD_ICACTIVER5 {

    /// ICACTIVER5
    pub mod ICACTIVER5 {
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

/// For interrupts ID
pub mod GICD_ICACTIVER6 {

    /// ICACTIVER6
    pub mod ICACTIVER6 {
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

/// For interrupts ID
pub mod GICD_ICACTIVER7 {

    /// ICACTIVER7
    pub mod ICACTIVER7 {
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

/// For interrupts ID
pub mod GICD_ICACTIVER8 {

    /// ICACTIVER8
    pub mod ICACTIVER8 {
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

/// GICD interrupt priority register 0
pub mod GICD_IPRIORITYR0 {

    /// PRIORITY0
    pub mod PRIORITY0 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (5 bits: 0b11111 << 3)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PRIORITY1
    pub mod PRIORITY1 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PRIORITY2
    pub mod PRIORITY2 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (5 bits: 0b11111 << 19)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PRIORITY3
    pub mod PRIORITY3 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (5 bits: 0b11111 << 27)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GICD interrupt priority register 1
pub mod GICD_IPRIORITYR1 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 2
pub mod GICD_IPRIORITYR2 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 3
pub mod GICD_IPRIORITYR3 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 4
pub mod GICD_IPRIORITYR4 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 5
pub mod GICD_IPRIORITYR5 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 6
pub mod GICD_IPRIORITYR6 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 7
pub mod GICD_IPRIORITYR7 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 8
pub mod GICD_IPRIORITYR8 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 9
pub mod GICD_IPRIORITYR9 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 10
pub mod GICD_IPRIORITYR10 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 11
pub mod GICD_IPRIORITYR11 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 12
pub mod GICD_IPRIORITYR12 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 13
pub mod GICD_IPRIORITYR13 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 14
pub mod GICD_IPRIORITYR14 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 15
pub mod GICD_IPRIORITYR15 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 16
pub mod GICD_IPRIORITYR16 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 17
pub mod GICD_IPRIORITYR17 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 18
pub mod GICD_IPRIORITYR18 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 19
pub mod GICD_IPRIORITYR19 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 20
pub mod GICD_IPRIORITYR20 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 21
pub mod GICD_IPRIORITYR21 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 22
pub mod GICD_IPRIORITYR22 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 23
pub mod GICD_IPRIORITYR23 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 24
pub mod GICD_IPRIORITYR24 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 25
pub mod GICD_IPRIORITYR25 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 26
pub mod GICD_IPRIORITYR26 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 27
pub mod GICD_IPRIORITYR27 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 28
pub mod GICD_IPRIORITYR28 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 29
pub mod GICD_IPRIORITYR29 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 30
pub mod GICD_IPRIORITYR30 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 31
pub mod GICD_IPRIORITYR31 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 32
pub mod GICD_IPRIORITYR32 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 33
pub mod GICD_IPRIORITYR33 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 34
pub mod GICD_IPRIORITYR34 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 35
pub mod GICD_IPRIORITYR35 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 36
pub mod GICD_IPRIORITYR36 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 37
pub mod GICD_IPRIORITYR37 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 38
pub mod GICD_IPRIORITYR38 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 39
pub mod GICD_IPRIORITYR39 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 40
pub mod GICD_IPRIORITYR40 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 41
pub mod GICD_IPRIORITYR41 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 42
pub mod GICD_IPRIORITYR42 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 43
pub mod GICD_IPRIORITYR43 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 44
pub mod GICD_IPRIORITYR44 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 45
pub mod GICD_IPRIORITYR45 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 46
pub mod GICD_IPRIORITYR46 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 47
pub mod GICD_IPRIORITYR47 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 48
pub mod GICD_IPRIORITYR48 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 49
pub mod GICD_IPRIORITYR49 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 50
pub mod GICD_IPRIORITYR50 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 51
pub mod GICD_IPRIORITYR51 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 52
pub mod GICD_IPRIORITYR52 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 53
pub mod GICD_IPRIORITYR53 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 54
pub mod GICD_IPRIORITYR54 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 55
pub mod GICD_IPRIORITYR55 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 56
pub mod GICD_IPRIORITYR56 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 57
pub mod GICD_IPRIORITYR57 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 58
pub mod GICD_IPRIORITYR58 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 59
pub mod GICD_IPRIORITYR59 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 60
pub mod GICD_IPRIORITYR60 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 61
pub mod GICD_IPRIORITYR61 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 62
pub mod GICD_IPRIORITYR62 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 63
pub mod GICD_IPRIORITYR63 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 64
pub mod GICD_IPRIORITYR64 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 65
pub mod GICD_IPRIORITYR65 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 66
pub mod GICD_IPRIORITYR66 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 67
pub mod GICD_IPRIORITYR67 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 68
pub mod GICD_IPRIORITYR68 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 69
pub mod GICD_IPRIORITYR69 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 70
pub mod GICD_IPRIORITYR70 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// GICD interrupt priority register 71
pub mod GICD_IPRIORITYR71 {
    pub use super::GICD_IPRIORITYR0::PRIORITY0;
    pub use super::GICD_IPRIORITYR0::PRIORITY1;
    pub use super::GICD_IPRIORITYR0::PRIORITY2;
    pub use super::GICD_IPRIORITYR0::PRIORITY3;
}

/// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod GICD_ITARGETSR0 {

    /// CPU_TARGETS0
    pub mod CPU_TARGETS0 {
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

    /// CPU_TARGETS1
    pub mod CPU_TARGETS1 {
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

    /// CPU_TARGETS2
    pub mod CPU_TARGETS2 {
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

    /// CPU_TARGETS3
    pub mod CPU_TARGETS3 {
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

/// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod GICD_ITARGETSR1 {
    pub use super::GICD_ITARGETSR0::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS3;
}

/// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod GICD_ITARGETSR2 {
    pub use super::GICD_ITARGETSR0::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS3;
}

/// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod GICD_ITARGETSR3 {
    pub use super::GICD_ITARGETSR0::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS3;
}

/// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod GICD_ITARGETSR4 {
    pub use super::GICD_ITARGETSR0::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS3;
}

/// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod GICD_ITARGETSR5 {
    pub use super::GICD_ITARGETSR0::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS3;
}

/// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod GICD_ITARGETSR6 {
    pub use super::GICD_ITARGETSR0::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS3;
}

/// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod GICD_ITARGETSR7 {
    pub use super::GICD_ITARGETSR0::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR0::CPU_TARGETS3;
}

/// GICD interrupt processor target register 8
pub mod GICD_ITARGETSR8 {

    /// CPU_TARGETS0
    pub mod CPU_TARGETS0 {
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

    /// CPU_TARGETS1
    pub mod CPU_TARGETS1 {
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

    /// CPU_TARGETS2
    pub mod CPU_TARGETS2 {
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

    /// CPU_TARGETS3
    pub mod CPU_TARGETS3 {
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

/// GICD interrupt processor target register 9
pub mod GICD_ITARGETSR9 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 10
pub mod GICD_ITARGETSR10 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 11
pub mod GICD_ITARGETSR11 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 12
pub mod GICD_ITARGETSR12 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 13
pub mod GICD_ITARGETSR13 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 14
pub mod GICD_ITARGETSR14 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 15
pub mod GICD_ITARGETSR15 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 16
pub mod GICD_ITARGETSR16 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 17
pub mod GICD_ITARGETSR17 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 18
pub mod GICD_ITARGETSR18 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 19
pub mod GICD_ITARGETSR19 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 20
pub mod GICD_ITARGETSR20 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 21
pub mod GICD_ITARGETSR21 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 22
pub mod GICD_ITARGETSR22 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 23
pub mod GICD_ITARGETSR23 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 24
pub mod GICD_ITARGETSR24 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 25
pub mod GICD_ITARGETSR25 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 26
pub mod GICD_ITARGETSR26 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 27
pub mod GICD_ITARGETSR27 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 28
pub mod GICD_ITARGETSR28 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 29
pub mod GICD_ITARGETSR29 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 30
pub mod GICD_ITARGETSR30 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 31
pub mod GICD_ITARGETSR31 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 32
pub mod GICD_ITARGETSR32 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 33
pub mod GICD_ITARGETSR33 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 34
pub mod GICD_ITARGETSR34 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 35
pub mod GICD_ITARGETSR35 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 36
pub mod GICD_ITARGETSR36 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 37
pub mod GICD_ITARGETSR37 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 38
pub mod GICD_ITARGETSR38 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 39
pub mod GICD_ITARGETSR39 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 40
pub mod GICD_ITARGETSR40 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 41
pub mod GICD_ITARGETSR41 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 42
pub mod GICD_ITARGETSR42 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 43
pub mod GICD_ITARGETSR43 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 44
pub mod GICD_ITARGETSR44 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 45
pub mod GICD_ITARGETSR45 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 46
pub mod GICD_ITARGETSR46 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 47
pub mod GICD_ITARGETSR47 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 48
pub mod GICD_ITARGETSR48 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 49
pub mod GICD_ITARGETSR49 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 50
pub mod GICD_ITARGETSR50 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 51
pub mod GICD_ITARGETSR51 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 52
pub mod GICD_ITARGETSR52 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 53
pub mod GICD_ITARGETSR53 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 54
pub mod GICD_ITARGETSR54 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 55
pub mod GICD_ITARGETSR55 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 56
pub mod GICD_ITARGETSR56 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 57
pub mod GICD_ITARGETSR57 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 58
pub mod GICD_ITARGETSR58 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 59
pub mod GICD_ITARGETSR59 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 60
pub mod GICD_ITARGETSR60 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 61
pub mod GICD_ITARGETSR61 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 62
pub mod GICD_ITARGETSR62 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 63
pub mod GICD_ITARGETSR63 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 64
pub mod GICD_ITARGETSR64 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 65
pub mod GICD_ITARGETSR65 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 66
pub mod GICD_ITARGETSR66 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 67
pub mod GICD_ITARGETSR67 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 68
pub mod GICD_ITARGETSR68 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 69
pub mod GICD_ITARGETSR69 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 70
pub mod GICD_ITARGETSR70 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt processor target register 71
pub mod GICD_ITARGETSR71 {
    pub use super::GICD_ITARGETSR8::CPU_TARGETS0;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS1;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS2;
    pub use super::GICD_ITARGETSR8::CPU_TARGETS3;
}

/// GICD interrupt configuration register
pub mod GICD_ICFGR0 {

    /// INT_CONFIG0
    pub mod INT_CONFIG0 {
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

    /// INT_CONFIG1
    pub mod INT_CONFIG1 {
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

    /// INT_CONFIG2
    pub mod INT_CONFIG2 {
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

    /// INT_CONFIG3
    pub mod INT_CONFIG3 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// INT_CONFIG4
    pub mod INT_CONFIG4 {
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

    /// INT_CONFIG5
    pub mod INT_CONFIG5 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// INT_CONFIG6
    pub mod INT_CONFIG6 {
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

    /// INT_CONFIG7
    pub mod INT_CONFIG7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// INT_CONFIG8
    pub mod INT_CONFIG8 {
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

    /// INT_CONFIG9
    pub mod INT_CONFIG9 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// INT_CONFIG10
    pub mod INT_CONFIG10 {
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

    /// INT_CONFIG11
    pub mod INT_CONFIG11 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// INT_CONFIG12
    pub mod INT_CONFIG12 {
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

    /// INT_CONFIG13
    pub mod INT_CONFIG13 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// INT_CONFIG14
    pub mod INT_CONFIG14 {
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

    /// INT_CONFIG15
    pub mod INT_CONFIG15 {
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

/// GICD interrupt configuration register
pub mod GICD_ICFGR1 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 2
pub mod GICD_ICFGR2 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 3
pub mod GICD_ICFGR3 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 4
pub mod GICD_ICFGR4 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 5
pub mod GICD_ICFGR5 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 6
pub mod GICD_ICFGR6 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 7
pub mod GICD_ICFGR7 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 8
pub mod GICD_ICFGR8 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 9
pub mod GICD_ICFGR9 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 10
pub mod GICD_ICFGR10 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 11
pub mod GICD_ICFGR11 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 12
pub mod GICD_ICFGR12 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 13
pub mod GICD_ICFGR13 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 14
pub mod GICD_ICFGR14 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 15
pub mod GICD_ICFGR15 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 16
pub mod GICD_ICFGR16 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD interrupt configuration register 17
pub mod GICD_ICFGR17 {
    pub use super::GICD_ICFGR0::INT_CONFIG0;
    pub use super::GICD_ICFGR0::INT_CONFIG1;
    pub use super::GICD_ICFGR0::INT_CONFIG10;
    pub use super::GICD_ICFGR0::INT_CONFIG11;
    pub use super::GICD_ICFGR0::INT_CONFIG12;
    pub use super::GICD_ICFGR0::INT_CONFIG13;
    pub use super::GICD_ICFGR0::INT_CONFIG14;
    pub use super::GICD_ICFGR0::INT_CONFIG15;
    pub use super::GICD_ICFGR0::INT_CONFIG2;
    pub use super::GICD_ICFGR0::INT_CONFIG3;
    pub use super::GICD_ICFGR0::INT_CONFIG4;
    pub use super::GICD_ICFGR0::INT_CONFIG5;
    pub use super::GICD_ICFGR0::INT_CONFIG6;
    pub use super::GICD_ICFGR0::INT_CONFIG7;
    pub use super::GICD_ICFGR0::INT_CONFIG8;
    pub use super::GICD_ICFGR0::INT_CONFIG9;
}

/// GICD private peripheral interrupt status register
pub mod GICD_PPISR {

    /// PPI6
    pub mod PPI6 {
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

    /// PPI5
    pub mod PPI5 {
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

    /// PPI4
    pub mod PPI4 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PPI0
    pub mod PPI0 {
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

    /// PPI1
    pub mod PPI1 {
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

    /// PPI2
    pub mod PPI2 {
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

    /// PPI3
    pub mod PPI3 {
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
}

/// For interrupts ID = SPI number+32, from SPI \[x*32+31\] to SPI \[x*32\]
pub mod GICD_SPISR1 {

    /// SPISR1
    pub mod SPISR1 {
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

/// For interrupts ID
pub mod GICD_SPISR2 {

    /// SPISR2
    pub mod SPISR2 {
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

/// For interrupts ID
pub mod GICD_SPISR3 {

    /// SPISR3
    pub mod SPISR3 {
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

/// For interrupts ID
pub mod GICD_SPISR4 {

    /// SPISR4
    pub mod SPISR4 {
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

/// For interrupts ID
pub mod GICD_SPISR5 {

    /// SPISR5
    pub mod SPISR5 {
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

/// For interrupts ID
pub mod GICD_SPISR6 {

    /// SPISR6
    pub mod SPISR6 {
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

/// For interrupts ID
pub mod GICD_SPISR7 {

    /// SPISR7
    pub mod SPISR7 {
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

/// GICD software generated interrupt register
pub mod GICD_SGIR {

    /// SGIINTID
    pub mod SGIINTID {
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

    /// NSATT
    pub mod NSATT {
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

    /// CPUTARGETLIST
    pub mod CPUTARGETLIST {
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

    /// TARGETLISTFILTER
    pub mod TARGETLISTFILTER {
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

/// For SGI x*4 to SGI x*4+3
pub mod GICD_CPENDSGIR0 {

    /// SGI_CLEAR_PENDING0
    pub mod SGI_CLEAR_PENDING0 {
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

    /// SGI_CLEAR_PENDING1
    pub mod SGI_CLEAR_PENDING1 {
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

    /// SGI_CLEAR_PENDING2
    pub mod SGI_CLEAR_PENDING2 {
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

    /// SGI_CLEAR_PENDING3
    pub mod SGI_CLEAR_PENDING3 {
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

/// For SGI x*4 to SGI x*4+3
pub mod GICD_CPENDSGIR1 {
    pub use super::GICD_CPENDSGIR0::SGI_CLEAR_PENDING0;
    pub use super::GICD_CPENDSGIR0::SGI_CLEAR_PENDING1;
    pub use super::GICD_CPENDSGIR0::SGI_CLEAR_PENDING2;
    pub use super::GICD_CPENDSGIR0::SGI_CLEAR_PENDING3;
}

/// For SGI x*4 to SGI x*4+3
pub mod GICD_CPENDSGIR2 {
    pub use super::GICD_CPENDSGIR0::SGI_CLEAR_PENDING0;
    pub use super::GICD_CPENDSGIR0::SGI_CLEAR_PENDING1;
    pub use super::GICD_CPENDSGIR0::SGI_CLEAR_PENDING2;
    pub use super::GICD_CPENDSGIR0::SGI_CLEAR_PENDING3;
}

/// For SGI x*4 to SGI x*4+3
pub mod GICD_CPENDSGIR3 {
    pub use super::GICD_CPENDSGIR0::SGI_CLEAR_PENDING0;
    pub use super::GICD_CPENDSGIR0::SGI_CLEAR_PENDING1;
    pub use super::GICD_CPENDSGIR0::SGI_CLEAR_PENDING2;
    pub use super::GICD_CPENDSGIR0::SGI_CLEAR_PENDING3;
}

/// For SGI x*4 to SGI x*4+3
pub mod GICD_SPENDSGIR0 {

    /// SGI_SET_PENDING0
    pub mod SGI_SET_PENDING0 {
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

    /// SGI_SET_PENDING1
    pub mod SGI_SET_PENDING1 {
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

    /// SGI_SET_PENDING2
    pub mod SGI_SET_PENDING2 {
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

    /// SGI_SET_PENDING3
    pub mod SGI_SET_PENDING3 {
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

/// For SGI x*4 to SGI x*4+3
pub mod GICD_SPENDSGIR1 {
    pub use super::GICD_SPENDSGIR0::SGI_SET_PENDING0;
    pub use super::GICD_SPENDSGIR0::SGI_SET_PENDING1;
    pub use super::GICD_SPENDSGIR0::SGI_SET_PENDING2;
    pub use super::GICD_SPENDSGIR0::SGI_SET_PENDING3;
}

/// For SGI x*4 to SGI x*4+3
pub mod GICD_SPENDSGIR2 {
    pub use super::GICD_SPENDSGIR0::SGI_SET_PENDING0;
    pub use super::GICD_SPENDSGIR0::SGI_SET_PENDING1;
    pub use super::GICD_SPENDSGIR0::SGI_SET_PENDING2;
    pub use super::GICD_SPENDSGIR0::SGI_SET_PENDING3;
}

/// For SGI x*4 to SGI x*4+3
pub mod GICD_SPENDSGIR3 {
    pub use super::GICD_SPENDSGIR0::SGI_SET_PENDING0;
    pub use super::GICD_SPENDSGIR0::SGI_SET_PENDING1;
    pub use super::GICD_SPENDSGIR0::SGI_SET_PENDING2;
    pub use super::GICD_SPENDSGIR0::SGI_SET_PENDING3;
}

/// GICD peripheral ID4 register
pub mod GICD_PIDR4 {

    /// PIDR4
    pub mod PIDR4 {
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

/// GICD peripheral ID5 to ID7 register 5
pub mod GICD_PIDR5 {

    /// PIDR5
    pub mod PIDR5 {
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

/// GICD peripheral ID5 to ID7 register 6
pub mod GICD_PIDR6 {

    /// PIDR6
    pub mod PIDR6 {
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

/// GICD peripheral ID5 to ID7 register 7
pub mod GICD_PIDR7 {

    /// PIDR7
    pub mod PIDR7 {
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

/// GICD peripheral ID0 register
pub mod GICD_PIDR0 {

    /// PIDR0
    pub mod PIDR0 {
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

/// GICD peripheral ID1 register
pub mod GICD_PIDR1 {

    /// PIDR1
    pub mod PIDR1 {
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

/// GICD peripheral ID2 register
pub mod GICD_PIDR2 {

    /// PIDR2
    pub mod PIDR2 {
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

/// GICD peripheral ID3 register
pub mod GICD_PIDR3 {

    /// PIDR3
    pub mod PIDR3 {
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

/// GICD component ID0 register
pub mod GICD_CIDR0 {

    /// CIDR0
    pub mod CIDR0 {
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

/// GICD component ID1 register
pub mod GICD_CIDR1 {

    /// CIDR1
    pub mod CIDR1 {
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

/// GICD component ID2 register
pub mod GICD_CIDR2 {

    /// CIDR2
    pub mod CIDR2 {
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

/// GICD component ID3 register
pub mod GICD_CIDR3 {

    /// CIDR3
    pub mod CIDR3 {
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
    /// GICD control register
    pub GICD_CTLR: RWRegister<u32>,

    /// GICD interrupt controller type register
    pub GICD_TYPER: RORegister<u32>,

    /// GICD implementer identification register
    pub GICD_IIDR: RORegister<u32>,

    _reserved1: [u8; 116],

    /// For interrupts ID
    pub GICD_IGROUPR0: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_IGROUPR1: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_IGROUPR2: RWRegister<u32>,

    /// For interrupts ID = x*32 to ID = x*32+31
    pub GICD_IGROUPR3: RWRegister<u32>,

    /// For interrupts ID = x*32 to ID = x*32+31
    pub GICD_IGROUPR4: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_IGROUPR5: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_IGROUPR6: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_IGROUPR7: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_IGROUPR8: RWRegister<u32>,

    _reserved2: [u8; 92],

    /// For interrupts ID = 0 to ID = 31
    pub GICD_ISENABLER0: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISENABLER1: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISENABLER2: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISENABLER3: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISENABLER4: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISENABLER5: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISENABLER6: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISENABLER7: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISENABLER8: RWRegister<u32>,

    _reserved3: [u8; 92],

    /// For interrupts ID = 0 to ID = 31
    pub GICD_ICENABLER0: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICENABLER1: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICENABLER2: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICENABLER3: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICENABLER4: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICENABLER5: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICENABLER6: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICENABLER7: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICENABLER8: RWRegister<u32>,

    _reserved4: [u8; 92],

    /// For interrupts ID
    pub GICD_ISPENDR0: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISPENDR1: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISPENDR2: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISPENDR3: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISPENDR4: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISPENDR5: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISPENDR6: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISPENDR7: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISPENDR8: RWRegister<u32>,

    _reserved5: [u8; 92],

    /// For interrupts ID
    pub GICD_ICPENDR0: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICPENDR1: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICPENDR2: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICPENDR3: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICPENDR4: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICPENDR5: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICPENDR6: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICPENDR7: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICPENDR8: RWRegister<u32>,

    _reserved6: [u8; 92],

    /// For interrupts ID
    pub GICD_ISACTIVER0: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISACTIVER1: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISACTIVER2: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISACTIVER3: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISACTIVER4: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISACTIVER5: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISACTIVER6: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISACTIVER7: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ISACTIVER8: RWRegister<u32>,

    _reserved7: [u8; 92],

    /// For interrupts ID
    pub GICD_ICACTIVER0: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICACTIVER1: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICACTIVER2: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICACTIVER3: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICACTIVER4: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICACTIVER5: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICACTIVER6: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICACTIVER7: RWRegister<u32>,

    /// For interrupts ID
    pub GICD_ICACTIVER8: RWRegister<u32>,

    _reserved8: [u8; 92],

    /// GICD interrupt priority register 0
    pub GICD_IPRIORITYR0: RWRegister<u32>,

    /// GICD interrupt priority register 1
    pub GICD_IPRIORITYR1: RWRegister<u32>,

    /// GICD interrupt priority register 2
    pub GICD_IPRIORITYR2: RWRegister<u32>,

    /// GICD interrupt priority register 3
    pub GICD_IPRIORITYR3: RWRegister<u32>,

    /// GICD interrupt priority register 4
    pub GICD_IPRIORITYR4: RWRegister<u32>,

    /// GICD interrupt priority register 5
    pub GICD_IPRIORITYR5: RWRegister<u32>,

    /// GICD interrupt priority register 6
    pub GICD_IPRIORITYR6: RWRegister<u32>,

    /// GICD interrupt priority register 7
    pub GICD_IPRIORITYR7: RWRegister<u32>,

    /// GICD interrupt priority register 8
    pub GICD_IPRIORITYR8: RWRegister<u32>,

    /// GICD interrupt priority register 9
    pub GICD_IPRIORITYR9: RWRegister<u32>,

    /// GICD interrupt priority register 10
    pub GICD_IPRIORITYR10: RWRegister<u32>,

    /// GICD interrupt priority register 11
    pub GICD_IPRIORITYR11: RWRegister<u32>,

    /// GICD interrupt priority register 12
    pub GICD_IPRIORITYR12: RWRegister<u32>,

    /// GICD interrupt priority register 13
    pub GICD_IPRIORITYR13: RWRegister<u32>,

    /// GICD interrupt priority register 14
    pub GICD_IPRIORITYR14: RWRegister<u32>,

    /// GICD interrupt priority register 15
    pub GICD_IPRIORITYR15: RWRegister<u32>,

    /// GICD interrupt priority register 16
    pub GICD_IPRIORITYR16: RWRegister<u32>,

    /// GICD interrupt priority register 17
    pub GICD_IPRIORITYR17: RWRegister<u32>,

    /// GICD interrupt priority register 18
    pub GICD_IPRIORITYR18: RWRegister<u32>,

    /// GICD interrupt priority register 19
    pub GICD_IPRIORITYR19: RWRegister<u32>,

    /// GICD interrupt priority register 20
    pub GICD_IPRIORITYR20: RWRegister<u32>,

    /// GICD interrupt priority register 21
    pub GICD_IPRIORITYR21: RWRegister<u32>,

    /// GICD interrupt priority register 22
    pub GICD_IPRIORITYR22: RWRegister<u32>,

    /// GICD interrupt priority register 23
    pub GICD_IPRIORITYR23: RWRegister<u32>,

    /// GICD interrupt priority register 24
    pub GICD_IPRIORITYR24: RWRegister<u32>,

    /// GICD interrupt priority register 25
    pub GICD_IPRIORITYR25: RWRegister<u32>,

    /// GICD interrupt priority register 26
    pub GICD_IPRIORITYR26: RWRegister<u32>,

    /// GICD interrupt priority register 27
    pub GICD_IPRIORITYR27: RWRegister<u32>,

    /// GICD interrupt priority register 28
    pub GICD_IPRIORITYR28: RWRegister<u32>,

    /// GICD interrupt priority register 29
    pub GICD_IPRIORITYR29: RWRegister<u32>,

    /// GICD interrupt priority register 30
    pub GICD_IPRIORITYR30: RWRegister<u32>,

    /// GICD interrupt priority register 31
    pub GICD_IPRIORITYR31: RWRegister<u32>,

    /// GICD interrupt priority register 32
    pub GICD_IPRIORITYR32: RWRegister<u32>,

    /// GICD interrupt priority register 33
    pub GICD_IPRIORITYR33: RWRegister<u32>,

    /// GICD interrupt priority register 34
    pub GICD_IPRIORITYR34: RWRegister<u32>,

    /// GICD interrupt priority register 35
    pub GICD_IPRIORITYR35: RWRegister<u32>,

    /// GICD interrupt priority register 36
    pub GICD_IPRIORITYR36: RWRegister<u32>,

    /// GICD interrupt priority register 37
    pub GICD_IPRIORITYR37: RWRegister<u32>,

    /// GICD interrupt priority register 38
    pub GICD_IPRIORITYR38: RWRegister<u32>,

    /// GICD interrupt priority register 39
    pub GICD_IPRIORITYR39: RWRegister<u32>,

    /// GICD interrupt priority register 40
    pub GICD_IPRIORITYR40: RWRegister<u32>,

    /// GICD interrupt priority register 41
    pub GICD_IPRIORITYR41: RWRegister<u32>,

    /// GICD interrupt priority register 42
    pub GICD_IPRIORITYR42: RWRegister<u32>,

    /// GICD interrupt priority register 43
    pub GICD_IPRIORITYR43: RWRegister<u32>,

    /// GICD interrupt priority register 44
    pub GICD_IPRIORITYR44: RWRegister<u32>,

    /// GICD interrupt priority register 45
    pub GICD_IPRIORITYR45: RWRegister<u32>,

    /// GICD interrupt priority register 46
    pub GICD_IPRIORITYR46: RWRegister<u32>,

    /// GICD interrupt priority register 47
    pub GICD_IPRIORITYR47: RWRegister<u32>,

    /// GICD interrupt priority register 48
    pub GICD_IPRIORITYR48: RWRegister<u32>,

    /// GICD interrupt priority register 49
    pub GICD_IPRIORITYR49: RWRegister<u32>,

    /// GICD interrupt priority register 50
    pub GICD_IPRIORITYR50: RWRegister<u32>,

    /// GICD interrupt priority register 51
    pub GICD_IPRIORITYR51: RWRegister<u32>,

    /// GICD interrupt priority register 52
    pub GICD_IPRIORITYR52: RWRegister<u32>,

    /// GICD interrupt priority register 53
    pub GICD_IPRIORITYR53: RWRegister<u32>,

    /// GICD interrupt priority register 54
    pub GICD_IPRIORITYR54: RWRegister<u32>,

    /// GICD interrupt priority register 55
    pub GICD_IPRIORITYR55: RWRegister<u32>,

    /// GICD interrupt priority register 56
    pub GICD_IPRIORITYR56: RWRegister<u32>,

    /// GICD interrupt priority register 57
    pub GICD_IPRIORITYR57: RWRegister<u32>,

    /// GICD interrupt priority register 58
    pub GICD_IPRIORITYR58: RWRegister<u32>,

    /// GICD interrupt priority register 59
    pub GICD_IPRIORITYR59: RWRegister<u32>,

    /// GICD interrupt priority register 60
    pub GICD_IPRIORITYR60: RWRegister<u32>,

    /// GICD interrupt priority register 61
    pub GICD_IPRIORITYR61: RWRegister<u32>,

    /// GICD interrupt priority register 62
    pub GICD_IPRIORITYR62: RWRegister<u32>,

    /// GICD interrupt priority register 63
    pub GICD_IPRIORITYR63: RWRegister<u32>,

    /// GICD interrupt priority register 64
    pub GICD_IPRIORITYR64: RWRegister<u32>,

    /// GICD interrupt priority register 65
    pub GICD_IPRIORITYR65: RWRegister<u32>,

    /// GICD interrupt priority register 66
    pub GICD_IPRIORITYR66: RWRegister<u32>,

    /// GICD interrupt priority register 67
    pub GICD_IPRIORITYR67: RWRegister<u32>,

    /// GICD interrupt priority register 68
    pub GICD_IPRIORITYR68: RWRegister<u32>,

    /// GICD interrupt priority register 69
    pub GICD_IPRIORITYR69: RWRegister<u32>,

    /// GICD interrupt priority register 70
    pub GICD_IPRIORITYR70: RWRegister<u32>,

    /// GICD interrupt priority register 71
    pub GICD_IPRIORITYR71: RWRegister<u32>,

    _reserved9: [u8; 736],

    /// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    pub GICD_ITARGETSR0: RORegister<u32>,

    /// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    pub GICD_ITARGETSR1: RORegister<u32>,

    /// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    pub GICD_ITARGETSR2: RORegister<u32>,

    /// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    pub GICD_ITARGETSR3: RORegister<u32>,

    /// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    pub GICD_ITARGETSR4: RORegister<u32>,

    /// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    pub GICD_ITARGETSR5: RORegister<u32>,

    /// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    pub GICD_ITARGETSR6: RORegister<u32>,

    /// For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    pub GICD_ITARGETSR7: RORegister<u32>,

    /// GICD interrupt processor target register 8
    pub GICD_ITARGETSR8: RWRegister<u32>,

    /// GICD interrupt processor target register 9
    pub GICD_ITARGETSR9: RWRegister<u32>,

    /// GICD interrupt processor target register 10
    pub GICD_ITARGETSR10: RWRegister<u32>,

    /// GICD interrupt processor target register 11
    pub GICD_ITARGETSR11: RWRegister<u32>,

    /// GICD interrupt processor target register 12
    pub GICD_ITARGETSR12: RWRegister<u32>,

    /// GICD interrupt processor target register 13
    pub GICD_ITARGETSR13: RWRegister<u32>,

    /// GICD interrupt processor target register 14
    pub GICD_ITARGETSR14: RWRegister<u32>,

    /// GICD interrupt processor target register 15
    pub GICD_ITARGETSR15: RWRegister<u32>,

    /// GICD interrupt processor target register 16
    pub GICD_ITARGETSR16: RWRegister<u32>,

    /// GICD interrupt processor target register 17
    pub GICD_ITARGETSR17: RWRegister<u32>,

    /// GICD interrupt processor target register 18
    pub GICD_ITARGETSR18: RWRegister<u32>,

    /// GICD interrupt processor target register 19
    pub GICD_ITARGETSR19: RWRegister<u32>,

    /// GICD interrupt processor target register 20
    pub GICD_ITARGETSR20: RWRegister<u32>,

    /// GICD interrupt processor target register 21
    pub GICD_ITARGETSR21: RWRegister<u32>,

    /// GICD interrupt processor target register 22
    pub GICD_ITARGETSR22: RWRegister<u32>,

    /// GICD interrupt processor target register 23
    pub GICD_ITARGETSR23: RWRegister<u32>,

    /// GICD interrupt processor target register 24
    pub GICD_ITARGETSR24: RWRegister<u32>,

    /// GICD interrupt processor target register 25
    pub GICD_ITARGETSR25: RWRegister<u32>,

    /// GICD interrupt processor target register 26
    pub GICD_ITARGETSR26: RWRegister<u32>,

    /// GICD interrupt processor target register 27
    pub GICD_ITARGETSR27: RWRegister<u32>,

    /// GICD interrupt processor target register 28
    pub GICD_ITARGETSR28: RWRegister<u32>,

    /// GICD interrupt processor target register 29
    pub GICD_ITARGETSR29: RWRegister<u32>,

    /// GICD interrupt processor target register 30
    pub GICD_ITARGETSR30: RWRegister<u32>,

    /// GICD interrupt processor target register 31
    pub GICD_ITARGETSR31: RWRegister<u32>,

    /// GICD interrupt processor target register 32
    pub GICD_ITARGETSR32: RWRegister<u32>,

    /// GICD interrupt processor target register 33
    pub GICD_ITARGETSR33: RWRegister<u32>,

    /// GICD interrupt processor target register 34
    pub GICD_ITARGETSR34: RWRegister<u32>,

    /// GICD interrupt processor target register 35
    pub GICD_ITARGETSR35: RWRegister<u32>,

    /// GICD interrupt processor target register 36
    pub GICD_ITARGETSR36: RWRegister<u32>,

    /// GICD interrupt processor target register 37
    pub GICD_ITARGETSR37: RWRegister<u32>,

    /// GICD interrupt processor target register 38
    pub GICD_ITARGETSR38: RWRegister<u32>,

    /// GICD interrupt processor target register 39
    pub GICD_ITARGETSR39: RWRegister<u32>,

    /// GICD interrupt processor target register 40
    pub GICD_ITARGETSR40: RWRegister<u32>,

    /// GICD interrupt processor target register 41
    pub GICD_ITARGETSR41: RWRegister<u32>,

    /// GICD interrupt processor target register 42
    pub GICD_ITARGETSR42: RWRegister<u32>,

    /// GICD interrupt processor target register 43
    pub GICD_ITARGETSR43: RWRegister<u32>,

    /// GICD interrupt processor target register 44
    pub GICD_ITARGETSR44: RWRegister<u32>,

    /// GICD interrupt processor target register 45
    pub GICD_ITARGETSR45: RWRegister<u32>,

    /// GICD interrupt processor target register 46
    pub GICD_ITARGETSR46: RWRegister<u32>,

    /// GICD interrupt processor target register 47
    pub GICD_ITARGETSR47: RWRegister<u32>,

    /// GICD interrupt processor target register 48
    pub GICD_ITARGETSR48: RWRegister<u32>,

    /// GICD interrupt processor target register 49
    pub GICD_ITARGETSR49: RWRegister<u32>,

    /// GICD interrupt processor target register 50
    pub GICD_ITARGETSR50: RWRegister<u32>,

    /// GICD interrupt processor target register 51
    pub GICD_ITARGETSR51: RWRegister<u32>,

    /// GICD interrupt processor target register 52
    pub GICD_ITARGETSR52: RWRegister<u32>,

    /// GICD interrupt processor target register 53
    pub GICD_ITARGETSR53: RWRegister<u32>,

    /// GICD interrupt processor target register 54
    pub GICD_ITARGETSR54: RWRegister<u32>,

    /// GICD interrupt processor target register 55
    pub GICD_ITARGETSR55: RWRegister<u32>,

    /// GICD interrupt processor target register 56
    pub GICD_ITARGETSR56: RWRegister<u32>,

    /// GICD interrupt processor target register 57
    pub GICD_ITARGETSR57: RWRegister<u32>,

    /// GICD interrupt processor target register 58
    pub GICD_ITARGETSR58: RWRegister<u32>,

    /// GICD interrupt processor target register 59
    pub GICD_ITARGETSR59: RWRegister<u32>,

    /// GICD interrupt processor target register 60
    pub GICD_ITARGETSR60: RWRegister<u32>,

    /// GICD interrupt processor target register 61
    pub GICD_ITARGETSR61: RWRegister<u32>,

    /// GICD interrupt processor target register 62
    pub GICD_ITARGETSR62: RWRegister<u32>,

    /// GICD interrupt processor target register 63
    pub GICD_ITARGETSR63: RWRegister<u32>,

    /// GICD interrupt processor target register 64
    pub GICD_ITARGETSR64: RWRegister<u32>,

    /// GICD interrupt processor target register 65
    pub GICD_ITARGETSR65: RWRegister<u32>,

    /// GICD interrupt processor target register 66
    pub GICD_ITARGETSR66: RWRegister<u32>,

    /// GICD interrupt processor target register 67
    pub GICD_ITARGETSR67: RWRegister<u32>,

    /// GICD interrupt processor target register 68
    pub GICD_ITARGETSR68: RWRegister<u32>,

    /// GICD interrupt processor target register 69
    pub GICD_ITARGETSR69: RWRegister<u32>,

    /// GICD interrupt processor target register 70
    pub GICD_ITARGETSR70: RWRegister<u32>,

    /// GICD interrupt processor target register 71
    pub GICD_ITARGETSR71: RWRegister<u32>,

    _reserved10: [u8; 736],

    /// GICD interrupt configuration register
    pub GICD_ICFGR0: RWRegister<u32>,

    /// GICD interrupt configuration register
    pub GICD_ICFGR1: RWRegister<u32>,

    /// GICD interrupt configuration register 2
    pub GICD_ICFGR2: RWRegister<u32>,

    /// GICD interrupt configuration register 3
    pub GICD_ICFGR3: RWRegister<u32>,

    /// GICD interrupt configuration register 4
    pub GICD_ICFGR4: RWRegister<u32>,

    /// GICD interrupt configuration register 5
    pub GICD_ICFGR5: RWRegister<u32>,

    /// GICD interrupt configuration register 6
    pub GICD_ICFGR6: RWRegister<u32>,

    /// GICD interrupt configuration register 7
    pub GICD_ICFGR7: RWRegister<u32>,

    /// GICD interrupt configuration register 8
    pub GICD_ICFGR8: RWRegister<u32>,

    /// GICD interrupt configuration register 9
    pub GICD_ICFGR9: RWRegister<u32>,

    /// GICD interrupt configuration register 10
    pub GICD_ICFGR10: RWRegister<u32>,

    /// GICD interrupt configuration register 11
    pub GICD_ICFGR11: RWRegister<u32>,

    /// GICD interrupt configuration register 12
    pub GICD_ICFGR12: RWRegister<u32>,

    /// GICD interrupt configuration register 13
    pub GICD_ICFGR13: RWRegister<u32>,

    /// GICD interrupt configuration register 14
    pub GICD_ICFGR14: RWRegister<u32>,

    /// GICD interrupt configuration register 15
    pub GICD_ICFGR15: RWRegister<u32>,

    /// GICD interrupt configuration register 16
    pub GICD_ICFGR16: RWRegister<u32>,

    /// GICD interrupt configuration register 17
    pub GICD_ICFGR17: RWRegister<u32>,

    _reserved11: [u8; 184],

    /// GICD private peripheral interrupt status register
    pub GICD_PPISR: RORegister<u32>,

    _reserved12: [u8; 4],

    /// For interrupts ID = SPI number+32, from SPI \[x*32+31\] to SPI \[x*32\]
    pub GICD_SPISR1: RORegister<u32>,

    /// For interrupts ID
    pub GICD_SPISR2: RORegister<u32>,

    /// For interrupts ID
    pub GICD_SPISR3: RORegister<u32>,

    /// For interrupts ID
    pub GICD_SPISR4: RORegister<u32>,

    /// For interrupts ID
    pub GICD_SPISR5: RORegister<u32>,

    /// For interrupts ID
    pub GICD_SPISR6: RORegister<u32>,

    /// For interrupts ID
    pub GICD_SPISR7: RORegister<u32>,

    _reserved13: [u8; 476],

    /// GICD software generated interrupt register
    pub GICD_SGIR: WORegister<u32>,

    _reserved14: [u8; 12],

    /// For SGI x*4 to SGI x*4+3
    pub GICD_CPENDSGIR0: RWRegister<u32>,

    /// For SGI x*4 to SGI x*4+3
    pub GICD_CPENDSGIR1: RWRegister<u32>,

    /// For SGI x*4 to SGI x*4+3
    pub GICD_CPENDSGIR2: RWRegister<u32>,

    /// For SGI x*4 to SGI x*4+3
    pub GICD_CPENDSGIR3: RWRegister<u32>,

    /// For SGI x*4 to SGI x*4+3
    pub GICD_SPENDSGIR0: RWRegister<u32>,

    /// For SGI x*4 to SGI x*4+3
    pub GICD_SPENDSGIR1: RWRegister<u32>,

    /// For SGI x*4 to SGI x*4+3
    pub GICD_SPENDSGIR2: RWRegister<u32>,

    /// For SGI x*4 to SGI x*4+3
    pub GICD_SPENDSGIR3: RWRegister<u32>,

    _reserved15: [u8; 160],

    /// GICD peripheral ID4 register
    pub GICD_PIDR4: RORegister<u32>,

    /// GICD peripheral ID5 to ID7 register 5
    pub GICD_PIDR5: RORegister<u32>,

    /// GICD peripheral ID5 to ID7 register 6
    pub GICD_PIDR6: RORegister<u32>,

    /// GICD peripheral ID5 to ID7 register 7
    pub GICD_PIDR7: RORegister<u32>,

    /// GICD peripheral ID0 register
    pub GICD_PIDR0: RORegister<u32>,

    /// GICD peripheral ID1 register
    pub GICD_PIDR1: RORegister<u32>,

    /// GICD peripheral ID2 register
    pub GICD_PIDR2: RORegister<u32>,

    /// GICD peripheral ID3 register
    pub GICD_PIDR3: RORegister<u32>,

    /// GICD component ID0 register
    pub GICD_CIDR0: RORegister<u32>,

    /// GICD component ID1 register
    pub GICD_CIDR1: RORegister<u32>,

    /// GICD component ID2 register
    pub GICD_CIDR2: RORegister<u32>,

    /// GICD component ID3 register
    pub GICD_CIDR3: RORegister<u32>,
}
pub struct ResetValues {
    pub GICD_CTLR: u32,
    pub GICD_TYPER: u32,
    pub GICD_IIDR: u32,
    pub GICD_IGROUPR0: u32,
    pub GICD_IGROUPR1: u32,
    pub GICD_IGROUPR2: u32,
    pub GICD_IGROUPR3: u32,
    pub GICD_IGROUPR4: u32,
    pub GICD_IGROUPR5: u32,
    pub GICD_IGROUPR6: u32,
    pub GICD_IGROUPR7: u32,
    pub GICD_IGROUPR8: u32,
    pub GICD_ISENABLER0: u32,
    pub GICD_ISENABLER1: u32,
    pub GICD_ISENABLER2: u32,
    pub GICD_ISENABLER3: u32,
    pub GICD_ISENABLER4: u32,
    pub GICD_ISENABLER5: u32,
    pub GICD_ISENABLER6: u32,
    pub GICD_ISENABLER7: u32,
    pub GICD_ISENABLER8: u32,
    pub GICD_ICENABLER0: u32,
    pub GICD_ICENABLER1: u32,
    pub GICD_ICENABLER2: u32,
    pub GICD_ICENABLER3: u32,
    pub GICD_ICENABLER4: u32,
    pub GICD_ICENABLER5: u32,
    pub GICD_ICENABLER6: u32,
    pub GICD_ICENABLER7: u32,
    pub GICD_ICENABLER8: u32,
    pub GICD_ISPENDR0: u32,
    pub GICD_ISPENDR1: u32,
    pub GICD_ISPENDR2: u32,
    pub GICD_ISPENDR3: u32,
    pub GICD_ISPENDR4: u32,
    pub GICD_ISPENDR5: u32,
    pub GICD_ISPENDR6: u32,
    pub GICD_ISPENDR7: u32,
    pub GICD_ISPENDR8: u32,
    pub GICD_ICPENDR0: u32,
    pub GICD_ICPENDR1: u32,
    pub GICD_ICPENDR2: u32,
    pub GICD_ICPENDR3: u32,
    pub GICD_ICPENDR4: u32,
    pub GICD_ICPENDR5: u32,
    pub GICD_ICPENDR6: u32,
    pub GICD_ICPENDR7: u32,
    pub GICD_ICPENDR8: u32,
    pub GICD_ISACTIVER0: u32,
    pub GICD_ISACTIVER1: u32,
    pub GICD_ISACTIVER2: u32,
    pub GICD_ISACTIVER3: u32,
    pub GICD_ISACTIVER4: u32,
    pub GICD_ISACTIVER5: u32,
    pub GICD_ISACTIVER6: u32,
    pub GICD_ISACTIVER7: u32,
    pub GICD_ISACTIVER8: u32,
    pub GICD_ICACTIVER0: u32,
    pub GICD_ICACTIVER1: u32,
    pub GICD_ICACTIVER2: u32,
    pub GICD_ICACTIVER3: u32,
    pub GICD_ICACTIVER4: u32,
    pub GICD_ICACTIVER5: u32,
    pub GICD_ICACTIVER6: u32,
    pub GICD_ICACTIVER7: u32,
    pub GICD_ICACTIVER8: u32,
    pub GICD_IPRIORITYR0: u32,
    pub GICD_IPRIORITYR1: u32,
    pub GICD_IPRIORITYR2: u32,
    pub GICD_IPRIORITYR3: u32,
    pub GICD_IPRIORITYR4: u32,
    pub GICD_IPRIORITYR5: u32,
    pub GICD_IPRIORITYR6: u32,
    pub GICD_IPRIORITYR7: u32,
    pub GICD_IPRIORITYR8: u32,
    pub GICD_IPRIORITYR9: u32,
    pub GICD_IPRIORITYR10: u32,
    pub GICD_IPRIORITYR11: u32,
    pub GICD_IPRIORITYR12: u32,
    pub GICD_IPRIORITYR13: u32,
    pub GICD_IPRIORITYR14: u32,
    pub GICD_IPRIORITYR15: u32,
    pub GICD_IPRIORITYR16: u32,
    pub GICD_IPRIORITYR17: u32,
    pub GICD_IPRIORITYR18: u32,
    pub GICD_IPRIORITYR19: u32,
    pub GICD_IPRIORITYR20: u32,
    pub GICD_IPRIORITYR21: u32,
    pub GICD_IPRIORITYR22: u32,
    pub GICD_IPRIORITYR23: u32,
    pub GICD_IPRIORITYR24: u32,
    pub GICD_IPRIORITYR25: u32,
    pub GICD_IPRIORITYR26: u32,
    pub GICD_IPRIORITYR27: u32,
    pub GICD_IPRIORITYR28: u32,
    pub GICD_IPRIORITYR29: u32,
    pub GICD_IPRIORITYR30: u32,
    pub GICD_IPRIORITYR31: u32,
    pub GICD_IPRIORITYR32: u32,
    pub GICD_IPRIORITYR33: u32,
    pub GICD_IPRIORITYR34: u32,
    pub GICD_IPRIORITYR35: u32,
    pub GICD_IPRIORITYR36: u32,
    pub GICD_IPRIORITYR37: u32,
    pub GICD_IPRIORITYR38: u32,
    pub GICD_IPRIORITYR39: u32,
    pub GICD_IPRIORITYR40: u32,
    pub GICD_IPRIORITYR41: u32,
    pub GICD_IPRIORITYR42: u32,
    pub GICD_IPRIORITYR43: u32,
    pub GICD_IPRIORITYR44: u32,
    pub GICD_IPRIORITYR45: u32,
    pub GICD_IPRIORITYR46: u32,
    pub GICD_IPRIORITYR47: u32,
    pub GICD_IPRIORITYR48: u32,
    pub GICD_IPRIORITYR49: u32,
    pub GICD_IPRIORITYR50: u32,
    pub GICD_IPRIORITYR51: u32,
    pub GICD_IPRIORITYR52: u32,
    pub GICD_IPRIORITYR53: u32,
    pub GICD_IPRIORITYR54: u32,
    pub GICD_IPRIORITYR55: u32,
    pub GICD_IPRIORITYR56: u32,
    pub GICD_IPRIORITYR57: u32,
    pub GICD_IPRIORITYR58: u32,
    pub GICD_IPRIORITYR59: u32,
    pub GICD_IPRIORITYR60: u32,
    pub GICD_IPRIORITYR61: u32,
    pub GICD_IPRIORITYR62: u32,
    pub GICD_IPRIORITYR63: u32,
    pub GICD_IPRIORITYR64: u32,
    pub GICD_IPRIORITYR65: u32,
    pub GICD_IPRIORITYR66: u32,
    pub GICD_IPRIORITYR67: u32,
    pub GICD_IPRIORITYR68: u32,
    pub GICD_IPRIORITYR69: u32,
    pub GICD_IPRIORITYR70: u32,
    pub GICD_IPRIORITYR71: u32,
    pub GICD_ITARGETSR0: u32,
    pub GICD_ITARGETSR1: u32,
    pub GICD_ITARGETSR2: u32,
    pub GICD_ITARGETSR3: u32,
    pub GICD_ITARGETSR4: u32,
    pub GICD_ITARGETSR5: u32,
    pub GICD_ITARGETSR6: u32,
    pub GICD_ITARGETSR7: u32,
    pub GICD_ITARGETSR8: u32,
    pub GICD_ITARGETSR9: u32,
    pub GICD_ITARGETSR10: u32,
    pub GICD_ITARGETSR11: u32,
    pub GICD_ITARGETSR12: u32,
    pub GICD_ITARGETSR13: u32,
    pub GICD_ITARGETSR14: u32,
    pub GICD_ITARGETSR15: u32,
    pub GICD_ITARGETSR16: u32,
    pub GICD_ITARGETSR17: u32,
    pub GICD_ITARGETSR18: u32,
    pub GICD_ITARGETSR19: u32,
    pub GICD_ITARGETSR20: u32,
    pub GICD_ITARGETSR21: u32,
    pub GICD_ITARGETSR22: u32,
    pub GICD_ITARGETSR23: u32,
    pub GICD_ITARGETSR24: u32,
    pub GICD_ITARGETSR25: u32,
    pub GICD_ITARGETSR26: u32,
    pub GICD_ITARGETSR27: u32,
    pub GICD_ITARGETSR28: u32,
    pub GICD_ITARGETSR29: u32,
    pub GICD_ITARGETSR30: u32,
    pub GICD_ITARGETSR31: u32,
    pub GICD_ITARGETSR32: u32,
    pub GICD_ITARGETSR33: u32,
    pub GICD_ITARGETSR34: u32,
    pub GICD_ITARGETSR35: u32,
    pub GICD_ITARGETSR36: u32,
    pub GICD_ITARGETSR37: u32,
    pub GICD_ITARGETSR38: u32,
    pub GICD_ITARGETSR39: u32,
    pub GICD_ITARGETSR40: u32,
    pub GICD_ITARGETSR41: u32,
    pub GICD_ITARGETSR42: u32,
    pub GICD_ITARGETSR43: u32,
    pub GICD_ITARGETSR44: u32,
    pub GICD_ITARGETSR45: u32,
    pub GICD_ITARGETSR46: u32,
    pub GICD_ITARGETSR47: u32,
    pub GICD_ITARGETSR48: u32,
    pub GICD_ITARGETSR49: u32,
    pub GICD_ITARGETSR50: u32,
    pub GICD_ITARGETSR51: u32,
    pub GICD_ITARGETSR52: u32,
    pub GICD_ITARGETSR53: u32,
    pub GICD_ITARGETSR54: u32,
    pub GICD_ITARGETSR55: u32,
    pub GICD_ITARGETSR56: u32,
    pub GICD_ITARGETSR57: u32,
    pub GICD_ITARGETSR58: u32,
    pub GICD_ITARGETSR59: u32,
    pub GICD_ITARGETSR60: u32,
    pub GICD_ITARGETSR61: u32,
    pub GICD_ITARGETSR62: u32,
    pub GICD_ITARGETSR63: u32,
    pub GICD_ITARGETSR64: u32,
    pub GICD_ITARGETSR65: u32,
    pub GICD_ITARGETSR66: u32,
    pub GICD_ITARGETSR67: u32,
    pub GICD_ITARGETSR68: u32,
    pub GICD_ITARGETSR69: u32,
    pub GICD_ITARGETSR70: u32,
    pub GICD_ITARGETSR71: u32,
    pub GICD_ICFGR0: u32,
    pub GICD_ICFGR1: u32,
    pub GICD_ICFGR2: u32,
    pub GICD_ICFGR3: u32,
    pub GICD_ICFGR4: u32,
    pub GICD_ICFGR5: u32,
    pub GICD_ICFGR6: u32,
    pub GICD_ICFGR7: u32,
    pub GICD_ICFGR8: u32,
    pub GICD_ICFGR9: u32,
    pub GICD_ICFGR10: u32,
    pub GICD_ICFGR11: u32,
    pub GICD_ICFGR12: u32,
    pub GICD_ICFGR13: u32,
    pub GICD_ICFGR14: u32,
    pub GICD_ICFGR15: u32,
    pub GICD_ICFGR16: u32,
    pub GICD_ICFGR17: u32,
    pub GICD_PPISR: u32,
    pub GICD_SPISR1: u32,
    pub GICD_SPISR2: u32,
    pub GICD_SPISR3: u32,
    pub GICD_SPISR4: u32,
    pub GICD_SPISR5: u32,
    pub GICD_SPISR6: u32,
    pub GICD_SPISR7: u32,
    pub GICD_SGIR: u32,
    pub GICD_CPENDSGIR0: u32,
    pub GICD_CPENDSGIR1: u32,
    pub GICD_CPENDSGIR2: u32,
    pub GICD_CPENDSGIR3: u32,
    pub GICD_SPENDSGIR0: u32,
    pub GICD_SPENDSGIR1: u32,
    pub GICD_SPENDSGIR2: u32,
    pub GICD_SPENDSGIR3: u32,
    pub GICD_PIDR4: u32,
    pub GICD_PIDR5: u32,
    pub GICD_PIDR6: u32,
    pub GICD_PIDR7: u32,
    pub GICD_PIDR0: u32,
    pub GICD_PIDR1: u32,
    pub GICD_PIDR2: u32,
    pub GICD_PIDR3: u32,
    pub GICD_CIDR0: u32,
    pub GICD_CIDR1: u32,
    pub GICD_CIDR2: u32,
    pub GICD_CIDR3: u32,
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
