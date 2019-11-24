#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Ethernet: MAC management counters
//!
//! Used by: stm32f745, stm32f765, stm32f7x6, stm32f7x7, stm32f7x9

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Ethernet MMC control register
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
        /// Read-write values
        pub mod RW {

            /// 0b1: Reset all counters. Cleared automatically
            pub const Reset: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Counters roll over to zero after reaching the maximum value
            pub const Disabled: u32 = 0b0;

            /// 0b1: Counters do not roll over to zero after reaching the maximum value
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC counters do not reset on read
            pub const Disabled: u32 = 0b0;

            /// 0b1: MMC counters reset to zero after read
            pub const Enabled: u32 = 0b1;
        }
    }

    /// MMC counter freeze
    pub mod MCF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: All MMC counters update normally
            pub const Unfrozen: u32 = 0b0;

            /// 0b1: All MMC counters frozen to their current value
            pub const Frozen: u32 = 0b1;
        }
    }

    /// MMC counter preset
    pub mod MCP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: MMC counters will be preset to almost full or almost half. Cleared automatically
            pub const Preset: u32 = 0b1;
        }
    }

    /// MMC counter Full-Half preset
    pub mod MCFHP {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: When MCP is set, MMC counters are preset to almost-half value 0x7FFF_FFF0
            pub const AlmostHalf: u32 = 0b0;

            /// 0b1: When MCP is set, MMC counters are preset to almost-full value 0xFFFF_FFF0
            pub const AlmostFull: u32 = 0b1;
        }
    }
}

/// Ethernet MMC receive interrupt register
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

    /// Received good Unicast frames status
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

/// Ethernet MMC transmit interrupt register
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

    /// Transmitted good frames more than single collision status
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

/// Ethernet MMC receive interrupt mask register
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Received-crc-error counter half-full interrupt enabled
            pub const Unmasked: u32 = 0b0;

            /// 0b1: Received-crc-error counter half-full interrupt disabled
            pub const Masked: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Received-alignment-error counter half-full interrupt enabled
            pub const Unmasked: u32 = 0b0;

            /// 0b1: Received-alignment-error counter half-full interrupt disabled
            pub const Masked: u32 = 0b1;
        }
    }

    /// Received good Unicast frames mask
    pub mod RGUFM {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Received-good-unicast counter half-full interrupt enabled
            pub const Unmasked: u32 = 0b0;

            /// 0b1: Received-good-unicast counter half-full interrupt disabled
            pub const Masked: u32 = 0b1;
        }
    }
}

/// Ethernet MMC transmit interrupt mask register
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmitted-good-single-collision half-full interrupt enabled
            pub const Unmasked: u32 = 0b0;

            /// 0b1: Transmitted-good-single-collision half-full interrupt disabled
            pub const Masked: u32 = 0b1;
        }
    }

    /// Transmitted good frames more than single collision mask
    pub mod TGFMSCM {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmitted-good-multiple-collision half-full interrupt enabled
            pub const Unmasked: u32 = 0b0;

            /// 0b1: Transmitted-good-multiple-collision half-full interrupt disabled
            pub const Masked: u32 = 0b1;
        }
    }

    /// Transmitted good frames mask
    pub mod TGFM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmitted-good counter half-full interrupt enabled
            pub const Unmasked: u32 = 0b0;

            /// 0b1: Transmitted-good counter half-full interrupt disabled
            pub const Masked: u32 = 0b1;
        }
    }
}

/// Ethernet MMC transmitted good frames after a single collision counter
pub mod MMCTGFSCCR {

    /// Transmitted good frames single collision counter
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

    /// TGFMSCC
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

    /// HTL
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

    /// RFCFC
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

    /// RFAEC
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

    /// RGUFC
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
    /// Ethernet MMC control register
    pub MMCCR: RWRegister<u32>,

    /// Ethernet MMC receive interrupt register
    pub MMCRIR: RWRegister<u32>,

    /// Ethernet MMC transmit interrupt register
    pub MMCTIR: RORegister<u32>,

    /// Ethernet MMC receive interrupt mask register
    pub MMCRIMR: RWRegister<u32>,

    /// Ethernet MMC transmit interrupt mask register
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
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}
