#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! HASH register block
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// HASH control register
pub mod HASH_CR {

    /// INIT
    pub mod INIT {
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

    /// DMAE
    pub mod DMAE {
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

    /// DATATYPE
    pub mod DATATYPE {
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

    /// MODE
    pub mod MODE {
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

    /// ALGO0
    pub mod ALGO0 {
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

    /// NBW
    pub mod NBW {
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

    /// DINNE
    pub mod DINNE {
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

    /// MDMAT
    pub mod MDMAT {
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

    /// DMAA
    pub mod DMAA {
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

    /// LKEY
    pub mod LKEY {
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

    /// ALGO1
    pub mod ALGO1 {
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
}

/// HASH_DIN is the data input register.
pub mod HASH_DIN {

    /// DATAIN
    pub mod DATAIN {
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

/// The HASH_STR register has two functions: It is used to define the number of valid bits in the last word of the message entered in the hash processor (that is the number of valid least significant bits in the last data written to the HASH_DIN register) It is used to start the processing of the last block in the message by writing the DCAL bit to 1
pub mod HASH_STR {

    /// NBLW
    pub mod NBLW {
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

    /// DCAL
    pub mod DCAL {
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

/// HASH digest register 0
pub mod HASH_HR0 {

    /// H0
    pub mod H0 {
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

/// HASH digest register 1
pub mod HASH_HR1 {

    /// H1
    pub mod H1 {
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

/// HASH digest register 2
pub mod HASH_HR2 {

    /// H2
    pub mod H2 {
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

/// HASH digest register 3
pub mod HASH_HR3 {

    /// H3
    pub mod H3 {
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

/// HASH digest register 4
pub mod HASH_HR4 {

    /// H4
    pub mod H4 {
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

/// HASH interrupt enable register
pub mod HASH_IMR {

    /// DINIE
    pub mod DINIE {
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

    /// DCIE
    pub mod DCIE {
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

/// HASH status register
pub mod HASH_SR {

    /// DINIS
    pub mod DINIS {
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

    /// DCIS
    pub mod DCIS {
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

    /// DMAS
    pub mod DMAS {
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

    /// BUSY
    pub mod BUSY {
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
}

/// These registers contain the complete internal register states of the hash processor. They are useful when a context swap has to be done because a high-priority task needs to use the hash processor while it is already used by another task. When such an event occurs, the HASH_CSRx registers have to be read and the read values have to be saved in the system memory space. Then the hash processor can be used by the preemptive task, and when the hash computation is complete, the saved context can be read from memory and written back into the HASH_CSRx registers.
pub mod HASH_CSR0 {

    /// CS0
    pub mod CS0 {
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

/// HASH context swap registers
pub mod HASH_CSR1 {

    /// CS1
    pub mod CS1 {
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

/// HASH context swap registers
pub mod HASH_CSR2 {

    /// CS2
    pub mod CS2 {
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

/// HASH context swap registers
pub mod HASH_CSR3 {

    /// CS3
    pub mod CS3 {
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

/// HASH context swap registers
pub mod HASH_CSR4 {

    /// CS4
    pub mod CS4 {
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

/// HASH context swap registers
pub mod HASH_CSR5 {

    /// CS5
    pub mod CS5 {
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

/// HASH context swap registers
pub mod HASH_CSR6 {

    /// CS6
    pub mod CS6 {
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

/// HASH context swap registers
pub mod HASH_CSR7 {

    /// CS7
    pub mod CS7 {
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

/// HASH context swap registers
pub mod HASH_CSR8 {

    /// CS8
    pub mod CS8 {
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

/// HASH context swap registers
pub mod HASH_CSR9 {

    /// CS9
    pub mod CS9 {
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

/// HASH context swap registers
pub mod HASH_CSR10 {

    /// CS10
    pub mod CS10 {
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

/// HASH context swap registers
pub mod HASH_CSR11 {

    /// CS11
    pub mod CS11 {
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

/// HASH context swap registers
pub mod HASH_CSR12 {

    /// CS12
    pub mod CS12 {
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

/// HASH context swap registers
pub mod HASH_CSR13 {

    /// CS13
    pub mod CS13 {
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

/// HASH context swap registers
pub mod HASH_CSR14 {

    /// CS14
    pub mod CS14 {
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

/// HASH context swap registers
pub mod HASH_CSR15 {

    /// CS15
    pub mod CS15 {
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

/// HASH context swap registers
pub mod HASH_CSR16 {

    /// CS16
    pub mod CS16 {
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

/// HASH context swap registers
pub mod HASH_CSR17 {

    /// CS17
    pub mod CS17 {
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

/// HASH context swap registers
pub mod HASH_CSR18 {

    /// CS18
    pub mod CS18 {
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

/// HASH context swap registers
pub mod HASH_CSR19 {

    /// CS19
    pub mod CS19 {
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

/// HASH context swap registers
pub mod HASH_CSR20 {

    /// CS20
    pub mod CS20 {
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

/// HASH context swap registers
pub mod HASH_CSR21 {

