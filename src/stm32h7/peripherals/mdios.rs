#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Management data input/output slave
//!
//! Used by: stm32h735, stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v, stm32h7b3

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

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
pub mod DINR1 {

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
pub mod DINR2 {

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
pub mod DINR3 {

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
pub mod DINR4 {

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
pub mod DINR5 {

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
pub mod DINR6 {

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
pub mod DINR7 {

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
pub mod DINR8 {

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
pub mod DINR9 {

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
pub mod DINR10 {

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
pub mod DINR11 {

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
pub mod DINR12 {

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
pub mod DINR13 {

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
pub mod DINR14 {

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
pub mod DINR15 {

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
pub mod DINR16 {

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
pub mod DINR17 {

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
pub mod DINR18 {

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
pub mod DINR19 {

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
pub mod DINR20 {

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
pub mod DINR21 {

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
pub mod DINR22 {

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
pub mod DINR23 {

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
pub mod DINR24 {

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
pub mod DINR25 {

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
pub mod DINR26 {

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
pub mod DINR27 {

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
pub mod DINR28 {

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
pub mod DINR29 {

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
pub mod DINR30 {

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
pub mod DINR31 {

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
pub mod DOUTR0 {

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
pub mod DOUTR1 {

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
pub mod DOUTR2 {

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
pub mod DOUTR3 {

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
pub mod DOUTR4 {

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
pub mod DOUTR5 {

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
pub mod DOUTR6 {

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
pub mod DOUTR7 {

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
pub mod DOUTR8 {

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
pub mod DOUTR9 {

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
pub mod DOUTR10 {

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
pub mod DOUTR11 {

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
pub mod DOUTR12 {

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
pub mod DOUTR13 {

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
pub mod DOUTR14 {

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
pub mod DOUTR15 {

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
pub mod DOUTR16 {

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
pub mod DOUTR17 {

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
pub mod DOUTR18 {

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
pub mod DOUTR19 {

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
pub mod DOUTR20 {

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
pub mod DOUTR21 {

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
pub mod DOUTR22 {

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
pub mod DOUTR23 {

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
pub mod DOUTR24 {

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
pub mod DOUTR25 {

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
pub mod DOUTR26 {

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
pub mod DOUTR27 {

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
pub mod DOUTR28 {

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
pub mod DOUTR29 {

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
pub mod DOUTR30 {

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
pub mod DOUTR31 {

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

    /// MDIOS input data register 1
    pub DINR1: RORegister<u32>,

    /// MDIOS input data register 2
    pub DINR2: RORegister<u32>,

    /// MDIOS input data register 3
    pub DINR3: RORegister<u32>,

    /// MDIOS input data register 4
    pub DINR4: RORegister<u32>,

    /// MDIOS input data register 5
    pub DINR5: RORegister<u32>,

    /// MDIOS input data register 6
    pub DINR6: RORegister<u32>,

    /// MDIOS input data register 7
    pub DINR7: RORegister<u32>,

    /// MDIOS input data register 8
    pub DINR8: RORegister<u32>,

    /// MDIOS input data register 9
    pub DINR9: RORegister<u32>,

    /// MDIOS input data register 10
    pub DINR10: RORegister<u32>,

    /// MDIOS input data register 11
    pub DINR11: RORegister<u32>,

    /// MDIOS input data register 12
    pub DINR12: RORegister<u32>,

    /// MDIOS input data register 13
    pub DINR13: RORegister<u32>,

    /// MDIOS input data register 14
    pub DINR14: RORegister<u32>,

    /// MDIOS input data register 15
    pub DINR15: RORegister<u32>,

    /// MDIOS input data register 16
    pub DINR16: RORegister<u32>,

    /// MDIOS input data register 17
    pub DINR17: RORegister<u32>,

    /// MDIOS input data register 18
    pub DINR18: RORegister<u32>,

