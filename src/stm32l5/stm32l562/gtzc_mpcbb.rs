#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GTZC_MPCBB1

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// MPCBB control register
pub mod CR {

    /// LCK
    pub mod LCK {
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

    /// INVSECSTATE
    pub mod INVSECSTATE {
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

    /// SRWILADIS
    pub mod SRWILADIS {
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

/// MPCBB control register
pub mod LCKVTR1 {

    /// LCKSB0
    pub mod LCKSB0 {
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

    /// LCKSB1
    pub mod LCKSB1 {
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

    /// LCKSB2
    pub mod LCKSB2 {
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

    /// LCKSB3
    pub mod LCKSB3 {
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

    /// LCKSB4
    pub mod LCKSB4 {
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

    /// LCKSB5
    pub mod LCKSB5 {
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

    /// LCKSB6
    pub mod LCKSB6 {
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

    /// LCKSB7
    pub mod LCKSB7 {
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

    /// LCKSB8
    pub mod LCKSB8 {
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

    /// LCKSB9
    pub mod LCKSB9 {
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

    /// LCKSB10
    pub mod LCKSB10 {
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

    /// LCKSB11
    pub mod LCKSB11 {
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

    /// LCKSB12
    pub mod LCKSB12 {
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

    /// LCKSB13
    pub mod LCKSB13 {
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

    /// LCKSB14
    pub mod LCKSB14 {
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

    /// LCKSB15
    pub mod LCKSB15 {
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

    /// LCKSB16
    pub mod LCKSB16 {
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

    /// LCKSB17
    pub mod LCKSB17 {
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

    /// LCKSB18
    pub mod LCKSB18 {
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

    /// LCKSB19
    pub mod LCKSB19 {
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

    /// LCKSB20
    pub mod LCKSB20 {
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

    /// LCKSB21
    pub mod LCKSB21 {
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

    /// LCKSB22
    pub mod LCKSB22 {
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

    /// LCKSB23
    pub mod LCKSB23 {
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

    /// LCKSB24
    pub mod LCKSB24 {
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

    /// LCKSB25
    pub mod LCKSB25 {
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

    /// LCKSB26
    pub mod LCKSB26 {
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

    /// LCKSB27
    pub mod LCKSB27 {
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

    /// LCKSB28
    pub mod LCKSB28 {
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

    /// LCKSB29
    pub mod LCKSB29 {
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

    /// LCKSB30
    pub mod LCKSB30 {
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

    /// LCKSB31
    pub mod LCKSB31 {
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

/// MPCBB control register
pub mod LCKVTR2 {

    /// LCKSB32
    pub mod LCKSB32 {
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

    /// LCKSB33
    pub mod LCKSB33 {
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

    /// LCKSB34
    pub mod LCKSB34 {
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

    /// LCKSB35
    pub mod LCKSB35 {
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

    /// LCKSB36
    pub mod LCKSB36 {
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

    /// LCKSB37
    pub mod LCKSB37 {
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

    /// LCKSB38
    pub mod LCKSB38 {
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

    /// LCKSB39
    pub mod LCKSB39 {
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

    /// LCKSB40
    pub mod LCKSB40 {
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

    /// LCKSB41
    pub mod LCKSB41 {
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

    /// LCKSB42
    pub mod LCKSB42 {
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

    /// LCKSB43
    pub mod LCKSB43 {
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

    /// LCKSB44
    pub mod LCKSB44 {
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

    /// LCKSB45
    pub mod LCKSB45 {
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

    /// LCKSB46
    pub mod LCKSB46 {
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

    /// LCKSB47
    pub mod LCKSB47 {
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

    /// LCKSB48
    pub mod LCKSB48 {
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

    /// LCKSB49
    pub mod LCKSB49 {
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

    /// LCKSB50
    pub mod LCKSB50 {
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

    /// LCKSB51
    pub mod LCKSB51 {
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