    /// CS21
    pub mod CS21 {
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

/// HASH context swap registers
pub mod HASH_CSR22 {

    /// CS22
    pub mod CS22 {
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

/// HASH context swap registers
pub mod HASH_CSR23 {

    /// CS23
    pub mod CS23 {
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

/// HASH context swap registers
pub mod HASH_CSR24 {

    /// CS24
    pub mod CS24 {
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

/// HASH context swap registers
pub mod HASH_CSR25 {

    /// CS25
    pub mod CS25 {
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

/// HASH context swap registers
pub mod HASH_CSR26 {

    /// CS26
    pub mod CS26 {
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

/// HASH context swap registers
pub mod HASH_CSR27 {

    /// CS27
    pub mod CS27 {
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

/// HASH context swap registers
pub mod HASH_CSR28 {

    /// CS28
    pub mod CS28 {
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

/// HASH context swap registers
pub mod HASH_CSR29 {

    /// CS29
    pub mod CS29 {
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

/// HASH context swap registers
pub mod HASH_CSR30 {

    /// CS30
    pub mod CS30 {
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

/// HASH context swap registers
pub mod HASH_CSR31 {

    /// CS31
    pub mod CS31 {
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

/// HASH context swap registers
pub mod HASH_CSR32 {

    /// CS32
    pub mod CS32 {
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

/// HASH context swap registers
pub mod HASH_CSR33 {

    /// CS33
    pub mod CS33 {
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

/// HASH context swap registers
pub mod HASH_CSR34 {

    /// CS34
    pub mod CS34 {
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

/// HASH context swap registers
pub mod HASH_CSR35 {

    /// CS35
    pub mod CS35 {
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

/// HASH context swap registers
pub mod HASH_CSR36 {

    /// CS36
    pub mod CS36 {
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

/// HASH context swap registers
pub mod HASH_CSR37 {

    /// CS37
    pub mod CS37 {
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

/// HASH context swap registers
pub mod HASH_CSR38 {

    /// CS38
    pub mod CS38 {
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

/// HASH context swap registers
pub mod HASH_CSR39 {

    /// CS39
    pub mod CS39 {
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

/// HASH context swap registers
pub mod HASH_CSR40 {

    /// CS40
    pub mod CS40 {
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

/// HASH context swap registers
pub mod HASH_CSR41 {

    /// CS41
    pub mod CS41 {
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

/// HASH context swap registers
pub mod HASH_CSR42 {

    /// CS42
    pub mod CS42 {
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

/// HASH context swap registers
pub mod HASH_CSR43 {

    /// CS43
    pub mod CS43 {
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

/// HASH context swap registers
pub mod HASH_CSR44 {

    /// CS44
    pub mod CS44 {
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

/// HASH context swap registers
pub mod HASH_CSR45 {

    /// CS45
    pub mod CS45 {
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

/// HASH context swap registers
pub mod HASH_CSR46 {

    /// CS46
    pub mod CS46 {
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

/// HASH context swap registers
pub mod HASH_CSR47 {

    /// CS47
    pub mod CS47 {
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

/// HASH context swap registers
pub mod HASH_CSR48 {

    /// CS48
    pub mod CS48 {
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

/// HASH context swap registers
pub mod HASH_CSR49 {

    /// CS49
    pub mod CS49 {
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

/// HASH context swap registers
pub mod HASH_CSR50 {

    /// CS50
    pub mod CS50 {
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

/// HASH context swap registers
pub mod HASH_CSR51 {

    /// CS51
    pub mod CS51 {
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

/// HASH context swap registers
pub mod HASH_CSR52 {

    /// CS52
    pub mod CS52 {
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

/// HASH context swap registers
pub mod HASH_CSR53 {

    /// CS53
    pub mod CS53 {
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

/// HASH digest register 5
pub mod HASH_HR5 {

    /// H5
    pub mod H5 {
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

/// HASH digest register 6
pub mod HASH_HR6 {

    /// H6
    pub mod H6 {
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

/// HASH digest register 7
pub mod HASH_HR7 {

    /// H7
    pub mod H7 {
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

/// HASH Hardware Configuration Register
pub mod HASH_HWCFGR {

    /// CFG1
    pub mod CFG1 {
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

/// HASH Version Register
pub mod HASH_VERR {

    /// VER
    pub mod VER {
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

/// HASH Identification
pub mod HASH_IPIDR {

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

/// HASH Hardware Magic ID
pub mod HASH_MID {

    /// MID
    pub mod MID {
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
    /// HASH control register
    pub HASH_CR: RWRegister<u32>,

    /// HASH_DIN is the data input register.
    pub HASH_DIN: RWRegister<u32>,

    /// The HASH_STR register has two functions: It is used to define the number of valid bits in the last word of the message entered in the hash processor (that is the number of valid least significant bits in the last data written to the HASH_DIN register) It is used to start the processing of the last block in the message by writing the DCAL bit to 1
    pub HASH_STR: RWRegister<u32>,

    /// HASH digest register 0
    pub HASH_HR0: RORegister<u32>,

    /// HASH digest register 1
    pub HASH_HR1: RORegister<u32>,

