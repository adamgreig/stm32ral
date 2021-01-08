#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MDIOS

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// MDIOS configuration register
pub mod MDIOS_CR {

    /// EN
    pub mod EN {
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

    /// WRIE
    pub mod WRIE {
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

    /// RDIE
    pub mod RDIE {
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

    /// EIE
    pub mod EIE {
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

    /// DPC
    pub mod DPC {
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

    /// PORT_ADDRESS
    pub mod PORT_ADDRESS {
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

/// MDIOS write flag register
pub mod MDIOS_WRFR {

    /// WRF
    pub mod WRF {
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

/// MDIOS clear write flag register
pub mod MDIOS_CWRFR {

    /// CWRF
    pub mod CWRF {
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

/// MDIOS read flag register
pub mod MDIOS_RDFR {

    /// RDF
    pub mod RDF {
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

/// MDIOS clear read flag register
pub mod MDIOS_CRDFR {

    /// CRDF
    pub mod CRDF {
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

/// MDIOS status register
pub mod MDIOS_SR {

    /// PERF
    pub mod PERF {
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

    /// SERF
    pub mod SERF {
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

    /// TERF
    pub mod TERF {
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

/// MDIOS clear flag register
pub mod MDIOS_CLRFR {

    /// CPERF
    pub mod CPERF {
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

    /// CSERF
    pub mod CSERF {
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

    /// CTERF
    pub mod CTERF {
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

/// MDIOS input data register
pub mod MDIOS_DINR0 {

    /// DIN
    pub mod DIN {
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

/// MDIOS input data register
pub mod MDIOS_DINR1 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR2 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR3 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR4 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR5 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR6 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR7 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR8 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR9 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR10 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR11 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR12 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR13 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR14 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR15 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR16 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR17 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR18 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR19 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR20 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR21 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR22 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR23 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR24 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR25 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR26 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR27 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR28 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR29 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR30 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DINR31 {
    pub use super::MDIOS_DINR0::DIN;
}

/// MDIOS input data register
pub mod MDIOS_DOUTR0 {

    /// DOUT
    pub mod DOUT {
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

/// MDIOS input data register
pub mod MDIOS_DOUTR1 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR2 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR3 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR4 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR5 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR6 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR7 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR8 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR9 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR10 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR11 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR12 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR13 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR14 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR15 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR16 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR17 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR18 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR19 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR20 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR21 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR22 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR23 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR24 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR25 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR26 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR27 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR28 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR29 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR30 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS output data register
pub mod MDIOS_DOUTR31 {
    pub use super::MDIOS_DOUTR0::DOUT;
}

/// MDIOS HW configuration register
pub mod MDIOS_HWCFGR {

    /// NBREG
    pub mod NBREG {
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

/// MDIOS version register
pub mod MDIOS_VERR {

    /// MINREV
    pub mod MINREV {
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

    /// MAJREV
    pub mod MAJREV {
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

/// MDIOS identification register
pub mod MDIOS_IPIDR {

    /// ID
    pub mod ID {
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

/// MDIOS size identification register
pub mod MDIOS_SIDR {

    /// SID
    pub mod SID {
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
    /// MDIOS configuration register
    pub MDIOS_CR: RWRegister<u32>,

    /// MDIOS write flag register
    pub MDIOS_WRFR: RORegister<u32>,

    /// MDIOS clear write flag register
    pub MDIOS_CWRFR: RWRegister<u32>,

    /// MDIOS read flag register
    pub MDIOS_RDFR: RORegister<u32>,

    /// MDIOS clear read flag register
    pub MDIOS_CRDFR: RWRegister<u32>,

    /// MDIOS status register
    pub MDIOS_SR: RORegister<u32>,

    /// MDIOS clear flag register
    pub MDIOS_CLRFR: RWRegister<u32>,

    _reserved1: [u32; 57],

    /// MDIOS input data register
    pub MDIOS_DINR0: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR1: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR2: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR3: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR4: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR5: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR6: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR7: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR8: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR9: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR10: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR11: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR12: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR13: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR14: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR15: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR16: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR17: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR18: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR19: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR20: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR21: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR22: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR23: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR24: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR25: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR26: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR27: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR28: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR29: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR30: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DINR31: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DOUTR0: RORegister<u32>,

