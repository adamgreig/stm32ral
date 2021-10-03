#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Management data input/output slave

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// MDIOS configuration register
pub mod MDIOS_CR {

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
pub mod MDIOS_WRFR {

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
pub mod MDIOS_CWRFR {

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
pub mod MDIOS_RDFR {

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
pub mod MDIOS_CRDFR {

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
pub mod MDIOS_SR {

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
pub mod MDIOS_CLRFR {

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
pub mod MDIOS_DINR0 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN0 {
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

/// MDIOS input data register 1
pub mod MDIOS_DINR1 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN1 {
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

/// MDIOS input data register 2
pub mod MDIOS_DINR2 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN2 {
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

/// MDIOS input data register 3
pub mod MDIOS_DINR3 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN3 {
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

/// MDIOS input data register 4
pub mod MDIOS_DINR4 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN4 {
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

/// MDIOS input data register 5
pub mod MDIOS_DINR5 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN5 {
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

/// MDIOS input data register 6
pub mod MDIOS_DINR6 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN6 {
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

/// MDIOS input data register 7
pub mod MDIOS_DINR7 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN7 {
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

/// MDIOS input data register 8
pub mod MDIOS_DINR8 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN8 {
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

/// MDIOS input data register 9
pub mod MDIOS_DINR9 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN9 {
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

/// MDIOS input data register 10
pub mod MDIOS_DINR10 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN10 {
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

/// MDIOS input data register 11
pub mod MDIOS_DINR11 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN11 {
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

/// MDIOS input data register 12
pub mod MDIOS_DINR12 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN12 {
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

/// MDIOS input data register 13
pub mod MDIOS_DINR13 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN13 {
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

/// MDIOS input data register 14
pub mod MDIOS_DINR14 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN14 {
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

/// MDIOS input data register 15
pub mod MDIOS_DINR15 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN15 {
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

/// MDIOS input data register 16
pub mod MDIOS_DINR16 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN16 {
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

/// MDIOS input data register 17
pub mod MDIOS_DINR17 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN17 {
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

/// MDIOS input data register 18
pub mod MDIOS_DINR18 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN18 {
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

/// MDIOS input data register 19
pub mod MDIOS_DINR19 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN19 {
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

/// MDIOS input data register 20
pub mod MDIOS_DINR20 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN20 {
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

/// MDIOS input data register 21
pub mod MDIOS_DINR21 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN21 {
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

/// MDIOS input data register 22
pub mod MDIOS_DINR22 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN22 {
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

/// MDIOS input data register 23
pub mod MDIOS_DINR23 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN23 {
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

/// MDIOS input data register 24
pub mod MDIOS_DINR24 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN24 {
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

/// MDIOS input data register 25
pub mod MDIOS_DINR25 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN25 {
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

/// MDIOS input data register 26
pub mod MDIOS_DINR26 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN26 {
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

/// MDIOS input data register 27
pub mod MDIOS_DINR27 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN27 {
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

/// MDIOS input data register 28
pub mod MDIOS_DINR28 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN28 {
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

/// MDIOS input data register 29
pub mod MDIOS_DINR29 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN29 {
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

/// MDIOS input data register 30
pub mod MDIOS_DINR30 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN30 {
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

/// MDIOS input data register 31
pub mod MDIOS_DINR31 {