    /// HASH digest register 2
    pub HASH_HR2: RORegister<u32>,

    /// HASH digest register 3
    pub HASH_HR3: RORegister<u32>,

    /// HASH digest register 4
    pub HASH_HR4: RORegister<u32>,

    /// HASH interrupt enable register
    pub HASH_IMR: RWRegister<u32>,

    /// HASH status register
    pub HASH_SR: RWRegister<u32>,

    _reserved1: [u8; 208],

    /// These registers contain the complete internal register states of the hash processor. They are useful when a context swap has to be done because a high-priority task needs to use the hash processor while it is already used by another task. When such an event occurs, the HASH_CSRx registers have to be read and the read values have to be saved in the system memory space. Then the hash processor can be used by the preemptive task, and when the hash computation is complete, the saved context can be read from memory and written back into the HASH_CSRx registers.
    pub HASH_CSR0: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR1: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR2: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR3: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR4: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR5: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR6: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR7: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR8: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR9: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR10: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR11: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR12: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR13: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR14: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR15: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR16: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR17: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR18: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR19: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR20: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR21: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR22: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR23: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR24: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR25: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR26: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR27: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR28: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR29: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR30: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR31: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR32: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR33: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR34: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR35: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR36: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR37: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR38: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR39: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR40: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR41: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR42: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR43: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR44: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR45: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR46: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR47: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR48: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR49: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR50: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR51: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR52: RWRegister<u32>,

    /// HASH context swap registers
    pub HASH_CSR53: RWRegister<u32>,

    _reserved2: [u8; 340],

    /// HASH digest register 5
    pub HASH_HR5: RORegister<u32>,

    /// HASH digest register 6
    pub HASH_HR6: RORegister<u32>,

    /// HASH digest register 7
    pub HASH_HR7: RORegister<u32>,

    _reserved3: [u8; 192],

    /// HASH Hardware Configuration Register
    pub HASH_HWCFGR: RORegister<u32>,

    /// HASH Version Register
    pub HASH_VERR: RORegister<u32>,

    /// HASH Identification
    pub HASH_IPIDR: RORegister<u32>,

    /// HASH Hardware Magic ID
    pub HASH_MID: RORegister<u32>,
}
pub struct ResetValues {
    pub HASH_CR: u32,
    pub HASH_DIN: u32,
    pub HASH_STR: u32,
    pub HASH_HR0: u32,
    pub HASH_HR1: u32,
    pub HASH_HR2: u32,
    pub HASH_HR3: u32,
    pub HASH_HR4: u32,
    pub HASH_IMR: u32,
    pub HASH_SR: u32,
    pub HASH_CSR0: u32,
    pub HASH_CSR1: u32,
    pub HASH_CSR2: u32,
    pub HASH_CSR3: u32,
    pub HASH_CSR4: u32,
    pub HASH_CSR5: u32,
    pub HASH_CSR6: u32,
    pub HASH_CSR7: u32,
    pub HASH_CSR8: u32,
    pub HASH_CSR9: u32,
    pub HASH_CSR10: u32,
    pub HASH_CSR11: u32,
    pub HASH_CSR12: u32,
    pub HASH_CSR13: u32,
    pub HASH_CSR14: u32,
    pub HASH_CSR15: u32,
    pub HASH_CSR16: u32,
    pub HASH_CSR17: u32,
    pub HASH_CSR18: u32,
    pub HASH_CSR19: u32,
    pub HASH_CSR20: u32,
    pub HASH_CSR21: u32,
    pub HASH_CSR22: u32,
    pub HASH_CSR23: u32,
    pub HASH_CSR24: u32,
    pub HASH_CSR25: u32,
    pub HASH_CSR26: u32,
    pub HASH_CSR27: u32,
    pub HASH_CSR28: u32,
    pub HASH_CSR29: u32,
    pub HASH_CSR30: u32,
    pub HASH_CSR31: u32,
    pub HASH_CSR32: u32,
    pub HASH_CSR33: u32,
    pub HASH_CSR34: u32,
    pub HASH_CSR35: u32,
    pub HASH_CSR36: u32,
    pub HASH_CSR37: u32,
    pub HASH_CSR38: u32,
    pub HASH_CSR39: u32,
    pub HASH_CSR40: u32,
    pub HASH_CSR41: u32,
    pub HASH_CSR42: u32,
    pub HASH_CSR43: u32,
    pub HASH_CSR44: u32,
    pub HASH_CSR45: u32,
    pub HASH_CSR46: u32,
    pub HASH_CSR47: u32,
    pub HASH_CSR48: u32,
    pub HASH_CSR49: u32,
    pub HASH_CSR50: u32,
    pub HASH_CSR51: u32,
    pub HASH_CSR52: u32,
    pub HASH_CSR53: u32,
    pub HASH_HR5: u32,
    pub HASH_HR6: u32,
    pub HASH_HR7: u32,
    pub HASH_HWCFGR: u32,
    pub HASH_VERR: u32,
    pub HASH_IPIDR: u32,
    pub HASH_MID: u32,
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