    /// MDIOS input data register
    pub MDIOS_DOUTR1: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR2: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR3: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR4: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR5: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR6: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR7: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR8: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR9: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR10: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR11: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR12: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR13: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR14: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR15: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR16: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR17: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR18: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR19: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR20: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR21: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR22: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR23: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR24: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR25: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR26: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR27: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR28: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR29: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR30: RORegister<u32>,

    /// MDIOS output data register
    pub MDIOS_DOUTR31: RORegister<u32>,

    _reserved2: [u32; 124],

    /// MDIOS HW configuration register
    pub MDIOS_HWCFGR: RORegister<u32>,

    /// MDIOS version register
    pub MDIOS_VERR: RORegister<u32>,

    /// MDIOS identification register
    pub MDIOS_IPIDR: RORegister<u32>,

    /// MDIOS size identification register
    pub MDIOS_SIDR: RORegister<u32>,
}
pub struct ResetValues {
    pub MDIOS_CR: u32,
    pub MDIOS_WRFR: u32,
    pub MDIOS_CWRFR: u32,
    pub MDIOS_RDFR: u32,
    pub MDIOS_CRDFR: u32,
    pub MDIOS_SR: u32,
    pub MDIOS_CLRFR: u32,
    pub MDIOS_DINR0: u32,
    pub MDIOS_DINR1: u32,
    pub MDIOS_DINR2: u32,
    pub MDIOS_DINR3: u32,
    pub MDIOS_DINR4: u32,
    pub MDIOS_DINR5: u32,
    pub MDIOS_DINR6: u32,
    pub MDIOS_DINR7: u32,
    pub MDIOS_DINR8: u32,
    pub MDIOS_DINR9: u32,
    pub MDIOS_DINR10: u32,
    pub MDIOS_DINR11: u32,
    pub MDIOS_DINR12: u32,
    pub MDIOS_DINR13: u32,
    pub MDIOS_DINR14: u32,
    pub MDIOS_DINR15: u32,
    pub MDIOS_DINR16: u32,
    pub MDIOS_DINR17: u32,
    pub MDIOS_DINR18: u32,
    pub MDIOS_DINR19: u32,
    pub MDIOS_DINR20: u32,
    pub MDIOS_DINR21: u32,
    pub MDIOS_DINR22: u32,
    pub MDIOS_DINR23: u32,
    pub MDIOS_DINR24: u32,
    pub MDIOS_DINR25: u32,
    pub MDIOS_DINR26: u32,
    pub MDIOS_DINR27: u32,
    pub MDIOS_DINR28: u32,
    pub MDIOS_DINR29: u32,
    pub MDIOS_DINR30: u32,
    pub MDIOS_DINR31: u32,
    pub MDIOS_DOUTR0: u32,
    pub MDIOS_DOUTR1: u32,
    pub MDIOS_DOUTR2: u32,
    pub MDIOS_DOUTR3: u32,
    pub MDIOS_DOUTR4: u32,
    pub MDIOS_DOUTR5: u32,
    pub MDIOS_DOUTR6: u32,
    pub MDIOS_DOUTR7: u32,
    pub MDIOS_DOUTR8: u32,
    pub MDIOS_DOUTR9: u32,
    pub MDIOS_DOUTR10: u32,
    pub MDIOS_DOUTR11: u32,
    pub MDIOS_DOUTR12: u32,
    pub MDIOS_DOUTR13: u32,
    pub MDIOS_DOUTR14: u32,
    pub MDIOS_DOUTR15: u32,
    pub MDIOS_DOUTR16: u32,
    pub MDIOS_DOUTR17: u32,
    pub MDIOS_DOUTR18: u32,
    pub MDIOS_DOUTR19: u32,
    pub MDIOS_DOUTR20: u32,
    pub MDIOS_DOUTR21: u32,
    pub MDIOS_DOUTR22: u32,
    pub MDIOS_DOUTR23: u32,
    pub MDIOS_DOUTR24: u32,
    pub MDIOS_DOUTR25: u32,
    pub MDIOS_DOUTR26: u32,
    pub MDIOS_DOUTR27: u32,
    pub MDIOS_DOUTR28: u32,
    pub MDIOS_DOUTR29: u32,
    pub MDIOS_DOUTR30: u32,
    pub MDIOS_DOUTR31: u32,
    pub MDIOS_HWCFGR: u32,
    pub MDIOS_VERR: u32,
    pub MDIOS_IPIDR: u32,
    pub MDIOS_SIDR: u32,
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

/// Access functions for the MDIOS peripheral instance
pub mod MDIOS {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4001c000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in MDIOS
    pub const reset: ResetValues = ResetValues {
        MDIOS_CR: 0x00000000,
        MDIOS_WRFR: 0x00000000,
        MDIOS_CWRFR: 0x00000000,
        MDIOS_RDFR: 0x00000000,
        MDIOS_CRDFR: 0x00000000,
        MDIOS_SR: 0x00000000,
        MDIOS_CLRFR: 0x00000000,
        MDIOS_DINR0: 0x00000000,
        MDIOS_DINR1: 0x00000000,
        MDIOS_DINR2: 0x00000000,
        MDIOS_DINR3: 0x00000000,
        MDIOS_DINR4: 0x00000000,
        MDIOS_DINR5: 0x00000000,
        MDIOS_DINR6: 0x00000000,
        MDIOS_DINR7: 0x00000000,
        MDIOS_DINR8: 0x00000000,
        MDIOS_DINR9: 0x00000000,
        MDIOS_DINR10: 0x00000000,
        MDIOS_DINR11: 0x00000000,
        MDIOS_DINR12: 0x00000000,
        MDIOS_DINR13: 0x00000000,
        MDIOS_DINR14: 0x00000000,
        MDIOS_DINR15: 0x00000000,
        MDIOS_DINR16: 0x00000000,
        MDIOS_DINR17: 0x00000000,
        MDIOS_DINR18: 0x00000000,
        MDIOS_DINR19: 0x00000000,
        MDIOS_DINR20: 0x00000000,
        MDIOS_DINR21: 0x00000000,
        MDIOS_DINR22: 0x00000000,
        MDIOS_DINR23: 0x00000000,
        MDIOS_DINR24: 0x00000000,
        MDIOS_DINR25: 0x00000000,
        MDIOS_DINR26: 0x00000000,
        MDIOS_DINR27: 0x00000000,
        MDIOS_DINR28: 0x00000000,
        MDIOS_DINR29: 0x00000000,
        MDIOS_DINR30: 0x00000000,
        MDIOS_DINR31: 0x00000000,
        MDIOS_DOUTR0: 0x00000000,
        MDIOS_DOUTR1: 0x00000000,
        MDIOS_DOUTR2: 0x00000000,
        MDIOS_DOUTR3: 0x00000000,
        MDIOS_DOUTR4: 0x00000000,
        MDIOS_DOUTR5: 0x00000000,
        MDIOS_DOUTR6: 0x00000000,
        MDIOS_DOUTR7: 0x00000000,
        MDIOS_DOUTR8: 0x00000000,
        MDIOS_DOUTR9: 0x00000000,
        MDIOS_DOUTR10: 0x00000000,
        MDIOS_DOUTR11: 0x00000000,
        MDIOS_DOUTR12: 0x00000000,
        MDIOS_DOUTR13: 0x00000000,
        MDIOS_DOUTR14: 0x00000000,
        MDIOS_DOUTR15: 0x00000000,
        MDIOS_DOUTR16: 0x00000000,
        MDIOS_DOUTR17: 0x00000000,
        MDIOS_DOUTR18: 0x00000000,
        MDIOS_DOUTR19: 0x00000000,
        MDIOS_DOUTR20: 0x00000000,
        MDIOS_DOUTR21: 0x00000000,
        MDIOS_DOUTR22: 0x00000000,
        MDIOS_DOUTR23: 0x00000000,
        MDIOS_DOUTR24: 0x00000000,
        MDIOS_DOUTR25: 0x00000000,
        MDIOS_DOUTR26: 0x00000000,
        MDIOS_DOUTR27: 0x00000000,
        MDIOS_DOUTR28: 0x00000000,
        MDIOS_DOUTR29: 0x00000000,
        MDIOS_DOUTR30: 0x00000000,
        MDIOS_DOUTR31: 0x00000000,
        MDIOS_HWCFGR: 0x00000020,
        MDIOS_VERR: 0x00000011,
        MDIOS_IPIDR: 0x00180001,
        MDIOS_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut MDIOS_TAKEN: bool = false;

    /// Safe access to MDIOS
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
            if MDIOS_TAKEN {
                None
            } else {
                MDIOS_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to MDIOS
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if MDIOS_TAKEN && inst.addr == INSTANCE.addr {
                MDIOS_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal MDIOS
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        MDIOS_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to MDIOS
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const MDIOS: *const RegisterBlock = 0x4001c000 as *const _;
