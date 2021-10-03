#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DDRPHYC
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DDRPHYC revision ID register
pub mod DDRPHYC_RIDR {

    /// PUBMNR
    pub mod PUBMNR {
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

    /// PUBMDR
    pub mod PUBMDR {
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

    /// PUBMJR
    pub mod PUBMJR {
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

    /// PHYMNR
    pub mod PHYMNR {
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

    /// PHYMDR
    pub mod PHYMDR {
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

    /// PHYMJR
    pub mod PHYMJR {
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

    /// UDRID
    pub mod UDRID {
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

/// DDRPHYC PHY initialization register
pub mod DDRPHYC_PIR {

    /// INIT
    pub mod INIT {
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

    /// DLLSRST
    pub mod DLLSRST {
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

    /// DLLLOCK
    pub mod DLLLOCK {
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

    /// ZCAL
    pub mod ZCAL {
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

    /// ITMSRST
    pub mod ITMSRST {
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

    /// DRAMRST
    pub mod DRAMRST {
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

    /// DRAMINIT
    pub mod DRAMINIT {
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

    /// QSTRN
    pub mod QSTRN {
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

    /// RVTRN
    pub mod RVTRN {
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

    /// ICPC
    pub mod ICPC {
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

    /// DLLBYP
    pub mod DLLBYP {
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

    /// CTLDINIT
    pub mod CTLDINIT {
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

    /// CLRSR
    pub mod CLRSR {
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

    /// LOCKBYP
    pub mod LOCKBYP {
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

    /// ZCALBYP
    pub mod ZCALBYP {
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

    /// INITBYP
    pub mod INITBYP {
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

/// DDRPHYC PHY global control register
pub mod DDRPHYC_PGCR {

    /// ITMDMD
    pub mod ITMDMD {
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

    /// DQSCFG
    pub mod DQSCFG {
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

    /// DFTCMP
    pub mod DFTCMP {
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

    /// DFTLMT
    pub mod DFTLMT {
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

    /// DTOSEL
    pub mod DTOSEL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (4 bits: 0b1111 << 5)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CKEN
    pub mod CKEN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CKDV
    pub mod CKDV {
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

    /// CKINV
    pub mod CKINV {
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

    /// IOLB
    pub mod IOLB {
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

    /// IODDRM
    pub mod IODDRM {
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

    /// RANKEN
    pub mod RANKEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (4 bits: 0b1111 << 18)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ZKSEL
    pub mod ZKSEL {
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

    /// PDDISDX
    pub mod PDDISDX {
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

    /// RFSHDT
    pub mod RFSHDT {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (4 bits: 0b1111 << 25)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LBDQSS
    pub mod LBDQSS {
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

    /// LBGDQS
    pub mod LBGDQS {
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

    /// LBMODE
    pub mod LBMODE {
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

/// DDRPHYC PHY global status register
pub mod DDRPHYC_PGSR {

    /// IDONE
    pub mod IDONE {
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

    /// DLDONE
    pub mod DLDONE {
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

    /// ZCDDONE
    pub mod ZCDDONE {
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

    /// DIDONE
    pub mod DIDONE {
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

    /// DTDONE
    pub mod DTDONE {
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

    /// DTERR
    pub mod DTERR {
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

    /// DTIERR
    pub mod DTIERR {
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

    /// DFTERR
    pub mod DFTERR {
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

    /// RVERR
    pub mod RVERR {
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

    /// RVEIRR
    pub mod RVEIRR {
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

    /// TQ
    pub mod TQ {
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

/// DDRPHYC DDR global control register
pub mod DDRPHYC_DLLGCR {

    /// DRES
    pub mod DRES {
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

    /// IPUMP
    pub mod IPUMP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (3 bits: 0b111 << 2)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TESTEN
    pub mod TESTEN {
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

    /// DTC
    pub mod DTC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ATC
    pub mod ATC {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TESTSW
    pub mod TESTSW {
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

    /// MBIAS
    pub mod MBIAS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (8 bits: 0xff << 12)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SBIAS2_0
    pub mod SBIAS2_0 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BPS200
    pub mod BPS200 {
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

    /// SBIAS5_3
    pub mod SBIAS5_3 {
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

    /// FDTRMSL
    pub mod FDTRMSL {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (2 bits: 0b11 << 27)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LOCKDET
    pub mod LOCKDET {
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

    /// DLLRSVD2
    pub mod DLLRSVD2 {
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

/// DDRPHYC AC DLL control register
pub mod DDRPHYC_ACDLLCR {

    /// MFBDLY
    pub mod MFBDLY {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MFWDLY
    pub mod MFWDLY {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ATESTEN
    pub mod ATESTEN {
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

    /// DLLSRST
    pub mod DLLSRST {
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

    /// DLLDIS
    pub mod DLLDIS {
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

/// DDRPHYC PT register 0
pub mod DDRPHYC_PTR0 {

    /// TDLLSRST
    pub mod TDLLSRST {
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

    /// TDLLLOCK
    pub mod TDLLLOCK {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (12 bits: 0xfff << 6)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TITMSRST
    pub mod TITMSRST {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (4 bits: 0b1111 << 18)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC PT register 1
pub mod DDRPHYC_PTR1 {

