#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Ethernet: MAC management counters

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

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
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
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
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
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
#[repr(C)]
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

    _reserved1: [u8; 56],

    /// Ethernet MMC transmitted good frames after a single collision counter
    pub MMCTGFSCCR: RORegister<u32>,

    /// Ethernet MMC transmitted good frames after more than a single collision
    pub MMCTGFMSCCR: RORegister<u32>,

    _reserved2: [u8; 20],

    /// Ethernet MMC transmitted good frames counter register
    pub MMCTGFCR: RORegister<u32>,

    _reserved3: [u8; 40],

    /// Ethernet MMC received frames with CRC error counter register
    pub MMCRFCECR: RORegister<u32>,

    /// Ethernet MMC received frames with alignment error counter register
    pub MMCRFAECR: RORegister<u32>,

    _reserved4: [u8; 40],

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
#[cfg(feature = "rtic")]
unsafe impl Send for Instance {}

/// Access functions for the Ethernet_MMC peripheral instance
pub mod Ethernet_MMC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40028100,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in Ethernet_MMC
    pub const reset: ResetValues = ResetValues {
        MMCCR: 0x00000000,
        MMCRIR: 0x00000000,
        MMCTIR: 0x00000000,
        MMCRIMR: 0x00000000,
        MMCTIMR: 0x00000000,
        MMCTGFSCCR: 0x00000000,
        MMCTGFMSCCR: 0x00000000,
        MMCTGFCR: 0x00000000,
        MMCRFCECR: 0x00000000,
        MMCRFAECR: 0x00000000,
        MMCRGUFCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut Ethernet_MMC_TAKEN: bool = false;

    /// Safe access to Ethernet_MMC
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
            if Ethernet_MMC_TAKEN {
                None
            } else {
                Ethernet_MMC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to Ethernet_MMC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if Ethernet_MMC_TAKEN && inst.addr == INSTANCE.addr {
                Ethernet_MMC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal Ethernet_MMC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        Ethernet_MMC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to Ethernet_MMC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const Ethernet_MMC: *const RegisterBlock = 0x40028100 as *const _;
