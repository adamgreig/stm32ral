#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Ethernet: MAC management counters
//!
//! Used by: stm32f101, stm32f103, stm32f107

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister};

/// Ethernet MMC control register (ETH_MMCCR)
pub mod MMCCR {

    /// Counter reset
    pub mod CR {
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

    /// Counter stop rollover
    pub mod CSR {
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

    /// Reset on read
    pub mod ROR {
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

    /// MMC counter freeze
    pub mod MCF {
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

/// Ethernet MMC receive interrupt register (ETH_MMCRIR)
pub mod MMCRIR {

    /// Received frames CRC error status
    pub mod RFCES {
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

    /// Received frames alignment error status
    pub mod RFAES {
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

    /// Received Good Unicast Frames Status
    pub mod RGUFS {
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
}

/// Ethernet MMC transmit interrupt register (ETH_MMCTIR)
pub mod MMCTIR {

    /// Transmitted good frames single collision status
    pub mod TGFSCS {
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

    /// Transmitted good frames more single collision status
    pub mod TGFMSCS {
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

    /// Transmitted good frames status
    pub mod TGFS {
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
}

/// Ethernet MMC receive interrupt mask register (ETH_MMCRIMR)
pub mod MMCRIMR {

    /// Received frame CRC error mask
    pub mod RFCEM {
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

    /// Received frames alignment error mask
    pub mod RFAEM {
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

    /// Received good unicast frames mask
    pub mod RGUFM {
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
}

/// Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR)
pub mod MMCTIMR {

    /// Transmitted good frames single collision mask
    pub mod TGFSCM {
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

    /// Transmitted good frames more single collision mask
    pub mod TGFMSCM {
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

    /// Transmitted good frames mask
    pub mod TGFM {
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
}

/// Ethernet MMC transmitted good frames after a single collision counter
pub mod MMCTGFSCCR {

    /// Transmitted good frames after a single collision counter
    pub mod TGFSCC {
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

/// Ethernet MMC transmitted good frames after more than a single collision
pub mod MMCTGFMSCCR {

    /// Transmitted good frames after more than a single collision counter
    pub mod TGFMSCC {
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

/// Ethernet MMC transmitted good frames counter register
pub mod MMCTGFCR {

    /// Transmitted good frames counter
    pub mod TGFC {
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

/// Ethernet MMC received frames with CRC error counter register
pub mod MMCRFCECR {

    /// Received frames with CRC error counter
    pub mod RFCFC {
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

/// Ethernet MMC received frames with alignment error counter register
pub mod MMCRFAECR {

    /// Received frames with alignment error counter
    pub mod RFAEC {
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

/// MMC received good unicast frames counter register
pub mod MMCRGUFCR {

    /// Received good unicast frames counter
    pub mod RGUFC {
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
pub struct RegisterBlock {
    /// Ethernet MMC control register (ETH_MMCCR)
    pub MMCCR: RWRegister<u32>,

    /// Ethernet MMC receive interrupt register (ETH_MMCRIR)
    pub MMCRIR: RWRegister<u32>,

    /// Ethernet MMC transmit interrupt register (ETH_MMCTIR)
    pub MMCTIR: RWRegister<u32>,

    /// Ethernet MMC receive interrupt mask register (ETH_MMCRIMR)
    pub MMCRIMR: RWRegister<u32>,

    /// Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR)
    pub MMCTIMR: RWRegister<u32>,

    _reserved1: [u32; 14],

    /// Ethernet MMC transmitted good frames after a single collision counter
    pub MMCTGFSCCR: RORegister<u32>,

    /// Ethernet MMC transmitted good frames after more than a single collision
    pub MMCTGFMSCCR: RORegister<u32>,

    _reserved2: [u32; 5],

    /// Ethernet MMC transmitted good frames counter register
    pub MMCTGFCR: RORegister<u32>,

    _reserved3: [u32; 10],

    /// Ethernet MMC received frames with CRC error counter register
    pub MMCRFCECR: RORegister<u32>,

    /// Ethernet MMC received frames with alignment error counter register
    pub MMCRFAECR: RORegister<u32>,

    _reserved4: [u32; 10],

    /// MMC received good unicast frames counter register
    pub MMCRGUFCR: RORegister<u32>,
}
pub struct ResetValues {
    pub MMCCR: u32,
    pub MMCRIR: u32,
    pub MMCTIR: u32,
    pub MMCRIMR: u32,
    pub MMCTIMR: u32,
    pub MMCTGFSCCR: u32,
    pub MMCTGFMSCCR: u32,
    pub MMCTGFCR: u32,
    pub MMCRFCECR: u32,
    pub MMCRFAECR: u32,
    pub MMCRGUFCR: u32,
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