    /// TDINIT0
    pub mod TDINIT0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (19 bits: 0x7ffff << 0)
        pub const mask: u32 = 0x7ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TDINIT1
    pub mod TDINIT1 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (8 bits: 0xff << 19)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC PT register 2
pub mod DDRPHYC_PTR2 {

    /// TDINIT2
    pub mod TDINIT2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (17 bits: 0x1ffff << 0)
        pub const mask: u32 = 0x1ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TDINIT3
    pub mod TDINIT3 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (10 bits: 0x3ff << 17)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC ACIOC register
pub mod DDRPHYC_ACIOCR {

    /// ACIOM
    pub mod ACIOM {
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

    /// ACOE
    pub mod ACOE {
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

    /// ACODT
    pub mod ACODT {
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

    /// ACPDD
    pub mod ACPDD {
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

    /// ACPDR
    pub mod ACPDR {
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

    /// CKODT
    pub mod CKODT {
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

    /// CKPDD
    pub mod CKPDD {
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

    /// CKPDR
    pub mod CKPDR {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (3 bits: 0b111 << 11)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RANKODT
    pub mod RANKODT {
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

    /// CSPDD
    pub mod CSPDD {
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

    /// RANKPDR
    pub mod RANKPDR {
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

    /// RSTODT
    pub mod RSTODT {
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

    /// RSTPDD
    pub mod RSTPDD {
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

    /// RSTPDR
    pub mod RSTPDR {
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

    /// RSTIOM
    pub mod RSTIOM {
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

    /// ACSR
    pub mod ACSR {
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

/// DDRPHYC DXCC register
pub mod DDRPHYC_DXCCR {

    /// DXODT
    pub mod DXODT {
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

    /// DXIOM
    pub mod DXIOM {
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

    /// DXPDD
    pub mod DXPDD {
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

    /// DXPDR
    pub mod DXPDR {
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

    /// DQSRES
    pub mod DQSRES {
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

    /// DQSNRES
    pub mod DQSNRES {
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

    /// DQSNRST
    pub mod DQSNRST {
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

    /// RVSEL
    pub mod RVSEL {
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

    /// AWDT
    pub mod AWDT {
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

/// DDRPHYC DSGC register
pub mod DDRPHYC_DSGCR {

    /// PUREN
    pub mod PUREN {
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

    /// BDISEN
    pub mod BDISEN {
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

    /// ZUEN
    pub mod ZUEN {
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

    /// LPIOPD
    pub mod LPIOPD {
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

    /// LPDLLPD
    pub mod LPDLLPD {
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

    /// DQSGX
    pub mod DQSGX {
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

    /// DQSGE
    pub mod DQSGE {
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

    /// NOBUB
    pub mod NOBUB {
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

    /// FXDLAT
    pub mod FXDLAT {
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

    /// CKEPDD
    pub mod CKEPDD {
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

    /// ODTPDD
    pub mod ODTPDD {
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

    /// NL2PD
    pub mod NL2PD {
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

    /// NL2OE
    pub mod NL2OE {
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

    /// TPDPD
    pub mod TPDPD {
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

    /// TPDOE
    pub mod TPDOE {
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

    /// CKOE
    pub mod CKOE {
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

    /// ODTOE
    pub mod ODTOE {
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

    /// RSTOE
    pub mod RSTOE {
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

    /// CKEOE
    pub mod CKEOE {
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

/// DDRPHYC DC register
pub mod DDRPHYC_DCR {

    /// DDRMD
    pub mod DDRMD {
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

    /// DDR8BNK
    pub mod DDR8BNK {
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

    /// PDQ
    pub mod PDQ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MPRDQ
    pub mod MPRDQ {
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

    /// DDRTYPE
    pub mod DDRTYPE {
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

    /// NOSRA
    pub mod NOSRA {
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

    /// DDR2T
    pub mod DDR2T {
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

    /// UDIMM
    pub mod UDIMM {
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

    /// RDIMM
    pub mod RDIMM {
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

    /// TPD
    pub mod TPD {
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

/// DDRPHYC DTP register 0
pub mod DDRPHYC_DTPR0 {

    /// TMRD
    pub mod TMRD {
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

    /// TRTP
    pub mod TRTP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (3 bits: 0b111 << 2)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TWTR
    pub mod TWTR {
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

    /// TRP
    pub mod TRP {
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

    /// TRCD
    pub mod TRCD {
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

    /// TRAS
    pub mod TRAS {
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

    /// TRRD
    pub mod TRRD {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (4 bits: 0b1111 << 21)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRC
    pub mod TRC {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (6 bits: 0x3f << 25)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TCCD
    pub mod TCCD {
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

/// DDRPHYC DTP register 1
pub mod DDRPHYC_DTPR1 {

    /// TAOND
    pub mod TAOND {
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

    /// TRTW
    pub mod TRTW {
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

    /// TFAW
    pub mod TFAW {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (6 bits: 0x3f << 3)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TMOD
    pub mod TMOD {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRTODT
    pub mod TRTODT {
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