    /// LCKSB52
    pub mod LCKSB52 {
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

    /// LCKSB53
    pub mod LCKSB53 {
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

    /// LCKSB54
    pub mod LCKSB54 {
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

    /// LCKSB55
    pub mod LCKSB55 {
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

    /// LCKSB56
    pub mod LCKSB56 {
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

    /// LCKSB57
    pub mod LCKSB57 {
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

    /// LCKSB58
    pub mod LCKSB58 {
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

    /// LCKSB59
    pub mod LCKSB59 {
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

    /// LCKSB60
    pub mod LCKSB60 {
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

    /// LCKSB61
    pub mod LCKSB61 {
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

    /// LCKSB62
    pub mod LCKSB62 {
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

    /// LCKSB63
    pub mod LCKSB63 {
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

/// MPCBBx vector register
pub mod VCTR0 {

    /// B0
    pub mod B0 {
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

    /// B1
    pub mod B1 {
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

    /// B2
    pub mod B2 {
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

    /// B3
    pub mod B3 {
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

    /// B4
    pub mod B4 {
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

    /// B5
    pub mod B5 {
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

    /// B6
    pub mod B6 {
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

    /// B7
    pub mod B7 {
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

    /// B8
    pub mod B8 {
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

    /// B9
    pub mod B9 {
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

    /// B10
    pub mod B10 {
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

    /// B11
    pub mod B11 {
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

    /// B12
    pub mod B12 {
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

    /// B13
    pub mod B13 {
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

    /// B14
    pub mod B14 {
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

    /// B15
    pub mod B15 {
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

    /// B16
    pub mod B16 {
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

    /// B17
    pub mod B17 {
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

    /// B18
    pub mod B18 {
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

    /// B19
    pub mod B19 {
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

    /// B20
    pub mod B20 {
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

    /// B21
    pub mod B21 {
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

    /// B22
    pub mod B22 {
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

    /// B23
    pub mod B23 {
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

    /// B24
    pub mod B24 {
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

    /// B25
    pub mod B25 {
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

    /// B26
    pub mod B26 {
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

    /// B27
    pub mod B27 {
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

    /// B28
    pub mod B28 {
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

    /// B29
    pub mod B29 {
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

    /// B30
    pub mod B30 {
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

    /// B31
    pub mod B31 {
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

/// MPCBBx vector register
pub mod VCTR1 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR2 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR3 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR4 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR5 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR6 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR7 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR8 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR9 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR10 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR11 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR12 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR13 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR14 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR15 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR16 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR17 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR18 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR19 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR20 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR21 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR22 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR23 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR24 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR25 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR26 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR27 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR28 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR29 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR30 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR31 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR32 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR33 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR34 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR35 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR36 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR37 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR38 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR39 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR40 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR41 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR42 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR43 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR44 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR45 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR46 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR47 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR48 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR49 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR50 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR51 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR52 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR53 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR54 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR55 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR56 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR57 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR58 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR59 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR60 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR61 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR62 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}

/// MPCBBx vector register
pub mod VCTR63 {
    pub use super::VCTR0::B0;
    pub use super::VCTR0::B1;
    pub use super::VCTR0::B10;
    pub use super::VCTR0::B11;
    pub use super::VCTR0::B12;
    pub use super::VCTR0::B13;
    pub use super::VCTR0::B14;
    pub use super::VCTR0::B15;
    pub use super::VCTR0::B16;
    pub use super::VCTR0::B17;
    pub use super::VCTR0::B18;
    pub use super::VCTR0::B19;
    pub use super::VCTR0::B2;
    pub use super::VCTR0::B20;
    pub use super::VCTR0::B21;
    pub use super::VCTR0::B22;
    pub use super::VCTR0::B23;
    pub use super::VCTR0::B24;
    pub use super::VCTR0::B25;
    pub use super::VCTR0::B26;
    pub use super::VCTR0::B27;
    pub use super::VCTR0::B28;
    pub use super::VCTR0::B29;
    pub use super::VCTR0::B3;
    pub use super::VCTR0::B30;
    pub use super::VCTR0::B31;
    pub use super::VCTR0::B4;
    pub use super::VCTR0::B5;
    pub use super::VCTR0::B6;
    pub use super::VCTR0::B7;
    pub use super::VCTR0::B8;
    pub use super::VCTR0::B9;
}
#[repr(C)]
pub struct RegisterBlock {
    /// MPCBB control register
    pub CR: RWRegister<u32>,