    /// Input data received from MDIO Master during write frames
    pub mod DIN31 {
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
pub mod MDIOS_DOUTR0 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT0 {
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

/// MDIOS output data register 1
pub mod MDIOS_DOUTR1 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT1 {
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

/// MDIOS output data register 2
pub mod MDIOS_DOUTR2 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT2 {
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

/// MDIOS output data register 3
pub mod MDIOS_DOUTR3 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT3 {
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

/// MDIOS output data register 4
pub mod MDIOS_DOUTR4 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT4 {
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

/// MDIOS output data register 5
pub mod MDIOS_DOUTR5 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT5 {
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

/// MDIOS output data register 6
pub mod MDIOS_DOUTR6 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT6 {
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

/// MDIOS output data register 7
pub mod MDIOS_DOUTR7 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT7 {
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

/// MDIOS output data register 8
pub mod MDIOS_DOUTR8 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT8 {
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

/// MDIOS output data register 9
pub mod MDIOS_DOUTR9 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT9 {
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

/// MDIOS output data register 10
pub mod MDIOS_DOUTR10 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT10 {
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

/// MDIOS output data register 11
pub mod MDIOS_DOUTR11 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT11 {
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

/// MDIOS output data register 12
pub mod MDIOS_DOUTR12 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT12 {
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

/// MDIOS output data register 13
pub mod MDIOS_DOUTR13 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT13 {
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

/// MDIOS output data register 14
pub mod MDIOS_DOUTR14 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT14 {
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

/// MDIOS output data register 15
pub mod MDIOS_DOUTR15 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT15 {
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

/// MDIOS output data register 16
pub mod MDIOS_DOUTR16 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT16 {
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

/// MDIOS output data register 17
pub mod MDIOS_DOUTR17 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT17 {
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

/// MDIOS output data register 18
pub mod MDIOS_DOUTR18 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT18 {
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

/// MDIOS output data register 19
pub mod MDIOS_DOUTR19 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT19 {
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

/// MDIOS output data register 20
pub mod MDIOS_DOUTR20 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT20 {
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

/// MDIOS output data register 21
pub mod MDIOS_DOUTR21 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT21 {
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

/// MDIOS output data register 22
pub mod MDIOS_DOUTR22 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT22 {
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

/// MDIOS output data register 23
pub mod MDIOS_DOUTR23 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT23 {
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

/// MDIOS output data register 24
pub mod MDIOS_DOUTR24 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT24 {
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

/// MDIOS output data register 25
pub mod MDIOS_DOUTR25 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT25 {
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

/// MDIOS output data register 26
pub mod MDIOS_DOUTR26 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT26 {
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

/// MDIOS output data register 27
pub mod MDIOS_DOUTR27 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT27 {
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

/// MDIOS output data register 28
pub mod MDIOS_DOUTR28 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT28 {
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

/// MDIOS output data register 29
pub mod MDIOS_DOUTR29 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT29 {
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

/// MDIOS output data register 30
pub mod MDIOS_DOUTR30 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT30 {
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

/// MDIOS output data register 31
pub mod MDIOS_DOUTR31 {

    /// Output data sent to MDIO Master during read frames
    pub mod DOUT31 {
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

    /// MDIOS input data register 0
    pub MDIOS_DINR0: RORegister<u32>,

    /// MDIOS input data register 1
    pub MDIOS_DINR1: RORegister<u32>,

    /// MDIOS input data register 2
    pub MDIOS_DINR2: RORegister<u32>,

    /// MDIOS input data register 3
    pub MDIOS_DINR3: RORegister<u32>,

    /// MDIOS input data register 4
    pub MDIOS_DINR4: RORegister<u32>,

    /// MDIOS input data register 5
    pub MDIOS_DINR5: RORegister<u32>,

    /// MDIOS input data register 6
    pub MDIOS_DINR6: RORegister<u32>,

    /// MDIOS input data register 7
    pub MDIOS_DINR7: RORegister<u32>,

    /// MDIOS input data register 8
    pub MDIOS_DINR8: RORegister<u32>,

    /// MDIOS input data register 9
    pub MDIOS_DINR9: RORegister<u32>,

    /// MDIOS input data register 10
    pub MDIOS_DINR10: RORegister<u32>,

    /// MDIOS input data register 11
    pub MDIOS_DINR11: RORegister<u32>,

    /// MDIOS input data register 12
    pub MDIOS_DINR12: RORegister<u32>,

    /// MDIOS input data register 13
    pub MDIOS_DINR13: RORegister<u32>,

    /// MDIOS input data register 14
    pub MDIOS_DINR14: RORegister<u32>,

    /// MDIOS input data register 15
    pub MDIOS_DINR15: RORegister<u32>,

    /// MDIOS input data register 16
    pub MDIOS_DINR16: RORegister<u32>,

    /// MDIOS input data register 17
    pub MDIOS_DINR17: RORegister<u32>,

    /// MDIOS input data register 18
    pub MDIOS_DINR18: RORegister<u32>,

    /// MDIOS input data register 19
    pub MDIOS_DINR19: RORegister<u32>,

    /// MDIOS input data register 20
    pub MDIOS_DINR20: RORegister<u32>,

    /// MDIOS input data register 21
    pub MDIOS_DINR21: RORegister<u32>,

    /// MDIOS input data register 22
    pub MDIOS_DINR22: RORegister<u32>,