    /// TRFC
    pub mod TRFC {
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

    /// TDQSCKMIN
    pub mod TDQSCKMIN {
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

    /// TDQSCKMAX
    pub mod TDQSCKMAX {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (3 bits: 0b111 << 27)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC DTP register 2
pub mod DDRPHYC_DTPR2 {

    /// TXS
    pub mod TXS {
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

    /// TXP
    pub mod TXP {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TCKE
    pub mod TCKE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (4 bits: 0b1111 << 15)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TDLLK
    pub mod TDLLK {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (10 bits: 0x3ff << 19)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC MR0 register for DDR3
pub mod DDRPHYC_DDR3_MR0 {

    /// BL
    pub mod BL {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CL0
    pub mod CL0 {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BT
    pub mod BT {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CL
    pub mod CL {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TM
    pub mod TM {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DR
    pub mod DR {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WR
    pub mod WR {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PD
    pub mod PD {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RSVD
    pub mod RSVD {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC MR1 register for DDR3
pub mod DDRPHYC_DDR3_MR1 {

    /// DE
    pub mod DE {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DIC0
    pub mod DIC0 {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RTT0
    pub mod RTT0 {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AL
    pub mod AL {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DIC1
    pub mod DIC1 {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RTT1
    pub mod RTT1 {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LEVEL
    pub mod LEVEL {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RTT2
    pub mod RTT2 {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TDQS
    pub mod TDQS {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// QOFF
    pub mod QOFF {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC MR2 register for DDR3
pub mod DDRPHYC_DDR3_MR2 {

    /// PASR
    pub mod PASR {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CWL
    pub mod CWL {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ASR
    pub mod ASR {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SRT
    pub mod SRT {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RTTWR
    pub mod RTTWR {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC MR3 register for DDR3
pub mod DDRPHYC_DDR3_MR3 {

    /// MPRLOC
    pub mod MPRLOC {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u8 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MPR
    pub mod MPR {
        /// Offset (2 bits)
        pub const offset: u8 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC ODTC register
pub mod DDRPHYC_ODTCR {

    /// RDODT
    pub mod RDODT {
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

    /// WRODT
    pub mod WRODT {
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

/// DDRPHYC DTA register
pub mod DDRPHYC_DTAR {

    /// DTCOL
    pub mod DTCOL {
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

    /// DTROW
    pub mod DTROW {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (16 bits: 0xffff << 12)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DTBANK
    pub mod DTBANK {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (3 bits: 0b111 << 28)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DTMPR
    pub mod DTMPR {
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

/// DDRPHYC DTD register 0
pub mod DDRPHYC_DTDR0 {

    /// DTBYTE0
    pub mod DTBYTE0 {
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

    /// DTBYTE1
    pub mod DTBYTE1 {
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

    /// DTBYTE2
    pub mod DTBYTE2 {
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

    /// DTBYTE3
    pub mod DTBYTE3 {
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

/// DDRPHYC DTD register 1
pub mod DDRPHYC_DTDR1 {

    /// DTBYTE4
    pub mod DTBYTE4 {
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

    /// DTBYTE5
    pub mod DTBYTE5 {
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

    /// DTBYTE6
    pub mod DTBYTE6 {
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

    /// DTBYTE7
    pub mod DTBYTE7 {
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

/// DDRPHYC general purpose register 0
pub mod DDRPHYC_GPR0 {

    /// GPR0
    pub mod GPR0 {
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

/// DDRPHYC general purpose register 1
pub mod DDRPHYC_GPR1 {

    /// GPR1
    pub mod GPR1 {
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

/// DDRPHYC ZQ0C register 0
pub mod DDRPHYC_ZQ0CR0 {

    /// ZDATA
    pub mod ZDATA {
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

    /// ZDEN
    pub mod ZDEN {
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

    /// ZCALBYP
    pub mod ZCALBYP {
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

    /// ZCAL
    pub mod ZCAL {
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

    /// ZQPD
    pub mod ZQPD {
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

/// DDRPHYC ZQ0CR1 register
pub mod DDRPHYC_ZQ0CR1 {

    /// ZPROG
    pub mod ZPROG {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u8 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC ZQ0S register 0
pub mod DDRPHYC_ZQ0SR0 {

    /// ZCTRL
    pub mod ZCTRL {
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

    /// ZERR
    pub mod ZERR {
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

    /// ZDONE
    pub mod ZDONE {
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

/// DDRPHYC ZQ0S register 1
pub mod DDRPHYC_ZQ0SR1 {

    /// ZPD
    pub mod ZPD {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u8 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ZPU
    pub mod ZPU {
        /// Offset (2 bits)
        pub const offset: u8 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u8 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OPD
    pub mod OPD {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u8 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OPU
    pub mod OPU {
        /// Offset (6 bits)
        pub const offset: u8 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u8 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC byte lane 0 GC register
pub mod DDRPHYC_DX0GCR {

    /// DXEN
    pub mod DXEN {
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

    /// DQSODT
    pub mod DQSODT {
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

    /// DQODT
    pub mod DQODT {
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

