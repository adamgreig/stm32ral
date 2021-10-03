#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TAMP
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod TAMP_CR1 {

    /// TAMP1E
    pub mod TAMP1E {
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

    /// TAMP2E
    pub mod TAMP2E {
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

    /// TAMP3E
    pub mod TAMP3E {
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

    /// ITAMP1E
    pub mod ITAMP1E {
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

    /// ITAMP2E
    pub mod ITAMP2E {
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

    /// ITAMP3E
    pub mod ITAMP3E {
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

    /// ITAMP4E
    pub mod ITAMP4E {
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

    /// ITAMP5E
    pub mod ITAMP5E {
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

    /// ITAMP8E
    pub mod ITAMP8E {
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
}

/// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod TAMP_CR2 {

    /// TAMP1NOER
    pub mod TAMP1NOER {
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

    /// TAMP2NOER
    pub mod TAMP2NOER {
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

    /// TAMP3NOER
    pub mod TAMP3NOER {
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

    /// TAMP1MSK
    pub mod TAMP1MSK {
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

    /// TAMP2MSK
    pub mod TAMP2MSK {
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

    /// TAMP3MSK
    pub mod TAMP3MSK {
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

    /// TAMP1TRG
    pub mod TAMP1TRG {
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

    /// TAMP2TRG
    pub mod TAMP2TRG {
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

    /// TAMP3TRG
    pub mod TAMP3TRG {
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
}

/// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod TAMP_FLTCR {

    /// TAMPFREQ
    pub mod TAMPFREQ {
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

    /// TAMPFLT
    pub mod TAMPFLT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TAMPPRCH
    pub mod TAMPPRCH {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TAMPPUDIS
    pub mod TAMPPUDIS {
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

/// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod TAMP_ATCR1 {

    /// TAMP1AM
    pub mod TAMP1AM {
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

    /// TAMP2AM
    pub mod TAMP2AM {
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

    /// TAMP3AM
    pub mod TAMP3AM {
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

    /// ATOSEL1
    pub mod ATOSEL1 {
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

    /// ATOSEL2
    pub mod ATOSEL2 {
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

    /// ATOSEL3
    pub mod ATOSEL3 {
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

    /// ATCKSEL
    pub mod ATCKSEL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ATPER
    pub mod ATPER {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ATOSHARE
    pub mod ATOSHARE {
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

    /// FLTEN
    pub mod FLTEN {
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

/// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod TAMP_ATSEEDR {

    /// SEED
    pub mod SEED {
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

/// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod TAMP_ATOR {

    /// PRNG
    pub mod PRNG {
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

    /// SEEDF
    pub mod SEEDF {
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

    /// INITS
    pub mod INITS {
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

/// This register can be written only when the APB access is secure.
pub mod TAMP_SMCR {

    /// BKPRWDPROT
    pub mod BKPRWDPROT {
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

    /// BKPWDPROT
    pub mod BKPWDPROT {
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

    /// TAMPDPROT
    pub mod TAMPDPROT {
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

/// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod TAMP_IER {

    /// TAMP1IE
    pub mod TAMP1IE {
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

    /// TAMP2IE
    pub mod TAMP2IE {
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

    /// TAMP3IE
    pub mod TAMP3IE {
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

    /// ITAMP1IE
    pub mod ITAMP1IE {
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

    /// ITAMP2IE
    pub mod ITAMP2IE {
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

    /// ITAMP3IE
    pub mod ITAMP3IE {
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

    /// ITAMP4IE
    pub mod ITAMP4IE {
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

    /// ITAMP5IE
    pub mod ITAMP5IE {
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

    /// ITAMP8IE
    pub mod ITAMP8IE {
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
}

/// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod TAMP_SR {

    /// TAMP1F
    pub mod TAMP1F {
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

    /// TAMP2F
    pub mod TAMP2F {
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

    /// TAMP3F
    pub mod TAMP3F {
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

    /// ITAMP1F
    pub mod ITAMP1F {
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

    /// ITAMP2F
    pub mod ITAMP2F {
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

    /// ITAMP3F
    pub mod ITAMP3F {
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

    /// ITAMP4F
    pub mod ITAMP4F {
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

    /// ITAMP5F
    pub mod ITAMP5F {
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

    /// ITAMP8F
    pub mod ITAMP8F {
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
}

/// TAMP non-secure masked interrupt status register
pub mod TAMP_MISR {

    /// TAMP1MF
    pub mod TAMP1MF {
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

    /// TAMP2MF
    pub mod TAMP2MF {
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

    /// TAMP3MF
    pub mod TAMP3MF {
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

    /// ITAMP1MF
    pub mod ITAMP1MF {
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

    /// ITAMP2MF
    pub mod ITAMP2MF {
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

    /// ITAMP3MF
    pub mod ITAMP3MF {
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

    /// ITAMP4MF
    pub mod ITAMP4MF {
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

    /// ITAMP5MF
    pub mod ITAMP5MF {
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

    /// ITAMP8MF
    pub mod ITAMP8MF {
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
}

/// TAMP secure masked interrupt status register
pub mod TAMP_SMISR {
    pub use super::TAMP_MISR::ITAMP1MF;
    pub use super::TAMP_MISR::ITAMP2MF;
    pub use super::TAMP_MISR::ITAMP3MF;
    pub use super::TAMP_MISR::ITAMP4MF;
    pub use super::TAMP_MISR::ITAMP5MF;
    pub use super::TAMP_MISR::ITAMP8MF;
    pub use super::TAMP_MISR::TAMP1MF;
    pub use super::TAMP_MISR::TAMP2MF;
    pub use super::TAMP_MISR::TAMP3MF;
}

/// TAMP status clear register
pub mod TAMP_SCR {

    /// CTAMP1F
    pub mod CTAMP1F {
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

    /// CTAMP2F
    pub mod CTAMP2F {
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

    /// CTAMP3F
    pub mod CTAMP3F {
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

    /// CITAMP1F
    pub mod CITAMP1F {
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

    /// CITAMP2F
    pub mod CITAMP2F {
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

    /// CITAMP3F
    pub mod CITAMP3F {
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

    /// CITAMP4F
    pub mod CITAMP4F {
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

    /// CITAMP5F
    pub mod CITAMP5F {
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

    /// CITAMP8F
    pub mod CITAMP8F {
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
}

/// TAMP monotonic counter register
pub mod TAMP_COUNTR {

    /// COUNT
    pub mod COUNT {
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

/// TAMP configuration register
pub mod TAMP_CFGR {

    /// OUT3_RMP
    pub mod OUT3_RMP {
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

/// TAMP backup 0 register
pub mod TAMP_BKP0R {

    /// BKP
    pub mod BKP {
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

/// TAMP backup 1 register
pub mod TAMP_BKP1R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 2 register
pub mod TAMP_BKP2R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 3 register
pub mod TAMP_BKP3R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 4 register
pub mod TAMP_BKP4R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 5 register
pub mod TAMP_BKP5R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 6 register
pub mod TAMP_BKP6R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 7 register
pub mod TAMP_BKP7R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 8 register
pub mod TAMP_BKP8R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 9 register
pub mod TAMP_BKP9R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 10 register
pub mod TAMP_BKP10R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 11 register
pub mod TAMP_BKP11R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 12 register
pub mod TAMP_BKP12R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 13 register
pub mod TAMP_BKP13R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 14 register
pub mod TAMP_BKP14R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 15 register
pub mod TAMP_BKP15R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 16 register
pub mod TAMP_BKP16R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 17 register
pub mod TAMP_BKP17R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 18 register
pub mod TAMP_BKP18R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 19 register
pub mod TAMP_BKP19R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 20 register
pub mod TAMP_BKP20R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 21 register
pub mod TAMP_BKP21R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 22 register
pub mod TAMP_BKP22R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 23 register
pub mod TAMP_BKP23R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 24 register
pub mod TAMP_BKP24R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 25 register
pub mod TAMP_BKP25R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 26 register
pub mod TAMP_BKP26R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 27 register
pub mod TAMP_BKP27R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 28 register
pub mod TAMP_BKP28R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 29 register
pub mod TAMP_BKP29R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 30 register
pub mod TAMP_BKP30R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP backup 31 register
pub mod TAMP_BKP31R {
    pub use super::TAMP_BKP0R::BKP;
}

/// TAMP hardware configuration register 2
pub mod TAMP_HWCFGR2 {

    /// OPTIONREG_OUT
    pub mod OPTIONREG_OUT {
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

    /// TRUST_ZONE
    pub mod TRUST_ZONE {
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

/// TAMP hardware configuration register 1
pub mod TAMP_HWCFGR1 {

    /// BACKUP_REGS
    pub mod BACKUP_REGS {
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

    /// TAMPER
    pub mod TAMPER {
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

    /// ACTIVE_TAMPER
    pub mod ACTIVE_TAMPER {
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

    /// INT_TAMPER
    pub mod INT_TAMPER {
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

/// TAMP version register
pub mod TAMP_VERR {

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

/// TAMP identification register
pub mod TAMP_IPIDR {

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

/// TAMP size identification register
pub mod TAMP_SIDR {

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
    /// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub TAMP_CR1: RWRegister<u32>,

    /// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub TAMP_CR2: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub TAMP_FLTCR: RWRegister<u32>,

    /// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub TAMP_ATCR1: RWRegister<u32>,

    /// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub TAMP_ATSEEDR: WORegister<u32>,

    /// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub TAMP_ATOR: RORegister<u32>,

    _reserved2: [u32; 1],

    /// This register can be written only when the APB access is secure.
    pub TAMP_SMCR: RWRegister<u32>,

    _reserved3: [u32; 2],

    /// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub TAMP_IER: RWRegister<u32>,

    /// This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub TAMP_SR: RORegister<u32>,

    /// TAMP non-secure masked interrupt status register
    pub TAMP_MISR: RORegister<u32>,

    /// TAMP secure masked interrupt status register
    pub TAMP_SMISR: RORegister<u32>,

    /// TAMP status clear register
    pub TAMP_SCR: WORegister<u32>,

    /// TAMP monotonic counter register
    pub TAMP_COUNTR: RORegister<u32>,

    _reserved4: [u32; 3],

    /// TAMP configuration register
    pub TAMP_CFGR: RWRegister<u32>,

    _reserved5: [u32; 43],

    /// TAMP backup 0 register
    pub TAMP_BKP0R: RWRegister<u32>,

    /// TAMP backup 1 register
    pub TAMP_BKP1R: RWRegister<u32>,

    /// TAMP backup 2 register
    pub TAMP_BKP2R: RWRegister<u32>,

    /// TAMP backup 3 register
    pub TAMP_BKP3R: RWRegister<u32>,

    /// TAMP backup 4 register
    pub TAMP_BKP4R: RWRegister<u32>,

    /// TAMP backup 5 register
    pub TAMP_BKP5R: RWRegister<u32>,

    /// TAMP backup 6 register
    pub TAMP_BKP6R: RWRegister<u32>,

    /// TAMP backup 7 register
    pub TAMP_BKP7R: RWRegister<u32>,

    /// TAMP backup 8 register
    pub TAMP_BKP8R: RWRegister<u32>,

    /// TAMP backup 9 register
    pub TAMP_BKP9R: RWRegister<u32>,

    /// TAMP backup 10 register
    pub TAMP_BKP10R: RWRegister<u32>,

    /// TAMP backup 11 register
    pub TAMP_BKP11R: RWRegister<u32>,

    /// TAMP backup 12 register
    pub TAMP_BKP12R: RWRegister<u32>,

    /// TAMP backup 13 register
    pub TAMP_BKP13R: RWRegister<u32>,

    /// TAMP backup 14 register
    pub TAMP_BKP14R: RWRegister<u32>,

    /// TAMP backup 15 register
    pub TAMP_BKP15R: RWRegister<u32>,

    /// TAMP backup 16 register
    pub TAMP_BKP16R: RWRegister<u32>,

    /// TAMP backup 17 register
    pub TAMP_BKP17R: RWRegister<u32>,

    /// TAMP backup 18 register
    pub TAMP_BKP18R: RWRegister<u32>,

    /// TAMP backup 19 register
    pub TAMP_BKP19R: RWRegister<u32>,

    /// TAMP backup 20 register
    pub TAMP_BKP20R: RWRegister<u32>,

    /// TAMP backup 21 register
    pub TAMP_BKP21R: RWRegister<u32>,

    /// TAMP backup 22 register
    pub TAMP_BKP22R: RWRegister<u32>,

    /// TAMP backup 23 register
    pub TAMP_BKP23R: RWRegister<u32>,

    /// TAMP backup 24 register
    pub TAMP_BKP24R: RWRegister<u32>,

    /// TAMP backup 25 register
    pub TAMP_BKP25R: RWRegister<u32>,

    /// TAMP backup 26 register
    pub TAMP_BKP26R: RWRegister<u32>,

    /// TAMP backup 27 register
    pub TAMP_BKP27R: RWRegister<u32>,

    /// TAMP backup 28 register
    pub TAMP_BKP28R: RWRegister<u32>,

    /// TAMP backup 29 register
    pub TAMP_BKP29R: RWRegister<u32>,

    /// TAMP backup 30 register
    pub TAMP_BKP30R: RWRegister<u32>,

    /// TAMP backup 31 register
    pub TAMP_BKP31R: RWRegister<u32>,

    _reserved6: [u32; 155],

    /// TAMP hardware configuration register 2
    pub TAMP_HWCFGR2: RORegister<u32>,

    /// TAMP hardware configuration register 1
    pub TAMP_HWCFGR1: RORegister<u32>,

    /// TAMP version register
    pub TAMP_VERR: RORegister<u32>,

    /// TAMP identification register
    pub TAMP_IPIDR: RORegister<u32>,

    /// TAMP size identification register
    pub TAMP_SIDR: RORegister<u32>,
}
pub struct ResetValues {
    pub TAMP_CR1: u32,
    pub TAMP_CR2: u32,
    pub TAMP_FLTCR: u32,
    pub TAMP_ATCR1: u32,
    pub TAMP_ATSEEDR: u32,
    pub TAMP_ATOR: u32,
    pub TAMP_SMCR: u32,
    pub TAMP_IER: u32,
    pub TAMP_SR: u32,
    pub TAMP_MISR: u32,
    pub TAMP_SMISR: u32,
    pub TAMP_SCR: u32,
    pub TAMP_COUNTR: u32,
    pub TAMP_CFGR: u32,
    pub TAMP_BKP0R: u32,
    pub TAMP_BKP1R: u32,
    pub TAMP_BKP2R: u32,
    pub TAMP_BKP3R: u32,
    pub TAMP_BKP4R: u32,
    pub TAMP_BKP5R: u32,
    pub TAMP_BKP6R: u32,
    pub TAMP_BKP7R: u32,
    pub TAMP_BKP8R: u32,
    pub TAMP_BKP9R: u32,
    pub TAMP_BKP10R: u32,
    pub TAMP_BKP11R: u32,
    pub TAMP_BKP12R: u32,
    pub TAMP_BKP13R: u32,
    pub TAMP_BKP14R: u32,
    pub TAMP_BKP15R: u32,
    pub TAMP_BKP16R: u32,
    pub TAMP_BKP17R: u32,
    pub TAMP_BKP18R: u32,
    pub TAMP_BKP19R: u32,
    pub TAMP_BKP20R: u32,
    pub TAMP_BKP21R: u32,
    pub TAMP_BKP22R: u32,
    pub TAMP_BKP23R: u32,
    pub TAMP_BKP24R: u32,
    pub TAMP_BKP25R: u32,
    pub TAMP_BKP26R: u32,
    pub TAMP_BKP27R: u32,
    pub TAMP_BKP28R: u32,
    pub TAMP_BKP29R: u32,
    pub TAMP_BKP30R: u32,
    pub TAMP_BKP31R: u32,
    pub TAMP_HWCFGR2: u32,
    pub TAMP_HWCFGR1: u32,
    pub TAMP_VERR: u32,
    pub TAMP_IPIDR: u32,
    pub TAMP_SIDR: u32,
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