    _reserved1: [u8; 12],

    /// MPCBB control register
    pub LCKVTR1: RWRegister<u32>,

    /// MPCBB control register
    pub LCKVTR2: RWRegister<u32>,

    _reserved2: [u8; 232],

    /// MPCBBx vector register
    pub VCTR0: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR1: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR2: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR3: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR4: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR5: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR6: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR7: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR8: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR9: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR10: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR11: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR12: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR13: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR14: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR15: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR16: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR17: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR18: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR19: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR20: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR21: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR22: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR23: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR24: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR25: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR26: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR27: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR28: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR29: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR30: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR31: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR32: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR33: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR34: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR35: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR36: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR37: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR38: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR39: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR40: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR41: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR42: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR43: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR44: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR45: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR46: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR47: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR48: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR49: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR50: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR51: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR52: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR53: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR54: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR55: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR56: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR57: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR58: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR59: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR60: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR61: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR62: RWRegister<u32>,

    /// MPCBBx vector register
    pub VCTR63: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub LCKVTR1: u32,
    pub LCKVTR2: u32,
    pub VCTR0: u32,
    pub VCTR1: u32,
    pub VCTR2: u32,
    pub VCTR3: u32,
    pub VCTR4: u32,
    pub VCTR5: u32,
    pub VCTR6: u32,
    pub VCTR7: u32,
    pub VCTR8: u32,
    pub VCTR9: u32,
    pub VCTR10: u32,
    pub VCTR11: u32,
    pub VCTR12: u32,
    pub VCTR13: u32,
    pub VCTR14: u32,
    pub VCTR15: u32,
    pub VCTR16: u32,
    pub VCTR17: u32,
    pub VCTR18: u32,
    pub VCTR19: u32,
    pub VCTR20: u32,
    pub VCTR21: u32,
    pub VCTR22: u32,
    pub VCTR23: u32,
    pub VCTR24: u32,
    pub VCTR25: u32,
    pub VCTR26: u32,
    pub VCTR27: u32,
    pub VCTR28: u32,
    pub VCTR29: u32,
    pub VCTR30: u32,
    pub VCTR31: u32,
    pub VCTR32: u32,
    pub VCTR33: u32,
    pub VCTR34: u32,
    pub VCTR35: u32,
    pub VCTR36: u32,
    pub VCTR37: u32,
    pub VCTR38: u32,
    pub VCTR39: u32,
    pub VCTR40: u32,
    pub VCTR41: u32,
    pub VCTR42: u32,
    pub VCTR43: u32,
    pub VCTR44: u32,
    pub VCTR45: u32,
    pub VCTR46: u32,
    pub VCTR47: u32,
    pub VCTR48: u32,
    pub VCTR49: u32,
    pub VCTR50: u32,
    pub VCTR51: u32,
    pub VCTR52: u32,
    pub VCTR53: u32,
    pub VCTR54: u32,
    pub VCTR55: u32,
    pub VCTR56: u32,
    pub VCTR57: u32,
    pub VCTR58: u32,
    pub VCTR59: u32,
    pub VCTR60: u32,
    pub VCTR61: u32,
    pub VCTR62: u32,
    pub VCTR63: u32,
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

/// Access functions for the GTZC_MPCBB1 peripheral instance
pub mod GTZC_MPCBB1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40032c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GTZC_MPCBB1
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        LCKVTR1: 0x00000000,
        LCKVTR2: 0x00000000,
        VCTR0: 0xFFFFFFFF,
        VCTR1: 0xFFFFFFFF,
        VCTR2: 0xFFFFFFFF,
        VCTR3: 0xFFFFFFFF,
        VCTR4: 0xFFFFFFFF,
        VCTR5: 0xFFFFFFFF,
        VCTR6: 0xFFFFFFFF,
        VCTR7: 0xFFFFFFFF,
        VCTR8: 0xFFFFFFFF,
        VCTR9: 0xFFFFFFFF,
        VCTR10: 0xFFFFFFFF,
        VCTR11: 0xFFFFFFFF,
        VCTR12: 0xFFFFFFFF,
        VCTR13: 0xFFFFFFFF,
        VCTR14: 0xFFFFFFFF,
        VCTR15: 0xFFFFFFFF,
        VCTR16: 0xFFFFFFFF,
        VCTR17: 0xFFFFFFFF,
        VCTR18: 0xFFFFFFFF,
        VCTR19: 0xFFFFFFFF,
        VCTR20: 0xFFFFFFFF,
        VCTR21: 0xFFFFFFFF,
        VCTR22: 0xFFFFFFFF,
        VCTR23: 0xFFFFFFFF,
        VCTR24: 0xFFFFFFFF,
        VCTR25: 0xFFFFFFFF,
        VCTR26: 0xFFFFFFFF,
        VCTR27: 0xFFFFFFFF,
        VCTR28: 0xFFFFFFFF,
        VCTR29: 0xFFFFFFFF,
        VCTR30: 0xFFFFFFFF,
        VCTR31: 0xFFFFFFFF,
        VCTR32: 0xFFFFFFFF,
        VCTR33: 0xFFFFFFFF,
        VCTR34: 0xFFFFFFFF,
        VCTR35: 0xFFFFFFFF,
        VCTR36: 0xFFFFFFFF,
        VCTR37: 0xFFFFFFFF,
        VCTR38: 0xFFFFFFFF,
        VCTR39: 0xFFFFFFFF,
        VCTR40: 0xFFFFFFFF,
        VCTR41: 0xFFFFFFFF,
        VCTR42: 0xFFFFFFFF,
        VCTR43: 0xFFFFFFFF,
        VCTR44: 0xFFFFFFFF,
        VCTR45: 0xFFFFFFFF,
        VCTR46: 0xFFFFFFFF,
        VCTR47: 0xFFFFFFFF,
        VCTR48: 0xFFFFFFFF,
        VCTR49: 0xFFFFFFFF,
        VCTR50: 0xFFFFFFFF,
        VCTR51: 0xFFFFFFFF,
        VCTR52: 0xFFFFFFFF,
        VCTR53: 0xFFFFFFFF,
        VCTR54: 0xFFFFFFFF,
        VCTR55: 0xFFFFFFFF,
        VCTR56: 0xFFFFFFFF,
        VCTR57: 0xFFFFFFFF,
        VCTR58: 0xFFFFFFFF,
        VCTR59: 0xFFFFFFFF,
        VCTR60: 0xFFFFFFFF,
        VCTR61: 0xFFFFFFFF,
        VCTR62: 0xFFFFFFFF,
        VCTR63: 0xFFFFFFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GTZC_MPCBB1_TAKEN: bool = false;