    /// DXIOM
    pub mod DXIOM {
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

    /// DXPDD
    pub mod DXPDD {
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

    /// DXPDR
    pub mod DXPDR {
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

    /// DQSRPD
    pub mod DQSRPD {
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

    /// DSEN
    pub mod DSEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (2 bits: 0b11 << 7)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DQSRTT
    pub mod DQSRTT {
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

    /// DQRTT
    pub mod DQRTT {
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

    /// RTTOH
    pub mod RTTOH {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RTTOAL
    pub mod RTTOAL {
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

    /// R0RVSL
    pub mod R0RVSL {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (3 bits: 0b111 << 14)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC byte lane 0 GS register 0
pub mod DDRPHYC_DX0GSR0 {

    /// DTDONE
    pub mod DTDONE {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DTERR
    pub mod DTERR {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DTIERR
    pub mod DTIERR {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DTPASS
    pub mod DTPASS {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC byte lane 0 GS register 1
pub mod DDRPHYC_DX0GSR1 {

    /// DFTERR
    pub mod DFTERR {
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

    /// DQSDFT
    pub mod DQSDFT {
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

    /// RVERR
    pub mod RVERR {
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

    /// RVIERR
    pub mod RVIERR {
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

    /// RVPASS
    pub mod RVPASS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC byte lane 0 DLLC register
pub mod DDRPHYC_DX0DLLCR {

    /// SFBDLY
    pub mod SFBDLY {
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

    /// SFWDLY
    pub mod SFWDLY {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MFBDLY
    pub mod MFBDLY {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MFWDLY
    pub mod MFWDLY {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SSTART
    pub mod SSTART {
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

    /// SDPHASE
    pub mod SDPHASE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (4 bits: 0b1111 << 14)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ATESTEN
    pub mod ATESTEN {
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

    /// SDLBMODE
    pub mod SDLBMODE {
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

    /// DLLSRST
    pub mod DLLSRST {
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

    /// DLLDIS
    pub mod DLLDIS {
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

/// DDRPHYC byte lane 0 DQT register
pub mod DDRPHYC_DX0DQTR {

    /// DQDLY0
    pub mod DQDLY0 {
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

    /// DQDLY1
    pub mod DQDLY1 {
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

    /// DQDLY2
    pub mod DQDLY2 {
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

    /// DQDLY3
    pub mod DQDLY3 {
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

    /// DQDLY4
    pub mod DQDLY4 {
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

    /// DQDLY5
    pub mod DQDLY5 {
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

    /// DQDLY6
    pub mod DQDLY6 {
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

    /// DQDLY7
    pub mod DQDLY7 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC byte lane 0 DQST register
pub mod DDRPHYC_DX0DQSTR {

    /// R0DGSL
    pub mod R0DGSL {
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

    /// R0DGPS
    pub mod R0DGPS {
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