    /// MDIOS input data register 23
    pub MDIOS_DINR23: RORegister<u32>,

    /// MDIOS input data register 24
    pub MDIOS_DINR24: RORegister<u32>,

    /// MDIOS input data register 25
    pub MDIOS_DINR25: RORegister<u32>,

    /// MDIOS input data register 26
    pub MDIOS_DINR26: RORegister<u32>,

    /// MDIOS input data register 27
    pub MDIOS_DINR27: RORegister<u32>,

    /// MDIOS input data register 28
    pub MDIOS_DINR28: RORegister<u32>,

    /// MDIOS input data register 29
    pub MDIOS_DINR29: RORegister<u32>,

    /// MDIOS input data register 30
    pub MDIOS_DINR30: RORegister<u32>,

    /// MDIOS input data register 31
    pub MDIOS_DINR31: RORegister<u32>,

    /// MDIOS output data register 0
    pub MDIOS_DOUTR0: RWRegister<u32>,

    /// MDIOS output data register 1
    pub MDIOS_DOUTR1: RWRegister<u32>,

    /// MDIOS output data register 2
    pub MDIOS_DOUTR2: RWRegister<u32>,

    /// MDIOS output data register 3
    pub MDIOS_DOUTR3: RWRegister<u32>,

    /// MDIOS output data register 4
    pub MDIOS_DOUTR4: RWRegister<u32>,

    /// MDIOS output data register 5
    pub MDIOS_DOUTR5: RWRegister<u32>,

    /// MDIOS output data register 6
    pub MDIOS_DOUTR6: RWRegister<u32>,

    /// MDIOS output data register 7
    pub MDIOS_DOUTR7: RWRegister<u32>,

    /// MDIOS output data register 8
    pub MDIOS_DOUTR8: RWRegister<u32>,

    /// MDIOS output data register 9
    pub MDIOS_DOUTR9: RWRegister<u32>,

    /// MDIOS output data register 10
    pub MDIOS_DOUTR10: RWRegister<u32>,

    /// MDIOS output data register 11
    pub MDIOS_DOUTR11: RWRegister<u32>,

    /// MDIOS output data register 12
    pub MDIOS_DOUTR12: RWRegister<u32>,

    /// MDIOS output data register 13
    pub MDIOS_DOUTR13: RWRegister<u32>,

    /// MDIOS output data register 14
    pub MDIOS_DOUTR14: RWRegister<u32>,

    /// MDIOS output data register 15
    pub MDIOS_DOUTR15: RWRegister<u32>,

    /// MDIOS output data register 16
    pub MDIOS_DOUTR16: RWRegister<u32>,

    /// MDIOS output data register 17
    pub MDIOS_DOUTR17: RWRegister<u32>,

    /// MDIOS output data register 18
    pub MDIOS_DOUTR18: RWRegister<u32>,

    /// MDIOS output data register 19
    pub MDIOS_DOUTR19: RWRegister<u32>,

    /// MDIOS output data register 20
    pub MDIOS_DOUTR20: RWRegister<u32>,

    /// MDIOS output data register 21
    pub MDIOS_DOUTR21: RWRegister<u32>,

    /// MDIOS output data register 22
    pub MDIOS_DOUTR22: RWRegister<u32>,

    /// MDIOS output data register 23
    pub MDIOS_DOUTR23: RWRegister<u32>,

    /// MDIOS output data register 24
    pub MDIOS_DOUTR24: RWRegister<u32>,

    /// MDIOS output data register 25
    pub MDIOS_DOUTR25: RWRegister<u32>,

    /// MDIOS output data register 26
    pub MDIOS_DOUTR26: RWRegister<u32>,

    /// MDIOS output data register 27
    pub MDIOS_DOUTR27: RWRegister<u32>,

    /// MDIOS output data register 28
    pub MDIOS_DOUTR28: RWRegister<u32>,

    /// MDIOS output data register 29
    pub MDIOS_DOUTR29: RWRegister<u32>,

    /// MDIOS output data register 30
    pub MDIOS_DOUTR30: RWRegister<u32>,

    /// MDIOS output data register 31
    pub MDIOS_DOUTR31: RWRegister<u32>,
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
        addr: 0x40009400,
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
pub const MDIOS: *const RegisterBlock = 0x40009400 as *const _;