    /// Safe access to GTZC_MPCBB1
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
            if GTZC_MPCBB1_TAKEN {
                None
            } else {
                GTZC_MPCBB1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GTZC_MPCBB1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GTZC_MPCBB1_TAKEN && inst.addr == INSTANCE.addr {
                GTZC_MPCBB1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GTZC_MPCBB1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GTZC_MPCBB1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GTZC_MPCBB1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GTZC_MPCBB1: *const RegisterBlock = 0x40032c00 as *const _;

/// Access functions for the GTZC_MPCBB2 peripheral instance
pub mod GTZC_MPCBB2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40033000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GTZC_MPCBB2
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        LCKVTR1: 0x00000000,
        LCKVTR2: 0x00000000,
        VCTR0: 0xFFFFFFFF,
        VCTR1: 0xFFFFFFFF,
        VCTR2: 0xFFFFFFFF,
        VCTR3: 0xFFFFFFFF,
        VCTR4: 0xFFFFFFFF,
        VCTR5: 0xFFFFFFFF,
        VCTR6: 0xFFFFFFFF,
        VCTR7: 0xFFFFFFFF,
        VCTR8: 0xFFFFFFFF,
        VCTR9: 0xFFFFFFFF,
        VCTR10: 0xFFFFFFFF,
        VCTR11: 0xFFFFFFFF,
        VCTR12: 0xFFFFFFFF,
        VCTR13: 0xFFFFFFFF,
        VCTR14: 0xFFFFFFFF,
        VCTR15: 0xFFFFFFFF,
        VCTR16: 0xFFFFFFFF,
        VCTR17: 0xFFFFFFFF,
        VCTR18: 0xFFFFFFFF,
        VCTR19: 0xFFFFFFFF,
        VCTR20: 0xFFFFFFFF,
        VCTR21: 0xFFFFFFFF,
        VCTR22: 0xFFFFFFFF,
        VCTR23: 0xFFFFFFFF,
        VCTR24: 0xFFFFFFFF,
        VCTR25: 0xFFFFFFFF,
        VCTR26: 0xFFFFFFFF,
        VCTR27: 0xFFFFFFFF,
        VCTR28: 0xFFFFFFFF,
        VCTR29: 0xFFFFFFFF,
        VCTR30: 0xFFFFFFFF,
        VCTR31: 0xFFFFFFFF,
        VCTR32: 0xFFFFFFFF,
        VCTR33: 0xFFFFFFFF,
        VCTR34: 0xFFFFFFFF,
        VCTR35: 0xFFFFFFFF,
        VCTR36: 0xFFFFFFFF,
        VCTR37: 0xFFFFFFFF,
        VCTR38: 0xFFFFFFFF,
        VCTR39: 0xFFFFFFFF,
        VCTR40: 0xFFFFFFFF,
        VCTR41: 0xFFFFFFFF,
        VCTR42: 0xFFFFFFFF,
        VCTR43: 0xFFFFFFFF,
        VCTR44: 0xFFFFFFFF,
        VCTR45: 0xFFFFFFFF,
        VCTR46: 0xFFFFFFFF,
        VCTR47: 0xFFFFFFFF,
        VCTR48: 0xFFFFFFFF,
        VCTR49: 0xFFFFFFFF,
        VCTR50: 0xFFFFFFFF,
        VCTR51: 0xFFFFFFFF,
        VCTR52: 0xFFFFFFFF,
        VCTR53: 0xFFFFFFFF,
        VCTR54: 0xFFFFFFFF,
        VCTR55: 0xFFFFFFFF,
        VCTR56: 0xFFFFFFFF,
        VCTR57: 0xFFFFFFFF,
        VCTR58: 0xFFFFFFFF,
        VCTR59: 0xFFFFFFFF,
        VCTR60: 0xFFFFFFFF,
        VCTR61: 0xFFFFFFFF,
        VCTR62: 0xFFFFFFFF,
        VCTR63: 0xFFFFFFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GTZC_MPCBB2_TAKEN: bool = false;

    /// Safe access to GTZC_MPCBB2
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
            if GTZC_MPCBB2_TAKEN {
                None
            } else {
                GTZC_MPCBB2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GTZC_MPCBB2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GTZC_MPCBB2_TAKEN && inst.addr == INSTANCE.addr {
                GTZC_MPCBB2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GTZC_MPCBB2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GTZC_MPCBB2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GTZC_MPCBB2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GTZC_MPCBB2: *const RegisterBlock = 0x40033000 as *const _;