    /// MDIOS input data register 19
    pub DINR19: RORegister<u32>,

    /// MDIOS input data register 20
    pub DINR20: RORegister<u32>,

    /// MDIOS input data register 21
    pub DINR21: RORegister<u32>,

    /// MDIOS input data register 22
    pub DINR22: RORegister<u32>,

    /// MDIOS input data register 23
    pub DINR23: RORegister<u32>,

    /// MDIOS input data register 24
    pub DINR24: RORegister<u32>,

    /// MDIOS input data register 25
    pub DINR25: RORegister<u32>,

    /// MDIOS input data register 26
    pub DINR26: RORegister<u32>,

    /// MDIOS input data register 27
    pub DINR27: RORegister<u32>,

    /// MDIOS input data register 28
    pub DINR28: RORegister<u32>,

    /// MDIOS input data register 29
    pub DINR29: RORegister<u32>,

    /// MDIOS input data register 30
    pub DINR30: RORegister<u32>,

    /// MDIOS input data register 31
    pub DINR31: RORegister<u32>,

    /// MDIOS output data register 0
    pub DOUTR0: RWRegister<u32>,

    /// MDIOS output data register 1
    pub DOUTR1: RWRegister<u32>,

    /// MDIOS output data register 2
    pub DOUTR2: RWRegister<u32>,

    /// MDIOS output data register 3
    pub DOUTR3: RWRegister<u32>,

    /// MDIOS output data register 4
    pub DOUTR4: RWRegister<u32>,

    /// MDIOS output data register 5
    pub DOUTR5: RWRegister<u32>,

    /// MDIOS output data register 6
    pub DOUTR6: RWRegister<u32>,

    /// MDIOS output data register 7
    pub DOUTR7: RWRegister<u32>,

    /// MDIOS output data register 8
    pub DOUTR8: RWRegister<u32>,

    /// MDIOS output data register 9
    pub DOUTR9: RWRegister<u32>,

    /// MDIOS output data register 10
    pub DOUTR10: RWRegister<u32>,

    /// MDIOS output data register 11
    pub DOUTR11: RWRegister<u32>,

    /// MDIOS output data register 12
    pub DOUTR12: RWRegister<u32>,

    /// MDIOS output data register 13
    pub DOUTR13: RWRegister<u32>,

    /// MDIOS output data register 14
    pub DOUTR14: RWRegister<u32>,

    /// MDIOS output data register 15
    pub DOUTR15: RWRegister<u32>,

    /// MDIOS output data register 16
    pub DOUTR16: RWRegister<u32>,

    /// MDIOS output data register 17
    pub DOUTR17: RWRegister<u32>,

    /// MDIOS output data register 18
    pub DOUTR18: RWRegister<u32>,

    /// MDIOS output data register 19
    pub DOUTR19: RWRegister<u32>,

    /// MDIOS output data register 20
    pub DOUTR20: RWRegister<u32>,

    /// MDIOS output data register 21
    pub DOUTR21: RWRegister<u32>,

    /// MDIOS output data register 22
    pub DOUTR22: RWRegister<u32>,

    /// MDIOS output data register 23
    pub DOUTR23: RWRegister<u32>,

    /// MDIOS output data register 24
    pub DOUTR24: RWRegister<u32>,

    /// MDIOS output data register 25
    pub DOUTR25: RWRegister<u32>,

    /// MDIOS output data register 26
    pub DOUTR26: RWRegister<u32>,

    /// MDIOS output data register 27
    pub DOUTR27: RWRegister<u32>,

    /// MDIOS output data register 28
    pub DOUTR28: RWRegister<u32>,

    /// MDIOS output data register 29
    pub DOUTR29: RWRegister<u32>,

    /// MDIOS output data register 30
    pub DOUTR30: RWRegister<u32>,

    /// MDIOS output data register 31
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
#[cfg(feature = "rtic")]
unsafe impl Send for Instance {}