    /// DQSDLY
    pub mod DQSDLY {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DQSNDLY
    pub mod DQSNDLY {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (3 bits: 0b111 << 23)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMDLY
    pub mod DMDLY {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (4 bits: 0b1111 << 26)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPHYC byte lane 1 GC register
pub mod DDRPHYC_DX1GCR {
    pub use super::DDRPHYC_DX0GCR::DQODT;
    pub use super::DDRPHYC_DX0GCR::DQRTT;
    pub use super::DDRPHYC_DX0GCR::DQSODT;
    pub use super::DDRPHYC_DX0GCR::DQSRPD;
    pub use super::DDRPHYC_DX0GCR::DQSRTT;
    pub use super::DDRPHYC_DX0GCR::DSEN;
    pub use super::DDRPHYC_DX0GCR::DXEN;
    pub use super::DDRPHYC_DX0GCR::DXIOM;
    pub use super::DDRPHYC_DX0GCR::DXPDD;
    pub use super::DDRPHYC_DX0GCR::DXPDR;
    pub use super::DDRPHYC_DX0GCR::R0RVSL;
    pub use super::DDRPHYC_DX0GCR::RTTOAL;
    pub use super::DDRPHYC_DX0GCR::RTTOH;
}

/// DDRPHYC byte lane 1 GS register 0
pub mod DDRPHYC_DX1GSR0 {
    pub use super::DDRPHYC_DX0GSR0::DTDONE;
    pub use super::DDRPHYC_DX0GSR0::DTERR;
    pub use super::DDRPHYC_DX0GSR0::DTIERR;
    pub use super::DDRPHYC_DX0GSR0::DTPASS;
}

/// DDRPHYC byte lane 1 GS register 1
pub mod DDRPHYC_DX1GSR1 {
    pub use super::DDRPHYC_DX0GSR1::DFTERR;
    pub use super::DDRPHYC_DX0GSR1::DQSDFT;
    pub use super::DDRPHYC_DX0GSR1::RVERR;
    pub use super::DDRPHYC_DX0GSR1::RVIERR;
    pub use super::DDRPHYC_DX0GSR1::RVPASS;
}

/// DDRPHYC byte lane 1 DLLC register
pub mod DDRPHYC_DX1DLLCR {
    pub use super::DDRPHYC_DX0DLLCR::ATESTEN;
    pub use super::DDRPHYC_DX0DLLCR::DLLDIS;
    pub use super::DDRPHYC_DX0DLLCR::DLLSRST;
    pub use super::DDRPHYC_DX0DLLCR::MFBDLY;
    pub use super::DDRPHYC_DX0DLLCR::MFWDLY;
    pub use super::DDRPHYC_DX0DLLCR::SDLBMODE;
    pub use super::DDRPHYC_DX0DLLCR::SDPHASE;
    pub use super::DDRPHYC_DX0DLLCR::SFBDLY;
    pub use super::DDRPHYC_DX0DLLCR::SFWDLY;
    pub use super::DDRPHYC_DX0DLLCR::SSTART;
}

/// DDRPHYC byte lane 1 DQT register
pub mod DDRPHYC_DX1DQTR {
    pub use super::DDRPHYC_DX0DQTR::DQDLY0;
    pub use super::DDRPHYC_DX0DQTR::DQDLY1;
    pub use super::DDRPHYC_DX0DQTR::DQDLY2;
    pub use super::DDRPHYC_DX0DQTR::DQDLY3;
    pub use super::DDRPHYC_DX0DQTR::DQDLY4;
    pub use super::DDRPHYC_DX0DQTR::DQDLY5;
    pub use super::DDRPHYC_DX0DQTR::DQDLY6;
    pub use super::DDRPHYC_DX0DQTR::DQDLY7;
}

/// DDRPHYC byte lane 1 DQST register
pub mod DDRPHYC_DX1DQSTR {
    pub use super::DDRPHYC_DX0DQSTR::DMDLY;
    pub use super::DDRPHYC_DX0DQSTR::DQSDLY;
    pub use super::DDRPHYC_DX0DQSTR::DQSNDLY;
    pub use super::DDRPHYC_DX0DQSTR::R0DGPS;
    pub use super::DDRPHYC_DX0DQSTR::R0DGSL;
}

/// DDRPHYC byte lane 2 GC register
pub mod DDRPHYC_DX2GCR {
    pub use super::DDRPHYC_DX0GCR::DQODT;
    pub use super::DDRPHYC_DX0GCR::DQRTT;
    pub use super::DDRPHYC_DX0GCR::DQSODT;
    pub use super::DDRPHYC_DX0GCR::DQSRPD;
    pub use super::DDRPHYC_DX0GCR::DQSRTT;
    pub use super::DDRPHYC_DX0GCR::DSEN;
    pub use super::DDRPHYC_DX0GCR::DXEN;
    pub use super::DDRPHYC_DX0GCR::DXIOM;
    pub use super::DDRPHYC_DX0GCR::DXPDD;
    pub use super::DDRPHYC_DX0GCR::DXPDR;
    pub use super::DDRPHYC_DX0GCR::R0RVSL;
    pub use super::DDRPHYC_DX0GCR::RTTOAL;
    pub use super::DDRPHYC_DX0GCR::RTTOH;
}

/// DDRPHYC byte lane 2 GS register 0
pub mod DDRPHYC_DX2GSR0 {
    pub use super::DDRPHYC_DX0GSR0::DTDONE;
    pub use super::DDRPHYC_DX0GSR0::DTERR;
    pub use super::DDRPHYC_DX0GSR0::DTIERR;
    pub use super::DDRPHYC_DX0GSR0::DTPASS;
}

/// DDRPHYC byte lane 2 GS register 1
pub mod DDRPHYC_DX2GSR1 {
    pub use super::DDRPHYC_DX0GSR1::DFTERR;
    pub use super::DDRPHYC_DX0GSR1::DQSDFT;
    pub use super::DDRPHYC_DX0GSR1::RVERR;
    pub use super::DDRPHYC_DX0GSR1::RVIERR;
    pub use super::DDRPHYC_DX0GSR1::RVPASS;
}

/// DDRPHYC byte lane 2 DLLC register
pub mod DDRPHYC_DX2DLLCR {
    pub use super::DDRPHYC_DX0DLLCR::ATESTEN;
    pub use super::DDRPHYC_DX0DLLCR::DLLDIS;
    pub use super::DDRPHYC_DX0DLLCR::DLLSRST;
    pub use super::DDRPHYC_DX0DLLCR::MFBDLY;
    pub use super::DDRPHYC_DX0DLLCR::MFWDLY;
    pub use super::DDRPHYC_DX0DLLCR::SDLBMODE;
    pub use super::DDRPHYC_DX0DLLCR::SDPHASE;
    pub use super::DDRPHYC_DX0DLLCR::SFBDLY;
    pub use super::DDRPHYC_DX0DLLCR::SFWDLY;
    pub use super::DDRPHYC_DX0DLLCR::SSTART;
}

/// DDRPHYC byte lane 2 DQT register
pub mod DDRPHYC_DX2DQTR {
    pub use super::DDRPHYC_DX0DQTR::DQDLY0;
    pub use super::DDRPHYC_DX0DQTR::DQDLY1;
    pub use super::DDRPHYC_DX0DQTR::DQDLY2;
    pub use super::DDRPHYC_DX0DQTR::DQDLY3;
    pub use super::DDRPHYC_DX0DQTR::DQDLY4;
    pub use super::DDRPHYC_DX0DQTR::DQDLY5;
    pub use super::DDRPHYC_DX0DQTR::DQDLY6;
    pub use super::DDRPHYC_DX0DQTR::DQDLY7;
}

/// DDRPHYC byte lane 2 DQST register
pub mod DDRPHYC_DX2DQSTR {
    pub use super::DDRPHYC_DX0DQSTR::DMDLY;
    pub use super::DDRPHYC_DX0DQSTR::DQSDLY;
    pub use super::DDRPHYC_DX0DQSTR::DQSNDLY;
    pub use super::DDRPHYC_DX0DQSTR::R0DGPS;
    pub use super::DDRPHYC_DX0DQSTR::R0DGSL;
}

/// DDRPHYC byte lane 3 GC register
pub mod DDRPHYC_DX3GCR {
    pub use super::DDRPHYC_DX0GCR::DQODT;
    pub use super::DDRPHYC_DX0GCR::DQRTT;
    pub use super::DDRPHYC_DX0GCR::DQSODT;
    pub use super::DDRPHYC_DX0GCR::DQSRPD;
    pub use super::DDRPHYC_DX0GCR::DQSRTT;
    pub use super::DDRPHYC_DX0GCR::DSEN;
    pub use super::DDRPHYC_DX0GCR::DXEN;
    pub use super::DDRPHYC_DX0GCR::DXIOM;
    pub use super::DDRPHYC_DX0GCR::DXPDD;
    pub use super::DDRPHYC_DX0GCR::DXPDR;
    pub use super::DDRPHYC_DX0GCR::R0RVSL;
    pub use super::DDRPHYC_DX0GCR::RTTOAL;
    pub use super::DDRPHYC_DX0GCR::RTTOH;
}

/// DDRPHYC byte lane 3 GS register 0
pub mod DDRPHYC_DX3GSR0 {
    pub use super::DDRPHYC_DX0GSR0::DTDONE;
    pub use super::DDRPHYC_DX0GSR0::DTERR;
    pub use super::DDRPHYC_DX0GSR0::DTIERR;
    pub use super::DDRPHYC_DX0GSR0::DTPASS;
}

/// DDRPHYC byte lane 3 GS register 1
pub mod DDRPHYC_DX3GSR1 {
    pub use super::DDRPHYC_DX0GSR1::DFTERR;
    pub use super::DDRPHYC_DX0GSR1::DQSDFT;
    pub use super::DDRPHYC_DX0GSR1::RVERR;
    pub use super::DDRPHYC_DX0GSR1::RVIERR;
    pub use super::DDRPHYC_DX0GSR1::RVPASS;
}

/// DDRPHYC byte lane 3 DLLC register
pub mod DDRPHYC_DX3DLLCR {
    pub use super::DDRPHYC_DX0DLLCR::ATESTEN;
    pub use super::DDRPHYC_DX0DLLCR::DLLDIS;
    pub use super::DDRPHYC_DX0DLLCR::DLLSRST;
    pub use super::DDRPHYC_DX0DLLCR::MFBDLY;
    pub use super::DDRPHYC_DX0DLLCR::MFWDLY;
    pub use super::DDRPHYC_DX0DLLCR::SDLBMODE;
    pub use super::DDRPHYC_DX0DLLCR::SDPHASE;
    pub use super::DDRPHYC_DX0DLLCR::SFBDLY;
    pub use super::DDRPHYC_DX0DLLCR::SFWDLY;
    pub use super::DDRPHYC_DX0DLLCR::SSTART;
}

/// DDRPHYC byte lane 3 DQT register
pub mod DDRPHYC_DX3DQTR {
    pub use super::DDRPHYC_DX0DQTR::DQDLY0;
    pub use super::DDRPHYC_DX0DQTR::DQDLY1;
    pub use super::DDRPHYC_DX0DQTR::DQDLY2;
    pub use super::DDRPHYC_DX0DQTR::DQDLY3;
    pub use super::DDRPHYC_DX0DQTR::DQDLY4;
    pub use super::DDRPHYC_DX0DQTR::DQDLY5;
    pub use super::DDRPHYC_DX0DQTR::DQDLY6;
    pub use super::DDRPHYC_DX0DQTR::DQDLY7;
}

/// DDRPHYC byte lane 3 DQST register
pub mod DDRPHYC_DX3DQSTR {
    pub use super::DDRPHYC_DX0DQSTR::DMDLY;
    pub use super::DDRPHYC_DX0DQSTR::DQSDLY;
    pub use super::DDRPHYC_DX0DQSTR::DQSNDLY;
    pub use super::DDRPHYC_DX0DQSTR::R0DGPS;
    pub use super::DDRPHYC_DX0DQSTR::R0DGSL;
}
#[repr(C)]
pub struct RegisterBlock {
    /// DDRPHYC revision ID register
    pub DDRPHYC_RIDR: RORegister<u32>,

