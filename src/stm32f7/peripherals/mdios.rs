#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Management data input/output slave
//!
//! Used by: stm32f7x7, stm32f7x9

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister};

/// MDIOS configuration register
pub mod CR {

    /// Peripheral enable
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

    /// Register write interrupt enable
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

    /// Register Read Interrupt Enable
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

    /// Error interrupt enable
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

    /// Disable Preamble Check
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

    /// Slaves's address
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
pub mod WRFR {

    /// Write flags for MDIO registers 0 to 31
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
pub mod CWRFR {

    /// Clear the write flag
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
pub mod RDFR {

    /// Read flags for MDIO registers 0 to 31
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
pub mod CRDFR {

    /// Clear the read flag
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
pub mod SR {

    /// Preamble error flag
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

    /// Start error flag
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

    /// Turnaround error flag
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
pub mod CLRFR {

    /// Clear the preamble error flag
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

    /// Clear the start error flag
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

    /// Clear the turnaround error flag
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

/// MDIOS input data register 0
pub mod DINR0 {

    /// Input data received from MDIO Master during write frames
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

/// MDIOS input data register 0
pub mod DINR1 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR2 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR3 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR4 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR5 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR6 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR7 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR8 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR9 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR10 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR11 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR12 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR13 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR14 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR15 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR16 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR17 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR18 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR19 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR20 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR21 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR22 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR23 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR24 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR25 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR26 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR27 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR28 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR29 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR30 {
    pub use super::DINR0::DIN;
}

/// MDIOS input data register 0
pub mod DINR31 {
    pub use super::DINR0::DIN;
}

/// MDIOS output data register 0
pub mod DOUTR0 {

    /// Output data sent to MDIO Master during read frames
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

/// MDIOS output data register 0
pub mod DOUTR1 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR2 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR3 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR4 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR5 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR6 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR7 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR8 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR9 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR10 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR11 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR12 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR13 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR14 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR15 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR16 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR17 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR18 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR19 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR20 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR21 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR22 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR23 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR24 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR25 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR26 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR27 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR28 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR29 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR30 {
    pub use super::DOUTR0::DOUT;
}

/// MDIOS output data register 0
pub mod DOUTR31 {
    pub use super::DOUTR0::DOUT;
}
pub struct RegisterBlock {
    /// MDIOS configuration register
    pub CR: RWRegister<u32>,

    /// MDIOS write flag register
    pub WRFR: RORegister<u32>,

    /// MDIOS clear write flag register
    pub CWRFR: RWRegister<u32>,

    /// MDIOS read flag register
    pub RDFR: RORegister<u32>,

    /// MDIOS clear read flag register
    pub CRDFR: RWRegister<u32>,

    /// MDIOS status register
    pub SR: RORegister<u32>,

    /// MDIOS clear flag register
    pub CLRFR: RWRegister<u32>,

    /// MDIOS input data register 0
    pub DINR0: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR1: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR2: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR3: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR4: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR5: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR6: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR7: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR8: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR9: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR10: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR11: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR12: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR13: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR14: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR15: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR16: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR17: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR18: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR19: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR20: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR21: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR22: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR23: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR24: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR25: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR26: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR27: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR28: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR29: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR30: RORegister<u32>,

    /// MDIOS input data register 0
    pub DINR31: RORegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR0: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR1: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR2: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR3: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR4: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR5: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR6: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR7: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR8: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR9: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR10: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR11: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR12: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR13: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR14: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR15: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR16: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR17: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR18: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR19: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR20: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR21: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR22: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR23: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR24: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR25: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR26: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR27: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR28: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR29: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR30: RWRegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR31: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub WRFR: u32,
    pub CWRFR: u32,
    pub RDFR: u32,
    pub CRDFR: u32,
    pub SR: u32,
    pub CLRFR: u32,
    pub DINR0: u32,
    pub DINR1: u32,
    pub DINR2: u32,
    pub DINR3: u32,
    pub DINR4: u32,
    pub DINR5: u32,
    pub DINR6: u32,
    pub DINR7: u32,
    pub DINR8: u32,
    pub DINR9: u32,
    pub DINR10: u32,
    pub DINR11: u32,
    pub DINR12: u32,
    pub DINR13: u32,
    pub DINR14: u32,
    pub DINR15: u32,
    pub DINR16: u32,
    pub DINR17: u32,
    pub DINR18: u32,
    pub DINR19: u32,
    pub DINR20: u32,
    pub DINR21: u32,
    pub DINR22: u32,
    pub DINR23: u32,
    pub DINR24: u32,
    pub DINR25: u32,
    pub DINR26: u32,
    pub DINR27: u32,
    pub DINR28: u32,
    pub DINR29: u32,
    pub DINR30: u32,
    pub DINR31: u32,
    pub DOUTR0: u32,
    pub DOUTR1: u32,
    pub DOUTR2: u32,
    pub DOUTR3: u32,
    pub DOUTR4: u32,
    pub DOUTR5: u32,
    pub DOUTR6: u32,
    pub DOUTR7: u32,
    pub DOUTR8: u32,
    pub DOUTR9: u32,
    pub DOUTR10: u32,
    pub DOUTR11: u32,
    pub DOUTR12: u32,
    pub DOUTR13: u32,
    pub DOUTR14: u32,
    pub DOUTR15: u32,
    pub DOUTR16: u32,
    pub DOUTR17: u32,
    pub DOUTR18: u32,
    pub DOUTR19: u32,
    pub DOUTR20: u32,
    pub DOUTR21: u32,
    pub DOUTR22: u32,
    pub DOUTR23: u32,
    pub DOUTR24: u32,
    pub DOUTR25: u32,
    pub DOUTR26: u32,
    pub DOUTR27: u32,
    pub DOUTR28: u32,
    pub DOUTR29: u32,
    pub DOUTR30: u32,
    pub DOUTR31: u32,
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