    /// DDRPHYC PHY initialization register
    pub DDRPHYC_PIR: WORegister<u32>,

    /// DDRPHYC PHY global control register
    pub DDRPHYC_PGCR: RWRegister<u32>,

    /// DDRPHYC PHY global status register
    pub DDRPHYC_PGSR: RORegister<u32>,

    /// DDRPHYC DDR global control register
    pub DDRPHYC_DLLGCR: RWRegister<u32>,

    /// DDRPHYC AC DLL control register
    pub DDRPHYC_ACDLLCR: RWRegister<u32>,

    /// DDRPHYC PT register 0
    pub DDRPHYC_PTR0: RWRegister<u32>,

    /// DDRPHYC PT register 1
    pub DDRPHYC_PTR1: RWRegister<u32>,

    /// DDRPHYC PT register 2
    pub DDRPHYC_PTR2: RWRegister<u32>,

    /// DDRPHYC ACIOC register
    pub DDRPHYC_ACIOCR: RWRegister<u32>,

    /// DDRPHYC DXCC register
    pub DDRPHYC_DXCCR: RWRegister<u32>,

    /// DDRPHYC DSGC register
    pub DDRPHYC_DSGCR: RWRegister<u32>,

    /// DDRPHYC DC register
    pub DDRPHYC_DCR: RWRegister<u32>,

    /// DDRPHYC DTP register 0
    pub DDRPHYC_DTPR0: RWRegister<u32>,

    /// DDRPHYC DTP register 1
    pub DDRPHYC_DTPR1: RWRegister<u32>,

    /// DDRPHYC DTP register 2
    pub DDRPHYC_DTPR2: RWRegister<u32>,

    /// DDRPHYC MR0 register for DDR3
    pub DDRPHYC_DDR3_MR0: RWRegister<u16>,

    _reserved1: [u16; 1],

    /// DDRPHYC MR1 register for DDR3
    pub DDRPHYC_DDR3_MR1: RWRegister<u16>,

    _reserved2: [u16; 1],

    /// DDRPHYC MR2 register for DDR3
    pub DDRPHYC_DDR3_MR2: RWRegister<u16>,

    _reserved3: [u16; 1],

    /// DDRPHYC MR3 register for DDR3
    pub DDRPHYC_DDR3_MR3: RWRegister<u8>,

    _reserved4: [u16; 1],
    _reserved5: [u8; 1],

    /// DDRPHYC ODTC register
    pub DDRPHYC_ODTCR: RWRegister<u32>,

    /// DDRPHYC DTA register
    pub DDRPHYC_DTAR: RWRegister<u32>,

    /// DDRPHYC DTD register 0
    pub DDRPHYC_DTDR0: RWRegister<u32>,

    /// DDRPHYC DTD register 1
    pub DDRPHYC_DTDR1: RWRegister<u32>,

    _reserved6: [u32; 70],

    /// DDRPHYC general purpose register 0
    pub DDRPHYC_GPR0: RWRegister<u32>,

    /// DDRPHYC general purpose register 1
    pub DDRPHYC_GPR1: RWRegister<u32>,

    /// DDRPHYC ZQ0C register 0
    pub DDRPHYC_ZQ0CR0: RWRegister<u32>,

    /// DDRPHYC ZQ0CR1 register
    pub DDRPHYC_ZQ0CR1: RWRegister<u8>,

    _reserved7: [u16; 1],
    _reserved8: [u8; 1],

    /// DDRPHYC ZQ0S register 0
    pub DDRPHYC_ZQ0SR0: RORegister<u32>,

    /// DDRPHYC ZQ0S register 1
    pub DDRPHYC_ZQ0SR1: RORegister<u8>,

    _reserved9: [u32; 12],
    _reserved10: [u16; 1],
    _reserved11: [u8; 1],

    /// DDRPHYC byte lane 0 GC register
    pub DDRPHYC_DX0GCR: RWRegister<u32>,

    /// DDRPHYC byte lane 0 GS register 0
    pub DDRPHYC_DX0GSR0: RORegister<u16>,

    _reserved12: [u16; 1],

    /// DDRPHYC byte lane 0 GS register 1
    pub DDRPHYC_DX0GSR1: RORegister<u32>,

    /// DDRPHYC byte lane 0 DLLC register
    pub DDRPHYC_DX0DLLCR: RWRegister<u32>,

    /// DDRPHYC byte lane 0 DQT register
    pub DDRPHYC_DX0DQTR: RWRegister<u32>,

    /// DDRPHYC byte lane 0 DQST register
    pub DDRPHYC_DX0DQSTR: RWRegister<u32>,

    _reserved13: [u32; 10],

    /// DDRPHYC byte lane 1 GC register
    pub DDRPHYC_DX1GCR: RWRegister<u32>,

    /// DDRPHYC byte lane 1 GS register 0
    pub DDRPHYC_DX1GSR0: RORegister<u16>,

    _reserved14: [u16; 1],

    /// DDRPHYC byte lane 1 GS register 1
    pub DDRPHYC_DX1GSR1: RORegister<u32>,

    /// DDRPHYC byte lane 1 DLLC register
    pub DDRPHYC_DX1DLLCR: RWRegister<u32>,

    /// DDRPHYC byte lane 1 DQT register
    pub DDRPHYC_DX1DQTR: RWRegister<u32>,

    /// DDRPHYC byte lane 1 DQST register
    pub DDRPHYC_DX1DQSTR: RWRegister<u32>,

    _reserved15: [u32; 10],

    /// DDRPHYC byte lane 2 GC register
    pub DDRPHYC_DX2GCR: RWRegister<u32>,

    /// DDRPHYC byte lane 2 GS register 0
    pub DDRPHYC_DX2GSR0: RORegister<u16>,

    _reserved16: [u16; 1],

    /// DDRPHYC byte lane 2 GS register 1
    pub DDRPHYC_DX2GSR1: RORegister<u32>,

    /// DDRPHYC byte lane 2 DLLC register
    pub DDRPHYC_DX2DLLCR: RWRegister<u32>,

    /// DDRPHYC byte lane 2 DQT register
    pub DDRPHYC_DX2DQTR: RWRegister<u32>,

    /// DDRPHYC byte lane 2 DQST register
    pub DDRPHYC_DX2DQSTR: RWRegister<u32>,

    _reserved17: [u32; 10],

    /// DDRPHYC byte lane 3 GC register
    pub DDRPHYC_DX3GCR: RWRegister<u32>,

    /// DDRPHYC byte lane 3 GS register 0
    pub DDRPHYC_DX3GSR0: RORegister<u16>,

    _reserved18: [u16; 1],

    /// DDRPHYC byte lane 3 GS register 1
    pub DDRPHYC_DX3GSR1: RORegister<u32>,

    /// DDRPHYC byte lane 3 DLLC register
    pub DDRPHYC_DX3DLLCR: RWRegister<u32>,

    /// DDRPHYC byte lane 3 DQT register
    pub DDRPHYC_DX3DQTR: RWRegister<u32>,

    /// DDRPHYC byte lane 3 DQST register
    pub DDRPHYC_DX3DQSTR: RWRegister<u32>,
}
pub struct ResetValues {
    pub DDRPHYC_RIDR: u32,
    pub DDRPHYC_PIR: u32,
    pub DDRPHYC_PGCR: u32,
    pub DDRPHYC_PGSR: u32,
    pub DDRPHYC_DLLGCR: u32,
    pub DDRPHYC_ACDLLCR: u32,
    pub DDRPHYC_PTR0: u32,
    pub DDRPHYC_PTR1: u32,
    pub DDRPHYC_PTR2: u32,
    pub DDRPHYC_ACIOCR: u32,
    pub DDRPHYC_DXCCR: u32,
    pub DDRPHYC_DSGCR: u32,
    pub DDRPHYC_DCR: u32,
    pub DDRPHYC_DTPR0: u32,
    pub DDRPHYC_DTPR1: u32,
    pub DDRPHYC_DTPR2: u32,
    pub DDRPHYC_DDR3_MR0: u16,
    pub DDRPHYC_DDR3_MR1: u16,
    pub DDRPHYC_DDR3_MR2: u16,
    pub DDRPHYC_DDR3_MR3: u8,
    pub DDRPHYC_ODTCR: u32,
    pub DDRPHYC_DTAR: u32,
    pub DDRPHYC_DTDR0: u32,
    pub DDRPHYC_DTDR1: u32,
    pub DDRPHYC_GPR0: u32,
    pub DDRPHYC_GPR1: u32,
    pub DDRPHYC_ZQ0CR0: u32,
    pub DDRPHYC_ZQ0CR1: u8,
    pub DDRPHYC_ZQ0SR0: u32,
    pub DDRPHYC_ZQ0SR1: u8,
    pub DDRPHYC_DX0GCR: u32,
    pub DDRPHYC_DX0GSR0: u16,
    pub DDRPHYC_DX0GSR1: u32,
    pub DDRPHYC_DX0DLLCR: u32,
    pub DDRPHYC_DX0DQTR: u32,
    pub DDRPHYC_DX0DQSTR: u32,
    pub DDRPHYC_DX1GCR: u32,
    pub DDRPHYC_DX1GSR0: u16,
    pub DDRPHYC_DX1GSR1: u32,
    pub DDRPHYC_DX1DLLCR: u32,
    pub DDRPHYC_DX1DQTR: u32,
    pub DDRPHYC_DX1DQSTR: u32,
    pub DDRPHYC_DX2GCR: u32,
    pub DDRPHYC_DX2GSR0: u16,
    pub DDRPHYC_DX2GSR1: u32,
    pub DDRPHYC_DX2DLLCR: u32,
    pub DDRPHYC_DX2DQTR: u32,
    pub DDRPHYC_DX2DQSTR: u32,
    pub DDRPHYC_DX3GCR: u32,
    pub DDRPHYC_DX3GSR0: u16,
    pub DDRPHYC_DX3GSR1: u32,
    pub DDRPHYC_DX3DLLCR: u32,
    pub DDRPHYC_DX3DQTR: u32,
    pub DDRPHYC_DX3DQSTR: u32,
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
